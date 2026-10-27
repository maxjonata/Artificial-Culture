/// Helper modules for domain-specific utility functions
/// Following data-oriented design principles with pure functions

pub mod movement_helpers;
pub mod needs_helpers;
pub mod pathfinding_helpers;
pub mod resource_helpers;
pub mod rumor_helpers;

// Re-export commonly used functions for convenience
pub use movement_helpers::*;
pub use pathfinding_helpers::*;
pub use rumor_helpers::*;
