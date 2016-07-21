use std::fmt;
use world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS};
use super::{AxialType, DefaultFloat, Vector2f};
use math::cgmath::{VectorSpace, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
               RemAssign, Sub, SubAssign};
use math::cgmath::prelude::{Array, MetricSpace};
use std::cmp;

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

/// <summary>
/// AxialVector defines a vector specificly for Axial cordinate system.
/// </summary>
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


    /// <summary>
    /// unit_q creates an default AxialVector with q:1 r:0.
    /// </summary>
    pub fn unit_q() -> AxialVector {
        AxialVector { q: 1, r: 0 }
    }

    /// <summary>
    /// unit_r creates an default AxialVector with q:0 r:1.
    /// </summary>
    pub fn unit_r() -> AxialVector {
        AxialVector { q: 0, r: 1 }
    }
}

/// <summary>
/// implements standart debug for AxialVector
/// </summary>
impl fmt::Debug for AxialVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.q)
            .field(&self.r)
            .finish()
    }
}

// ********************* Basic Arithmetic (OPS) *********************


/// <summary>
/// implements Neg for AxialVector.
/// </summary>
impl Neg for AxialVector {
    type Output = AxialVector;

    /// <summary>
    /// returns a new negated AxialVector.
    /// </summary>
    fn neg(self) -> Self::Output {
        AxialVector {
            q: -self.q,
            r: -self.r,
        }
    }
}

/// <summary>
/// implements Add for AxialVector.
/// </summary>
impl Add<AxialVector> for AxialVector {
    type Output = AxialVector;

    /// <summary>
    /// adds two AxialVector and returns a new AxialVector with the result.
    /// </summary>
    /// <param name"arg2">second AxialVector to perform operation with.</param>
    fn add(self, arg2: AxialVector) -> AxialVector {
        AxialVector {
            r: self.r + arg2.r,
            q: self.q + arg2.q,
        }
    }
}

/// <summary>
/// implements AddAssign for AxialVector.
/// </summary>
impl AddAssign<AxialVector> for AxialVector {
    /// <summary>
    /// adds a Vector to itself.
    /// </summary>
    /// <param name"arg2">second AxialVector to perform operation with.</param>
    fn add_assign(&mut self, arg2: AxialVector) {
        self.r += arg2.r;
        self.q += arg2.q;

    }
}

/// <summary>
/// implements Sub for AxialVector.
/// </summary>
impl Sub<AxialVector> for AxialVector {
    type Output = AxialVector;

    /// <summary>
    /// subtracts two AxialVector and returns a new AxialVector with the result.
    /// </summary>
    /// <param name"arg2">second AxialVector to perform operation with.</param>
    fn sub(self, arg2: AxialVector) -> AxialVector {
        AxialVector {
            r: self.r - arg2.r,
            q: self.q - arg2.q,
        }
    }
}

/// <summary>
/// implements SubAssign for AxialVector.
/// </summary>
impl SubAssign<AxialVector> for AxialVector {
    /// <summary>
    /// subtracts AxialVector from itself.
    /// </summary>
    /// <param name"arg2">second AxialVector to perform operation with.</param>
    fn sub_assign(&mut self, arg2: AxialVector) {
        self.r -= arg2.r;
        self.q -= arg2.q;

    }
}

/// <summary>
/// implements Mul for AxialVector.
/// </summary>
impl Mul<AxialType> for AxialVector {
    type Output = AxialVector;

    /// <summary>
    /// multiplicates two AxialVector and returns a new AxialVector with the
    /// result.
    /// </summary>
    /// <param name"arg2">second AxialVector to perform operation with.</param>
    fn mul(self, arg2: AxialType) -> AxialVector {
        AxialVector {
            r: self.r * arg2,
            q: self.q * arg2,
        }
    }
}

/// <summary>
/// implements MulAssign for AxialVector.
/// </summary>
impl MulAssign<AxialType> for AxialVector {
    /// <summary>
    /// multiplicates AxialVector from itself.
    /// </summary>
    /// <param name"arg2">second AxialVector to perform operation with.</param>
    fn mul_assign(&mut self, arg2: AxialType) {
        self.r *= arg2;
        self.q *= arg2;

    }
}

/// <summary>
/// implements Div for AxialVector.
/// </summary>
impl Div<AxialType> for AxialVector {
    type Output = AxialVector;

    /// <summary>
    /// divides two AxialVector and returns a new AxialVector with the result.
    /// </summary>
    /// <param name"arg2">second AxialVector to perform operation with.</param>
    fn div(self, arg2: AxialType) -> AxialVector {
        AxialVector {
            r: self.r / arg2,
            q: self.q / arg2,
        }
    }
}

