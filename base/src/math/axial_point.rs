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

    /// Return the `AxialPoint` from a `Point2f`
    pub fn from_real(real: Point2f) -> Self {
        let q: f32 = (real.x * ::math::SQRT_3 / 3.0 - real.y / 3.0) / HEX_OUTER_RADIUS;
        let r: f32 = (real.y * 2.0 / 3.0) / HEX_OUTER_RADIUS;

        let y: f32 = -q - r;

        // Rounding
        let mut rx: i32 = (q + 0.5) as i32;
        let ry: i32 = (y + 0.5) as i32;
        let mut rz: i32 = (r + 0.5) as i32;

        // To test the right rounding
        let x_diff = (rx as f32 - q).abs();
        let y_diff = (ry as f32 - y).abs();
        let z_diff = (rz as f32 - r).abs();
        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff <= z_diff {
            rz = -rx - ry;
        }

        AxialPoint { q: rx, r: rz }
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
fn arithmetic_test_point() {
    let a: AxialPoint = AxialPoint { q: 5, r: 7 };
    let b: AxialPoint = AxialPoint { q: -2, r: 0 };
    let v: AxialVector = AxialVector { q: 8, r: 1 };
    assert!(a.add(v) == AxialPoint { q: 13, r: 8 });
    assert!(a.sub(b) == AxialVector { q: 7, r: 7 });
    assert!(b.mul(4) == AxialPoint { q: -8, r: 0 });
    assert!(b.div(2) == AxialPoint { q: -1, r: 0 });
    assert!(a.rem(3) == AxialPoint { q: 2, r: 1 });
}
#[test]
fn index_test_point() {
    let a: AxialPoint = AxialPoint { q: 5, r: 7 };
    let mut b: AxialPoint = AxialPoint { q: -2, r: 0 };
    assert!(a[0] == 5);
    assert!(a[1] == 7);
    b[0] = 5;
    b[1] = 5;
    assert!(b.q == 5 && b.r == 5);
}
#[test]
fn array_test_point() {
    let a = AxialPoint::from_value(7);
    let b: AxialPoint = AxialPoint { q: 5, r: 7 };
    assert!(a.q == 7 && a.r == 7);
    assert!(a.sum() == 14);
    assert!(a.product() == 49);
    assert!(b.min() == 5);
    assert!(b.max() == 7);
}
#[test]
fn meticspace_test_point() {
    let a = AxialPoint { q: 0, r: 0 };
    let b = AxialPoint { q: 1, r: 1 };
    assert!(a.distance2(b) == 2.0);
}
#[test]
fn euclideanspace_test_point() {
    let v: AxialVector = AxialVector { q: 8, r: 1 };
    let a = AxialPoint { q: 1, r: 1 };
    assert!(AxialPoint::from_vec(v) == AxialPoint { q: 8, r: 1 });
    assert!(a.to_vec() == AxialVector { q: 1, r: 1 });
    assert!(a.dot(v) == 9);
}
