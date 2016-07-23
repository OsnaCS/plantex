mod tree;

use self::tree::TreeGen;
use prop::Plant;

use rand::{Rand, Rng};

/// Plant generation entry point.
///
/// This struct will randomly generate a plant using a more specific plant
/// generator.
pub enum PlantGenerator {
    Tree(TreeGen),
}

impl PlantGenerator {
    pub fn generate<R: Rng>(self, rng: &mut R) -> Plant {
        match self {
            PlantGenerator::Tree(treegen) => Plant::Tree { branches: treegen.generate(rng) },
        }
    }
}

impl Rand for PlantGenerator {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        PlantGenerator::Tree(TreeGen::rand(rng))
    }
}
