use crate::components::components_constants::GameConstants;
use crate::components::components_needs::{BasicNeeds, Desire, DesireThresholds};

/// Helper function to decay needs over time based on physiological models
/// Based on Homeostatic Drive Theory - all needs naturally decrease over time without intervention
/// FIXED: All decay functions now use consistent "higher = better satisfied" semantics
pub fn decay_needs(needs: &mut BasicNeeds, game_constants: &GameConstants, delta_time: f32) -> (f32, f32, f32, f32, f32) {
    let hunger_change = -game_constants.hunger_decay * delta_time; // Hunger satisfaction DECREASES over time
    let thirst_change = -game_constants.thirst_decay * delta_time; // Thirst satisfaction DECREASES over time
    let rest_change = -game_constants.fatigue_regen * delta_time; // Rest level DECREASES over time (fatigue increases)
    let safety_change = -game_constants.safety_decay * delta_time; // Safety DECREASES over time
    let social_change = -game_constants.loneliness_decay * delta_time; // Social satisfaction DECREASES over time

    needs.hunger = (needs.hunger + hunger_change).clamp(0.0, 1.0);
    needs.thirst = (needs.thirst + thirst_change).clamp(0.0, 1.0);
    needs.rest = (needs.rest + rest_change).clamp(0.0, 1.0);
    needs.safety = (needs.safety + safety_change).clamp(0.0, 1.0);
    needs.social = (needs.social + social_change).clamp(0.0, 1.0);

    (hunger_change, thirst_change, rest_change, safety_change, social_change)
}

/// Helper function to increase social satisfaction based on Social Exchange Theory
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
pub fn increase_social_satisfaction(needs: &mut BasicNeeds, amount: f32) -> f32 {
    let old_social = needs.social;
    needs.social = (needs.social + amount).clamp(0.0, 1.0);
    needs.social - old_social // Return actual change for ML tracking
}

/// Helper function implementing Maslow's Hierarchy of Needs for desire evaluation
/// System based on Maslow's Hierarchy of Needs and Threshold Psychology
/// All values are normalized between 0.0-1.0
pub fn evaluate_most_urgent_desire(needs: &BasicNeeds, thresholds: &DesireThresholds) -> (Desire, f32) {
    let mut desire_utilities = Vec::new();

    // Calculate weighted utility for each potential desire using the new formula
    // ML-HOOK: Each utility calculation provides quantifiable state for observation space

    let safety_utility = calculate_desire_utility(Desire::FindSafety, needs, thresholds);
    if safety_utility > 0.0 {
        desire_utilities.push((Desire::FindSafety, safety_utility));
    }

    let water_utility = calculate_desire_utility(Desire::FindWater, needs, thresholds);
    if water_utility > 0.0 {
        desire_utilities.push((Desire::FindWater, water_utility));
    }

    let food_utility = calculate_desire_utility(Desire::FindFood, needs, thresholds);
    if food_utility > 0.0 {
        desire_utilities.push((Desire::FindFood, food_utility));
    }

    let rest_utility = calculate_desire_utility(Desire::Rest, needs, thresholds);
    if rest_utility > 0.0 {
        desire_utilities.push((Desire::Rest, rest_utility));
    }

    let social_utility = calculate_desire_utility(Desire::Socialize, needs, thresholds);
    if social_utility > 0.0 {
        desire_utilities.push((Desire::Socialize, social_utility));
    }

    // Return the desire with highest utility score and the score itself for ML tracking
    desire_utilities
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or((Desire::Wander, 0.5)) // Default to Wander if no urgent desires
}

/// Helper function to get quantifiable satisfaction levels for ML observation space
/// ML-HOOK: Provides normalized satisfaction metrics for reward calculation
/// All values are already normalized between 0.0-1.0
/// FIXED: Now uses consistent "higher = better satisfied" semantics
pub fn get_satisfaction_level(needs: &BasicNeeds, desire: &Desire) -> f32 {
    match desire {
        Desire::FindFood => needs.hunger,
        Desire::FindWater => needs.thirst,
        Desire::Rest => needs.rest,
        Desire::FindSafety => needs.safety,
        Desire::Socialize => needs.social,
        Desire::Wander => 0.5, // Neutral satisfaction for wandering
    }
}

