//! Generates random trees and tree-like plants.

use prop::plant::Branch;
use math::{Basis3, Deg, Euler, InnerSpace, Point3f, Rotation, Vector1, Vector3f};
use rand::{Rand, Rng};
use rand::distributions::range::SampleRange;
use rand::distributions::{self, IndependentSample};
use std::ops::Range;

/// Parameters for the tree generator.
#[derive(Debug)]
struct Preset {
    /// Diameter of the first branch we create (the trunk).
    trunk_diameter: Range<f32>,
    /// Trunk height. Note that branches going upward can increase plant height
    /// beyond this.
    trunk_height: Range<f32>,
    /// Trunk diameter at `trunk_height`. Should be smaller than
    /// `trunk_diameter`.
    trunk_diameter_top: Range<f32>,
    /// Trunk height at which we start creating branches.
    min_branch_height: Range<f32>,
    /// Range of subbranch diameters as a factor of the parent branch.
    branch_diameter_factor: Range<f32>,
    /// Range of subbranch angles in degrees.
    branch_angle_deg: Range<f32>,
    /// Factor by which to reduce segment diameter between consecutive points,
    /// sampled per branch.
    branch_diam_reduction: Range<f32>,
    /// Range of angles to use for rotation of new segments.
    ///
    /// The higher the angle, the more "twisted" branches appear.
    branch_segment_angle: Range<f32>,
    /// Range of segment counts for branches.
    ///
    /// Together with `branch_segment_length`, this defines the overall branch
    /// length.
    branch_segment_count: Range<u32>,
    /// Range of segment lengths to use for branches.
    ///
    /// Each segment will have a random length in this range.
    branch_segment_length: Range<f32>, // FIXME unused
}

static PRESETS: &'static [Preset] = &[Preset {
                                          trunk_diameter: 0.3..0.5,
                                          trunk_height: 3.0..6.0,
                                          trunk_diameter_top: 0.2..0.4,
                                          min_branch_height: 0.4..0.6,
                                          branch_diameter_factor: 0.3..0.5,
                                          branch_angle_deg: 70.0..110.0,
                                          branch_diam_reduction: 0.75..0.85,
                                          branch_segment_angle: 2.0..10.0,
                                          branch_segment_count: 5..20,
                                          branch_segment_length: 0.05..0.15,
                                      }];

pub struct TreeGen {
    preset: &'static Preset,
    /// Buffer for branches, filled as they're created.
    branches: Vec<Branch>,
}

impl TreeGen {
    /// Generates a new branch at the given start point.
    ///
    /// # Parameters
    ///
    /// * `rng`: The RNG to use
    /// * `start`: The point at which the branch should start
    /// * `dir`: Direction in which the branch should grow
    /// * `depth`: Recursion depth, used to limit the maximum branch depth
    /// * `parent_diam`: Diameter of the node of the parent branch where this
    ///   branch starts
    fn create_branch<R: Rng>(&mut self,
                             rng: &mut R,
                             start: Point3f,
                             dir: Vector3f,
                             depth: u32,
                             parent_diam: f32) {
        if depth > 5 {
            // Limit recursion
            return;
        }

        // Current normalized growing direction, variated slightly as segments are
        // generated
        let mut dir = dir.normalize();

        // Determine starting diameter of the new branch
        let mut diam = range_sample(&self.preset.branch_diameter_factor, rng) * parent_diam;
        // Determine how much segment diameter is reduced
        let diam_factor = range_sample(&self.preset.branch_diam_reduction, rng) * parent_diam;
        // How many segments should this branch get?
        let segment_count = range_sample(&self.preset.branch_segment_count, rng);

        let mut points = vec![(start, diam)];
        {
            let mut last = start;
            // Helper for adding a new point to this branch, which possibly grows a new
            // branch.
            let mut add_point = |dist, diam| {
                // First, get a random angle by which to variate this segment.
                let x_angle = range_sample(&self.preset.branch_segment_angle, rng);
                let y_angle = range_sample(&self.preset.branch_segment_angle, rng);

                let rotation = Basis3::from(Euler {
                    x: Deg::new(x_angle),
                    y: Deg::new(y_angle),
                    z: Deg::new(0.0),
                });
                dir = rotation.rotate_vector(dir);

                // We need to add a point with a distance of `dist` from `last`.
                let point = last + dir * dist;
                last = point;

                // FIXME Make branch spawn chance configurable
                // 1/20 chance to spawn a branch at any point
                if rng.gen_weighted_bool(20) {
                    // Build a vector for the branch direction (Z is up)
                    let dir = self.gen_branch_direction(rng, dir);
                    self.create_branch(rng, point, dir, depth + 1, diam);
                }

                points.push((point, diam));
            };

            // In a loop, get the length of the next segment from the current diameter.
            for _ in 0..segment_count {
                let length = segment_dist(diam);
                diam *= diam_factor;
                add_point(length, diam);
            }
        }

        assert!(points.len() >= 2,
                "should've generated at least 2 points :(");
        self.branches.push(Branch {
            points: points,
            // FIXME Fixed color for now, we should use a configurable random color (or at least
            // make it brown).
            color: Vector3f::new(0.0, 0.0, 1.0),
        });
    }

