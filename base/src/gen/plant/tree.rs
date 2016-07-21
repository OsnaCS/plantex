//! Generates random trees and tree-like plants.

use prop::plant::Branch;
use math::{InnerSpace, Point3f, Vector1, Vector3f};
use rand::{Rand, Rng};
use rand::distributions::range::SampleRange;
use rand::distributions::{self, IndependentSample};
use std::ops::Range;

/// Parameters for the tree generator.
#[derive(Debug)]
struct Params {
    /// Diameter of the first branch we create (the trunk).
    trunk_diameter: f32,
    /// Trunk height. Note that branches going upward can increase plant height
    /// beyond this.
    trunk_height: f32,
    /// Trunk diameter at `trunk_height`. Should be smaller than
    /// `trunk_diameter`.
    trunk_diameter_top: f32,
    /// Trunk height at which we start creating branches.
    min_branch_height: f32,
    /// Range of subbranch diameters as a factor of the parent branch.
    branch_diameter_factor: Range<f32>,
    /// Range of subbranch angles in degrees.
    branch_angle_deg: Range<f32>,
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
    branch_segment_length: Range<f32>,
}

/// Describes parameter ranges to generate a random tree.
///
/// We store a list of user-defined presets, one of which is selected to
/// generate a tree.
#[derive(Clone)]
struct Preset {
    trunk_diameter: Range<f32>,
    trunk_height: Range<f32>,
    trunk_diameter_top: Range<f32>,
    min_branch_height: Range<f32>,
    branch_diameter_factor: Range<f32>,
    branch_angle_deg: Range<f32>,
    branch_segment_angle: Range<f32>,
    branch_segment_count: Range<u32>,
    branch_segment_length: Range<f32>,
}

static PRESETS: &'static [Preset] = &[Preset {
                                          trunk_diameter: 0.3..0.5,
                                          trunk_height: 3.0..6.0,
                                          trunk_diameter_top: 0.2..0.4,
                                          min_branch_height: 0.4..0.6,
                                          branch_diameter_factor: 0.3..0.5,
                                          branch_angle_deg: 70.0..110.0,
                                          branch_segment_angle: 2.0..10.0,
                                          branch_segment_count: 5..20,
                                          branch_segment_length: 0.05..0.15,
                                      }];

pub struct TreeGen {
    params: Params,
    /// Buffer for branches, filled as they're created.
    branches: Vec<Branch>,
}

impl TreeGen {
    fn create_trunk<R: Rng>(&mut self, _rng: &mut R) {
        let mut points = Vec::new();

        {
            let mut add_point = |height, diam| {
                points.push((Point3f::new(0.0, height, 0.0), diam));
            };

            let diam_start = Vector1::new(self.params.trunk_diameter);
            let diam_end = Vector1::new(self.params.trunk_diameter_top);
            let mut height = 0.0;
            while height < self.params.trunk_height {
                // Current height as a fraction of the total height
                let height_frac =
                    if height == 0.0 { 0.0 } else { self.params.trunk_height / height };
                let diam = diam_start.lerp(diam_end, height_frac);

                add_point(height, diam.x);

                let segment_len = segment_dist(diam.x);
                height += segment_len;
            }

            // FIXME Do we need to create another point here?
        }

        self.branches.push(Branch {
            points: points,
            // FIXME Fixed color for now, we should use a configurable random color (or at least
            // make it brown).
            color: Vector3f::new(0.0, 0.0, 1.0),
        });
    }

    /// Generates a random tree according to the stored parameters.
    ///
    /// The tree is returned as a list of branches for now.
    pub fn generate<R: Rng>(mut self, rng: &mut R) -> Vec<Branch> {
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

        // Copy all `Range`s, and pick a random sample for tree properties.
        let params = Params {
            trunk_diameter: range_sample(preset.trunk_diameter, rng),
            trunk_height: range_sample(preset.trunk_height, rng),
            trunk_diameter_top: range_sample(preset.trunk_diameter_top, rng),
            min_branch_height: range_sample(preset.min_branch_height, rng),
            branch_diameter_factor: preset.branch_diameter_factor,
            branch_angle_deg: preset.branch_angle_deg,
            branch_segment_angle: preset.branch_segment_angle,
            branch_segment_count: preset.branch_segment_count,
            branch_segment_length: preset.branch_segment_length,
        };

        debug!("treegen params: {:?}", params);

        TreeGen {
            params: params,
            branches: Vec::new(),
        }
    }
}

/// Samples a random element from a range.
fn range_sample<T: SampleRange + PartialOrd, R: Rng>(range: Range<T>, rng: &mut R) -> T {
    // Build a `rand` crate Range. We use `std`s Range for the cool `a..b` syntax ;)
    distributions::Range::new(range.start, range.end).ind_sample(rng)
}

/// Approximation of real-world distance of branch segments, depending on the
/// starting branch diameter.
fn segment_dist(diameter: f32) -> f32 {
    diameter * 11.25
}
