use crate::components::components_environment::ResourceType;
use bevy::math::Vec2;
use bevy::prelude::{Entity, Event};

// ML-HOOK: Events for quantifiable pathfinding behavior tracking and reward calculation
#[derive(Event)]
pub struct PathTargetSetEvent {
    pub npc_entity: Entity,
    pub target_position: Vec2,
    pub target_entity: Option<Entity>,
    pub target_type: ResourceType,
    pub distance_to_target: f32, // ML-HOOK: Quantifiable pathfinding efficiency
}

#[derive(Event)]
pub struct PathTargetReachedEvent {
    pub npc_entity: Entity,
    pub target_position: Vec2,
    pub target_entity: Option<Entity>,
    pub time_to_reach: f32, // ML-HOOK: Performance metric for navigation efficiency
}

#[derive(Event)]
pub struct ResourceDiscoveredEvent {
    pub npc_entity: Entity,
    pub resource_position: Vec2,
    pub resource_entity: Entity,
    pub resource_type: ResourceType,
    pub discovery_distance: f32, // ML-HOOK: Spatial cognition metrics
}