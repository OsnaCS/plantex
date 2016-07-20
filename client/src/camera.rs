use base::math::*;

pub struct Camera {
    // TODO
}

impl Camera {
    /// Returns the projection matrix
    pub fn proj_matrix(&self) -> Matrix4<f32> {
        perspective(deg(60.0), 16.0 / 9.0, 0.1, 100.0)
    }

    /// Returns view matrix
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(Point3::new(0.0, 0.0, 60.0),
                         Point3::new(10.0, 10.0, 30.0),
                         Vector3::unit_z())
    }
}
