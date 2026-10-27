use bevy::prelude::*;

/// Marker component for NPCs
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct Npc;

/// Component representing NPC personality traits based on Big Five model
/// Based on "The Big Five Personality Dimensions and Job Performance" (Barrick & Mount, 1991)
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct Personality {
    /// Openness to new ideas and experiences (affects rumor spread and curiosity)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub openness: f32,
    /// Extraversion - sociability and assertiveness (affects social interactions)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub extraversion: f32,
    /// Agreeableness - cooperation and trust (affects rumor content and social harmony)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub agreeableness: f32,
    /// Conscientiousness - organization and responsibility (affects goal pursuit)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub conscientiousness: f32,
    /// Neuroticism - emotional stability (affects stress response and decision-making)
    /// Range: 0.0-1.0 (normalized for ML compatibility)
    pub neuroticism: f32,
}

/// Component tracking NPC's refilling/interaction state with resources
/// Based on Behavioral State Theory - agents have distinct behavioral modes
#[derive(Component, Reflect, PartialEq, Debug, Default)]
#[reflect(Component)]
pub struct RefillState {
    /// Whether the NPC is currently refilling/interacting with a resource
    pub is_refilling: bool,
    /// Time when refilling started (for duration tracking)
    pub refill_start_time: f32,
    /// Duration of the refill action in seconds
    pub refill_duration: f32,
    /// Entity of the resource being used (if any)
    pub resource_entity: Option<Entity>,
}

/// Component representing externally visible state of an entity
/// Based on Theory of Mind - what others can observe about an agent
/// CRITICAL: This contains ONLY externally apparent information, never internal state
#[derive(Component, Reflect, PartialEq, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ApparentState {
    /// Whether the entity appears to be running/moving quickly
    pub is_running: bool,
    /// Whether the entity appears to be carrying an item
    pub is_carrying_item: bool,
    /// General posture/stance of the entity
    pub posture: Posture,
    /// Whether the entity appears to be interacting with something
    pub is_interacting: bool,
}

/// Enum representing observable postures/stances
#[derive(Reflect, PartialEq, Debug, Clone, Copy)]
pub enum Posture {
    /// Neutral, relaxed stance
    Neutral,
    /// Appears alert or vigilant
    Alert,
    /// Appears friendly or approachable
    Friendly,
    /// Appears defensive or wary
    Defensive,
    /// Appears aggressive or threatening
    Aggressive,
}

/// Component storing what an agent currently perceives about other entities
/// Based on Cognitive Psychology - perception is subjective and limited
/// CRITICAL: This is the agent's internal model of the world, not ground truth
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct PerceivedEntities {
    /// Entities currently within visual range and their apparent state
    /// Limited by line-of-sight, distance, and attention
    pub in_sight: Vec<(Entity, ApparentState)>,
    /// Maximum number of entities that can be tracked simultaneously
    /// Based on cognitive attention limits (Miller's 7±2 rule)
    pub attention_limit: usize,
}

/// Component defining vision capabilities and parameters
/// Based on Human Visual Perception research
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct VisionRange {
    /// Maximum distance for clear vision (in world units)
    /// Based on human visual acuity research
    pub max_distance: f32,
    /// Field of view angle in radians (π = 180 degrees)
    /// Human FOV is approximately 120 degrees for clear vision
    pub field_of_view: f32,
    /// Whether line-of-sight checking is enabled
    pub requires_line_of_sight: bool,
}