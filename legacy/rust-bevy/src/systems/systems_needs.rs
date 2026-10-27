use crate::components::components_needs::{BasicNeeds, CurrentDesire, Desire, DesireThresholds};
use crate::components::components_pathfinding::PathTarget;
use crate::components::{components_constants::GameConstants, components_npc::{Npc, RefillState}};
use crate::systems::events::events_needs::{
    ActionCompleted, ActionCompletionReason, CurrentDesireSet, DecisionTrigger, DesireChangeEvent, DesireChangeReason,
    DesireFulfillmentAttemptEvent, EvaluateDecision, NeedChangeEvent, NeedDecayEvent,
    NeedSatisfactionEvent, NeedType, SocialInteractionEvent, ThresholdCrossedEvent, ThresholdDirection,
};
use crate::utils::helpers::needs_helpers::{
    calculate_desire_utility, decay_needs, evaluate_most_urgent_desire, get_satisfaction_level,
    increase_social_satisfaction, should_activate_desire, should_deactivate_desire,
};
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

/// System implementing homeostatic need decay over time
/// System based on Homeostatic Drive Theory - maintains internal physiological balance
/// Now fires NeedChangeEvent for event-driven threshold monitoring
/// FIXED: All needs now use "higher = better satisfied" semantics
pub fn decay_basic_needs(
    mut query: Query<(Entity, &mut BasicNeeds), With<Npc>>,
    game_constants: Res<GameConstants>,
    mut need_decay_events: EventWriter<NeedDecayEvent>,
    mut need_change_events: EventWriter<NeedChangeEvent>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (entity, mut needs) in query.iter_mut() {
        let old_needs = *needs; // Capture old values for event firing

        let (hunger_change, thirst_change, rest_change, safety_change, social_change) =
            decay_needs(&mut needs, &game_constants, delta_time);

        // Fire individual need change events for threshold monitoring
        if hunger_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Hunger,
                old_value: old_needs.hunger,
                new_value: needs.hunger,
                change_amount: hunger_change,
            });
        }

        if thirst_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Thirst,
                old_value: old_needs.thirst,
                new_value: needs.thirst,
                change_amount: thirst_change,
            });
        }

        if rest_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Rest,
                old_value: old_needs.rest,
                new_value: needs.rest,
                change_amount: rest_change,
            });
        }

        if safety_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Safety,
                old_value: old_needs.safety,
                new_value: needs.safety,
                change_amount: safety_change,
            });
        }

        if social_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Social,
                old_value: old_needs.social,
                new_value: needs.social,
                change_amount: social_change,
            });
        }

        // ML-HOOK: Fire event for quantifiable state change tracking
        need_decay_events.write(NeedDecayEvent {
            entity,
            hunger_change,
            thirst_change,
            rest_change,
            safety_change,
            social_change,
        });
    }
}

/// Event-driven system that handles social interactions based on Social Exchange Theory
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
/// Only triggers when collision events occur, not on every frame
/// FIXED: Now uses correct social field and function name
pub fn handle_social_interactions(
    mut collision_events: EventReader<CollisionEvent>,
    mut social_events: EventWriter<SocialInteractionEvent>,
    mut need_change_events: EventWriter<NeedChangeEvent>,
    mut needs_query: Query<&mut BasicNeeds, With<Npc>>,
) {
    const SOCIAL_INTERACTION_BOOST: f32 = 0.1; // Normalized boost for 0.0-1.0 scale

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to get both entities' BasicNeeds components
            if let Ok([mut needs1, mut needs2]) = needs_query.get_many_mut([*entity1, *entity2]) {
                let old_social_1 = needs1.social;
                let old_social_2 = needs2.social;

                // Both NPCs gain social satisfaction from the interaction using helper
                let boost1 = increase_social_satisfaction(&mut needs1, SOCIAL_INTERACTION_BOOST);
                let boost2 = increase_social_satisfaction(&mut needs2, SOCIAL_INTERACTION_BOOST);

                // Fire individual need change events for threshold monitoring
                if boost1 > 0.0 {
                    need_change_events.write(NeedChangeEvent {
                        entity: *entity1,
                        need_type: NeedType::Social,
                        old_value: old_social_1,
                        new_value: needs1.social,
                        change_amount: boost1,
                    });
                }

                if boost2 > 0.0 {
                    need_change_events.write(NeedChangeEvent {
                        entity: *entity2,
                        need_type: NeedType::Social,
                        old_value: old_social_2,
                        new_value: needs2.social,
                        change_amount: boost2,
                    });
                }

                // ML-HOOK: Fire events for quantifiable interaction tracking
                social_events.write(SocialInteractionEvent {
                    entity_1: *entity1,
                    entity_2: *entity2,
                    social_boost: (boost1 + boost2) / 2.0, // Average boost for symmetric interaction
                });
            }
        }
    }
}

