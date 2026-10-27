use bevy::prelude::*;

/// Component representing an NPC's pathfinding target and navigation state
/// System based on Goal-Oriented Action Planning (GOAP) theory
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PathTarget {
    /// Target position to navigate towards
    pub target_position: Vec2,
    /// Target entity (if navigating to a specific entity like a well/restaurant)
    pub target_entity: Option<Entity>,
    /// Distance threshold to consider target "reached"
    pub arrival_threshold: f32,
    /// Whether the NPC has a valid path target
    pub has_target: bool,
    /// Time when the target was set (for timeout purposes)
    pub target_set_time: f32,
    /// Maximum time to pursue a target before giving up
    pub max_pursuit_time: f32,
}

/// Component for steering behavior towards targets
/// System based on Craig Reynolds' Steering Behaviors for autonomous agents
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SteeringBehavior {
    /// Desired velocity towards target
    pub desired_velocity: Vec2,
    /// Steering force applied to reach target
    pub steering_force: Vec2,
    /// Maximum steering force that can be applied
    pub max_steering_force: f32,
    /// Weight for seek behavior (moving towards target)
    pub seek_weight: f32,
    /// Weight for wander behavior (random exploration)
    pub wander_weight: f32,
    /// Current wander angle for autonomous movement
    pub wander_angle: f32,
    /// How much the wander angle changes per frame
    pub wander_angle_change: f32,
}

/// Component tracking NPC's knowledge of resource locations
/// System based on Spatial Cognition and Mental Maps theory
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ResourceMemory {
    /// Known well locations
    pub known_wells: Vec<Vec2>,
    /// Known restaurant locations
    pub known_restaurants: Vec<Vec2>,
    /// Known hotel locations
    pub known_hotels: Vec<Vec2>,
    /// Known safe zone locations
    pub known_safe_zones: Vec<Vec2>,
    /// Discovery radius - how close NPC needs to be to "discover" a resource
    pub discovery_radius: f32,
    /// Memory decay factor - how quickly forgotten locations become less reliable
    pub memory_decay_rate: f32,
}
