use bevy::prelude::*;

use crate::components::components_constants::{ColorConstants, GameConstants, RumorTimer};
use crate::components::components_environment::{Hotel, InteractableResource, Resource, ResourceOwnership, ResourceTransfer, ResourceType, Restaurant, SafeZone, Well};
use crate::components::components_knowledge::KnowledgeBase;
use crate::components::components_needs::{BasicNeeds, CurrentDesire, Desire, DesirePriorities, DesireThresholds, DualThreshold};
use crate::components::components_npc::{ApparentState, Npc, PerceivedEntities, Personality, Posture, RefillState, VisionRange};
use crate::components::components_pathfinding::{PathTarget, ResourceMemory, SteeringBehavior};

/// Plugin for registering all custom components with Bevy's reflection system
pub struct CustomComponentsPlugin;

impl Plugin for CustomComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            // NPC components
            .register_type::<Npc>()
            .register_type::<Personality>()
            .register_type::<RefillState>()
            // Vision components - NEW for 1.3.1 Perception System
            .register_type::<ApparentState>()
            .register_type::<PerceivedEntities>()
            .register_type::<VisionRange>()
            .register_type::<Posture>()
            // Knowledge components
            .register_type::<KnowledgeBase>()
            // Needs components
            .register_type::<BasicNeeds>()
            .register_type::<Desire>()
            .register_type::<DesireThresholds>()
            .register_type::<DesirePriorities>()
            .register_type::<DualThreshold>()
            .register_type::<CurrentDesire>()
            // Environment components - New unified resource system
            .register_type::<Resource>()
            .register_type::<ResourceType>()
            .register_type::<ResourceOwnership>()
            .register_type::<ResourceTransfer>()
            .register_type::<InteractableResource>()
            // Environment components - Legacy (for backward compatibility)
            .register_type::<Well>()
            .register_type::<Restaurant>()
            .register_type::<Hotel>()
            .register_type::<SafeZone>()
            // Pathfinding components
            .register_type::<PathTarget>()
            .register_type::<SteeringBehavior>()
            .register_type::<ResourceMemory>()
            // Resources
            .register_type::<RumorTimer>()
            .register_type::<GameConstants>()
            .register_type::<ColorConstants>()
        ;
    }
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS FOR ALL COMPONENTS
// ============================================================================


impl Default for DesireThresholds {
    fn default() -> Self {
        Self {
            // Critical thresholds based on psychological research - using DualThreshold for hysteresis
            // FIXED: All thresholds now use "higher satisfaction = better" semantics
            // Desires activate when satisfaction drops BELOW high_threshold
            hunger_threshold: DualThreshold {
                high_threshold: 0.7,   // Activate FindFood when hunger satisfaction drops below 70%
                low_threshold: 0.3,    // Start pathfinding when hunger satisfaction drops below 30%
            },
            thirst_threshold: DualThreshold {
                high_threshold: 0.75,  // More urgent - activate when thirst satisfaction drops below 75%
                low_threshold: 0.25,   // Start pathfinding when thirst satisfaction drops below 25%
            },
            rest_threshold: DualThreshold {
                high_threshold: 0.6,   // Rest when rest level drops below 60%
                low_threshold: 0.2,    // Start pathfinding when rest level drops below 20%
            },
            safety_threshold: DualThreshold {
                high_threshold: 0.7,   // Seek safety when safety satisfaction drops below 70%
                low_threshold: 0.4,    // Start pathfinding when safety satisfaction drops below 40%
            },
            social_threshold: DualThreshold {
                high_threshold: 0.6,   // Socialize when social satisfaction drops below 60%
                low_threshold: 0.3,    // Start pathfinding when social satisfaction drops below 30%
            },
            priority_weights: DesirePriorities::default(),
        }
    }
}

impl Default for DesirePriorities {
    fn default() -> Self {
        // Based on Maslow's hierarchy of needs - normalized 0.0-1.0
        Self {
            thirst: 1.0,       // Maximum priority - dehydration is fastest killer
            hunger: 0.85,      // Very high priority - survival need
            safety: 0.75,      // High priority - security need
            rest: 0.6,         // Medium priority - physiological need
            social: 0.3,       // Lower priority - social need
        }
    }
}

