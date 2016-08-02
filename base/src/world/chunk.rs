use super::{CHUNK_SIZE, ChunkIndex, HexPillar, PillarIndexComponent};
use std::ops;
use math::*;
use std::iter::Iterator;

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
#[derive(Debug)]
pub struct Chunk {
    /// All pillars are layed out in this one dimensional vector which saves
    /// all rows (same r-value) consecutive.
    pub pillars: Vec<HexPillar>,
}

pub struct ChunkPillars<'a> {
    pub pillars: &'a [HexPillar],
    pub i: u16,
}

impl<'a> Iterator for ChunkPillars<'a> {
    type Item = (AxialVector, &'a HexPillar);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < CHUNK_SIZE * CHUNK_SIZE {
            let axial = AxialVector::new((self.i / CHUNK_SIZE).into(),
                                         (self.i % CHUNK_SIZE).into());
            let item = (axial, &self.pillars[self.i as usize]);
            self.i += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl Chunk {
    /// Creates a chunk from a `Vec<HexPillar>`
    pub fn from_pillars(pillars: Vec<HexPillar>) -> Self {
        assert_eq!(pillars.len() as usize, CHUNK_SIZE.pow(2) as usize);

        Chunk { pillars: pillars }
    }

    pub fn pillars(&self) -> ChunkPillars {
        ChunkPillars {
            pillars: &self.pillars,
            i: 0,
        }
    }

    /// Safer method to get through a chunk with an ìndex
    pub fn get(&self, pos: AxialPoint) -> Option<&HexPillar> {
        let chunk_size: PillarIndexComponent = CHUNK_SIZE.into();
        if pos.q >= 0 && pos.q < chunk_size && pos.r >= 0 && pos.r < chunk_size {
            Some(&self.pillars[(pos.r as usize) * (CHUNK_SIZE as usize) + (pos.q as usize)])
        } else {
            None
        }
    }

    /// Safer method to get through a chunk with an ìndex
    pub fn get_mut(&mut self, pos: AxialPoint) -> Option<&mut HexPillar> {
        let chunk_size: PillarIndexComponent = CHUNK_SIZE.into();
        if pos.q >= 0 && pos.q < chunk_size && pos.r >= 0 && pos.r < chunk_size {
            Some(&mut self.pillars[(pos.r as usize) * (CHUNK_SIZE as usize) + (pos.q as usize)])
        } else {
            None
        }
    }

    /// Calls the given closure with all pillar positions
    /// that are contained in a `Chunk`
    pub fn for_pillars_positions<F>(mut func: F)
        where F: FnMut(AxialPoint)
    {
        for q in 0..CHUNK_SIZE {
            for r in 0..CHUNK_SIZE {
                let pos = AxialPoint::new(q.into(), r.into());
                func(pos);
            }
        }
    }

    /// Creates a `Chunk` using individual pillars returned by a closure
    pub fn with_pillars<F>(chunk_index: ChunkIndex, mut func: F) -> Chunk
        where F: FnMut(AxialPoint) -> HexPillar
    {
        let mut hec = Vec::new();
        let start_q = CHUNK_SIZE as i32 * chunk_index.0.q;
        let start_r = CHUNK_SIZE as i32 * chunk_index.0.r;

        for q in start_q..start_q + CHUNK_SIZE as i32 {
            for r in start_r..start_r + CHUNK_SIZE as i32 {
                let pos = AxialPoint::new(q.into(), r.into());
                hec.push(func(pos));
            }
        }
        Chunk { pillars: hec }
    }
}

impl ops::Index<AxialPoint> for Chunk {
    type Output = HexPillar;

    fn index(&self, pos: AxialPoint) -> &Self::Output {
        self.get(pos).unwrap_or_else(|| {
            panic!("Index out of Bounds length is: {} index was {:?}",
                   self.pillars.len(),
                   pos)
        })
    }
}

impl ops::IndexMut<AxialPoint> for Chunk {
    fn index_mut(&mut self, pos: AxialPoint) -> &mut Self::Output {
        self.get_mut(pos).unwrap()
    }
}

