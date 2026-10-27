mod components;
mod systems;
mod entity_builders;
mod utils;

use crate::components::components_constants::{ColorConstants, GameConstants, RumorTimer};
use crate::components::components_default::CustomComponentsPlugin;
use crate::entity_builders::entity_builders_default::{spawn_environmental_resources, spawn_test_npcs};
use crate::systems::events::events_environment::{ResourceDepletionEvent, ResourceInteractionAttemptEvent, ResourceInteractionEvent, ResourceInteractionSuccessEvent, ResourceProximityEvent, ResourceRegenerationEvent};
use crate::systems::events::events_needs::{ActionCompleted, CurrentDesireSet, DesireChangeEvent, DesireFulfillmentAttemptEvent, EvaluateDecision, NeedChangeEvent, NeedDecayEvent, NeedSatisfactionEvent, SocialInteractionEvent, ThresholdCrossedEvent};
use crate::systems::systems_environment::{
    refill_management_system,
    resource_interaction_system,
    resource_regeneration_system,
};
use crate::systems::systems_movement::{
    boundary_collision_system,
    movement_analytics_system,
    movement_pattern_analysis_system,
    physics_movement_system,
};
use crate::systems::systems_needs::{
    action_failure_handling_system,
    debug_npc_status,
    decay_basic_needs,
    decision_making_system,
    desire_fulfillment_system,
    desire_update_system,
    handle_social_interactions,
    optimized_threshold_monitoring_system,
    periodic_decision_trigger_system,
    threshold_monitoring_system,
};
use crate::systems::systems_pathfinding::{
    desire_pathfinding_system,
    resource_discovery_system,
    steering_behavior_system,
};
// Import all the systems we need
use crate::systems::systems_rumor::{
    rumor_decay_system,
    rumor_injection_system,
    rumor_interaction_detection_system,
    rumor_transmission_system,
};
use crate::systems::systems_visual::{color_system, update_apparent_state_system, vision_system};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::WorldInspectorPlugin,
};
use bevy_rapier2d::prelude::*;
use systems::events::events_movement::{BoundaryCollisionEvent, MovementBehaviorEvent};
use systems::events::events_pathfinding::{PathTargetReachedEvent, PathTargetSetEvent, ResourceDiscoveredEvent};
use systems::events::events_rumor::{RumorInjectionEvent, RumorSpreadAttemptEvent, RumorSpreadEvent};

