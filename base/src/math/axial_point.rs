use std::fmt;
use world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS};
use super::{AxialType, DefaultFloat, Point2f};
use std::ops::{Add, Div, Mul, Sub};

/// A 2-dimensional point in axial coordinates. See [here][hex-blog] for more
/// information.
///
/// [hex-blog]: http://www.redblobgames.com/grids/hexagons/#coordinates
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, packed)]
pub struct AxialPoint {
    pub q: AxialType,
    pub r: AxialType,
}

// TODO: implement cgmath::Array
// TODO: implement cgmath::MatricSpace
// TODO: implement cgmath::EuclideanSpace
// TODO: implement ops::{ ... }
// For all of the above, see
// http://bjz.github.io/cgmath/cgmath/struct.Point2.html
//

impl AxialPoint {
    pub fn new(q: AxialType, r: AxialType) -> Self {
        AxialPoint { q: q, r: r }
    }

    /// Returns the position of the hexagons center in the standard coordinate
    /// system using `world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS}`.
    pub fn to_real(&self) -> Point2f {
        Point2f {
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

impl fmt::Debug for AxialPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.q)
            .field(&self.r)
            .finish()
    }
}

impl Add<AxialPoint> for AxialPoint {
    type Output = AxialPoint;

    /// adds two Points together similar to vectors
    /// Returns an AxialPoint
    fn add(self, _rhs: AxialPoint) -> AxialPoint {
        AxialPoint {
            q: self.q + _rhs.q,
            r: self.r + _rhs.r,
        }
    }
}

impl Sub<AxialPoint> for AxialPoint {
    type Output = AxialPoint;

    /// substracts two Points similar to vectors
    /// Returns an AxialPoint
    fn sub(self, _rhs: AxialPoint) -> AxialPoint {
        AxialPoint {
            q: self.q - _rhs.q,
            r: self.r - _rhs.r,
        }
    }
}

impl Mul<AxialType> for AxialPoint {
    type Output = AxialPoint;

    /// Multiplies a point and a scalar
    /// Returns an AxialPoint
    fn mul(self, _rhs: AxialType) -> AxialPoint {
        AxialPoint {
            q: self.q * _rhs,
            r: self.r * _rhs,
        }
    }
}

impl Div<AxialType> for AxialPoint {
    type Output = AxialPoint;

    /// Divides a point and a scalar
    /// Returns an AxialPoint
    fn div(self, _rhs: AxialType) -> AxialPoint {
        AxialPoint {
            q: self.q / _rhs,
            r: self.r / _rhs,
        }
    }
}
