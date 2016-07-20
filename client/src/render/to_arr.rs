use base::math::*;

/// Helper trait to easily convert various `cgmath` types into array form to
/// use them in glium
pub trait ToArr {
    type Output;

    fn to_arr(&self) -> Self::Output;
}

impl<T: BaseNum> ToArr for Matrix4<T> {
    type Output = [[T; 4]; 4];

    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}

// TODO: Add more impls