    /// Given the growing direction of the parent branch, calculates a growing
    /// direction to use for a new child branch.
    fn gen_branch_direction<R: Rng>(&self, rng: &mut R, parent_dir: Vector3f) -> Vector3f {
        let x_angle = range_sample(&self.preset.branch_angle_deg, rng);
        let y_angle = range_sample(&self.preset.branch_angle_deg, rng);

        let rotation = Basis3::from(Euler {
            x: Deg::new(x_angle),
            y: Deg::new(y_angle),
            z: Deg::new(0.0),
        });
        rotation.rotate_vector(parent_dir)
    }

    fn create_trunk<R: Rng>(&mut self, rng: &mut R) {
        let trunk_diameter = range_sample(&self.preset.trunk_diameter, rng);
        let trunk_height = range_sample(&self.preset.trunk_height, rng);
        let trunk_diameter_top = range_sample(&self.preset.trunk_diameter_top, rng);
        let min_branch_height = range_sample(&self.preset.min_branch_height, rng);

        let mut points = Vec::new();

        {
            let mut add_point = |height, diam| {
                let point = Point3f::new(0.0, height, 0.0);
                if height >= min_branch_height {
                    // FIXME Make branch spawn chance configurable
                    // 1/5 chance to spawn a branch at any point
                    if rng.gen_weighted_bool(5) {
                        // Build a vector for the branch direction (Z is up)
                        let dir = self.gen_branch_direction(rng, Vector3f::new(0.0, 0.0, 1.0));
                        self.create_branch(rng, point, dir, 1, diam);
                    }
                }

                points.push((point, diam));
            };

            let diam_start = Vector1::new(trunk_diameter);
            let diam_end = Vector1::new(trunk_diameter_top);
            let mut height = 0.0;
            while height < trunk_height {
                // Current height as a fraction of the total height
                let height_frac = if height == 0.0 { 0.0 } else { trunk_height / height };
                let diam = diam_start.lerp(diam_end, height_frac);

                add_point(height, diam.x);

                let segment_len = segment_dist(diam.x);
                height += segment_len;
            }

            // FIXME Do we need to create another point here?
        }

        assert!(points.len() >= 2,
                "should've generated at least 2 points :(");
        self.branches.push(Branch {
            points: points,
            // FIXME Fixed color for now, we should use a configurable random color (or at least
            // make it brown).
            color: Vector3f::new(0.0, 0.0, 1.0),
        });

        debug!("generated tree with {} branches", self.branches.len());
    }

    /// Generates a random tree according to the stored parameters.
    ///
    /// The tree is returned as a list of branches for now.
    pub fn generate<R: Rng>(mut self, rng: &mut R) -> Vec<Branch> {
        info!("treegen activated!");    // deleteme
        // Recursively create the tree and put all branches in a buffer.
        self.create_trunk(rng);
        self.branches
    }
}

impl Rand for TreeGen {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        // Create a tree generator with random parameters.
        // First, select a random preset:
        let preset = rng.choose(PRESETS).unwrap().clone();

        TreeGen {
            preset: preset,
            branches: Vec::new(),
        }
    }
}

/// Samples a random element from a range.
fn range_sample<T: SampleRange + PartialOrd + Copy, R: Rng>(range: &Range<T>, rng: &mut R) -> T {
    // Build a `rand` crate Range. We use `std`s Range for the cool `a..b` syntax ;)
    distributions::Range::new(range.start, range.end).ind_sample(rng)
}

/// Approximation of real-world distance of branch segments, depending on the
/// starting branch diameter.
fn segment_dist(diameter: f32) -> f32 {
    diameter * 11.25
}