/// Event-driven system that handles desire fulfillment attempts
/// Triggers when NPCs with specific desires attempt to satisfy them
/// Much more performant than polling all NPCs every frame
pub fn desire_fulfillment_system(
    mut desire_events: EventReader<DesireChangeEvent>,
    mut fulfillment_events: EventWriter<DesireFulfillmentAttemptEvent>,
    mut satisfaction_events: EventWriter<NeedSatisfactionEvent>,
    mut need_change_events: EventWriter<NeedChangeEvent>,
    mut needs_query: Query<&mut BasicNeeds>,
) {
    for event in desire_events.read() {
        // Only process desires that indicate seeking behavior
        if matches!(event.new_desire, Desire::FindFood | Desire::FindWater | Desire::Rest | Desire::FindSafety)
            && let Ok(mut needs) = needs_query.get_mut(event.entity)
        {
            // Simulate resource interaction based on desire type
            let (need_type, satisfaction_amount, success) = match event.new_desire {
                Desire::FindFood => {
                    let old_hunger = needs.hunger;
                    let boost = 0.4;
                    needs.hunger = (needs.hunger + boost).clamp(0.0, 1.0);
                    let actual_boost = needs.hunger - old_hunger;

                    if actual_boost > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Hunger,
                            old_value: old_hunger,
                            new_value: needs.hunger,
                            change_amount: actual_boost,
                        });
                    }

                    info!("NPC found food! Hunger increased to {:.2}", needs.hunger);
                    (NeedType::Hunger, actual_boost, true)
                }
                Desire::FindWater => {
                    let old_thirst = needs.thirst;
                    let boost = 0.5;
                    needs.thirst = (needs.thirst + boost).clamp(0.0, 1.0);
                    let actual_boost = needs.thirst - old_thirst;

                    if actual_boost > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Thirst,
                            old_value: old_thirst,
                            new_value: needs.thirst,
                            change_amount: actual_boost,
                        });
                    }

                    info!("NPC found water! Thirst increased to {:.2}", needs.thirst);
                    (NeedType::Thirst, actual_boost, true)
                }
                Desire::Rest => {
                    let old_rest = needs.rest;
                    let recovery = 0.3;
                    needs.rest = (needs.rest + recovery).clamp(0.0, 1.0);
                    let actual_recovery = needs.rest - old_rest;

                    if actual_recovery > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Rest,
                            old_value: old_rest,
                            new_value: needs.rest,
                            change_amount: actual_recovery,
                        });
                    }

                    info!("NPC is resting! Rest level increased to {:.2}", needs.rest);
                    (NeedType::Rest, actual_recovery, true)
                }
                Desire::FindSafety => {
                    let old_safety = needs.safety;
                    let boost = 0.35;
                    needs.safety = (needs.safety + boost).clamp(0.0, 1.0);
                    let actual_boost = needs.safety - old_safety;

                    if actual_boost > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Safety,
                            old_value: old_safety,
                            new_value: needs.safety,
                            change_amount: actual_boost,
                        });
                    }

                    info!("NPC found safety! Safety increased to {:.2}", needs.safety);
                    (NeedType::Safety, actual_boost, true)
                }
                _ => return, // Should not happen given our filter above
            };

            // Fire events for ML tracking
            fulfillment_events.write(DesireFulfillmentAttemptEvent {
                entity: event.entity,
                desire: event.new_desire,
                success,
                satisfaction_gained: satisfaction_amount,
            });

            if success && satisfaction_amount > 0.0 {
                satisfaction_events.write(NeedSatisfactionEvent {
                    entity: event.entity,
                    need_type,
                    satisfaction_amount,
                    resource_entity: None, // For now, no specific resource entities
                });
            }
        }
    }
}