impl Default for DualThreshold {
    fn default() -> Self {
        Self {
            high_threshold: 0.7,  // Activate desire when need reaches 70%
            low_threshold: 0.3,   // Deactivate desire when need drops to 30%
        }
    }
}

impl Default for GameConstants {
    fn default() -> Self {
        Self {
            num_npcs: 20,
            npc_radius: 15.0,
            npc_speed: 200.0,
            social_distance: 100.0,
            // Differentiated decay rates based on physiological urgency
            hunger_decay: 0.008,    // Moderate decay - can survive weeks without food
            thirst_decay: 0.015,    // Faster decay - can only survive days without water
            fatigue_regen: 0.005,   // Slower accumulation - fatigue builds gradually
            safety_decay: 0.012,    // Moderate-fast decay - stress builds quickly in unsafe environments
            loneliness_decay: 0.003,    // Slowest decay - social isolation takes time to affect wellbeing
            num_wells: 1,
            num_restaurants: 1,
            num_hotels: 1,
            num_safe_zones: 1,

            // NEW: Action Failure Handling Constants (1.3.3+)
            // Values based on cognitive psychology research
            max_failure_attempts: 3,        // 3 attempts before cognitive flexibility kicks in
            default_action_timeout: 15.0,   // 15 seconds focused attention span
            stuck_distance_threshold: 50.0, // 25% of default vision range (200 units)
            timeout_retry_multiplier: 1.2,  // 20% increase per retry for adaptive patience
        }
    }
}

impl Default for ColorConstants {
    fn default() -> Self {
        Self {
            red: Color::srgb(1.0, 0.2, 0.2),
            green: Color::srgb(0.2, 1.0, 0.2),
        }
    }
}

impl Default for InteractableResource {
    fn default() -> Self {
        Self {
            resource_type: ResourceType::Water,
            availability: 1.0,        // Normalized: 1.0 = full availability
            max_interactions: 5,      // Data-oriented: u8 for memory efficiency
            current_interactions: 0,
            regeneration_timer: 0.0,
        }
    }
}

impl Default for PathTarget {
    fn default() -> Self {
        Self {
            target_position: Vec2::ZERO,
            target_entity: None,
            arrival_threshold: 30.0,
            has_target: false,
            target_set_time: 0.0,
            max_pursuit_time: 10.0, // 10 seconds max pursuit
        }
    }
}

impl Default for SteeringBehavior {
    fn default() -> Self {
        Self {
            desired_velocity: Vec2::ZERO,
            steering_force: Vec2::ZERO,
            max_steering_force: 500.0,
            seek_weight: 1.0,
            wander_weight: 0.3,
            wander_angle: 0.0,
            wander_angle_change: 0.1,
        }
    }
}

impl Default for ResourceMemory {
    fn default() -> Self {
        Self {
            known_wells: Vec::new(),
            known_restaurants: Vec::new(),
            known_hotels: Vec::new(),
            known_safe_zones: Vec::new(),
            discovery_radius: 100.0,
            memory_decay_rate: 0.01,
        }
    }
}

impl Default for ApparentState {
    fn default() -> Self {
        Self {
            is_running: false,
            is_carrying_item: false,
            posture: Posture::Neutral,
            is_interacting: false,
        }
    }
}

impl Default for VisionRange {
    fn default() -> Self {
        Self {
            // Default vision range of 200 units - reasonable for 2D simulation
            max_distance: 200.0,
            // Default FOV of 120 degrees (2.094 radians) - human-like peripheral vision
            field_of_view: 2.094,
            requires_line_of_sight: true,
        }
    }
}

impl Default for PerceivedEntities {
    fn default() -> Self {
        Self {
            in_sight: Vec::new(),
            // Default attention limit of 7 entities - based on Miller's cognitive research
            attention_limit: 7,
        }
    }
}
