use bevy::prelude::*;
use std::marker::PhantomData;

/// Truly generic type-safe entity builder that enforces mandatory components at compile time
/// Based on the Type-State Builder Pattern following the "Artificial Society" architectural principles
///
/// This builder is completely generic and can be used for ANY entity type by defining appropriate state types
pub struct TypeSafeEntityBuilder<State> {
    entity: Entity,
    _phantom: PhantomData<State>,
}

/// Generic state marker types for any entity validation
pub mod validation_states {
    /// Marker for missing component category
    pub struct Missing;
    /// Marker for present component category
    pub struct Present;
}

/// Type aliases following the architectural pattern
pub type EmptyBuilder = TypeSafeEntityBuilder<validation_states::Missing>;

// =============================================================================
// GENERIC CORE IMPLEMENTATION
// =============================================================================

impl EmptyBuilder {
    /// Creates a new generic entity builder
    /// This is the entry point for ALL type-safe entity creation
    pub fn new(commands: &mut Commands) -> Self {
        let entity = commands.spawn_empty().id();

        Self {
            entity,
            _phantom: PhantomData,
        }
    }
}

impl<State> TypeSafeEntityBuilder<State> {
    /// Generic method to add any bundle to the entity
    /// This is the core primitive that all other methods build upon
    pub fn add_bundle<B: Bundle>(self, commands: &mut Commands, bundle: B) -> Self {
        commands.entity(self.entity).insert(bundle);
        self
    }

    /// Get the entity ID (useful for post-build operations)
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Internal method to transform the builder state without consuming fields
    /// This enables safe state transitions for any entity type
    pub fn transform_to<NewState>(self) -> TypeSafeEntityBuilder<NewState> {
        TypeSafeEntityBuilder {
            entity: self.entity,
            _phantom: PhantomData,
        }
    }

    /// Simple build method for entities that don't need validation
    /// Use this for simple entities with minimal component requirements
    pub fn build_simple(self) -> Entity {
        self.entity
    }
}