/// Event-driven system that fires threshold crossing events when needs change
/// This replaces the polling-based threshold checking for better performance
/// Now uses DualThreshold structure with high/low thresholds for hysteresis
/// FIXED: Updated to use new field names and correct threshold logic
pub fn threshold_monitoring_system(
    mut need_change_events: EventReader<NeedChangeEvent>,
    mut threshold_events: EventWriter<ThresholdCrossedEvent>,
    thresholds_query: Query<&DesireThresholds>,
) {
    for event in need_change_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let Ok(thresholds) = thresholds_query.get(event.entity) {
            let dual_threshold = match event.need_type {
                NeedType::Hunger => &thresholds.hunger_threshold,
                NeedType::Thirst => &thresholds.thirst_threshold,
                NeedType::Rest => &thresholds.rest_threshold,
                NeedType::Safety => &thresholds.safety_threshold,
                NeedType::Social => &thresholds.social_threshold,
            };

            // NEW LOGIC: For "higher = better" semantics
            // - Activate desires when satisfaction drops BELOW high_threshold
            // - Start pathfinding when satisfaction drops BELOW low_threshold
            // - Deactivate when satisfaction rises ABOVE high_threshold
            let old_below_high = event.old_value < dual_threshold.high_threshold;
            let new_below_high = event.new_value < dual_threshold.high_threshold;
            let old_below_low = event.old_value < dual_threshold.low_threshold;
            let new_below_low = event.new_value < dual_threshold.low_threshold;

            // Fire events when crossing BELOW high threshold (desire activation)
            if !old_below_high && new_below_high {
                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value: dual_threshold.high_threshold,
                    current_value: event.new_value,
                    crossed_direction: ThresholdDirection::Below,
                    should_trigger_desire: true,
                });
            }

            // Fire events when crossing ABOVE high threshold (desire deactivation)
            if old_below_high && !new_below_high {
                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value: dual_threshold.high_threshold,
                    current_value: event.new_value,
                    crossed_direction: ThresholdDirection::Above,
                    should_trigger_desire: false,
                });
            }

            // Fire events when crossing BELOW low threshold (urgent pathfinding)
            if !old_below_low && new_below_low {
                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value: dual_threshold.low_threshold,
                    current_value: event.new_value,
                    crossed_direction: ThresholdDirection::Below,
                    should_trigger_desire: true, // Even more urgent
                });
            }
        }
    }
}

