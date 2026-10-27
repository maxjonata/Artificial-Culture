use bevy::prelude::*;

/// Component representing an NPC's basic needs
/// All values are normalized between 0.0-1.0 for ML compatibility
/// Based on Homeostatic Drive Theory - organisms maintain internal balance
/// IMPORTANT: All needs follow "higher value = better satisfied" semantics
#[derive(Component, Debug, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct BasicNeeds {
    /// Hunger satisfaction level (0.0 = starving, 1.0 = well fed)
    pub hunger: f32,
    /// Thirst satisfaction level (0.0 = dehydrated, 1.0 = hydrated)
    pub thirst: f32,
    /// Rest level (0.0 = exhausted, 1.0 = well rested)
    pub rest: f32,
    /// Safety satisfaction level (0.0 = very unsafe, 1.0 = safe)
    pub safety: f32,
    /// Social satisfaction level (0.0 = very lonely, 1.0 = socially satisfied)
    pub social: f32,
}

/// Enum representing an NPC's current desire/goal
#[derive(Component, Reflect, PartialEq, Debug, Default, Clone, Copy, Hash, Eq)]
#[reflect(Component)]
pub enum Desire {
    /// Default desire - wander around
    #[default]
    Wander,
    /// Find food to satisfy hunger
    FindFood,
    /// Find water to satisfy thirst
    FindWater,
    /// Find safety when threatened
    FindSafety,
    /// Rest to recover from fatigue
    Rest,
    /// Socialize with other NPCs
    Socialize,
}

/// Component that defines thresholds for when desires should be activated
/// Follows Single Responsibility Principle - manages only desire thresholds
/// NEW LOGIC: Desires activate when needs drop BELOW high_threshold, pathfinding starts when BELOW low_threshold
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct DesireThresholds {
    /// Hunger threshold below which FindFood desire is triggered
    pub hunger_threshold: DualThreshold,
    /// Thirst threshold below which FindWater desire is triggered
    pub thirst_threshold: DualThreshold,
    /// Rest threshold below which Rest desire is triggered
    pub rest_threshold: DualThreshold,
    /// Safety threshold below which FindSafety desire is triggered
    pub safety_threshold: DualThreshold,
    /// Social threshold below which Socialize desire is triggered
    pub social_threshold: DualThreshold,
    /// Priority weights for competing desires (higher = more important)
    pub priority_weights: DesirePriorities,
}

#[derive(Reflect, Debug)]
pub struct DualThreshold {
    pub high_threshold: f32, // Threshold below which desire is activated (start filling)
    pub low_threshold: f32,  // Threshold below which pathfinding starts (urgent action)
}

/// Priority system for resolving competing desires
/// Based on Maslow's hierarchy with physiological needs having highest priority
#[derive(Reflect, Debug)]
pub struct DesirePriorities {
    pub hunger: f32,    // Highest priority - survival need
    pub thirst: f32,    // Highest priority - survival need
    pub safety: f32,    // High priority - security need
    pub rest: f32,      // Medium priority - physiological need
    pub social: f32,    // Lower priority - social need
}

/// Component that stores an agent's current decision/desire after evaluation
/// This is the missing component from roadmap 1.3.2 that gets set by the decision_making_system
#[derive(Component, Debug, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct CurrentDesire {
    /// The agent's current active desire/goal
    pub desire: Desire,
    /// The utility score of this desire when it was selected
    /// ML-HOOK: Quantifiable decision strength for observation space
    pub utility_score: f32,
    /// Timestamp when this desire was last evaluated
    pub last_evaluated: f32,
    /// NEW: Failure tracking for adaptive behavior
    pub failure_count: u8,
    /// NEW: Time when the current attempt started
    pub attempt_start_time: f32,
    /// NEW: Maximum time to spend on this desire before considering it failed
    pub timeout_duration: f32,
    /// NEW: Last known target entity (resource, NPC, etc.) for this desire
    pub last_target: Option<Entity>,
}
