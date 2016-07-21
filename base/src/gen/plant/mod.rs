mod tree;

use self::tree::TreeGen;

use rand::{Rng, Rand};

/// Plant generation entry point.
///
/// This struct will randomly generate a plant using a more specific plant generator
pub enum PlantGenerator {
    Tree(TreeGen),
}

impl Rand for PlantGenerator {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        PlantGenerator::Tree(TreeGen::rand(rng))
    }
}
