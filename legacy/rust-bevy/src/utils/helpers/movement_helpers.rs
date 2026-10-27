use crate::components::components_needs::Desire;
use bevy::prelude::*;

/// Helper function implementing boundary physics based on elastic collision theory
/// Based on Physics - elastic collision model for boundary interactions
pub fn calculate_boundary_reflection(velocity: Vec2, normal: Vec2) -> Vec2 {
    velocity - 2.0 * velocity.dot(normal) * normal
}

/// Helper function to extract normalized direction from current velocity
/// Based on Vector Mathematics - provides consistent directional data for ML agents
pub fn get_normalized_direction(velocity: Vec2) -> Vec2 {
    if velocity.length() > 0.1 {
        velocity.normalize()
    } else {
        Vec2::ZERO
    }
}

/// Helper function implementing elastic reflection for boundary collisions
/// Based on Classical Physics - conservation of momentum in elastic collisions
pub fn reflect_velocity_off_boundary(velocity: Vec2, boundary_normal: Vec2) -> Vec2 {
    velocity - 2.0 * velocity.dot(boundary_normal) * boundary_normal
}

/// Helper function for boundary detection with predictive collision system
/// Based on Collision Detection Theory - predicts future collisions for smooth movement
pub fn detect_boundary_collision(
    position: Vec2,
    velocity: Vec2,
    boundary_min: Vec2,
    boundary_max: Vec2,
    entity_radius: f32,
    prediction_time: f32,
) -> Option<Vec2> {
    let future_position = position + velocity * prediction_time;

    let mut collision_normal = Vec2::ZERO;
    let mut has_collision = false;

    // Check X boundaries
    if future_position.x - entity_radius < boundary_min.x {
        collision_normal.x = 1.0;
        has_collision = true;
    } else if future_position.x + entity_radius > boundary_max.x {
        collision_normal.x = -1.0;
        has_collision = true;
    }

    // Check Y boundaries
    if future_position.y - entity_radius < boundary_min.y {
        collision_normal.y = 1.0;
        has_collision = true;
    } else if future_position.y + entity_radius > boundary_max.y {
        collision_normal.y = -1.0;
        has_collision = true;
    }

    if has_collision {
        Some(collision_normal.normalize_or_zero())
    } else {
        None
    }
}

/// Helper function for vector normalization with fallback handling
/// Based on Vector Mathematics - safe normalization preventing division by zero
pub fn safe_normalize(vector: Vec2, fallback: Vec2) -> Vec2 {
    if vector.length() > f32::EPSILON {
        vector.normalize()
    } else {
        fallback
    }
}

/// Helper function implementing desire-driven movement behavior
/// Based on Behavioral Psychology - desires influence movement patterns and speed
pub fn calculate_desire_movement_modifier(desire: &Desire) -> (f32, f32) {
    match desire {
        Desire::FindFood => (1.2, 0.8),      // Faster, more focused when hungry
        Desire::FindWater => (1.4, 0.7),     // Even faster when thirsty (more urgent)
        Desire::Rest => (0.6, 1.2),          // Slower, more wandering when tired
        Desire::FindSafety => (1.8, 0.5),    // Very fast, very focused when threatened
        Desire::Socialize => (0.8, 1.5),     // Slower, more wandering when seeking social interaction
        Desire::Wander => (1.0, 1.0),        // Normal speed and wandering
    }
}

/// Helper function for calculating movement efficiency metrics
/// Based on Behavioral Analysis - quantifies movement patterns for ML observation
pub fn calculate_movement_efficiency(
    distance_traveled: f32,
    direct_distance: f32,
    time_taken: f32,
) -> f32 {
    if direct_distance > 0.0 && time_taken > 0.0 {
        direct_distance / distance_traveled.max(direct_distance)
    } else {
        0.0
    }
}