/// Event-driven system that updates desires based on threshold crossing events
/// Much more performant than polling all NPCs every frame
/// Now uses weighted utility formula and dual threshold logic
/// FIXED: Updated to use correct field names for new need types
pub fn desire_update_system(
    mut threshold_events: EventReader<ThresholdCrossedEvent>,
    mut desire_events: EventWriter<DesireChangeEvent>,
    needs_query: Query<&BasicNeeds>,
    thresholds_query: Query<&DesireThresholds>,
    mut desires_query: Query<&mut Desire>,
) {
    for event in threshold_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let (Ok(needs), Ok(thresholds), Ok(mut current_desire)) = (
            needs_query.get(event.entity),
            thresholds_query.get(event.entity),
            desires_query.get_mut(event.entity)
        ) {
            if event.should_trigger_desire {
                // Determine new desire based on need type
                let new_desire = match event.need_type {
                    NeedType::Hunger => Desire::FindFood,
                    NeedType::Thirst => Desire::FindWater,
                    NeedType::Rest => Desire::Rest,
                    NeedType::Safety => Desire::FindSafety,
                    NeedType::Social => Desire::Socialize,
                };

                // Calculate weighted utility score using the new formula
                let utility = calculate_desire_utility(new_desire, &needs, &thresholds);

                if *current_desire != new_desire {
                    info!("NPC desire changed from {:?} to {:?} due to {:?} threshold crossing",
                          *current_desire, new_desire, event.need_type);

                    desire_events.write(DesireChangeEvent {
                        entity: event.entity,
                        old_desire: *current_desire,
                        new_desire,
                        urgency_score: utility,
                        trigger_reason: DesireChangeReason::ThresholdCrossed,
                    });

                    *current_desire = new_desire;
                }
            } else {
                // Use the new helper functions to check if we should deactivate the current desire
                let should_deactivate = should_deactivate_desire(*current_desire, &needs, &thresholds);

                if should_deactivate {
                    desire_events.write(DesireChangeEvent {
                        entity: event.entity,
                        old_desire: *current_desire,
                        new_desire: Desire::Wander,
                        urgency_score: 0.0,
                        trigger_reason: DesireChangeReason::NeedSatisfied,
                    });

                    *current_desire = Desire::Wander;
                }
            }
        }
    }
}

/// System that logs current status of NPCs for debugging
/// Can be disabled in production builds
/// FIXED: Updated to use correct field names
pub fn debug_npc_status(
    query: Query<(&BasicNeeds, &Desire), With<Npc>>,
    mut last_debug_time: Local<f32>,
    time: Res<Time>,
) {
    // Log status every 5 seconds to avoid spam
    *last_debug_time += time.delta_secs();
    if *last_debug_time >= 5.0 {
        *last_debug_time = 0.0;
        for (needs, desire) in query.iter() {
            debug!(
                "NPC Status - Desire: {:?}, Hunger: {:.2}, Thirst: {:.2}, Rest: {:.2}, Safety: {:.2}, Social: {:.2}",
                desire, needs.hunger, needs.thirst, needs.rest, needs.safety, needs.social
            );
        }
    }
}

/// The missing decision_making_system from roadmap 1.3.2
/// Event-driven system that evaluates all competing desires and selects the highest utility one
/// Triggered by EvaluateDecision events for better performance than polling
/// Uses the existing evaluate_most_urgent_desire helper function for proper decision-making
pub fn decision_making_system(
    mut evaluation_events: EventReader<EvaluateDecision>,
    mut current_desire_events: EventWriter<CurrentDesireSet>,
    mut desire_change_events: EventWriter<DesireChangeEvent>,
    needs_query: Query<&BasicNeeds>,
    thresholds_query: Query<&DesireThresholds>,
    mut current_desires_query: Query<&mut CurrentDesire>,
    time: Res<Time>,
) {
    for event in evaluation_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let (Ok(needs), Ok(thresholds), Ok(mut current_desire)) = (
            needs_query.get(event.entity),
            thresholds_query.get(event.entity),
            current_desires_query.get_mut(event.entity)
        ) {
            // Use the existing helper function that evaluates ALL competing desires
            let (best_desire, utility_score) = evaluate_most_urgent_desire(&needs, &thresholds);

            // ML-HOOK: Calculate utility for all desires for observation space
            let competing_desires = vec![
                (Desire::FindSafety, calculate_desire_utility(Desire::FindSafety, &needs, &thresholds)),
                (Desire::FindWater, calculate_desire_utility(Desire::FindWater, &needs, &thresholds)),
                (Desire::FindFood, calculate_desire_utility(Desire::FindFood, &needs, &thresholds)),
                (Desire::Rest, calculate_desire_utility(Desire::Rest, &needs, &thresholds)),
                (Desire::Socialize, calculate_desire_utility(Desire::Socialize, &needs, &thresholds)),
            ];

            // Only update if the desire actually changed
            if current_desire.desire != best_desire {
                let old_desire = current_desire.desire;

                // Update the CurrentDesire component
                current_desire.desire = best_desire;
                current_desire.utility_score = utility_score;
                current_desire.last_evaluated = time.elapsed_secs();

                // Fire events for system communication and ML tracking
                current_desire_events.write(CurrentDesireSet {
                    entity: event.entity,
                    desire: best_desire,
                    utility_score,
                    competing_desires: competing_desires.clone(),
                });

                desire_change_events.write(DesireChangeEvent {
                    entity: event.entity,
                    old_desire,
                    new_desire: best_desire,
                    urgency_score: utility_score,
                    trigger_reason: match event.trigger_reason {
                        DecisionTrigger::NeedChanged => DesireChangeReason::ThresholdCrossed,
                        _ => DesireChangeReason::ManualOverride,
                    },
                });

                info!("Decision made for NPC: {:?} -> {:?} (utility: {:.2})",
                      old_desire, best_desire, utility_score);
            }
        }
    }
}

