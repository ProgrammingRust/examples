//! Simulate the growth of ferns, from the level of
//! individual cells on up.

pub mod cells;
pub mod plant_structures;
pub mod simulation;
pub mod spores;

pub use plant_structures::Fern;
pub use simulation::Terrarium;

pub mod net;
pub use net::connect;
