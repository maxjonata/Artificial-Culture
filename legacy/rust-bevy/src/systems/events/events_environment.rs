use crate::components::components_environment::ResourceType;
use bevy::prelude::{Entity, Event};

/// Fired when an NPC attempts to interact with a resource
/// Based on Environmental Psychology - resource seeking behavior
#[derive(Event, Debug)]
pub struct ResourceInteractionAttemptEvent {
    /// The NPC entity attempting interaction
    pub npc_entity: Entity,
    /// The resource entity being approached
    pub resource_entity: Entity,
    /// Type of resource being sought
    pub resource_type: ResourceType,
    /// Distance from NPC to resource
    pub distance: f32,
}

/// Fired when an NPC successfully interacts with a resource
/// Based on Operant Conditioning - reinforcement of successful behavior
#[derive(Event, Debug)]
pub struct ResourceInteractionSuccessEvent {
    /// The NPC entity that interacted
    pub npc_entity: Entity,
    /// The resource entity used
    pub resource_entity: Entity,
    /// Type of resource consumed
    pub resource_type: ResourceType,
    /// Amount of satisfaction gained (0.0-1.0)
    pub satisfaction_gained: f32,
    /// Resource availability after interaction (0.0-1.0)
    pub resource_availability_after: f32,
}

/// Fired when a resource regenerates capacity
/// Based on Resource Economics - regeneration cycles
#[derive(Event, Debug)]
pub struct ResourceRegenerationEvent {
    /// The resource entity that regenerated
    pub resource_entity: Entity,
    /// Type of resource
    pub resource_type: ResourceType,
    /// Availability before regeneration
    pub availability_before: f32,
    /// Availability after regeneration
    pub availability_after: f32,
    /// Amount regenerated
    pub regeneration_amount: f32,
}

/// Fired when an NPC enters proximity of a resource
/// Based on Affordance Theory - environmental opportunities for action
#[derive(Event, Debug)]
pub struct ResourceProximityEvent {
    /// The NPC entity in proximity
    pub npc_entity: Entity,
    /// The resource entity nearby
    pub resource_entity: Entity,
    /// Type of resource detected
    pub resource_type: ResourceType,
    /// Distance to resource
    pub distance: f32,
    /// Whether the NPC can interact (based on need/desire)
    pub can_interact: bool,
}

// ML-HOOK: Legacy events for backward compatibility and quantifiable tracking
#[derive(Event, Debug)]
pub struct ResourceInteractionEvent {
    pub npc_entity: Entity,
    pub resource_entity: Entity,
    pub resource_type: ResourceType,
    pub satisfaction_gained: f32,
    pub resource_consumed: f32, // ML-HOOK: Quantifiable resource usage
}

#[derive(Event, Debug)]
pub struct ResourceDepletionEvent {
    pub resource_entity: Entity,
    pub resource_type: ResourceType,
    pub remaining_capacity: f32,
}