/// System that triggers periodic decision re-evaluation
/// Replaces the need to poll all NPCs every frame by firing EvaluateDecision events
/// Based on bounded rationality theory - agents don't constantly re-evaluate
pub fn periodic_decision_trigger_system(
    query: Query<Entity, With<CurrentDesire>>,
    mut evaluation_events: EventWriter<EvaluateDecision>,
    mut last_evaluation: Local<f32>,
    time: Res<Time>,
) {
    const EVALUATION_INTERVAL: f32 = 2.0; // Re-evaluate every 2 seconds

    *last_evaluation += time.delta_secs();
    if *last_evaluation >= EVALUATION_INTERVAL {
        *last_evaluation = 0.0;

        // Trigger evaluation for all agents
        for entity in query.iter() {
            evaluation_events.write(EvaluateDecision {
                entity,
                trigger_reason: DecisionTrigger::Periodic,
            });
        }
    }
}

/// Optimized threshold monitoring system that triggers decision evaluation
/// Instead of directly setting desires, it triggers the decision_making_system
/// This allows for proper utility comparison between all competing desires
pub fn optimized_threshold_monitoring_system(
    mut need_change_events: EventReader<NeedChangeEvent>,
    mut threshold_events: EventWriter<ThresholdCrossedEvent>,
    mut evaluation_events: EventWriter<EvaluateDecision>,
    thresholds_query: Query<&DesireThresholds>,
) {
    for event in need_change_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let Ok(thresholds) = thresholds_query.get(event.entity) {
            let dual_threshold = match event.need_type {
                NeedType::Hunger => &thresholds.hunger_threshold,
                NeedType::Thirst => &thresholds.thirst_threshold,
                NeedType::Rest => &thresholds.rest_threshold,
                NeedType::Safety => &thresholds.safety_threshold,
                NeedType::Social => &thresholds.social_threshold,
            };

            // Check if significant threshold crossings occurred
            let old_below_high = event.old_value < dual_threshold.high_threshold;
            let new_below_high = event.new_value < dual_threshold.high_threshold;
            let old_below_low = event.old_value < dual_threshold.low_threshold;
            let new_below_low = event.new_value < dual_threshold.low_threshold;

            let significant_change = old_below_high != new_below_high || old_below_low != new_below_low;

            if significant_change {
                // Fire threshold crossed event for logging/debugging
                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value: dual_threshold.high_threshold,
                    current_value: event.new_value,
                    crossed_direction: if new_below_high {
                        ThresholdDirection::Below
                    } else {
                        ThresholdDirection::Above
                    },
                    should_trigger_desire: new_below_high,
                });

                // Trigger decision evaluation instead of directly setting desires
                evaluation_events.write(EvaluateDecision {
                    entity: event.entity,
                    trigger_reason: DecisionTrigger::NeedChanged,
                });
            }
        }
    }
}

