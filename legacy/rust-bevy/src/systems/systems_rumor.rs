use bevy::ecs::event::{EventReader, EventWriter};
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::components::components_constants::{GameConstants, RumorTimer};
use crate::components::components_knowledge::KnowledgeBase;
use crate::components::components_npc::{Npc, Personality};
use crate::systems::events::events_rumor::{RumorInjectionEvent, RumorSpreadAttemptEvent, RumorSpreadEvent};
use crate::utils::helpers::{
    calculate_rumor_decay, calculate_rumor_transmission_probability,
    should_inject_rumor, should_rumor_spread,
};

/// System that periodically injects new rumors into the simulation
/// Based on Social Psychology - certain individuals are more prone to starting rumors
pub fn rumor_injection_system(
    mut query: Query<(Entity, &Personality, &mut KnowledgeBase), With<Npc>>,
    mut rumor_timer: ResMut<RumorTimer>,
    time: Res<Time>,
    mut injection_events: EventWriter<RumorInjectionEvent>,
) {
    rumor_timer.0.tick(time.delta());

    if rumor_timer.0.just_finished() {
        for (entity, personality, mut knowledge) in query.iter_mut() {
            // Use helper function to determine if this NPC should inject a rumor
            if should_inject_rumor(personality, 0.3) {
                let rumor_content = format!("Rumor_{}", rand::random::<u32>() % 1000);
                knowledge.known_rumors.insert(rumor_content.clone(), 1.0);

                injection_events.write(RumorInjectionEvent {
                    entity,
                    target_entity: entity, // Same as entity for compatibility
                    rumor_content,
                    initial_belief: 1.0,
                    injection_time: time.elapsed_secs(),
                });

                break; // Only one rumor per cycle
            }
        }
    }
}

/// System that detects social interactions and attempts rumor transmission
/// Based on Social Influence Theory - rumors spread through social contact
/// **Single Responsibility:** Only handles interaction detection and attempt creation
pub fn rumor_interaction_detection_system(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(&Personality, &KnowledgeBase), With<Npc>>,
    mut attempt_events: EventWriter<RumorSpreadAttemptEvent>,
    game_constants: Res<GameConstants>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            if let Ok([(personality1, knowledge1), (personality2, knowledge2)]) =
                query.get_many([*entity1, *entity2])
            {
                // Check rumors from entity1 to entity2
                process_rumor_transmission_attempts(
                    *entity1, *entity2,
                    personality1, personality2,
                    knowledge1, knowledge2,
                    &game_constants,
                    &mut attempt_events,
                );

                // Check rumors from entity2 to entity1
                process_rumor_transmission_attempts(
                    *entity2, *entity1,
                    personality2, personality1,
                    knowledge2, knowledge1,
                    &game_constants,
                    &mut attempt_events,
                );
            }
        }
    }
}

/// Helper function to process rumor transmission attempts between two NPCs
/// **Single Responsibility:** Only calculates transmission probability and fires attempt events
fn process_rumor_transmission_attempts(
    sender: Entity,
    receiver: Entity,
    sender_personality: &Personality,
    receiver_personality: &Personality,
    sender_knowledge: &KnowledgeBase,
    receiver_knowledge: &KnowledgeBase,
    game_constants: &GameConstants,
    attempt_events: &mut EventWriter<RumorSpreadAttemptEvent>,
) {
    for (rumor_content, &belief_strength) in sender_knowledge.known_rumors.iter() {
        if !receiver_knowledge.known_rumors.contains_key(rumor_content) {
            let transmission_prob = calculate_rumor_transmission_probability(
                sender_personality,
                receiver_personality,
                game_constants.social_distance,
                0.5, // base transmission rate
            );

            attempt_events.write(RumorSpreadAttemptEvent {
                sender,
                receiver,
                spreader_entity: sender, // Compatibility alias
                receiver_entity: receiver, // Compatibility alias
                rumor_content: rumor_content.clone(),
                transmission_probability: transmission_prob,
                success: false, // Will be determined later
                calculated_probability: transmission_prob, // Same as transmission_probability
            });
        }
    }
}

/// Event-driven system that processes rumor transmission attempts
/// **Single Responsibility:** Only handles the actual rumor spread based on attempts
/// Much more performant than polling - only runs when attempts are made
pub fn rumor_transmission_system(
    mut attempt_events: EventReader<RumorSpreadAttemptEvent>,
    mut query: Query<&mut KnowledgeBase, With<Npc>>,
    mut spread_events: EventWriter<RumorSpreadEvent>,
) {
    for attempt in attempt_events.read() {
        if should_rumor_spread(attempt.transmission_probability) {
            if let Ok([sender_knowledge, mut receiver_knowledge]) =
                query.get_many_mut([attempt.sender, attempt.receiver])
            {
                if let Some(&belief_strength) = sender_knowledge.known_rumors.get(&attempt.rumor_content) {
                    let new_belief = belief_strength * 0.8; // Slight decay in transmission
                    receiver_knowledge.known_rumors.insert(attempt.rumor_content.clone(), new_belief);

                    spread_events.write(RumorSpreadEvent {
                        sender: attempt.sender,
                        receiver: attempt.receiver,
                        spreader_entity: attempt.sender, // Compatibility alias
                        receiver_entity: attempt.receiver, // Compatibility alias
                        rumor_content: attempt.rumor_content.clone(),
                        belief_strength: new_belief,
                        spreader_openness: 0.5, // Default value - could be retrieved from personality
                        receiver_openness: 0.5, // Default value - could be retrieved from personality
                        spread_probability: attempt.transmission_probability,
                    });
                }
            }
        }
    }
}

/// System that applies rumor decay over time
/// Based on Information Diffusion Theory - rumors lose credibility over time
/// **Single Responsibility:** Only handles rumor decay, nothing else
pub fn rumor_decay_system(
    mut query: Query<&mut KnowledgeBase, With<Npc>>,
    time: Res<Time>,
) {
    const DECAY_RATE: f32 = 0.01; // 1% decay per second

    for mut knowledge in query.iter_mut() {
        let mut rumors_to_remove = Vec::new();

        for (rumor_content, belief_strength) in knowledge.known_rumors.iter_mut() {
            // Use helper function to calculate decay
            let new_belief = calculate_rumor_decay(*belief_strength, DECAY_RATE, time.delta_secs());
            *belief_strength = new_belief;

            // Remove rumors that have decayed below threshold
            if new_belief < 0.1 {
                rumors_to_remove.push(rumor_content.clone());
            }
        }

        // Remove decayed rumors
        for rumor_content in rumors_to_remove {
            knowledge.known_rumors.remove(&rumor_content);
        }
    }
}