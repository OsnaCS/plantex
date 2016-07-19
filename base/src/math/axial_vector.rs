use std::fmt;
use world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS};
use super::{AxialType, DefaultFloat, Vector2f};

/// A 2-dimensional vector in axial coordinates. See [here][hex-blog] for more
/// information.
///
/// [hex-blog]: http://www.redblobgames.com/grids/hexagons/#coordinates
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, packed)]
pub struct AxialVector {
    pub q: AxialType,
    pub r: AxialType,
}

// TODO: implement cgmath::Zero
// TODO: implement cgmath::Array
// TODO: implement cgmath::MatricSpace
// TODO: implement cgmath::VectorSpace
// TODO: implement cgmath::InnerSpace
// TODO: implement ops::{ ... }
// TODO: Add `unit_q()` and `unit_r()` (see `Vector2::unit_x()` for reference)
// For all of the above, see
// http://bjz.github.io/cgmath/cgmath/struct.Vector2.html
//

impl AxialVector {
    pub fn new(q: AxialType, r: AxialType) -> Self {
        AxialVector { q: q, r: r }
    }

    /// Returns the position of the hexagons center in the standard coordinate
    /// system using `world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS}`.
    pub fn to_real(&self) -> Vector2f {
        Vector2f {
            x: ((2 * self.q + self.r) as DefaultFloat) * HEX_INNER_RADIUS,
            y: (self.r as DefaultFloat) * (3.0 / 2.0) * HEX_OUTER_RADIUS,
        }
    }

    /// Returns the `s` component of corresponding cube coordinates. In cube
    /// coordinates 'q + r + s = 0', so saving `s` is redundant and can be
    /// calculated on the fly when needed.
    pub fn s(&self) -> AxialType {
        -self.q - self.r
    }
}

impl fmt::Debug for AxialVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.q)
            .field(&self.r)
            .finish()
    }
}