/// NEW: Action Failure Handling System (1.3.3+)
/// Makes characters look for alternative ways to fulfill desires or switch to different desires
/// Based on Adaptive Goal Management and Cognitive Flexibility research
pub fn action_failure_handling_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CurrentDesire, &BasicNeeds, &DesireThresholds, &PathTarget, &RefillState)>,
    mut action_completed_events: EventWriter<ActionCompleted>,
    mut evaluation_events: EventWriter<EvaluateDecision>,
    game_constants: Res<GameConstants>,
    time: Res<Time>,
) {
    // Use scientifically-grounded constants from GameConstants instead of hardcoded values
    let max_failure_count = game_constants.max_failure_attempts;
    let default_timeout = game_constants.default_action_timeout;
    let stuck_distance_threshold = game_constants.stuck_distance_threshold;
    let timeout_multiplier = game_constants.timeout_retry_multiplier;

    for (entity, mut current_desire, needs, thresholds, path_target, refill_state) in query.iter_mut() {
        let current_time = time.elapsed_secs();

        // Initialize timeout duration if not set
        if current_desire.timeout_duration <= 0.0 {
            current_desire.timeout_duration = default_timeout;
        }

        // Initialize attempt start time if not set
        if current_desire.attempt_start_time <= 0.0 {
            current_desire.attempt_start_time = current_time;
        }

        let attempt_duration = current_time - current_desire.attempt_start_time;
        let mut should_handle_failure = false;
        let mut failure_reason = ActionCompletionReason::Failed;

        // Check for various failure conditions based on cognitive psychology research

        // 1. TIMEOUT: Desire has been active too long without success
        // Based on attention span research (Posner & Petersen, 1990)
        if attempt_duration > current_desire.timeout_duration {
            should_handle_failure = true;
            failure_reason = ActionCompletionReason::Timeout;
            info!("NPC {:?} timed out on desire {:?} after {:.1}s", entity, current_desire.desire, attempt_duration);
        }

        // 2. STUCK: Has a target but hasn't reached it and isn't making progress
        // Based on spatial cognition research - targets beyond 25% of vision range are "far"
        else if let Some(target_entity) = path_target.target_entity {
            if path_target.has_target && !refill_state.is_refilling {
                let distance_to_target = path_target.target_position.distance(Vec2::ZERO); // Simplified check
                if distance_to_target > stuck_distance_threshold {
                    should_handle_failure = true;
                    failure_reason = ActionCompletionReason::Failed;
                    info!("NPC {:?} appears stuck trying to reach target {:?}", entity, target_entity);
                }
            }
        }

        // 3. TARGET LOST: Had a target but it no longer exists or is unreachable
        else if current_desire.last_target.is_some() && !path_target.has_target && !refill_state.is_refilling {
            // Lost target and not currently pathing to a new one
            should_handle_failure = true;
            failure_reason = ActionCompletionReason::Failed;
            info!("NPC {:?} lost target for desire {:?}", entity, current_desire.desire);
        }

        if should_handle_failure {
            current_desire.failure_count += 1;

            // Send ActionCompleted event to track the failure (ML-HOOK)
            action_completed_events.write(ActionCompleted {
                entity,
                completed_desire: current_desire.desire,
                completion_reason: failure_reason,
                duration: attempt_duration,
                success: false,
            });

            // ADAPTIVE STRATEGY: Based on Cognitive Flexibility research (Miyake et al., 2000)
            if current_desire.failure_count >= max_failure_count {
                // Too many failures - switch to a different desire or fallback to wandering
                info!("NPC {:?} giving up on {:?} after {} failures, switching desires",
                      entity, current_desire.desire, current_desire.failure_count);

                // Find the next most urgent desire or fall back to wandering
                let fallback_desire = find_alternative_desire(current_desire.desire, needs, thresholds);

                // Reset for new desire
                current_desire.desire = fallback_desire;
                current_desire.failure_count = 0;
                current_desire.attempt_start_time = current_time;
                current_desire.last_target = None;
                current_desire.timeout_duration = default_timeout;

                // Clear current pathfinding target to force new target search
                commands.entity(entity).insert(PathTarget {
                    has_target: false,
                    target_entity: None,
                    target_position: Vec2::ZERO,
                    arrival_threshold: 30.0,
                    target_set_time: 0.0,
                    max_pursuit_time: 10.0,
                });

                // Trigger immediate re-evaluation for the new desire
                evaluation_events.write(EvaluateDecision {
                    entity,
                    trigger_reason: DecisionTrigger::Forced,
                });
            } else {
                // Try again with the same desire but look for alternative targets
                // Based on adaptive patience research (Anderson & Lebiere, 1998)
                info!("NPC {:?} retrying {:?} (attempt {} of {})",
                      entity, current_desire.desire, current_desire.failure_count + 1, max_failure_count);

                // Reset attempt timing but keep the same desire
                current_desire.attempt_start_time = current_time;
                current_desire.last_target = None; // Clear last target to force new search

                // Increase timeout using adaptive patience multiplier
                current_desire.timeout_duration *= timeout_multiplier;

                // Clear current pathfinding target to force new target search
                commands.entity(entity).insert(PathTarget {
                    has_target: false,
                    target_entity: None,
                    target_position: Vec2::ZERO,
                    arrival_threshold: 30.0,
                    target_set_time: 0.0,
                    max_pursuit_time: 10.0,
                });

                // Trigger resource discovery to find alternative targets
                evaluation_events.write(EvaluateDecision {
                    entity,
                    trigger_reason: DecisionTrigger::Forced,
                });
            }
        }
    }
}

