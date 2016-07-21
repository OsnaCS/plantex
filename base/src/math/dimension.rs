use super::{BaseNum, PartialOrd};
use std::ops::{Div, Mul};

/// A two-dimensional dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Returns the area of a `Dimension`
    pub fn area(&self) -> <T as Mul>::Output {
        self.width * self.height
    }
    /// Scales the `Dimension2` with a scalar
    pub fn scale(&self, scale: T) -> Dimension2<T> {
        Dimension2 {
            width: self.width * scale,
            height: self.height * scale,
        }
    }
    /// Calculates the aspect ratio of a `Dimension`
    pub fn aspect_ratio(&self) -> <T as Div>::Output {
        assert!(!self.height.is_zero());
        self.width / self.height
    }
    /// Shrinks a `Dimension` until it fits into another `Dimension`
    pub fn fitting(&self, other: Dimension2<T>) -> Dimension2<T> {
        let scale = PartialOrd::partial_min(other.width / self.width, other.height / self.height);
        self.scale(scale)
    }
    /// Expands a `Dimension` until it fills another `Dimension`
    pub fn filling(&self, other: Dimension2<T>) -> Dimension2<T> {
        let scale = PartialOrd::partial_max(other.width / self.width, other.height / self.height);
        self.scale(scale)
    }
}


#[test]
fn test_area() {
    let test1 = Dimension2::new(3, 5);
    let test2 = Dimension2::new(0, 0);

    assert_eq!(test2.area(), 0);
    assert_eq!(test1.area(), 15);
}
#[test]
fn test_scale() {
    let test1 = Dimension2::new(3, 5);
    let test2 = Dimension2::new(12, 20);

    assert_eq!(test1.scale(4).width, 12);
    assert_eq!(test2.scale(0).width, 0);
}
#[test]
fn test_aspect_ratio() {
    let test1 = Dimension2::new(3, 5);

    assert_eq!(test1.aspect_ratio(), 3 / 5);
}
#[test]
fn test_fitting() {
    let test1 = Dimension2::new(2.0, 1.0);
    let test2 = Dimension2::new(4.0, 3.0);

    assert_eq!(test2.fitting(test1), Dimension2::new(4.0 / 3.0, 1.0));
}
#[test]
fn test_filling() {
    let test1 = Dimension2::new(2.0, 1.0);
    let test2 = Dimension2::new(4.0, 3.0);

    assert_eq!(test1.filling(test2), Dimension2::new(6.0, 3.0));
}
