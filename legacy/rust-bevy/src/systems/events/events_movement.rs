use crate::components::components_needs::Desire;
use bevy::math::Vec2;
use bevy::prelude::{Entity, Event};

// ML-HOOK: Events for quantifiable movement behavior tracking and reward calculation
#[derive(Event)]
pub struct BoundaryCollisionEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub old_direction: Vec2,
    pub new_direction: Vec2,
    pub collision_normal: Vec2, // ML-HOOK: Quantifiable collision data
}

#[derive(Event)]
pub struct MovementBehaviorEvent {
    pub entity: Entity,
    pub current_desire: Desire,
    pub velocity: Vec2,
    pub movement_efficiency: f32, // ML-HOOK: Performance metric for learning
}