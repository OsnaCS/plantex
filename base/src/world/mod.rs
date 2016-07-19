use std::fmt;

mod chunk;
mod ground;
mod hex_pillar;
mod world;

pub use self::chunk::Chunk;
pub use self::ground::*;
pub use self::hex_pillar::*;
pub use self::world::World;

pub const HEX_INNER_RADIUS: f32 = 3.0;
pub const HEX_OUTER_RADIUS: f32 = HEX_INNER_RADIUS * (::math::SQRT_3 / 2.0);

pub const PILLAR_STEP_HEIGHT: f32 = 0.5;
pub const CHUNK_SIZE: u16 = 16;

pub type ChunkIndex = i32;

/// Represents one discrete height of a pillar section.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HeightType(pub u16);

impl HeightType {
    /// Calculates the real (world position) height from the underlying
    /// representation.
    pub fn to_real(&self) -> f32 {
        (self.0 as f32) * PILLAR_STEP_HEIGHT
    }
}

impl fmt::Display for HeightType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} -> {}]", self.0, self.to_real())
    }
}
impl fmt::Debug for HeightType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
