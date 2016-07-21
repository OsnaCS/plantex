use std::fmt;
use std::cmp::{max, min};
use world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS};
use super::{AxialType, DefaultFloat, Point2f};
use std::ops::{Add, Div, Index, IndexMut, Mul, Rem, Sub};
use math::cgmath::{Array, EuclideanSpace, MetricSpace};
use super::AxialVector;
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
impl Add<AxialVector> for AxialPoint {
    type Output = AxialPoint;
    fn add(self, rhs: AxialVector) -> AxialPoint {
        AxialPoint {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}
impl Sub<AxialPoint> for AxialPoint {
    type Output = AxialVector;
    fn sub(self, rhs: AxialPoint) -> AxialVector {
        AxialVector {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}
impl Mul<AxialType> for AxialPoint {
    type Output = AxialPoint;
    fn mul(self, rhs: AxialType) -> AxialPoint {
        AxialPoint {
            q: self.q * rhs,
            r: self.r * rhs,
        }
    }
}
impl Div<AxialType> for AxialPoint {
    type Output = AxialPoint;
    fn div(self, rhs: AxialType) -> AxialPoint {
        AxialPoint {
            q: self.q / rhs,
            r: self.r / rhs,
        }
    }
}
impl Rem<AxialType> for AxialPoint {
    type Output = AxialPoint;
    fn rem(self, d: AxialType) -> AxialPoint {
        AxialPoint {
            q: self.q % d,
            r: self.r % d,
        }
    }
}
/// ********************Index************
impl Index<usize> for AxialPoint {
    type Output = AxialType;
    fn index(&self, index: usize) -> &AxialType {
        match index {
            0 => &self.q,
            1 => &self.r,
            _ => panic!("Index out of bounds!"),
        }
    }
}
impl IndexMut<usize> for AxialPoint {
    fn index_mut(&mut self, index: usize) -> &mut AxialType {
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
        min(self.q, self.r)
    }
    fn max(self) -> AxialType {
        max(self.q, self.r)
    }
}
/// ******************* Metric-Space************************
impl MetricSpace for AxialPoint {
    type Metric = f32;
    fn distance2(self, other: AxialPoint) -> Self::Metric {
        (((self.q - other.q) * (self.q - other.q)) +
         ((self.r - other.r) * (self.r - other.r))) as f32
    }
    fn distance(self, other: AxialPoint) -> Self::Metric {
        self.distance2(other).sqrt()
    }
}
/// ******************* EuclideanSpace**********************
impl EuclideanSpace for AxialPoint {
    type Scalar = i32;
    type Diff = AxialVector;
    fn origin() -> Self {
        AxialPoint { q: 0, r: 0 }
    }
    fn from_vec(v: Self::Diff) -> Self {
        AxialPoint { q: v.q, r: v.r }
    }
    fn to_vec(self) -> Self::Diff {
        AxialVector {
            q: self.q,
            r: self.r,
        }
    }
    fn dot(self, v: Self::Diff) -> Self::Scalar {
        (self.q * v.q) + (self.r * v.r)
    }
}

#[test]
fn test() {
    let a: AxialPoint = AxialPoint { q: 5, r: 7 };
    let mut b: AxialPoint = AxialPoint { q: -2, r: 0 };
    let v: AxialVector = AxialVector { q: 8, r: 1 };
    assert!(a.add(v) == AxialPoint { q: 13, r: 8 });
    assert!(a.sub(b) == AxialVector { q: 7, r: 7 });
    assert!(b.mul(4) == AxialPoint { q: -8, r: 0 });
    assert!(b.div(2) == AxialPoint { q: -1, r: 0 });
    assert!(a.rem(3) == AxialPoint { q: 2, r: 1 });
    assert!(a[0] == 5);
    assert!(a[1] == 7);
    b[0] = 5;
    b[1] = 5;
    assert!(b.q == 5 && b.r == 5);
    let c = AxialPoint::from_value(7);
    assert!(c.q == 7 && c.r == 7);
    assert!(c.sum() == 14);
    assert!(c.product() == 49);
    assert!(a.min() == 5);
    assert!(a.max() == 7);
    let d = AxialPoint { q: 0, r: 0 };
    let e = AxialPoint { q: 1, r: 1 };
    let f = 2.0;
    assert!(d.distance2(e) == f);
    assert!(d.distance(e) == f.sqrt());
    assert!(AxialPoint::from_vec(v) == AxialPoint { q: 8, r: 1 });
    assert!(e.to_vec() == AxialVector { q: 1, r: 1 });
    assert!(e.dot(v) == 9);
}