/// <summary>
/// implements DivAssign for AxialVector.
/// </summary>
impl DivAssign<AxialType> for AxialVector {
    /// <summary>
    /// divides AxialVector from itself.
    /// </summary>
    /// <param name"arg2">second AxialVector to perform operation with.</param>
    fn div_assign(&mut self, arg2: AxialType) {
        self.r /= arg2;
        self.q /= arg2;

    }
}

/// <summary>
/// implements Rem for AxialVector.
/// </summary>
impl Rem<AxialType> for AxialVector {
    type Output = AxialVector;

    /// <summary>
    /// calculates the modulo of AxialVector with given AxialType.
    /// </summary>
    /// <param name"arg2">second AxialType to perform operation with.</param>
    fn rem(self, arg2: AxialType) -> AxialVector {
        AxialVector {
            r: self.r % arg2,
            q: self.q % arg2,
        }
    }
}

/// <summary>
/// implements Rem for AxialVector.
/// </summary>
impl RemAssign<AxialType> for AxialVector {
    /// <summary>
    /// calculates the modulo of AxialVector with given AxialType and assignes
    /// it to itself.
    /// </summary>
    /// <param name"arg2">second AxialType to perform operation with.</param>
    fn rem_assign(&mut self, arg2: AxialType) {
        self.r %= arg2;
        self.q %= arg2;

    }
}


// ************ Metric Space ************

/// <summary>
/// implements Rem for AxialVector.
/// </summary>
impl MetricSpace for AxialVector {
    type Metric = DefaultFloat;
    /// <summary>
    /// calculates the distance betweem given AxialVector.
    /// </summary>
    /// <param name"arg2">second AxialType to perform operation with.</param>
    fn distance(self, other: AxialVector) -> DefaultFloat {
        (self.distance2(other)).sqrt()
    }
    /// <summary>
    /// calculates the distance betweem given AxialVector without using sqrt.
    /// </summary>
    /// <param name"arg2">second AxialType to perform operation with.</param>
    fn distance2(self, other: AxialVector) -> DefaultFloat {
        ((self.q - other.q).pow(2) + (self.r - other.r).pow(2)) as DefaultFloat
    }
}

// ************ Vector Space ************

/// <summary>
/// implements VectorSpace for AxialVector.
/// </summary>
impl VectorSpace for AxialVector {
    type Scalar = AxialType;
}


// ************** Zero ****************

/// <summary>
/// implements Zero for AxialVector.
/// </summary>
impl Zero for AxialVector {
    /// <summary>
    /// returns a neutral AxialVector.
    /// </summary>
    fn zero() -> AxialVector {
        AxialVector { q: 0, r: 0 }
    }

    /// <summary>
    /// checks if AxialVector is neutral element.
    /// </summary>
    fn is_zero(&self) -> bool {
        self.q == 0 && self.r == 0
    }
}


// *************** Array & Index *******************

/// <summary>
/// implements Zero for AxialVector.
/// </summary>
impl Array for AxialVector {
    type Element = AxialType;

    /// <summary>
    /// creates an AxialVector from a given value.
    /// </summary>
    /// <param name"value">value from which AxialVector is created.</param>
    fn from_value(value: Self::Element) -> Self {
        AxialVector::new(value, value)
    }

    /// <summary>
    /// sums up all elements in AxialVector and returns them.
    /// </summary>
    fn sum(self) -> Self::Element {
        self.q + self.r
    }

    /// <summary>
    /// multiplicates up all elements in AxialVector and returns them.
    /// </summary>
    fn product(self) -> Self::Element {
        self.q * self.r
    }

    /// <summary>
    /// returns smallest element in AxialVector.
    /// </summary>
    fn min(self) -> Self::Element {
        cmp::min(self.q, self.r)
    }

    /// <summary>
    /// returns largest element in AxialVector.
    /// </summary>
    fn max(self) -> Self::Element {
        cmp::max(self.q, self.r)
    }
}

/// <summary>
/// implements Index for AxialVector.
/// </summary>
impl Index<usize> for AxialVector {
    type Output = AxialType;

    /// <summary>
    /// returns AxialType of the given index number.
    /// </summary>
    /// <param name"index">element index number.</param>
    fn index(&self, index: usize) -> &Self::Output {
        let ret: &AxialType = match index {
            0 => &self.q,
            1 => &self.r,
            _ => panic!("Illegal Index Argument: was {:?}", index),
        };
        ret
    }
}

/// <summary>
/// implements IndexMut for AxialVector.
/// </summary>
impl IndexMut<usize> for AxialVector {
    /// <summary>
    /// returns a mutable AxialType of the given index number.
    /// </summary>
    /// <param name"index">element index number.</param>
    fn index_mut(&mut self, index: usize) -> &mut AxialType {
        match index {
            0 => &mut self.q,
            1 => &mut self.r,
            _ => panic!("Illegal Index Argument: was {:?}", index),
        }
    }
}
