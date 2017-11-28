#![allow(dead_code, unused_variables)]

//! Fern reproduction.

use cells::Cell;

/// A cell made by an adult fern. It disperses on the wind as part of
/// the fern life cycle. A spore grows into a prothallus -- a whole
/// separate organism, up to 5mm across -- which produces a zygote,
/// which becomes a new fern. (Plant sex is complicated.)
pub struct Spore {
    x: bool
}

/// A compartment, usually on the bottom of a leaf, where spores form.
pub struct Sporangium {
    x: bool
}

/// Simulate the production of a spore by meiosis.
pub fn produce_spore(factory: &mut Sporangium) -> Spore {
    Spore { x: false }
}

/// Mix genes to prepare for meiosis (part of interphase).
fn recombine(parent: &mut Cell) {
}