fn setup_simulation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_constants: Res<GameConstants>,
    windows: Query<&Window>,
) {
    commands.spawn(Camera2d);

    // Spawn NPCs first
    spawn_test_npcs(&mut commands, &asset_server, &game_constants);

    // Spawn environmental resources randomly across the space
    if let Ok(window) = windows.single() {
        spawn_environmental_resources(
            &mut commands,
            &asset_server,
            &game_constants,
            window.width(),
            window.height(),
        );
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            CustomComponentsPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        // Resources initialization
        .insert_resource(RumorTimer(Timer::from_seconds(3.0, TimerMode::Once)))
        .insert_resource(GameConstants::default())
        .insert_resource(ColorConstants::default())

        // Register Rapier debug render context for inspector control
        .register_type::<DebugRenderContext>()

        // ML-HOOK: Register all events for quantifiable behavior tracking
        .add_event::<NeedDecayEvent>()
        .add_event::<DesireChangeEvent>()
        .add_event::<SocialInteractionEvent>()
        .add_event::<ThresholdCrossedEvent>()
        .add_event::<DesireFulfillmentAttemptEvent>()
        .add_event::<NeedSatisfactionEvent>()
        .add_event::<NeedChangeEvent>()
        // NEW: Decision-making events from roadmap 1.3.2
        .add_event::<EvaluateDecision>()
        .add_event::<CurrentDesireSet>()
        // NEW: Action Management events from roadmap 1.3.3
        .add_event::<ActionCompleted>()
        .add_event::<RumorInjectionEvent>()
        .add_event::<RumorSpreadEvent>()
        .add_event::<RumorSpreadAttemptEvent>()
        .add_event::<BoundaryCollisionEvent>()
        .add_event::<MovementBehaviorEvent>()
        .add_event::<ResourceInteractionEvent>()
        .add_event::<ResourceDepletionEvent>()
        .add_event::<ResourceInteractionAttemptEvent>()
        .add_event::<ResourceInteractionSuccessEvent>()
        .add_event::<ResourceRegenerationEvent>()
        .add_event::<ResourceProximityEvent>()
        .add_event::<PathTargetSetEvent>()
        .add_event::<PathTargetReachedEvent>()
        .add_event::<ResourceDiscoveredEvent>()


        // Startup systems
        .add_systems(Startup, setup_simulation)

        // Update systems organized by event flow and dependencies for optimal performance
        .add_systems(Update, (
            // PHASE 0: Decision Triggers (Event Producers)
            // NEW: Periodic decision evaluation system from roadmap 1.3.2
            periodic_decision_trigger_system,   // Fires EvaluateDecision events periodically

            // PHASE 1: Core State Updates and Perception (Event Producers)
            // NEW: Vision System 1.3.1 - Must run early to populate perception data
            (
                update_apparent_state_system,           // NEW: Updates externally visible state
                vision_system,                          // NEW: Populates perception data using spatial queries
                decay_basic_needs,                      // Produces NeedChangeEvent, NeedDecayEvent
                optimized_threshold_monitoring_system,  // NEW: Optimized version that triggers decision evaluation
            ),

            // PHASE 2: Decision Making (Event Consumers → Event Producers)
            // NEW: Core decision-making system from roadmap 1.3.2
            (
                decision_making_system,         // NEW: Uses evaluate_most_urgent_desire for holistic decisions
                threshold_monitoring_system,    // Legacy: Still used for logging/debugging threshold crossings
                desire_update_system,           // Legacy: Individual desire updates (less optimal)
                resource_discovery_system,      // Produces ResourceDiscoveredEvent, PathTargetSetEvent
            ),

            // PHASE 3: Action Execution (Event Consumers)
            // These systems execute the decisions made in Phase 2
            (
                // Movement systems - execute movement decisions
                desire_pathfinding_system,      // Consumes DesireChangeEvent, PathTargetSetEvent
                steering_behavior_system,       // Consumes pathfinding data, applies weighted utility
                physics_movement_system,        // Executes actual movement
                boundary_collision_system,      // Handles movement constraints
            ),

            // PHASE 4: Interaction Systems (Event Consumers → Event Producers)
            // These systems handle entity interactions based on movement/proximity
            (
                // Refill management - handles NPC refilling state
                refill_management_system,            // Manages refilling state transitions

                // NEW: Action failure handling - makes characters adaptive when desires fail
                action_failure_handling_system,      // NEW: Handles failed desires with adaptive retry/switching

                // Social interactions - handle NPC-to-NPC interactions
                rumor_interaction_detection_system,  // Detects proximity for rumors
                rumor_transmission_system,           // Handles rumor spread events
                handle_social_interactions,          // Processes social need fulfillment

                // Resource interactions - handle NPC-to-resource interactions
                resource_interaction_system,         // Processes resource interaction attempts
                desire_fulfillment_system,           // Handles desire satisfaction from interactions
            ),

            // PHASE 5: World State Management (Event Consumers)
            // These systems update world state based on interactions
            (
                resource_regeneration_system,   // Regenerates depleted resources
                rumor_injection_system,         // Injects new rumors into the system
                rumor_decay_system,             // Decays existing rumors over time
            ),

            // PHASE 6: Feedback and Analysis (Event Consumers, Low Priority)
            // These systems provide visual feedback and analytics
            (
                color_system,                   // Visual feedback based on current state
                movement_pattern_analysis_system, // Analytics for movement patterns
                movement_analytics_system,      // General movement analytics
                debug_npc_status,              // Debug information display
            ),
        ))
        .run();
}
