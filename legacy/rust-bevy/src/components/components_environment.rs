use bevy::prelude::*;

/// Unified resource component representing any type of resource in the environment
/// Based on Environmental Psychology - resource availability affects behavior
/// All values normalized to 0.0-1.0 for ML compatibility and consistency
#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Resource {
    /// Type of resource this entity provides
    pub resource_type: ResourceType,
    /// Current availability (0.0 = depleted, 1.0 = full capacity) - only for consumable resources
    pub availability: f32,
    /// Maximum number of simultaneous interactions (data-oriented: prefer u8 for memory efficiency)
    pub max_interactions: u8,
    /// Current number of NPCs interacting with this resource
    pub current_interactions: u8,
    /// Regeneration rate per second (0.0-1.0 normalized) - only for consumable resources
    pub regeneration_rate: f32,
    /// Timer for regeneration calculations (internal system use)
    pub regeneration_timer: f32,
}

/// Resource types that satisfy different needs
/// Based on Maslow's hierarchy - different resources satisfy different need levels
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResourceType {
    /// Satisfies thirst need - highest priority survival resource
    #[default]
    Water,
    /// Satisfies hunger need - essential survival resource
    Food,
    /// Satisfies fatigue need - physiological restoration
    Rest,
    /// Satisfies safety need - security and protection
    Safety,
    /// Social gathering spaces - satisfies loneliness by providing social interaction
    Loneliness,
}

/// Component for entities that can own and transfer resources
/// Based on Economic Theory - resource ownership and exchange
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct ResourceOwnership {
    /// Resources currently owned by this entity (data-oriented: use Entity IDs for O(1) access)
    pub owned_resources: Vec<Entity>,
    /// Maximum resources this entity can own (u8 for memory efficiency)
    pub max_ownership_capacity: u8,
    /// Current resource transfer in progress (None if no active transfer)
    pub active_transfer: Option<ResourceTransfer>,
}

/// Resource transfer data structure
/// Based on Transaction Cost Economics - resource exchange mechanisms
#[derive(Debug, Reflect, Clone)]
pub struct ResourceTransfer {
    /// Source entity transferring the resource
    pub from_entity: Entity,
    /// Target entity receiving the resource
    pub to_entity: Entity,
    /// The resource being transferred
    pub resource_entity: Entity,
    /// Transfer progress (0.0 = started, 1.0 = completed)
    pub progress: f32,
    /// Transfer duration in seconds
    pub duration: f32,
}

/// Component marking entities as interactable resources with usage tracking
/// Based on Environmental Psychology - resource interaction patterns
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractableResource {
    /// Type of resource for need satisfaction
    pub resource_type: ResourceType,
    /// Current availability (0.0-1.0 normalized)
    pub availability: f32,
    /// Maximum simultaneous users (data-oriented: u8 for memory efficiency)
    pub max_interactions: u8,
    /// Current active interactions
    pub current_interactions: u8,
    /// Time until next regeneration tick
    pub regeneration_timer: f32,
}

// ================================
// LEGACY COMPONENTS FOR BACKWARD COMPATIBILITY
// These will be phased out in favor of the unified Resource system
// ================================

/// Legacy component for water sources (wells)
/// Based on Resource Economics - water as a finite renewable resource
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Well {
    /// Current water capacity (0.0-1.0 normalized)
    pub water_capacity: f32,
    /// Water consumption rate per interaction
    pub consumption_rate: f32,
}

/// Legacy component for food sources (restaurants)
/// Based on Resource Economics - food as a finite renewable resource
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Restaurant {
    /// Current food capacity (0.0-1.0 normalized)
    pub food_capacity: f32,
    /// Food consumption rate per interaction
    pub consumption_rate: f32,
}

/// Legacy component for rest areas (hotels)
/// Based on Environmental Psychology - hotels as safe restoration zones
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Hotel {
    /// Rest capacity - always 1.0 for hotels (unlimited rest)
    pub rest_capacity: f32,
    /// Number of available beds (u8 for memory efficiency)
    pub available_beds: u8,
    /// Maximum bed capacity
    pub max_beds: u8,
    pub comfort_level: f64,
}

/// Legacy component for safety zones
/// Based on Environmental Psychology - safe spaces for security needs
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct SafeZone {
    /// Safety level provided (0.0-1.0 normalized)
    pub safety_level: f32,
    /// Radius of safety influence
    pub influence_radius: f32,
    pub capacity: i32,
    pub current_occupancy: i32,
}
