use bevy::ecs::event::EventWriter;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::components_constants::GameConstants;
use crate::components::components_environment::{Hotel, ResourceType, Restaurant, SafeZone, Well};
use crate::components::components_needs::Desire;
use crate::components::components_npc::{Npc, RefillState};
use crate::components::components_pathfinding::{PathTarget, ResourceMemory, SteeringBehavior};
use crate::systems::events::events_pathfinding::{PathTargetReachedEvent, PathTargetSetEvent, ResourceDiscoveredEvent};
use crate::utils::helpers::{
    calculate_seek_force, calculate_wander_force, find_nearest_npc_position,
    find_nearest_resource_position, has_reached_target, should_timeout_pursuit,
};


/// System for discovering resources within range and updating NPCs' memory
/// Based on Spatial Cognition Theory - agents use spatial memory for resource location
pub fn resource_discovery_system(
    mut npc_query: Query<(Entity, &Transform, &mut ResourceMemory), With<Npc>>,
    well_query: Query<&Transform, (With<Well>, Without<Npc>)>,
    restaurant_query: Query<&Transform, (With<Restaurant>, Without<Npc>)>,
    hotel_query: Query<&Transform, (With<Hotel>, Without<Npc>)>,
    safe_zone_query: Query<&Transform, (With<SafeZone>, Without<Npc>)>,
    mut discovery_events: EventWriter<ResourceDiscoveredEvent>,
) {
    for (entity, npc_transform, mut memory) in npc_query.iter_mut() {
        let npc_position = npc_transform.translation.truncate();

        // Discover wells within range
        for well_transform in well_query.iter() {
            let well_position = well_transform.translation.truncate();
            if npc_position.distance(well_position) <= memory.discovery_radius {
                if !memory.known_wells.contains(&well_position) {
                    memory.known_wells.push(well_position);
                    discovery_events.write(ResourceDiscoveredEvent {
                        npc_entity: entity,
                        resource_position: well_position,
                        resource_entity: Entity::PLACEHOLDER, // TODO: Get actual well entity
                        resource_type: ResourceType::Water,
                        discovery_distance: npc_position.distance(well_position),
                    });
                }
            }
        }

        // Discover restaurants within range
        for restaurant_transform in restaurant_query.iter() {
            let restaurant_position = restaurant_transform.translation.truncate();
            if npc_position.distance(restaurant_position) <= memory.discovery_radius {
                if !memory.known_restaurants.contains(&restaurant_position) {
                    memory.known_restaurants.push(restaurant_position);
                    discovery_events.write(ResourceDiscoveredEvent {
                        npc_entity: entity,
                        resource_position: restaurant_position,
                        resource_entity: Entity::PLACEHOLDER, // TODO: Get actual restaurant entity
                        resource_type: ResourceType::Food,
                        discovery_distance: npc_position.distance(restaurant_position),
                    });
                }
            }
        }

        // Discover hotels within range
        for hotel_transform in hotel_query.iter() {
            let hotel_position = hotel_transform.translation.truncate();
            if npc_position.distance(hotel_position) <= memory.discovery_radius {
                if !memory.known_hotels.contains(&hotel_position) {
                    memory.known_hotels.push(hotel_position);
                    discovery_events.write(ResourceDiscoveredEvent {
                        npc_entity: entity,
                        resource_position: hotel_position,
                        resource_entity: Entity::PLACEHOLDER, // TODO: Get actual hotel entity
                        resource_type: ResourceType::Rest,
                        discovery_distance: npc_position.distance(hotel_position),
                    });
                }
            }
        }

        // Discover safe zones within range
        for safe_zone_transform in safe_zone_query.iter() {
            let safe_zone_position = safe_zone_transform.translation.truncate();
            if npc_position.distance(safe_zone_position) <= memory.discovery_radius {
                if !memory.known_safe_zones.contains(&safe_zone_position) {
                    memory.known_safe_zones.push(safe_zone_position);
                    discovery_events.write(ResourceDiscoveredEvent {
                        npc_entity: entity,
                        resource_position: safe_zone_position,
                        resource_entity: Entity::PLACEHOLDER, // TODO: Get actual safe zone entity
                        resource_type: ResourceType::Safety,
                        discovery_distance: npc_position.distance(safe_zone_position),
                    });
                }
            }
        }
    }
}

