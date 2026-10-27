use crate::entity_builders::generic_type_safe_builder::{validation_states::*, TypeSafeEntityBuilder};
use bevy::prelude::*;
use std::marker::PhantomData;

/// NPC-specific state validation types
/// These define the compile-time requirements for NPC entities

/// NPC state marker - tracks 6 required components for complete NPC
pub struct NpcState<Core, Needs, Pathfinding, Visual, Physics, Movement> {
    pub _core: PhantomData<Core>,
    pub _needs: PhantomData<Needs>,
    pub _pathfinding: PhantomData<Pathfinding>,
    pub _visual: PhantomData<Visual>,
    pub _physics: PhantomData<Physics>,
    pub _movement: PhantomData<Movement>,
}

/// Type aliases for NPC builder states
pub type NpcBuilder<Core, Needs, Pathfinding, Visual, Physics, Movement> =
TypeSafeEntityBuilder<NpcState<Core, Needs, Pathfinding, Visual, Physics, Movement>>;

/// Type alias for a fully validated NPC
pub type ValidatedNpc = NpcBuilder<Present, Present, Present, Present, Present, Present>;

/// Extension trait for NPC building - no component imports here, just state transitions
pub trait NpcBuilderExt {
    fn with_npc_core(self, commands: &mut Commands) -> NpcBuilder<Present, Missing, Missing, Missing, Missing, Missing>;
}

/// NPC chaining methods - each advances the state machine
pub trait NpcCoreExt {
    fn with_needs(self, commands: &mut Commands) -> NpcBuilder<Present, Present, Missing, Missing, Missing, Missing>;
}

pub trait NpcNeedsExt {
    fn with_pathfinding(self, commands: &mut Commands) -> NpcBuilder<Present, Present, Present, Missing, Missing, Missing>;
}

pub trait NpcPathfindingExt {
    fn with_visual(
        self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game_constants: &crate::components::components_constants::GameConstants,
    ) -> NpcBuilder<Present, Present, Present, Present, Missing, Missing>;
}

pub trait NpcVisualExt {
    fn with_physics(
        self,
        commands: &mut Commands,
        game_constants: &crate::components::components_constants::GameConstants,
    ) -> NpcBuilder<Present, Present, Present, Present, Present, Missing>;
}

pub trait NpcPhysicsExt {
    fn with_movement(
        self,
        commands: &mut Commands,
        game_constants: &crate::components::components_constants::GameConstants,
    ) -> ValidatedNpc;
}

/// Validated NPC builder trait
pub trait ValidatedNpcBuilder {
    fn build(self) -> Entity;
}
