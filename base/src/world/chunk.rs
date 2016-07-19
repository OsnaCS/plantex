use super::{CHUNK_SIZE, HexPillar, PillarIndexComponent};
use std::ops;
use std::iter;
use math::*;

/// Represents one part of the game world.
///
/// A chunk saves `CHUNK_SIZE`² many hex pillars which are arranged rougly in
/// the form of a parallelogram. See [this blog post][1] for more information
/// (the shape is called "rhombus" there).
///
/// This type implements the `Index` trait and can be indexed with an
/// `AxialPoint`.
///
/// [1]: http://www.redblobgames.com/grids/hexagons/#map-storage
pub struct Chunk {
    /// All pillars are layed out in this one dimensional vector which saves
    /// all rows (same r-value) consecutive.
    pillars: Vec<HexPillar>,
}

impl Chunk {
    /// Creates a dummy chunk for early testing. FIXME: remove
    pub fn dummy() -> Self {
        let pillars = iter::repeat(HexPillar::dummy())
            .take(CHUNK_SIZE.pow(2) as usize)
            .collect();
        Chunk { pillars: pillars }
    }

    pub fn pillars(&self) -> &[HexPillar] {
        &self.pillars
    }
}

impl ops::Index<AxialPoint> for Chunk {
    type Output = HexPillar;

    fn index(&self, pos: AxialPoint) -> &Self::Output {
        let chunk_size: PillarIndexComponent = CHUNK_SIZE.into();
        assert!(pos.q >= 0 && pos.q < chunk_size && pos.r >= 0 && pos.r < chunk_size,
                "axial position to index `Chunk` are out of bounds: {:?}",
                pos);

        &self.pillars[(pos.r as usize) * (CHUNK_SIZE as usize) + (pos.q as usize)]
    }
}
