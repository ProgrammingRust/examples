#![allow(dead_code, unused_variables)]

//! Fern reproduction.

use cells::{Cell, Gene};

/// A cell made by an adult fern. It disperses on the wind as part of
/// the fern life cycle. A spore grows into a prothallus -- a whole
/// separate organism, up to 5mm across -- which produces the zygote
/// that grows into a new fern. (Plant sex is complicated.)
pub struct Spore {
    size: f64
}

/// Simulate the production of a spore by meiosis.
pub fn produce_spore(factory: &mut Sporangium) -> Spore {
    Spore { size: 1.0 }
}

/// Extract the genes in a particular spore.
pub(crate) fn genes(spore: &Spore) -> Vec<Gene> {
    todo!()
}

/// Mix genes to prepare for meiosis (part of interphase).
fn recombine(parent: &mut Cell) {
    todo!()
}

pub struct Sporangium;

mod cells {
    //! The simulation of biological cells, which is as low-level as we go.

    pub struct Cell {
        x: f64,
        y: f64
    }

    impl Cell {
        pub fn distance_from_origin(&self) -> f64 {
            f64::hypot(self.x, self.y)
        }
    }

    pub struct Gene;
}
