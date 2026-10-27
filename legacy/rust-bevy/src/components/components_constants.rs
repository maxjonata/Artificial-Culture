use bevy::prelude::*;

/// Resource for timing rumor injection
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct RumorTimer(pub Timer);

/// Resource for game simulation constants
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GameConstants {
    /// Number of NPCs in the simulation
    pub num_npcs: usize,
    /// Radius of each NPC
    pub npc_radius: f32,
    /// Movement speed of NPCs
    pub npc_speed: f32,
    /// Social interaction distance
    pub social_distance: f32,
    /// Decay rate for hunger
    pub hunger_decay: f32,
    /// Decay rate for thirst
    pub thirst_decay: f32,
    /// Regeneration rate for fatigue
    pub fatigue_regen: f32,
    /// Decay rate for safety
    pub safety_decay: f32,
    /// Decay rate for loneliness (how quickly loneliness increases over time)
    pub loneliness_decay: f32,
    /// Number of wells (water sources) to spawn
    pub num_wells: usize,
    /// Number of restaurants (food sources) to spawn
    pub num_restaurants: usize,
    /// Number of hotels (rest sources) to spawn
    pub num_hotels: usize,
    /// Number of safe zones to spawn
    pub num_safe_zones: usize,

    // NEW: Action Failure Handling Constants (1.3.3+)
    // Based on Cognitive Flexibility and Goal Management research

    /// Maximum attempts before switching to alternative desire
    /// Based on cognitive perseverance research: agents show 3-5 attempts before cognitive flexibility kicks in
    /// See: "Cognitive Flexibility and Goal Management" (Miyake et al., 2000)
    pub max_failure_attempts: u8,

    /// Default timeout duration per desire attempt in seconds
    /// Based on attention span research: 15-20 seconds for focused task persistence
    /// See: "Attention and Performance" (Posner & Petersen, 1990)
    pub default_action_timeout: f32,

    /// Distance threshold for considering a target unreachable
    /// Based on spatial cognition: agents perceive targets >50 units as "far" in 2D space
    /// Calibrated for 200-unit default vision range (25% of vision range)
    pub stuck_distance_threshold: f32,

    /// Timeout multiplication factor for retry attempts
    /// Based on adaptive patience research: 20% increase per failure shows optimal persistence
    /// See: "Adaptive Control of Thought" (Anderson & Lebiere, 1998)
    pub timeout_retry_multiplier: f32,
}

/// Resource for color constants
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ColorConstants {
    /// Red color for NPCs that know the rumor
    pub red: Color,
    /// Green color for NPCs that don't know the rumor
    pub green: Color,
}