/// Helper function to find an alternative desire when the current one repeatedly fails
/// Based on Cognitive Flexibility and Goal Hierarchy research
fn find_alternative_desire(failed_desire: Desire, needs: &BasicNeeds, thresholds: &DesireThresholds) -> Desire {
    use crate::utils::helpers::needs_helpers::evaluate_most_urgent_desire;

    // Get the most urgent desire based on current needs
    let (most_urgent, _utility) = evaluate_most_urgent_desire(needs, thresholds);

    // If the most urgent desire is the same one that failed, find the second most urgent
    if most_urgent == failed_desire {
        // Manually check other desires in priority order
        let mut alternatives = vec![
            (Desire::FindWater, needs.thirst),
            (Desire::FindFood, needs.hunger),
            (Desire::FindSafety, needs.safety),
            (Desire::Rest, needs.rest),
            (Desire::Socialize, needs.social),
        ];

        // Sort by need level (lower need = higher priority)
        alternatives.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Find the first alternative that isn't the failed desire
        for (desire, need_level) in alternatives {
            if desire != failed_desire {
                // Check if this desire should be active based on thresholds
                let should_activate = match desire {
                    Desire::FindWater => need_level < thresholds.thirst_threshold.high_threshold,
                    Desire::FindFood => need_level < thresholds.hunger_threshold.high_threshold,
                    Desire::FindSafety => need_level < thresholds.safety_threshold.high_threshold,
                    Desire::Rest => need_level < thresholds.rest_threshold.high_threshold,
                    Desire::Socialize => need_level < thresholds.social_threshold.high_threshold,
                    Desire::Wander => true, // Always available as fallback
                };

                if should_activate {
                    return desire;
                }
            }
        }
    } else {
        // The most urgent desire is different from the failed one, use it
        return most_urgent;
    }

    // Ultimate fallback
    Desire::Wander
}
