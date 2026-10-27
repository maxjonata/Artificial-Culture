use crate::components::components_needs::Desire;
use bevy::prelude::{Entity, Event};

// ML-HOOK: Events for quantifiable behavior tracking and reward calculation

/// Event fired when any need value changes (from decay, satisfaction, etc.)
#[derive(Event)]
pub struct NeedChangeEvent {
    pub entity: Entity,
    pub need_type: NeedType,
    pub old_value: f32,
    pub new_value: f32,
    pub change_amount: f32,
}

/// Event fired when a need crosses a threshold (either above or below)
#[derive(Event)]
pub struct ThresholdCrossedEvent {
    pub entity: Entity,
    pub need_type: NeedType,
    pub threshold_value: f32,
    pub current_value: f32,
    pub crossed_direction: ThresholdDirection,
    pub should_trigger_desire: bool,
}

/// Event fired when an NPC's desire changes due to threshold crossing
#[derive(Event)]
pub struct DesireChangeEvent {
    pub entity: Entity,
    pub old_desire: Desire,
    pub new_desire: Desire,
    pub urgency_score: f32, // ML-HOOK: Quantifiable urgency for reward calculation
    pub trigger_reason: DesireChangeReason,
}

/// Event fired when needs decay over time
#[derive(Event)]
pub struct NeedDecayEvent {
    pub entity: Entity,
    pub hunger_change: f32,
    pub thirst_change: f32,
    pub rest_change: f32,
    pub safety_change: f32,
    pub social_change: f32,
}

/// Event fired when NPCs have social interactions
#[derive(Event)]
pub struct SocialInteractionEvent {
    pub entity_1: Entity,
    pub entity_2: Entity,
    pub social_boost: f32,
}

/// Event fired when an NPC attempts to fulfill a desire (success or failure)
#[derive(Event)]
pub struct DesireFulfillmentAttemptEvent {
    pub entity: Entity,
    pub desire: Desire,
    pub success: bool,
    pub satisfaction_gained: f32,
}

/// Event fired when a need is satisfied through resource interaction
#[derive(Event)]
pub struct NeedSatisfactionEvent {
    pub entity: Entity,
    pub need_type: NeedType,
    pub satisfaction_amount: f32,
    pub resource_entity: Option<Entity>, // The resource that provided satisfaction
}

/// NEW: Event fired when an action is completed (1.3.3 Action Management)
/// This event is sent when a CurrentDesire is fulfilled or abandoned
#[derive(Event)]
pub struct ActionCompleted {
    pub entity: Entity,
    pub completed_desire: Desire,
    pub completion_reason: ActionCompletionReason,
    pub duration: f32, // How long the action took to complete
    pub success: bool, // Whether the action achieved its goal
}

/// Event that triggers decision-making evaluation for an agent
/// This is the missing event from roadmap 1.3.2 that should trigger the decision_making_system
#[derive(Event)]
pub struct EvaluateDecision {
    pub entity: Entity,
    pub trigger_reason: DecisionTrigger,
}

/// Reasons why decision evaluation was triggered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionTrigger {
    /// Periodic re-evaluation (every few seconds)
    Periodic,
    /// Need value changed significantly
    NeedChanged,
    /// External stimulus (collision, interaction)
    Stimulus,
    /// Forced re-evaluation (e.g., from ML agent)
    Forced,
}

/// Event fired when an agent's current desire is set after decision evaluation
#[derive(Event)]
pub struct CurrentDesireSet {
    pub entity: Entity,
    pub desire: Desire,
    pub utility_score: f32,
    pub competing_desires: Vec<(Desire, f32)>, // ML-HOOK: All evaluated desires for observation space
}

/// Types of needs that can change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeedType {
    Hunger,
    Thirst,
    Rest,
    Safety,
    Social,
}

/// Direction of threshold crossing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdDirection {
    /// Need value dropped below threshold (triggers seeking behavior)
    Below,
    /// Need value rose above threshold (stops seeking behavior)
    Above,
}

/// Reason why a desire changed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesireChangeReason {
    /// Threshold was crossed due to need decay
    ThresholdCrossed,
    /// Need was satisfied enough to stop seeking
    NeedSatisfied,
    /// Manual override (e.g., from ML agent)
    ManualOverride,
}

/// Enum describing why an action was completed
#[derive(Debug, Clone, Copy)]
pub enum ActionCompletionReason {
    /// Action succeeded - need was satisfied
    Success,
    /// Action was interrupted by higher priority need
    Interrupted,
    /// Action failed or was abandoned
    Failed,
    /// Action timed out
    Timeout,
}
