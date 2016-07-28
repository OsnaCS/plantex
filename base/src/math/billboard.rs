use math::Matrix4;

/// Modifies a view matrix so that transformed objects always face the camera.
pub fn spherical(mut matrix: Matrix4<f32>) -> Matrix4<f32> {
    matrix[0][0] = 1.0;
    matrix[0][1] = 0.0;
    matrix[0][2] = 0.0;

    matrix[1][0] = 0.0;
    matrix[1][1] = 1.0;
    matrix[1][2] = 0.0;

    matrix[2][0] = 0.0;
    matrix[2][1] = 0.0;
    matrix[2][2] = 1.0;

    matrix
}
