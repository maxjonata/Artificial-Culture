use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::components_knowledge::KnowledgeBase;
use crate::components::components_npc::{ApparentState, Npc, PerceivedEntities, Posture, RefillState, VisionRange};

/// System for updating NPC sprites based on rumor knowledge
/// System based on Visual Information Theory - visual cues affect social perception
pub fn color_system(
    mut query: Query<(&KnowledgeBase, &mut Sprite), With<Npc>>,
    asset_server: Res<AssetServer>,
) {
    for (knowledge, mut sprite) in query.iter_mut() {
        if knowledge.knows_rumor {
            // ML-HOOK: Visual state change provides quantifiable feedback for RL agents
            // Change to contaminated person sprite when NPC knows the rumor
            sprite.image = asset_server.load("contamined_person.png");
        }
        // else {
        //     // Use normal person sprite when NPC doesn't know the rumor
        //     sprite.image = asset_server.load("person.png");
        // }
    }
}

/// PERCEPTION SYSTEM: Updates agents' apparent state based on their internal state
/// This system translates internal components into externally observable information
/// Based on Theory of Mind - what others can observe about an agent's behavior
pub fn update_apparent_state_system(
    mut query: Query<(
        &mut ApparentState,
        &Velocity,
        &RefillState,
        Option<&KnowledgeBase>,
    ), With<Npc>>,
) {
    for (mut apparent_state, velocity, refill_state, knowledge) in query.iter_mut() {
        // Update running status based on velocity magnitude
        // Running threshold of 150 units/sec - roughly 75% of default NPC speed
        apparent_state.is_running = velocity.linvel.length() > 150.0;

        // Update interaction status based on refill state
        apparent_state.is_interacting = refill_state.is_refilling;

        // Update posture based on behavior and knowledge
        apparent_state.posture = if refill_state.is_refilling {
            Posture::Neutral // Focused on interaction
        } else if apparent_state.is_running {
            Posture::Alert // Moving with purpose
        } else if let Some(kb) = knowledge {
            if kb.knows_rumor {
                Posture::Defensive // Appears wary if knows contamination rumor
            } else {
                Posture::Friendly // Default friendly posture
            }
        } else {
            Posture::Neutral
        };

        // Note: is_carrying_item would be updated when inventory system is implemented
        // For now, keeping it as false
    }
}

/// PERCEPTION SYSTEM: The ONLY system that can query other entities' state broadly
/// Updates each agent's PerceivedEntities based on vision range and line-of-sight
/// Based on Human Visual Perception research and Cognitive Psychology
/// CRITICAL: Follows "Mantle of Ignorance" - agents only see apparent state, never internal state
/// NOTE: Simplified version without line-of-sight for initial implementation
pub fn vision_system(
    mut observer_query: Query<(
        Entity,
        &Transform,
        &mut PerceivedEntities,
        &VisionRange,
    ), With<Npc>>,
    world_query: Query<(Entity, &Transform, &ApparentState), With<Npc>>,
) {
    for (observer_entity, observer_transform, mut perception, vision_range) in observer_query.iter_mut() {
        // Clear previous perception data
        perception.in_sight.clear();

        let observer_pos = observer_transform.translation.truncate();
        let mut visible_entities = Vec::new();

        // Scan all other entities in the world
        for (other_entity, other_transform, apparent_state) in world_query.iter() {
            // Skip self
            if other_entity == observer_entity {
                continue;
            }

            let other_pos = other_transform.translation.truncate();
            let distance = observer_pos.distance(other_pos);

            // Check if within vision range
            if distance > vision_range.max_distance {
                continue;
            }

            // Check field of view
            let to_target = (other_pos - observer_pos).normalize();
            let observer_forward = observer_transform.rotation * Vec3::Y; // Assuming Y-up forward
            let observer_forward_2d = observer_forward.truncate().normalize();

            let angle_to_target = to_target.dot(observer_forward_2d).acos();
            if angle_to_target > vision_range.field_of_view / 2.0 {
                continue;
            }

            // TODO: Add line-of-sight checking in future version when obstacles are implemented
            // For now, we assume clear line of sight for all entities within range and FOV

            // Entity is visible - add to list with distance for sorting
            visible_entities.push((other_entity, *apparent_state, distance));
        }

        // Sort by distance (closest first) and apply attention limit
        visible_entities.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        // Apply cognitive attention limit (Miller's 7±2 rule)
        let max_entities = perception.attention_limit.min(visible_entities.len());

        for i in 0..max_entities {
            let (entity, apparent_state, _distance) = visible_entities[i];
            perception.in_sight.push((entity, apparent_state));
        }

        // ML-HOOK: Perception data is now quantifiable for RL agents
        // Number of perceived entities and their states can be used as observation space
    }
}
