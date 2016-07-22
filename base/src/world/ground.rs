#[derive(Clone, Debug)]
pub enum GroundMaterial {
    Dirt,
    Grass,
    Stone,
}

impl GroundMaterial {
    // Returns color of Texture in RGBA
    pub fn get_color(&self) -> [f32; 4] {
        match *self {
            GroundMaterial::Dirt => [0.37, 0.13, 0.001, 1.0],
            GroundMaterial::Grass => [0.0, 0.5, 0.0, 1.0],
            GroundMaterial::Stone => [0.5, 0.5, 0.5, 1.0],
        }
    }
}
