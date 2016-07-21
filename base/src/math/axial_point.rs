use std::fmt;
use world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS};
use super::{AxialType, DefaultFloat, Point2f};
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use math::cgmath::{Array, Zero};

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

/// ********************Basic Arithmetics************

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
/// ********************Zero-Implementation************
impl Zero for AxialPoint {
    fn zero() -> AxialPoint {
        AxialPoint { q: 0, r: 0 }
    }
    fn is_zero(&self) -> bool {
        self.q == 0 && self.r == 0
    }
}

/// ********************Index************

impl Index<usize> for AxialPoint {
    type Output = AxialType;
    fn index<'a>(&'a self, index: usize) -> &'a AxialType {
        match index {
            0 => &self.q,
            1 => &self.r,
            _ => panic!("Index out of bounds!"),
        }
    }
}

impl IndexMut<usize> for AxialPoint {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut AxialType {
        match index {
            0 => &mut self.q,
            1 => &mut self.r,
            _ => panic!("Index out of bounds!"),
        }
    }
}

/// ********************Array************

impl Array for AxialPoint {
    type Element = AxialType;

    fn from_value(x: AxialType) -> AxialPoint {
        AxialPoint { q: x, r: x }
    }

    fn sum(self) -> AxialType {
        self.q + self.r
    }

    fn product(self) -> AxialType {
        self.q * self.r
    }

    fn min(self) -> AxialType {
        if self.q < self.r { self.q } else { self.r }
    }
    fn max(self) -> AxialType {
        if self.q < self.r { self.r } else { self.q }
    }
}
