use world::GroundMaterial;

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
}
