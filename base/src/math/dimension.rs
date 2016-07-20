use super::{BaseNum, PartialOrd};
use std::ops::{Div, Mul};

/// A two-dimensional dimension.
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
        //     assert!(self.height != 0);
        self.width / self.height
    }
    pub fn fitting(&self, other: Dimension2<T>) -> Dimension2<T> {
        let scale = PartialOrd::partial_min(other.width / self.width, other.height / self.height);
        self.scale(scale)

    }
    pub fn filling(&self, other: Dimension2<T>) -> Dimension<T>
}
