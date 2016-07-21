

use super::{BaseNum, PartialOrd};
use std::ops::{Div, Mul};
use num_traits::Zero;

/// A two-dimensional dimension.
#[derive(Debug,Clone,Copy)]
pub struct Dimension2<T: BaseNum> {
    pub width: T,
    pub height: T,
}

pub type Dimension2f = Dimension2<super::DefaultFloat>;
pub type Dimension2i = Dimension2<super::DefaultInt>;
pub type Dimension2u = Dimension2<super::DefaultUnsigned>;

impl<T: BaseNum> Dimension2<T> {
    pub fn new(width: T, height: T) -> Self {
        Dimension2 {
            width: width,
            height: height,
        }
    }
    /// returns the area of a Dimension<T>
    pub fn area(&self) -> <T as Mul>::Output {
        self.width * self.height
    }
    /// scales the Dimension2 with a scalar
    pub fn scale(&self, scale: T) -> Dimension2<T> {
        Dimension2 {
            width: self.width * scale,
            height: self.height * scale,
        }
    }
    /// calculates the aspectratio of a Dimension
    pub fn aspect_ratio(&self) -> <T as Div>::Output {
        assert!(!self.height.is_zero());
        self.width / self.height
    }
    /// Shrinks a dimension until it fits into another dimension
    pub fn fitting(&self, other: Dimension2<T>) -> Dimension2<T> {
        let scale = PartialOrd::partial_min(other.width / self.width, other.height / self.height);
        self.scale(scale)

    }

    /// Expands a dimension until it fills another dimension
    pub fn filling(&self, other: Dimension2<T>) -> Dimension2<T> {
        let scale = PartialOrd::partial_max(other.width / self.width, other.height / self.height);
        self.scale(scale)
    }
}


#[test]
fn test_area() {
    let test1 = Dimension2 {
        width: 3,
        height: 5,
    };
    let test2 = Dimension2 {
        width: 0,
        height: 0,
    };
    assert_eq!(test2.area(), 0);
    assert_eq!(test1.area(), 15);
}
#[test]
fn test_scale() {
    let test1 = Dimension2 {
        width: 3,
        height: 5,
    };
    let test2 = Dimension2 {
        width: 12,
        height: 20,
    };
    let scale = 4;
    let scale1 = 0;
    assert_eq!(test1.scale(scale).width, 12);
    assert_eq!(test2.scale(scale1).width, 0);
}
#[test]
fn test_aspect_ratio() {
    let test1 = Dimension2 {
        width: 3,
        height: 5,
    };
    // let test2 = Dimension2 {
    //     width: 3,
    //     height: 0,
    // };

    assert_eq!(test1.aspect_ratio(), 3 / 5);
}
#[test]
fn test_fitting() {
    let test1 = Dimension2 {
        width: 2.0,
        height: 1.0,
    };
    let test2 = Dimension2 {
        width: 4.0,
        height: 3.0,
    };
    assert_eq!(test2.fitting(test1).width, 2.0);
    assert_eq!(test2.fitting(test1).height, 1.5);
}
#[test]
fn test_filling() {
    let test1 = Dimension2 {
        width: 2.0,
        height: 1.0,
    };
    let test2 = Dimension2 {
        width: 4.0,
        height: 3.0,
    };
    assert_eq!(test1.filling(test2).width, 6.0);
    assert_eq!(test1.filling(test2).height, 3.0);
}
