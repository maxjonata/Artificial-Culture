use bevy::ecs::event::EventWriter;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::components::{components_constants::GameConstants, components_npc::Npc, Desire};
use crate::systems::events::events_movement::{BoundaryCollisionEvent, MovementBehaviorEvent};
use crate::utils::helpers::{
    calculate_boundary_reflection, calculate_movement_efficiency,
    detect_boundary_collision, get_normalized_direction, reflect_velocity_off_boundary,
    safe_normalize,
};

/// System for detecting and handling boundary collisions
/// **Single Responsibility:** Only handles boundary physics and collision detection
/// Based on Classical Physics - elastic collision model for boundary interactions
pub fn boundary_collision_system(
    mut query: Query<(Entity, &mut Transform, &mut Velocity), With<Npc>>,
    game_constants: Res<GameConstants>,
    windows: Query<&Window>,
    mut boundary_events: EventWriter<BoundaryCollisionEvent>,
) {
    let Ok(window) = windows.single() else {
        return; // Exit early if no window found
    };

    let bounds = Vec2::new(
        window.width() / 2.0 - game_constants.npc_radius,
        window.height() / 2.0 - game_constants.npc_radius,
    );

    for (entity, mut transform, mut velocity) in query.iter_mut() {
        let position = transform.translation.truncate();
        let current_direction = get_normalized_direction(velocity.linvel);

        if let Some(collision_normal) = detect_boundary_collision(
            position,
            velocity.linvel,
            -bounds,
            bounds,
            game_constants.npc_radius,
            0.1, // prediction time
        ) {
            // Reflect velocity using helper function
            let old_direction = current_direction;
            velocity.linvel = reflect_velocity_off_boundary(velocity.linvel, collision_normal);

            // Ensure NPC stays within boundaries
            let clamped_position = position.clamp(
                -bounds + Vec2::splat(game_constants.npc_radius),
                bounds - Vec2::splat(game_constants.npc_radius),
            );
            transform.translation = clamped_position.extend(transform.translation.z);

            // ML-HOOK: Fire event for quantifiable boundary collision tracking
            boundary_events.write(BoundaryCollisionEvent {
                entity,
                position,
                old_direction,
                new_direction: velocity.linvel.normalize(),
                collision_normal,
            });
        }
    }
}

/// System for applying basic physics movement
/// **Single Responsibility:** Only handles position updates based on velocity
/// Based on Classical Mechanics - basic kinematic movement
pub fn physics_movement_system(
    mut query: Query<(&mut Transform, &Velocity), With<Npc>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        // Apply velocity-based movement
        let movement = velocity.linvel * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}

/// System for tracking movement behavior metrics for ML
/// **Single Responsibility:** Only collects and reports movement analytics
/// ML-HOOK: Provides quantifiable movement analytics for learning optimization
pub fn movement_analytics_system(
    query: Query<(Entity, &Velocity, &Desire), With<Npc>>,
    mut movement_events: EventWriter<MovementBehaviorEvent>,
    game_constants: Res<GameConstants>,
) {
    for (entity, velocity, desire) in query.iter() {
        // Calculate movement efficiency metrics
        let efficiency = calculate_movement_efficiency(
            velocity.linvel.length(),
            game_constants.npc_speed,
            1.0, // normalized time for this frame
        );

        movement_events.write(MovementBehaviorEvent {
            entity,
            current_desire: *desire,
            velocity: velocity.linvel,
            movement_efficiency: efficiency,
        });
    }
}

/// System for analyzing movement patterns over time
/// **Single Responsibility:** Only handles periodic movement pattern analysis
/// ML-HOOK: Provides quantifiable movement analytics for learning optimization
pub fn movement_pattern_analysis_system(
    query: Query<(&Transform, &Velocity, &Desire), With<Npc>>,
    mut last_analysis_time: Local<f32>,
    time: Res<Time>,
) {
    // Analyze every 3 seconds to track movement patterns
    *last_analysis_time += time.delta_secs();
    if *last_analysis_time >= 3.0 {
        *last_analysis_time = 0.0;

        let mut desire_speeds: std::collections::HashMap<Desire, Vec<f32>> = std::collections::HashMap::new();
        let mut total_movement_energy = 0.0;

        for (_transform, velocity, desire) in query.iter() {
            let speed = velocity.linvel.length();
            desire_speeds.entry(*desire).or_insert_with(Vec::new).push(speed);
            total_movement_energy += speed * speed; // Kinetic energy approximation
        }

        // ML-HOOK: Calculate quantifiable movement statistics by desire type
        for (desire, speeds) in desire_speeds.iter() {
            if !speeds.is_empty() {
                let avg_speed = speeds.iter().sum::<f32>() / speeds.len() as f32;
                let max_speed = speeds.iter().fold(0.0f32, |a, &b| a.max(b));

                debug!(
                    "Movement Analysis - Desire: {:?}, Avg Speed: {:.1}, Max Speed: {:.1}, Count: {}",
                    desire, avg_speed, max_speed, speeds.len()
                );
            }
        }

        // ML-HOOK: Overall system energy for behavioral optimization
        debug!(
            "Movement Energy Analysis - Total Kinetic Energy: {:.1}, NPCs: {}",
            total_movement_energy,
            query.iter().count()
        );
    }
}
