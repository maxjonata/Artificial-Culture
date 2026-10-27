use crate::entity_builders::generic_type_safe_builder::{validation_states::*, EmptyBuilder};
use crate::entity_builders::npc_entity_domain::*;
use bevy::prelude::*;

// Import all NPC-related components
use crate::components::{
    components_constants::GameConstants,
    components_knowledge::KnowledgeBase,
    components_needs::{Desire, DesireThresholds},
    components_npc::{ApparentState, Npc, PerceivedEntities, Personality, RefillState, VisionRange},
    components_pathfinding::{PathTarget, ResourceMemory, SteeringBehavior},
};
use crate::utils::helpers::needs_helpers::create_random_basic_needs;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

// =============================================================================
// NPC BUILDER IMPLEMENTATIONS - Updated for new generic builder pattern
// =============================================================================

/// Implementation for starting NPC building from EmptyBuilder
impl NpcBuilderExt for EmptyBuilder {
    fn with_npc_core(self, commands: &mut Commands) -> NpcBuilder<Present, Missing, Missing, Missing, Missing, Missing> {
        let mut rng = rand::rng();

        let builder = self.add_bundle(commands, (
            Npc,
            Personality {
                openness: rng.random_range(0.0..1.0),
                extraversion: rng.random_range(0.0..1.0),
                agreeableness: rng.random_range(0.0..1.0),
                conscientiousness: rng.random_range(0.0..1.0),
                neuroticism: rng.random_range(0.0..1.0),
            },
            RefillState::default(),
            KnowledgeBase {
                knows_rumor: false,
                known_rumors: std::collections::HashMap::new(),
            },
        ));

        builder.transform_to()
    }
}

/// Implementation for adding needs after core is present
impl NpcCoreExt for NpcBuilder<Present, Missing, Missing, Missing, Missing, Missing> {
    fn with_needs(self, commands: &mut Commands) -> NpcBuilder<Present, Present, Missing, Missing, Missing, Missing> {
        let builder = self.add_bundle(commands, (
            create_random_basic_needs(),
            Desire::default(),
            DesireThresholds::default(),
        ));

        builder.transform_to()
    }
}

/// Implementation for adding pathfinding after needs are present
impl NpcNeedsExt for NpcBuilder<Present, Present, Missing, Missing, Missing, Missing> {
    fn with_pathfinding(self, commands: &mut Commands) -> NpcBuilder<Present, Present, Present, Missing, Missing, Missing> {
        let builder = self.add_bundle(commands, (
            PathTarget::default(),
            SteeringBehavior::default(),
            ResourceMemory::default(),
        ));

        builder.transform_to()
    }
}

/// Implementation for adding visual components after pathfinding is present
/// NEW: Now includes Vision System 1.3.1 components for perception
impl NpcPathfindingExt for NpcBuilder<Present, Present, Present, Missing, Missing, Missing> {
    fn with_visual(
        self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game_constants: &GameConstants,
    ) -> NpcBuilder<Present, Present, Present, Present, Missing, Missing> {
        let sprite_size = Vec2::splat(game_constants.npc_radius * 2.0);

        let builder = self.add_bundle(commands, (
            // Visual rendering components
            Sprite {
                image: asset_server.load("person.png"),
                custom_size: Some(sprite_size),
                ..default()
            },
            Name::new("NPC".to_string()),
            Transform::default(),
            // NEW: Vision System 1.3.1 components following "Mantle of Ignorance"
            ApparentState::default(),      // What others can observe about this agent
            PerceivedEntities::default(),  // What this agent perceives about others
            VisionRange::default(),        // This agent's visual perception capabilities
        ));

        builder.transform_to()
    }
}

/// Implementation for adding physics after visual is present
/// Now configured to prevent body pushing and reduce inertia
impl NpcVisualExt for NpcBuilder<Present, Present, Present, Present, Missing, Missing> {
    fn with_physics(
        self,
        commands: &mut Commands,
        game_constants: &GameConstants,
    ) -> NpcBuilder<Present, Present, Present, Present, Present, Missing> {
        let builder = self.add_bundle(commands, (
            RigidBody::Dynamic,
            GravityScale(0.0),
            Collider::ball(game_constants.npc_radius),
            Restitution::coefficient(0.0), // No bouncing to reduce pushing
            Friction::coefficient(1.0),    // High friction to prevent sliding
            ActiveEvents::COLLISION_EVENTS,
            LockedAxes::ROTATION_LOCKED,   // Prevent rotation
            Ccd::enabled(),                // Continuous collision detection for better stability
        ));

        builder.transform_to()
    }
}

/// Implementation for adding movement (final step) after physics is present
impl NpcPhysicsExt for NpcBuilder<Present, Present, Present, Present, Present, Missing> {
    fn with_movement(
        self,
        commands: &mut Commands,
        game_constants: &GameConstants,
    ) -> ValidatedNpc {
        let mut rng = rand::rng();
        let initial_velocity = Vec2::new(
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
        ).normalize_or_zero() * game_constants.npc_speed;

        let builder = self.add_bundle(commands, (
            Velocity {
                linvel: initial_velocity,
                angvel: 0.0,
            },
            Damping {
                linear_damping: 0.1,
                angular_damping: 0.8,
            },
        ));

        builder.transform_to()
    }
}

/// Implementation for building a fully validated NPC
impl ValidatedNpcBuilder for ValidatedNpc {
    fn build(self) -> Entity {
        self.entity()
    }
}

/// Fluent creation method for complete NPCs
impl EmptyBuilder {
    pub fn create_complete_npc(
        self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game_constants: &GameConstants,
    ) -> Entity {
        self.with_npc_core(commands)
            .with_needs(commands)
            .with_pathfinding(commands)
            .with_visual(commands, asset_server, game_constants)
            .with_physics(commands, game_constants)
            .with_movement(commands, game_constants)
            .build()
    }
}
