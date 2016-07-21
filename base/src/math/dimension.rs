

use super::{BaseNum, PartialOrd};
use std::ops::{Div, Mul};
use num_traits::Zero;

/// A two-dimensional dimension.
#[derive(Debug)]
pub struct Dimension2<T> {
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
    ///
    pub fn area(&self) -> <T as Mul>::Output {
        self.width * self.height
    }
    pub fn scale(&self, scale: T) -> Dimension2<T> {
        Dimension2 {
            width: self.width * scale,
            height: self.height * scale,
        }
    }
    pub fn aspect_ratio(&self) -> <T as Div>::Output {
        // assert!(self.height.Zero::is_zero());
        self.width / self.height
    }
    pub fn fitting(&self, other: Dimension2<T>) -> Dimension2<T> {
        let scale = PartialOrd::partial_min(other.width / self.width, other.height / self.height);
        self.scale(scale)

    }
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
fn test_aspect_ratio() {}
