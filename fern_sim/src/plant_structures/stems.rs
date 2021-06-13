//! Stems hold the weight of the plant and are largely responsible for its
//! shape.  Parameters on `Stem` (not `Leaf`) are responsible for pinnation,
//! the feathery leaf structure that's the most immediately recognizable
//! property of ferns.

// in plant_structures/stems.rs
pub mod xylem;
pub mod phloem;

pub struct Stem {
    pub furled: bool
}

pub type StemSet = Vec<Stem>;