/// Creates randomized initial needs based on realistic biological variation
/// Based on Individual Differences Theory - people have varying baseline needs
/// References: Maslow (1943) - individual variation in need satisfaction patterns
/// FIXED: Now uses consistent "higher = better satisfied" semantics for all needs
pub fn create_random_basic_needs() -> BasicNeeds {
    use rand::prelude::*;
    let mut rng = rand::rng();

    BasicNeeds {
        // Hunger: Start moderately satisfied (0.6-0.9 range)
        // Higher values = more satisfied = less urgent
        hunger: rng.random_range(0.6..0.9),

        // Thirst: Start moderately satisfied (0.7-0.9 range)
        // Higher values = more satisfied = less urgent
        thirst: rng.random_range(0.7..0.9),

        // Rest: Start moderately to well rested (0.4-0.8 range)
        // Higher values = more rested = less urgent
        rest: rng.random_range(0.4..0.8),

        // Safety: Start feeling relatively safe (0.7-0.95 range)
        // Higher values = more safe = less urgent
        safety: rng.random_range(0.7..0.95),

        // Social: Start with moderate social satisfaction (0.3-0.8 range)
        // Higher values = more socially satisfied = less urgent to socialize
        social: rng.random_range(0.3..0.8),
    }
}

/// Helper function to calculate weighted utility for a desire using the formula:
/// FIXED: Now all needs use consistent "higher = better satisfied" semantics
/// For "higher = better" needs, we use (1.0 - Current_Need_Value) to get urgency
/// Utility = ((1.0 - Current_Need_Value) / (1.0 - High_Threshold)) * Priority_Weight
/// Higher utility = more urgent movement behavior
pub fn calculate_desire_utility(desire: Desire, basic_needs: &BasicNeeds, thresholds: &DesireThresholds) -> f32 {
    match desire {
        Desire::FindSafety => {
            let urgency = 1.0 - basic_needs.safety;
            let max_urgency = 1.0 - thresholds.safety_threshold.high_threshold.min(0.999);
            (urgency / max_urgency.max(0.001)) * thresholds.priority_weights.safety
        }
        Desire::FindWater => {
            let urgency = 1.0 - basic_needs.thirst;
            let max_urgency = 1.0 - thresholds.thirst_threshold.high_threshold.min(0.999);
            (urgency / max_urgency.max(0.001)) * thresholds.priority_weights.thirst
        }
        Desire::FindFood => {
            let urgency = 1.0 - basic_needs.hunger;
            let max_urgency = 1.0 - thresholds.hunger_threshold.high_threshold.min(0.999);
            (urgency / max_urgency.max(0.001)) * thresholds.priority_weights.hunger
        }
        Desire::Rest => {
            let urgency = 1.0 - basic_needs.rest;
            let max_urgency = 1.0 - thresholds.rest_threshold.high_threshold.min(0.999);
            (urgency / max_urgency.max(0.001)) * thresholds.priority_weights.rest
        }
        Desire::Socialize => {
            let urgency = 1.0 - basic_needs.social;
            let max_urgency = 1.0 - thresholds.social_threshold.high_threshold.min(0.999);
            (urgency / max_urgency.max(0.001)) * thresholds.priority_weights.social
        }
        Desire::Wander => 0.5, // Low utility for wandering
    }
}

/// Helper function to check if a desire should be activated (need value < high_threshold)
/// FIXED: Now activates when satisfaction is BELOW threshold (need is urgent)
pub fn should_activate_desire(desire: Desire, basic_needs: &BasicNeeds, thresholds: &DesireThresholds) -> bool {
    match desire {
        Desire::FindSafety => basic_needs.safety < thresholds.safety_threshold.high_threshold,
        Desire::FindWater => basic_needs.thirst < thresholds.thirst_threshold.high_threshold,
        Desire::FindFood => basic_needs.hunger < thresholds.hunger_threshold.high_threshold,
        Desire::Rest => basic_needs.rest < thresholds.rest_threshold.high_threshold,
        Desire::Socialize => basic_needs.social < thresholds.social_threshold.high_threshold,
        _ => false,
    }
}

/// Helper function to check if a desire should be deactivated (need value > low_threshold)
/// FIXED: Now deactivates when satisfaction is ABOVE low threshold (need is satisfied enough)
pub fn should_deactivate_desire(desire: Desire, basic_needs: &BasicNeeds, thresholds: &DesireThresholds) -> bool {
    match desire {
        Desire::FindSafety => basic_needs.safety > thresholds.safety_threshold.low_threshold,
        Desire::FindWater => basic_needs.thirst > thresholds.thirst_threshold.low_threshold,
        Desire::FindFood => basic_needs.hunger > thresholds.hunger_threshold.low_threshold,
        Desire::Rest => basic_needs.rest > thresholds.rest_threshold.low_threshold,
        Desire::Socialize => basic_needs.social > thresholds.social_threshold.low_threshold,
        _ => false,
    }
}

/// Helper function to decrease social satisfaction (no longer needed but kept for compatibility)
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
/// All values are normalized between 0.0-1.0
/// DEPRECATED: Use increase_social_satisfaction instead
pub fn decrease_loneliness(needs: &mut BasicNeeds, amount: f32) -> f32 {
    let old_social = needs.social;
    needs.social = (needs.social + amount).clamp(0.0, 1.0);
    needs.social - old_social // Return actual change for ML tracking
}
