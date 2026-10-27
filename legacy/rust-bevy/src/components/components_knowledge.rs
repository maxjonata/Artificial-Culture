use bevy::prelude::*;
use std::collections::HashMap;

/// Component representing an NPC's knowledge base
/// Based on Cognitive Science - memory as associative networks
#[derive(Component, Reflect, PartialEq, Debug, Default)]
#[reflect(Component)]
pub struct KnowledgeBase {
    /// Whether the NPC knows the rumor (legacy field for backward compatibility)
    pub knows_rumor: bool,
    /// Map of rumor content to belief strength (0.0-1.0)
    /// Based on Social Psychology - rumors have varying belief levels
    pub known_rumors: HashMap<String, f32>,
}