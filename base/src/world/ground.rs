#[derive(Clone, Debug)]
pub enum GroundMaterial {
    Dirt,
    Grass,
    Stone,
    Sand,
    Snow,
    Jungle,
    Mulch,
    Grey(f32),
    Color(f32, f32, f32),
}

impl GroundMaterial {
    // Returns color of Texture in RGB
    pub fn get_color(&self) -> [f32; 3] {
        match *self {
            GroundMaterial::Dirt => [0.38, 0.13, 0.03],
            GroundMaterial::Grass => [0.0, 0.5, 0.0],
            GroundMaterial::Stone => [0.3, 0.3, 0.3],
            GroundMaterial::Grey(greyv) => [greyv, greyv, greyv],
            GroundMaterial::Color(r, g, b) => [r, g, b],
            GroundMaterial::Snow => [0.95, 0.95, 1.0],
            GroundMaterial::Sand => [1.0, 1.0, 0.0],
            GroundMaterial::Jungle => [0.1, 0.4, 0.2],
            GroundMaterial::Mulch => [0.5, 0.1, 0.1],
        }
    }
}
