pub mod tree;

use self::tree::{PlantType, TreeGen};
use prop::Plant;
use rand::Rng;

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
            PlantGenerator::Tree(treegen) => Plant::Tree(treegen.generate(rng)),
        }
    }

    pub fn new(plant_type: PlantType) -> Self {
        PlantGenerator::Tree(TreeGen::new(plant_type))
    }
}