/// System for setting pathfinding targets based on NPCs' desires and known resources
/// Based on Goal-Oriented Action Planning - agents plan paths to satisfy needs
pub fn desire_pathfinding_system(
    mut npc_query: Query<(Entity, &Transform, &Desire, &ResourceMemory, &mut PathTarget), With<Npc>>,
    other_npcs_query: Query<(Entity, &Transform), (With<Npc>, Without<PathTarget>)>,
    mut target_events: EventWriter<PathTargetSetEvent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (entity, transform, desire, memory, mut path_target) in npc_query.iter_mut() {
        let npc_position = transform.translation.truncate();

        // Skip if already has a valid target
        if path_target.has_target && !should_timeout_pursuit(&path_target, current_time) {
            continue;
        }

        // Find appropriate target based on desire using helper function
        let target_position = match *desire {
            Desire::FindWater => find_nearest_resource_position(npc_position, &memory.known_wells),
            Desire::FindFood => find_nearest_resource_position(npc_position, &memory.known_restaurants),
            Desire::Rest => find_nearest_resource_position(npc_position, &memory.known_hotels),
            Desire::FindSafety => find_nearest_resource_position(npc_position, &memory.known_safe_zones),
            Desire::Socialize => {
                // Find nearest other NPC for social interaction
                find_nearest_npc_position(entity, npc_position, &other_npcs_query)
            }
            _ => None, // Wander doesn't have specific targets
        };

        if let Some(target_pos) = target_position {
            path_target.target_position = target_pos;
            path_target.has_target = true;
            path_target.target_set_time = current_time;

            target_events.write(PathTargetSetEvent {
                npc_entity: entity,
                target_position: target_pos,
                target_entity: None, // TODO: Store actual target entity for social interactions
                target_type: match *desire {
                    Desire::FindWater => ResourceType::Water,
                    Desire::FindFood => ResourceType::Food,
                    Desire::Rest => ResourceType::Rest,
                    Desire::FindSafety => ResourceType::Safety,
                    Desire::Socialize => ResourceType::Water, // TODO: Add Social resource type
                    _ => ResourceType::Water, // Default
                },
                distance_to_target: npc_position.distance(target_pos),
            });
        }
    }
}

/// System implementing steering behaviors for autonomous NPC movement
/// Based on Craig Reynolds' Boids algorithm and steering behaviors
/// Now respects RefillState to stop movement during resource interactions
pub fn steering_behavior_system(
    mut npc_query: Query<(Entity, &Transform, &mut Velocity, &mut SteeringBehavior, &PathTarget, &Desire, &RefillState), With<Npc>>,
    game_constants: Res<GameConstants>,
    mut reached_events: EventWriter<PathTargetReachedEvent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (entity, transform, mut velocity, mut steering, path_target, desire, refill_state) in npc_query.iter_mut() {
        let current_position = transform.translation.truncate();
        let current_velocity = velocity.linvel;

        // Stop movement if NPC is refilling
        if refill_state.is_refilling {
            velocity.linvel = Vec2::ZERO;
            steering.steering_force = Vec2::ZERO;
            continue;
        }

        if has_reached_target(current_position, path_target) {
            reached_events.write(PathTargetReachedEvent {
                npc_entity: entity,
                target_position: path_target.target_position,
                target_entity: path_target.target_entity,
                time_to_reach: current_time - path_target.target_set_time,
            });
            continue;
        }

        let mut steering_force = Vec2::ZERO;

        if path_target.has_target && !should_timeout_pursuit(&path_target, current_time) {
            // Calculate seek force towards target using helper
            let seek_force = calculate_seek_force(
                current_position,
                path_target.target_position,
                current_velocity,
                game_constants.npc_speed,
                steering.max_steering_force,
            );
            steering_force += seek_force * steering.seek_weight;
        } else {
            // Store values before mutable borrow to avoid borrow checker issues
            let max_steering_force = steering.max_steering_force;
            let wander_weight = steering.wander_weight;

            // Calculate wander force for exploration using helper
            let wander_force = calculate_wander_force(
                &mut steering,
                current_velocity,
                game_constants.npc_speed,
                max_steering_force,
                50.0, // wander_radius
                100.0, // wander_distance
                time.delta_secs(),
            );
            steering_force += wander_force * wander_weight;
        }

        // Apply steering force to velocity
        steering.steering_force = steering_force;
        velocity.linvel += steering_force * time.delta_secs();
        velocity.linvel = velocity.linvel.clamp_length_max(game_constants.npc_speed);
    }
}
