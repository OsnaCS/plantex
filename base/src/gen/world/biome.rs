use world::GroundMaterial;
use gen::plant::tree::PlantType;

#[derive(Clone, Debug, PartialEq)]
pub enum Biome {
    GrassLand,
    Desert,
    Snow,
    Forest,
    RainForest,
    Savanna,
    Stone,
    Debug,
}

impl Default for Biome {
    fn default() -> Biome {
        Biome::Debug
    }
}

impl Biome {
    pub fn material(&self) -> GroundMaterial {
        match *self {
            Biome::GrassLand => GroundMaterial::Grass,
            Biome::Desert => GroundMaterial::Sand,
            Biome::Snow => GroundMaterial::Snow,
            Biome::Forest => GroundMaterial::Mulch,
            Biome::RainForest => GroundMaterial::JungleGrass,
            Biome::Savanna => GroundMaterial::Dirt,
            Biome::Stone => GroundMaterial::Stone,
            Biome::Debug => GroundMaterial::Debug,
        }
    }

    pub fn plant_threshold(&self) -> f32 {
        0.05 +
        match *self {
            Biome::GrassLand => 0.3,
            Biome::Desert => 0.46,
            Biome::Snow => 0.35,
            Biome::Forest => 0.25,
            Biome::RainForest => 0.2,
            Biome::Savanna => 0.375,
            Biome::Stone => 0.45,
            Biome::Debug => 1.0,
        }
    }

    pub fn from_climate(temperature: f32, humidity: f32) -> Biome {
        match (temperature, humidity) {
            (0.0...0.2, 0.0...0.2) => Biome::Stone,
            (0.0...0.2, 0.2...0.4) => Biome::Snow,
            (0.0...0.2, 0.4...1.0) => Biome::Snow,
            (0.2...0.4, 0.0...0.2) => Biome::GrassLand,
            (0.2...0.4, 0.2...0.4) => Biome::GrassLand,
            (0.2...0.4, 0.4...1.0) => Biome::Forest,
            (0.4...1.0, 0.0...0.2) => Biome::Desert,
            (0.4...1.0, 0.2...0.4) => Biome::Savanna,
            (0.4...1.0, 0.4...1.0) => Biome::RainForest,
            _ => Biome::Debug,
        }
    }

    pub fn plant_distribution(&self) -> &'static [PlantType] {
        match *self {
            Biome::GrassLand => {
                static PLANTS: &'static [PlantType] = &[PlantType::RegularTree,
                                                        PlantType::RegularTree,
                                                        PlantType::OakTree,
                                                        PlantType::ClumpOfGrass,
                                                        PlantType::ClumpOfGrass];
                PLANTS
            }
            Biome::Desert => {
                static PLANTS: &'static [PlantType] = &[PlantType::Cactus];
                PLANTS
            }
            Biome::Snow => {
                static PLANTS: &'static [PlantType] = &[PlantType::Conifer];
                PLANTS
            }
            Biome::Forest => {
                static PLANTS: &'static [PlantType] = &[PlantType::RegularTree,
                                                        PlantType::RegularTree,
                                                        PlantType::Conifer,
                                                        PlantType::OakTree,
                                                        PlantType::OakTree,
                                                        PlantType::Conifer,
                                                        PlantType::ClumpOfGrass,
                                                        PlantType::Conifer,
                                                        PlantType::Shrub,
                                                        PlantType::Shrub];
                PLANTS
            }
            Biome::RainForest => {
                static PLANTS: &'static [PlantType] = &[PlantType::JungleTree,
                                                        PlantType::JungleTree,
                                                        PlantType::JungleTree,
                                                        PlantType::JungleTree,
                                                        PlantType::OakTree,
                                                        PlantType::ClumpOfGrass,
                                                        PlantType::ClumpOfGrass,
                                                        PlantType::OakTree,
                                                        PlantType::Shrub,
                                                        PlantType::RegularTree];
                PLANTS
            }
            Biome::Savanna => {
                static PLANTS: &'static [PlantType] = &[PlantType::OakTree,
                                                        PlantType::ClumpOfGrass,
                                                        PlantType::Shrub,
                                                        PlantType::Shrub,
                                                        PlantType::Shrub,
                                                        PlantType::Shrub,
                                                        PlantType::Shrub,
                                                        PlantType::Shrub,
                                                        PlantType::Shrub,
                                                        PlantType::Shrub];
                PLANTS
            }
            Biome::Stone => {
                static PLANTS: &'static [PlantType] = &[PlantType::Conifer,
                                                        PlantType::Conifer,
                                                        PlantType::Conifer,
                                                        PlantType::Conifer,
                                                        PlantType::OakTree,
                                                        PlantType::Conifer,
                                                        PlantType::Conifer,
                                                        PlantType::Conifer];
                PLANTS
            }
            Biome::Debug => {
                static PLANTS: &'static [PlantType] = &[PlantType::ClumpOfGrass];
                PLANTS
            }
        }
    }
}
