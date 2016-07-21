//! Types and constants to represent a game world.
//!
use std::fmt;
use math;

mod chunk;
mod ground;
mod hex_pillar;
mod provider;
mod world;

pub use self::chunk::Chunk;
pub use self::ground::*;
pub use self::hex_pillar::*;
pub use self::provider::*;
pub use self::world::World;

/// Outer radius of the hexagons (from center to corner)
pub const HEX_OUTER_RADIUS: f32 = 5.0;

/// Inner radius of the hexagons
pub const HEX_INNER_RADIUS: f32 = HEX_OUTER_RADIUS * (::math::SQRT_3 / 2.0);

/// The height of the hex pillars is discretized. So instead of saving a `f32`
/// to represent the height, we have fixed steps of heights and we will just
/// save a `u16` to represent the height.
pub const PILLAR_STEP_HEIGHT: f32 = 5.0;

/// How many hex pillars a chunk is long. So the number of hex pillars in a
/// chunk is `CHUNK_SIZE`².
pub const CHUNK_SIZE: u16 = 16;

/// This type is used to index into one dimension of the world. Thus we can
/// "only" index `(PillarIndexComponent::max_value() -
/// PillarIndexComponent::min_value())`² many hex pillars.
pub type PillarIndexComponent = math::AxialType;

/// A new-type to index Pillars. Always represents a specific Pillar in
/// absolute world coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PillarIndex(pub math::AxialPoint);

/// A new-type to index chunks. This is different from the `PillarIndex` type
/// which always represents a pillar position. So two different `PillarIndex`es
/// could refer to two pillars in the same chunk, while two different
/// `ChunkIndex`es always refer to two different chunks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkIndex(pub math::AxialPoint);

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

impl fmt::Debug for HeightType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} -> {}]", self.0, self.to_real())
    }
}
