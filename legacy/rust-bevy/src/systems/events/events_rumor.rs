use bevy::prelude::{Entity, Event};

// ML-HOOK: Events for quantifiable rumor dynamics tracking and reward calculation
#[derive(Event)]
pub struct RumorInjectionEvent {
    pub entity: Entity,           // NPC receiving the rumor
    pub target_entity: Entity,    // Same as entity (for compatibility)
    pub rumor_content: String,    // Content of the rumor
    pub initial_belief: f32,      // Initial belief strength (0.0-1.0)
    pub injection_time: f32,      // When the rumor was injected
}

#[derive(Event)]
pub struct RumorSpreadEvent {
    pub sender: Entity,             // NPC spreading the rumor
    pub receiver: Entity,           // NPC receiving the rumor
    pub spreader_entity: Entity,    // Same as sender (for compatibility)
    pub receiver_entity: Entity,    // Same as receiver (for compatibility)
    pub rumor_content: String,      // Content of the rumor
    pub belief_strength: f32,       // Belief strength transferred
    pub spreader_openness: f32,
    pub receiver_openness: f32,
    pub spread_probability: f32,    // ML-HOOK: Quantifiable spread dynamics
}

#[derive(Event)]
pub struct RumorSpreadAttemptEvent {
    pub sender: Entity,                    // NPC attempting to spread
    pub receiver: Entity,                  // NPC receiving attempt
    pub spreader_entity: Entity,           // Same as sender (for compatibility)
    pub receiver_entity: Entity,           // Same as receiver (for compatibility)
    pub rumor_content: String,             // Content being spread
    pub transmission_probability: f32,     // Calculated transmission probability
    pub success: bool,
    pub calculated_probability: f32,       // ML-HOOK: Track all attempts for learning
}