use super::{Point2i, Vector2i};

/// Types that can be constructed from their axial representation.
pub trait FromAxial<T> {
    /// Converts from the axial type into `Self`. This is a no-op.
    fn from_axial(axial: T) -> Self;
}

impl FromAxial<AxialPoint> for Point2i {
    fn from_axial(axial: AxialPoint) -> Self {
        Point2i {
            x: axial.q,
            y: axial.r,
        }
    }
}
impl FromAxial<AxialVector> for Vector2i {
    fn from_axial(axial: AxialVector) -> Self {
        Vector2i {
            x: axial.q,
            y: axial.r,
        }
    }
}

/// Types that can be converted into their axial representation.
pub trait IntoAxial<T> {
    /// Converts from `self` into the axial type. This is a no-op.
    fn into_axial(self) -> T;
}

impl IntoAxial<AxialPoint> for Point2i {
    fn into_axial(self) -> AxialPoint {
        AxialPoint {
            q: axial.x,
            r: axial.y,
        }
    }
}
impl IntoAxial<AxialVector> for Vector2i {
    fn into_axial(self) -> AxialVector {
        AxialVector {
            q: axial.x,
            r: axial.y,
        }
    }
}
