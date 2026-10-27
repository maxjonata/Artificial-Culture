use crate::components::components_constants::GameConstants;
use crate::entity_builders::generic_type_safe_builder::EmptyBuilder;

// Import ALL the domain-specific extension traits
use crate::entity_builders::environmental_entity_domains::*;

use bevy::prelude::*;
use rand::prelude::*;

/// Centralized entity creation using the truly generic type-safe builder
/// ALL entity types now use the generic builder foundation following "Generalization over Specialization"

// =============================================================================
// NPC ENTITY CREATION (Using Generic Type-Safe Builder Pattern)
// =============================================================================

/// Creates a complete NPC entity using the proper generic type-safe builder pattern
/// This demonstrates the full fluent chain following the Type-State Builder Pattern
pub fn create_npc_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_constants: &GameConstants,
    position: Vec2,
) -> Entity {
    // Use the proper extension trait pattern for type-safe entity building
    // This follows the "Generic Foundation -> Specific Implementation" principle
    let entity = EmptyBuilder::new(commands)
        .create_complete_npc(commands, asset_server, game_constants);

    // Set custom position after building - this is a post-build modification
    commands.entity(entity).insert(Transform::from_xyz(position.x, position.y, 0.0));
    entity
}

/// Legacy-compatible function expected by main.rs
pub fn spawn_test_npcs(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_constants: &GameConstants,
) {
    let mut rng = rand::rng();

    for i in 0..game_constants.num_npcs {
        let position = Vec2::new(
            rng.random_range(-400.0..=400.0),
            rng.random_range(-300.0..=300.0),
        );

        let entity = create_npc_entity(commands, asset_server, game_constants, position);

        // Add custom name for this specific NPC
        commands.entity(entity).insert(Name::new(format!("NPC {}", i + 1)));
    }

    println!("Simulation started with {} NPCs using type-safe builders.", game_constants.num_npcs);
}

// =============================================================================
// ENVIRONMENT ENTITY CREATION (Using Generic Type-Safe Builder Pattern)
// =============================================================================

/// Creates a Well entity using the proper generic type-safe builder pattern
pub fn create_well_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) -> Entity {
    EmptyBuilder::new(commands)
        .with_well_resource(commands)
        .with_well_visual(commands, asset_server, position)
        .build()
}

/// Creates a Restaurant entity using the proper generic type-safe builder pattern
pub fn create_restaurant_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) -> Entity {
    EmptyBuilder::new(commands)
        .with_restaurant_resource(commands)
        .with_restaurant_visual(commands, asset_server, position)
        .build()
}

/// Creates a Hotel entity using the proper generic type-safe builder pattern
pub fn create_hotel_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) -> Entity {
    EmptyBuilder::new(commands)
        .with_hotel_resource(commands)
        .with_hotel_visual(commands, asset_server, position)
        .with_hotel_comfort(commands)
        .build()
}

/// Creates a SafeZone entity using the proper generic type-safe builder pattern
pub fn create_safe_zone_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) -> Entity {
    EmptyBuilder::new(commands)
        .with_safety_zone(commands)
        .with_safezone_visual(commands, asset_server, position)
        .build()
}

/// Legacy-compatible function expected by main.rs
/// Spawns environmental resources randomly across the map
pub fn spawn_environmental_resources(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    _game_constants: &GameConstants,
    window_width: f32,
    window_height: f32,
) {
    let mut rng = rand::rng();

    // Calculate spawn boundaries (leave some margin from edges)
    let margin = 50.0;
    let min_x = -window_width / 2.0 + margin;
    let max_x = window_width / 2.0 - margin;
    let min_y = -window_height / 2.0 + margin;
    let max_y = window_height / 2.0 - margin;

    // Spawn Wells (3-5 wells)
    let num_wells = rng.random_range(3..=5);
    for _ in 0..num_wells {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );
        create_well_entity(commands, asset_server, position);
    }

    // Spawn Restaurants (2-4 restaurants)
    let num_restaurants = rng.random_range(2..=4);
    for _ in 0..num_restaurants {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );
        create_restaurant_entity(commands, asset_server, position);
    }

    // Spawn Hotels (1-3 hotels)
    let num_hotels = rng.random_range(1..=3);
    for _ in 0..num_hotels {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );
        create_hotel_entity(commands, asset_server, position);
    }

    // Spawn Safe Zones (1-2 safe zones)
    let num_safe_zones = rng.random_range(1..=2);
    for _ in 0..num_safe_zones {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );
        create_safe_zone_entity(commands, asset_server, position);
    }

    println!("Environmental resources spawned: {} wells, {} restaurants, {} hotels, {} safe zones",
             num_wells, num_restaurants, num_hotels, num_safe_zones);
}
