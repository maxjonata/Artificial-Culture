use crate::entity_builders::environmental_entity_domains::*;
use crate::entity_builders::generic_type_safe_builder::{validation_states::*, EmptyBuilder};
use bevy::prelude::*;

// Import all environmental-related components
use crate::components::components_environment::{
    Hotel, Resource, ResourceType, Restaurant, SafeZone, Well,
};

// =============================================================================
// WELL BUILDER IMPLEMENTATIONS
// =============================================================================

impl WellBuilderExt for EmptyBuilder {
    fn with_well_resource(self, commands: &mut Commands) -> WellBuilder<Present, Missing> {
        let builder = self.add_bundle(commands, (
            Well {
                water_capacity: 1.0,
                consumption_rate: 0.02,
            },
            Resource {
                resource_type: ResourceType::Water,
                availability: 1.0,
                max_interactions: 5,
                current_interactions: 0,
                regeneration_rate: 0.02,
                regeneration_timer: 0.0,
            },
        ));

        builder.transform_to()
    }
}

impl WellResourceExt for WellBuilder<Present, Missing> {
    fn with_well_visual(self, commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2) -> ValidatedWell {
        let builder = self.add_bundle(commands, (
            Sprite {
                image: asset_server.load("well.png"),
                ..default()
            },
            Name::new("Well"),
            Transform::from_xyz(position.x, position.y, 0.0),
        ));

        builder.transform_to()
    }
}

impl ValidatedEnvironmentalBuilder for ValidatedWell {
    fn build(self) -> Entity {
        self.entity()
    }
}

// =============================================================================
// RESTAURANT BUILDER IMPLEMENTATIONS
// =============================================================================

impl RestaurantBuilderExt for EmptyBuilder {
    fn with_restaurant_resource(self, commands: &mut Commands) -> RestaurantBuilder<Present, Missing> {
        let builder = self.add_bundle(commands, (
            Restaurant {
                food_capacity: 1.0,
                consumption_rate: 0.01,
            },
            Resource {
                resource_type: ResourceType::Food,
                availability: 1.0,
                max_interactions: 6,
                current_interactions: 0,
                regeneration_rate: 0.01,
                regeneration_timer: 0.0,
            },
        ));

        builder.transform_to()
    }
}

impl RestaurantResourceExt for RestaurantBuilder<Present, Missing> {
    fn with_restaurant_visual(self, commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2) -> ValidatedRestaurant {
        let builder = self.add_bundle(commands, (
            Sprite {
                image: asset_server.load("restaurant.png"),
                ..default()
            },
            Name::new("Restaurant"),
            Transform::from_xyz(position.x, position.y, 0.0),
        ));

        builder.transform_to()
    }
}

impl ValidatedEnvironmentalBuilder for ValidatedRestaurant {
    fn build(self) -> Entity {
        self.entity()
    }
}

// =============================================================================
// HOTEL BUILDER IMPLEMENTATIONS
// =============================================================================

impl HotelBuilderExt for EmptyBuilder {
    fn with_hotel_resource(self, commands: &mut Commands) -> HotelBuilder<Present, Missing, Missing> {
        let builder = self.add_bundle(commands, (
            Resource {
                resource_type: ResourceType::Rest,
                availability: 1.0,
                max_interactions: 8,
                current_interactions: 0,
                regeneration_rate: 0.025,
                regeneration_timer: 0.0,
            },
        ));

        builder.transform_to()
    }
}

impl HotelResourceExt for HotelBuilder<Present, Missing, Missing> {
    fn with_hotel_visual(self, commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2) -> HotelBuilder<Present, Present, Missing> {
        let builder = self.add_bundle(commands, (
            Sprite {
                image: asset_server.load("hotel.png"),
                ..default()
            },
            Name::new("Hotel"),
            Transform::from_xyz(position.x, position.y, 0.0),
        ));

        builder.transform_to()
    }
}

impl HotelVisualExt for HotelBuilder<Present, Present, Missing> {
    fn with_hotel_comfort(self, commands: &mut Commands) -> ValidatedHotel {
        let builder = self.add_bundle(commands, (
            Hotel {
                rest_capacity: 1.0,
                available_beds: 10,
                max_beds: 10,
                comfort_level: 0.8,
            },
        ));

        builder.transform_to()
    }
}

impl ValidatedEnvironmentalBuilder for ValidatedHotel {
    fn build(self) -> Entity {
        self.entity()
    }
}

// =============================================================================
// SAFE ZONE BUILDER IMPLEMENTATIONS
// =============================================================================

impl SafeZoneBuilderExt for EmptyBuilder {
    fn with_safety_zone(self, commands: &mut Commands) -> SafeZoneBuilder<Present, Missing> {
        let builder = self.add_bundle(commands, (
            SafeZone {
                safety_level: 0.9,
                influence_radius: 50.0,
                capacity: 15,
                current_occupancy: 0,
            },
        ));

        builder.transform_to()
    }
}

impl SafetyZoneExt for SafeZoneBuilder<Present, Missing> {
    fn with_safezone_visual(self, commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2) -> ValidatedSafeZone {
        let builder = self.add_bundle(commands, (
            Sprite {
                image: asset_server.load("safezone.png"),
                ..default()
            },
            Name::new("Safe Zone"),
            Transform::from_xyz(position.x, position.y, 0.0),
        ));

        builder.transform_to()
    }
}

impl ValidatedEnvironmentalBuilder for ValidatedSafeZone {
    fn build(self) -> Entity {
        self.entity()
    }
}
