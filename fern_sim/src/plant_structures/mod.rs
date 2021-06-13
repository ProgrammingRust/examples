//! Higher-level biological structures.
//!
//! We always simulate a sample of all chemical interactions at the cellular
//! level, but simulating everything that way is just too computationally
//! expensive.  Therefore we keep higher-level data structures representing
//! each fern's roots, leaves, and so on.  When we simulate physics (light, air
//! currents, gravity) we always use these structures as shorthand for the
//! millions of cells they typically represent. On a more morbid note, these
//! structures stick around when stuff dies, so that dead fronds have weight,
//! cast shadows, and so on.

// in plant_structures/mod.rs
pub mod roots;
pub mod stems;
pub mod leaves;

pub use self::leaves::Leaf;
pub use self::roots::Root;

use self::roots::RootSet;
use self::stems::StemSet;

pub enum FernType {
    Fiddlehead
}

pub struct Fern {
    pub roots: RootSet,
    pub stems: StemSet
}

impl Fern {
    pub fn new(_type: FernType) -> Fern {
        Fern {
            roots: vec![],
            stems: vec![stems::Stem { furled: true }]
        }
    }

    pub fn is_furled(&self) -> bool { !self.is_fully_unfurled() }

    pub fn is_fully_unfurled(&self) -> bool {
        self.stems.iter().all(|s| !s.furled)
    }
}

/// Create and return a [`VascularPath`] which represents the path of
/// nutrients from the given [`Root`][r] to the given [`Leaf`](leaves::Leaf).
///
/// [r]: roots::Root
pub fn trace_path(leaf: &leaves::Leaf, root: &roots::Root) -> VascularPath {
    VascularPath { from: leaf.x, to: root.x }
}

#[doc(alias = "route")]
pub struct VascularPath {
    pub from: bool,
    pub to: bool,
}
