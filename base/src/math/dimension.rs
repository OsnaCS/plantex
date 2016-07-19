use super::BaseNum;

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

    // TODO: area()
    // TODO: scale(scalar)
    // TODO: aspect_ratio()
    // TODO: fitting(other: Dimension2)
    // TODO: filling(other: Dimension2)
}
