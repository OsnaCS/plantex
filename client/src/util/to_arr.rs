use base::math::*;

/// Helper trait to easily convert various `cgmath` types into array form to
/// use them in glium
pub trait ToArr {
    type Output;

    fn to_arr(&self) -> Self::Output;
}
/// Convert a `Matrix4` into a 4x4 array
impl<T: BaseNum> ToArr for Matrix4<T> {
    type Output = [[T; 4]; 4];

    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}

/// Convert a `Matrix3` into a 3x3 array
impl<T: BaseNum> ToArr for Matrix3<T> {
    type Output = [[T; 3]; 3];

    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}

/// Convert a `Matrix2` into a 2x2 array
impl<T: BaseNum> ToArr for Matrix2<T> {
    type Output = [[T; 2]; 2];

    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}

/// Convert a `Vector4` into an array
impl<T: BaseNum> ToArr for Vector4<T> {
    type Output = [T; 4];

    fn to_arr(&self) -> [T; 4] {
        (*self).into()
    }
}

/// Convert a `Vector3` into an array
impl<T: BaseNum> ToArr for Vector3<T> {
    type Output = [T; 3];

    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}

/// Convert a `Vector2` into an array
impl<T: BaseFloat> ToArr for Vector2<T> {
    type Output = [T; 2];

    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}

impl<T: BaseNum> ToArr for Point2<T> {
    type Output = [T; 2];

    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}

impl<T: BaseNum> ToArr for Point3<T> {
    type Output = [T; 3];

    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}
