use crate::entity_builders::generic_type_safe_builder::{validation_states::*, TypeSafeEntityBuilder};
use bevy::prelude::*;
use std::marker::PhantomData;

/// Environmental entity state validation types
/// These define the compile-time requirements for environmental entities

// =============================================================================
// WELL ENTITY DOMAIN
// =============================================================================

/// Well state marker - tracks 2 required components (Resource + Visual)
pub struct WellState<Resource, Visual> {
    pub _resource: PhantomData<Resource>,
    pub _visual: PhantomData<Visual>,
}

/// Type aliases for Well builder states
pub type WellBuilder<Resource, Visual> =
TypeSafeEntityBuilder<WellState<Resource, Visual>>;

/// Type alias for a fully validated Well
pub type ValidatedWell = WellBuilder<Present, Present>;

/// Extension traits for Well building
pub trait WellBuilderExt {
    fn with_well_resource(self, commands: &mut Commands) -> WellBuilder<Present, Missing>;
}

pub trait WellResourceExt {
    fn with_well_visual(self, commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2) -> ValidatedWell;
}

// =============================================================================
// RESTAURANT ENTITY DOMAIN
// =============================================================================

/// Restaurant state marker - tracks 2 required components (Resource + Visual)
pub struct RestaurantState<Resource, Visual> {
    pub _resource: PhantomData<Resource>,
    pub _visual: PhantomData<Visual>,
}

/// Type aliases for Restaurant builder states
pub type RestaurantBuilder<Resource, Visual> =
TypeSafeEntityBuilder<RestaurantState<Resource, Visual>>;

/// Type alias for a fully validated Restaurant
pub type ValidatedRestaurant = RestaurantBuilder<Present, Present>;

/// Extension traits for Restaurant building
pub trait RestaurantBuilderExt {
    fn with_restaurant_resource(self, commands: &mut Commands) -> RestaurantBuilder<Present, Missing>;
}

pub trait RestaurantResourceExt {
    fn with_restaurant_visual(self, commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2) -> ValidatedRestaurant;
}

// =============================================================================
// HOTEL ENTITY DOMAIN
// =============================================================================

/// Hotel state marker - tracks 3 required components (Resource + Visual + Comfort)
pub struct HotelState<Resource, Visual, Comfort> {
    pub _resource: PhantomData<Resource>,
    pub _visual: PhantomData<Visual>,
    pub _comfort: PhantomData<Comfort>,
}

/// Type aliases for Hotel builder states
pub type HotelBuilder<Resource, Visual, Comfort> =
TypeSafeEntityBuilder<HotelState<Resource, Visual, Comfort>>;

/// Type alias for a fully validated Hotel
pub type ValidatedHotel = HotelBuilder<Present, Present, Present>;

/// Extension traits for Hotel building
pub trait HotelBuilderExt {
    fn with_hotel_resource(self, commands: &mut Commands) -> HotelBuilder<Present, Missing, Missing>;
}

pub trait HotelResourceExt {
    fn with_hotel_visual(self, commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2) -> HotelBuilder<Present, Present, Missing>;
}

pub trait HotelVisualExt {
    fn with_hotel_comfort(self, commands: &mut Commands) -> ValidatedHotel;
}

// =============================================================================
// SAFE ZONE ENTITY DOMAIN
// =============================================================================

/// SafeZone state marker - tracks 2 required components (Safety + Visual)
pub struct SafeZoneState<Safety, Visual> {
    pub _safety: PhantomData<Safety>,
    pub _visual: PhantomData<Visual>,
}

/// Type aliases for SafeZone builder states
pub type SafeZoneBuilder<Safety, Visual> =
TypeSafeEntityBuilder<SafeZoneState<Safety, Visual>>;

/// Type alias for a fully validated SafeZone
pub type ValidatedSafeZone = SafeZoneBuilder<Present, Present>;

/// Extension traits for SafeZone building
pub trait SafeZoneBuilderExt {
    fn with_safety_zone(self, commands: &mut Commands) -> SafeZoneBuilder<Present, Missing>;
}

pub trait SafetyZoneExt {
    fn with_safezone_visual(self, commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2) -> ValidatedSafeZone;
}

// =============================================================================
// VALIDATED BUILDER TRAITS
// =============================================================================

/// Generic trait for all validated environmental entity builders
pub trait ValidatedEnvironmentalBuilder {
    fn build(self) -> Entity;
}
