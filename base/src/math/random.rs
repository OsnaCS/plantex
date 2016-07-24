//! Utilities for random generation

use math::*;

use rand::{Rand, Rng};

/// Generates a random `Vector3f` that has a specific angle to `v`.
///
/// The resulting vector will have a random rotation around `v`.
pub fn random_vec_with_angle<R: Rng>(rng: &mut R, v: Vector3f, angle: f32) -> Vector3f {
    // AWESOME hack: Generate *any* vector that is not parallel to `v`,
    // calculate the cross product between it and `v`.
    // This gets us a vector perpendicular to `v`, which we can rotate
    // `v` around to tilt it by `angle` degrees.
    let mut rand_vec = Vector3f::rand(rng);
    while rand_vec.angle(v).approx_eq(&Rad::new(0.0)) {
        // They're parallel, so generate a new vector. This is probably unnecessary,
        // but someone who *actually* knows their math instead of faking it should
        // check.
        rand_vec = Vector3f::rand(rng);
    }

    // Create vector around we'll tilt `v`
    let rot_vec = v.cross(rand_vec);

    // Tilt `v` by `angle` to get some vector with the right angle
    let tilt_rotation = Basis3::from_axis_angle(rot_vec, Deg::new(angle).into());

    // Then rotate this vector randomly (0-360°) around `v`
    let spin_angle = rng.gen_range(0.0, 360.0);
    let around_we_go = Basis3::from_axis_angle(v, Deg::new(spin_angle).into());

    around_we_go.rotate_vector(tilt_rotation.rotate_vector(v))
}
