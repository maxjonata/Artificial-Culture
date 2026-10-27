use crate::components::components_environment::{Resource, ResourceType};
use crate::components::components_needs::BasicNeeds;

/// Helper function to calculate satisfaction gain from resource interaction
/// Based on Diminishing Returns Theory - satisfaction gain decreases as need is fulfilled
pub fn calculate_satisfaction_gain(
    resource_type: ResourceType,
    resource_availability: f32,
    current_need_level: f32,
) -> f32 {
    // Base satisfaction rates for different resource types
    let base_satisfaction = match resource_type {
        ResourceType::Water => 0.5,   // High satisfaction for survival need
        ResourceType::Food => 0.4,    // High satisfaction for survival need
        ResourceType::Rest => 0.3,    // Moderate satisfaction for recovery
        ResourceType::Safety => 0.35, // Moderate satisfaction for security
        ResourceType::Loneliness => 0.2,  // Lower but sustained satisfaction
    };

    // Diminishing returns: satisfaction decreases as need is already fulfilled
    let need_factor = 1.0 - current_need_level;

    // Resource availability affects satisfaction (depleted resources give less)
    let availability_factor = resource_availability.clamp(0.1, 1.0);

    (base_satisfaction * need_factor * availability_factor).clamp(0.0, 1.0)
}

/// Helper function to calculate resource consumption rate
/// Based on Resource Economics - consumption varies by resource type and need urgency
pub fn calculate_consumption_rate(
    resource_type: ResourceType,
    need_urgency: f32,
) -> f32 {
    let base_consumption = match resource_type {
        ResourceType::Water => 0.1,   // Moderate consumption
        ResourceType::Food => 0.08,   // Moderate consumption
        ResourceType::Rest => 0.0,    // Rest doesn't deplete (hotels are safe zones)
        ResourceType::Safety => 0.0,  // Safety doesn't deplete
        ResourceType::Loneliness => 0.05, // Light consumption of loneliness resources
    };

    // Higher urgency leads to higher consumption (desperate behavior)
    let urgency_multiplier = 0.5 + (need_urgency * 0.5);

    (base_consumption * urgency_multiplier).clamp(0.0, 0.2)
}

/// Helper function to calculate resource regeneration
/// Based on Natural Resource Economics - different regeneration patterns
pub fn calculate_regeneration_amount(
    resource: &Resource,
    delta_time: f32,
) -> f32 {
    if resource.availability >= 1.0 {
        return 0.0; // Already at full capacity
    }

    let regeneration_rate = match resource.resource_type {
        ResourceType::Water => 0.02,  // Steady regeneration (wells refill)
        ResourceType::Food => 0.015,  // Slower regeneration (food preparation)
        ResourceType::Rest => 0.0,    // Hotels don't regenerate capacity
        ResourceType::Safety => 0.0,  // Safety zones don't regenerate
        ResourceType::Loneliness => 0.03, // Social spaces recover quickly
    };

    let max_regeneration = 1.0 - resource.availability;
    (regeneration_rate * delta_time).min(max_regeneration)
}

/// Helper function to check if a resource type matches a need type
/// Used for determining if an NPC can satisfy their current need with a specific resource
pub fn resource_matches_need(resource_type: ResourceType, need_type: crate::systems::events::events_needs::NeedType) -> bool {
    use crate::systems::events::events_needs::NeedType;

    match (resource_type, need_type) {
        (ResourceType::Water, NeedType::Thirst) => true,
        (ResourceType::Food, NeedType::Hunger) => true,
        (ResourceType::Rest, NeedType::Rest) => true,
        (ResourceType::Safety, NeedType::Safety) => true,
        (ResourceType::Loneliness, NeedType::Social) => true,
        _ => false,
    }
}

/// Helper function to get need level from BasicNeeds based on resource type
/// Based on Component Data Access Optimization
/// FIXED: Updated to use new field names and correct semantics
pub fn get_need_level_for_resource(
    needs: &BasicNeeds,
    resource_type: ResourceType,
) -> f32 {
    match resource_type {
        ResourceType::Water => needs.thirst,
        ResourceType::Food => needs.hunger,
        ResourceType::Rest => needs.rest,
        ResourceType::Safety => needs.safety,
        ResourceType::Loneliness => needs.social,
    }
}

/// Helper function to apply satisfaction to needs
/// Based on Homeostatic Regulation Theory
/// FIXED: Updated to use new field names and correct semantics
pub fn apply_satisfaction_to_needs(
    needs: &mut BasicNeeds,
    resource_type: ResourceType,
    satisfaction_amount: f32,
) -> f32 {
    let old_value = get_need_level_for_resource(needs, resource_type);

    match resource_type {
        ResourceType::Water => {
            needs.thirst = (needs.thirst + satisfaction_amount).clamp(0.0, 1.0);
            needs.thirst - old_value
        }
        ResourceType::Food => {
            needs.hunger = (needs.hunger + satisfaction_amount).clamp(0.0, 1.0);
            needs.hunger - old_value
        }
        ResourceType::Rest => {
            // Rest satisfaction increases rest level directly
            needs.rest = (needs.rest + satisfaction_amount).clamp(0.0, 1.0);
            needs.rest - old_value
        }
        ResourceType::Safety => {
            needs.safety = (needs.safety + satisfaction_amount).clamp(0.0, 1.0);
            needs.safety - old_value
        }
        ResourceType::Loneliness => {
            // Social satisfaction increases social satisfaction level directly
            needs.social = (needs.social + satisfaction_amount).clamp(0.0, 1.0);
            needs.social - old_value
        }
    }
}
