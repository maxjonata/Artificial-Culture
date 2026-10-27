use crate::components::components_npc::Npc;
use crate::components::components_pathfinding::{PathTarget, SteeringBehavior};
use bevy::prelude::*;

/// Helper function implementing Craig Reynolds' Seek steering behavior
/// Based on Boids algorithm and steering behaviors for autonomous agents
pub fn calculate_seek_force(
    current_position: Vec2,
    target_position: Vec2,
    current_velocity: Vec2,
    max_speed: f32,
    max_force: f32,
) -> Vec2 {
    let desired_velocity = (target_position - current_position).normalize_or_zero() * max_speed;
    let steering_force = desired_velocity - current_velocity;
    steering_force.clamp_length_max(max_force)
}

/// Helper function implementing Wander steering behavior for autonomous movement
/// Based on Craig Reynolds' autonomous agent behaviors for emergent movement patterns
pub fn calculate_wander_force(
    steering_behavior: &mut SteeringBehavior,
    current_velocity: Vec2,
    max_speed: f32,
    max_force: f32,
    wander_radius: f32,
    wander_distance: f32,
    delta_time: f32,
) -> Vec2 {
    // Update wander angle with random variation scaled by delta_time for frame-rate independence
    // This ensures consistent wandering behavior regardless of FPS
    steering_behavior.wander_angle += (rand::random::<f32>() - 0.5) * steering_behavior.wander_angle_change * delta_time;

    // Calculate circle center in front of agent
    let circle_center = current_velocity.normalize_or_zero() * wander_distance;

    // Calculate displacement around circle
    let displacement = Vec2::new(
        steering_behavior.wander_angle.cos() * wander_radius,
        steering_behavior.wander_angle.sin() * wander_radius,
    );

    let wander_target = circle_center + displacement;
    let desired_velocity = wander_target.normalize_or_zero() * max_speed;
    let steering_force = desired_velocity - current_velocity;

    steering_force.clamp_length_max(max_force)
}

/// Helper function to find nearest resource of a specific type
/// Based on Spatial Cognition Theory - agents use spatial memory for resource location
pub fn find_nearest_resource_position(
    agent_position: Vec2,
    resource_positions: &[Vec2],
) -> Option<Vec2> {
    resource_positions
        .iter()
        .min_by(|a, b| {
            let dist_a = agent_position.distance_squared(**a);
            let dist_b = agent_position.distance_squared(**b);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .copied()
}

/// Helper function to check if target has been reached
/// Based on Goal-Oriented Action Planning - determines successful target arrival
pub fn has_reached_target(current_position: Vec2, target: &PathTarget) -> bool {
    if !target.has_target {
        return false;
    }

    current_position.distance(target.target_position) <= target.arrival_threshold
}

/// Helper function to check if pursuit should timeout
/// Based on Behavioral Economics - prevents infinite pursuit of unreachable goals
pub fn should_timeout_pursuit(target: &PathTarget, current_time: f32) -> bool {
    if !target.has_target {
        return false;
    }

    (current_time - target.target_set_time) > target.max_pursuit_time
}

/// Helper function to find the nearest NPC position for social interaction
/// Based on Social Psychology - agents seek proximity to others for social needs
pub fn find_nearest_npc_position(
    self_entity: Entity,
    current_position: Vec2,
    other_npcs_query: &Query<(Entity, &Transform), (With<Npc>, Without<PathTarget>)>,
) -> Option<Vec2> {
    other_npcs_query
        .iter()
        .filter(|(entity, _)| *entity != self_entity) // Don't target self
        .min_by(|(_, transform_a), (_, transform_b)| {
            let pos_a = transform_a.translation.truncate();
            let pos_b = transform_b.translation.truncate();
            let dist_a = current_position.distance(pos_a);
            let dist_b = current_position.distance(pos_b);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, transform)| transform.translation.truncate())
}