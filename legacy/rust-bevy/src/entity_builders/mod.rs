pub mod entity_builders_default;
pub mod generic_type_safe_builder;

// Domain-specific entity state definitions (no component imports)
pub mod npc_entity_domain;
pub mod environmental_entity_domains;

// Implementation files with actual component imports and logic
pub mod npc_builder_implementations;
pub mod environmental_builder_implementations;
