//! Generates random trees and tree-like plants.

use math::*;
use prop::plant::{Branch, ControlPoint, Tree};
use rand::distributions::range::SampleRange;
use rand::distributions::{self, IndependentSample};
use rand::Rng;
use std::cmp;
use std::ops::Range;

/// Parameters for the tree generator.
#[derive(Debug)]
pub struct Preset {
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
    /// How many branches will the tree have
    branch_chance: f32,
    /// Range of subbranch diameters as a factor of the parent branch.
    branch_diameter_factor: Range<f32>,
    /// Range of subbranch angles in degrees.
    branch_angle_deg: Range<f32>,
    /// Factor by which to reduce segment diameter between consecutive points,
    /// sampled per branch.
    branch_diam_reduction: Range<f32>,
    /// Factor determining branch length depending on the branch diameter.
    /// If 0 => standard value (11.25) is used
    branch_segment_length: Range<f32>,

    /// Factor determining branch length depending on the branch diameter for
    /// branches starting with recursion depth 2
    branch_segment_length2: Range<f32>,
    /// Range of angles to use for rotation of new segments.
    /// The higher the angle, the more "twisted" branches appear.
    branch_segment_angle: Range<f32>,
    /// Range of segment counts for branches.
    ///
    /// Together with `branch_segment_length`, this defines the overall branch
    /// length.
    branch_segment_count: Range<u32>,
    /// The color of the trunk :^)
    trunk_color: (Range<f32>, Range<f32>, Range<f32>),
    /// The color of the leafs :^)
    leaf_color: (Range<f32>, Range<f32>, Range<f32>),
    /// At which recursion depth should the color switch.
    /// x > 3 basically means only trunk_color will be used (because plants are
    /// generated with a max depth of 3). The actual trunk though will always
    /// get trunk_color, even if leaf_depth is 0.
    leaf_depth: u16,

    /// for conifer trees the branches become smaller with height
    height_branchlength_dependence: fn(f32) -> f32,
}

#[derive(Debug, Clone, Copy)]
pub enum PlantType {
    WitheredTree,
    Shrub,
    Cactus,
    JungleTree,
    ClumpOfGrass,
    Conifer,
    OakTree,
    Flower,
}

impl PlantType {
    fn preset(&self) -> Preset {
        match *self {
            PlantType::WitheredTree => Preset {
                trunk_diameter: 0.3..0.5,
                trunk_height: 3.0..6.0,
                trunk_diameter_top: 0.2..0.4,
                min_branch_height: 0.4..0.6,
                branch_chance: 0.6,
                branch_diameter_factor: 0.3..0.5,
                branch_angle_deg: 70.0..110.0,
                branch_diam_reduction: 0.9..0.99,

                branch_segment_length: 11.25..11.26,
                branch_segment_length2: 40.0..50.0,
                branch_segment_angle: 5.0..15.0,
                branch_segment_count: 1..4,
                trunk_color: (0.2..0.4001, 0.15..0.3001, 0.1..0.2001),
                leaf_color: (0.2..0.4001, 0.15..0.3001, 0.1..0.2001),
                leaf_depth: 2,
                height_branchlength_dependence: {
                    fn f(_: f32) -> f32 {
                        1.0
                    }
                    f
                },
            },
            PlantType::Shrub => Preset {
                trunk_diameter: 0.05..0.15,
                trunk_height: 0.5..1.5,
                trunk_diameter_top: 0.6..0.60001,
                min_branch_height: 0.4..0.6,
                branch_chance: 10.0,
                branch_diameter_factor: 0.3..0.5,
                branch_angle_deg: 60.0..100.0,
                branch_diam_reduction: 0.70..0.80,
                branch_segment_length: 11.25..11.26,
                branch_segment_length2: 11.25..11.26,
                branch_segment_angle: 15.0..20.0,
                branch_segment_count: 1..4,
                trunk_color: (0.3..0.4, 0.03..0.07, 0.0..0.03),
                leaf_color: (0.6..0.8, 0.06..0.07, 0.0..0.03),
                leaf_depth: 1,
                height_branchlength_dependence: {
                    fn f(_: f32) -> f32 {
                        1.0
                    }
                    f
                },
            },
            PlantType::Cactus => Preset {
                trunk_diameter: 0.15..0.2,
                trunk_height: 1.5..3.0,
                trunk_diameter_top: 0.6..0.60001,
                min_branch_height: 0.05..0.1,
                branch_chance: 3.0,
                branch_diameter_factor: 0.3..0.4,
                branch_angle_deg: 90.0..90.00001,
                branch_diam_reduction: 0.90..0.95,
                branch_segment_length: 4.0..5.0,
                branch_segment_length2: 4.0..5.0,
                branch_segment_angle: 0.0..0.00001,
                branch_segment_count: 1..2,
                trunk_color: (0.313..0.39, 0.35..0.39, 0.2519..0.252),
                leaf_color: (0.313..0.39, 0.35..0.39, 0.2519..0.252),
                leaf_depth: 4,
                height_branchlength_dependence: {
                    fn f(_: f32) -> f32 {
                        1.0
                    }
                    f
                },
            },
            PlantType::JungleTree => Preset {
                trunk_diameter: 1.0..2.0,
                trunk_height: 17.0..21.0,
                trunk_diameter_top: 0.6..1.0,
                min_branch_height: 0.7..0.8,
                branch_chance: 1.2,
                branch_diameter_factor: 0.3..0.5,
                branch_angle_deg: 80.0..115.0,
                branch_diam_reduction: 0.5..0.75,
                branch_segment_length: 11.25..11.26,
                branch_segment_length2: 11.25..50.26,
                branch_segment_angle: 10.0..20.0,
                branch_segment_count: 3..4,
                trunk_color: (0.2..0.3, 0.1..0.2, 0.07..0.17),
                leaf_color: (0.1..0.2, 0.2..0.5, 0.0..0.1),
                leaf_depth: 1,
                height_branchlength_dependence: {
                    fn f(_: f32) -> f32 {
                        1.0
                    }
                    f
                },
            },
            PlantType::ClumpOfGrass => Preset {
                trunk_diameter: 0.03..0.8,
                trunk_height: 0.3..0.8,
                trunk_diameter_top: 0.03..0.8,
                min_branch_height: 0.1..0.3,
                branch_chance: 12.0,
                branch_diameter_factor: 0.3..0.5,
                branch_angle_deg: 60.0..100.0,
                branch_diam_reduction: 0.70..0.80,
                branch_segment_length: 8.0..9.0,
                branch_segment_length2: 8.0..9.0,
                branch_segment_angle: 25.0..30.0,
                branch_segment_count: 1..4,
                trunk_color: (0.2..0.25, 0.7..0.8, 0.0..0.02),
                leaf_color: (0.0..0.05, 0.3..0.4, 0.8..1.0),
                leaf_depth: 2,
                height_branchlength_dependence: {
                    fn f(_: f32) -> f32 {
                        1.0
                    }
                    f
                },
            },
            PlantType::Conifer => Preset {
                trunk_diameter: 0.175..0.3,
                trunk_height: 5.0..8.0,
                trunk_diameter_top: 0.2..0.3,
                min_branch_height: 0.1..0.2,
                branch_chance: 3.4,
                branch_diameter_factor: 0.6..0.75,
                branch_angle_deg: 90.0..90.00001,
                branch_diam_reduction: 0.75..0.85,
                branch_segment_length: 23.0..27.0,
                branch_segment_length2: 23.0..27.0,
                branch_segment_angle: 1.0..2.0,
                branch_segment_count: 1..4,
                trunk_color: (0.4..0.4001, 0.3..0.3001, 0.2..0.2001),
                leaf_color: (0.1..0.15, 0.15..0.18, 0.05..0.09),
                leaf_depth: 1,
                height_branchlength_dependence: {
                    fn f(height: f32) -> f32 {
                        1.0 - 0.125 * height
                    }
                    f
                },
            },
            PlantType::OakTree => Preset {
                trunk_diameter: 0.4..0.6,
                trunk_height: 5.9..6.0,
                trunk_diameter_top: 0.3..0.5,
                min_branch_height: 0.4..0.51,
                branch_chance: 6.0,
                branch_diameter_factor: 0.7..0.85,
                branch_angle_deg: 80.0..100.0001,
                branch_diam_reduction: 0.6..0.7,
                branch_segment_length: 0.15..0.2,
                branch_segment_length2: 0.5..0.75,
                branch_segment_angle: 3.0..5.0,
                branch_segment_count: 3..4,
                trunk_color: (0.4..0.4001, 0.3..0.3001, 0.2..0.2001),
                leaf_color: (0.2..0.21, 0.45..0.46, 0.2..0.21),
                leaf_depth: 1,
                height_branchlength_dependence: {
                    fn f(height: f32) -> f32 {
                        let mut result = 25.0 - 7.6 * (height - 4.5) * (height - 4.5);
                        if result <= 0.0 {
                            result = 0.1;
                        }
                        result
                    }
                    f
                },
            },

            PlantType::Flower => Preset {
                trunk_diameter: 0.025..0.03,
                trunk_height: 0.4..0.75,
                trunk_diameter_top: 0.4..0.6,
                min_branch_height: 0.9..0.91,
                branch_chance: 10.0,
                branch_diameter_factor: 0.45..0.55,
                branch_angle_deg: 80.0..95.0,
                branch_diam_reduction: 0.9..0.95,
                branch_segment_length: 22.5..22.51,
                branch_segment_length2: 22.5..22.51,
                branch_segment_angle: 3.0..7.0,
                branch_segment_count: 1..5,
                trunk_color: (0.3..0.33, 0.9..0.99, 0.0..0.02),
                leaf_color: (0.4..0.8, 0.05..0.1, 0.4..0.6),
                leaf_depth: 1,
                height_branchlength_dependence: {
                    fn f(_: f32) -> f32 {
                        1.0
                    }
                    f
                },
            },
        }
    }
}

pub struct TreeGen {
    preset: Preset,
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
    fn create_branch<R: Rng>(
        &mut self,
        rng: &mut R,
        start: Point3f,
        dir: Vector3f,
        depth: u16,
        parent_diam: f32,
        height_branchlength_dependence: fn(_: f32) -> f32,
    ) {
        if depth > 3 {
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
        // How long should the segment be?
        let segment_length = range_sample(&self.preset.branch_segment_length, rng);
        let segment_length2 = range_sample(&self.preset.branch_segment_length2, rng);

        let mut points = vec![ControlPoint {
            point: start,
            diameter: diam,
        }];

        {
            let mut last = start;
            // Helper for adding a new point to this branch, which possibly grows a new
            // branch.
            let mut add_point = |dist, diam| {
                // First, get a random angle by which to variate this segment.
                let mut x_angle = range_sample(&self.preset.branch_segment_angle, rng);
                let mut y_angle = range_sample(&self.preset.branch_segment_angle, rng);

                // Invert sign with 50%, to mirror the specified range to the other side
                x_angle = if rng.gen() { -x_angle } else { x_angle };
                y_angle = if rng.gen() { -y_angle } else { y_angle };

                let rotation = Basis3::from(Euler {
                    x: Deg::new(x_angle),
                    y: Deg::new(y_angle),
                    z: Deg::new(0.0),
                });
                dir = rotation.rotate_vector(dir);

                // We need to add a point with a distance of `dist` from `last`.
                let point = last + dir * dist;
                last = point;

                if rng.gen_weighted_bool(depth as u32 * 2) {
                    // Build a vector for the branch direction (Z is up)
                    let dir = self.gen_branch_direction(rng, dir);
                    self.create_branch(
                        rng,
                        point,
                        dir,
                        depth + 1,
                        diam,
                        height_branchlength_dependence,
                    );
                }

                points.push(ControlPoint {
                    point: point,
                    diameter: diam,
                });
            };

            // In a loop, get the length of the next segment from the current diameter.
            for _ in 0..segment_count {
                assert!(height_branchlength_dependence(start.z) > 0.0);
                let length = height_branchlength_dependence(start.z)
                    * segment_dist(segment_length, segment_length2, diam, depth);
                diam *= diam_factor;

                add_point(length, diam);

                if diam < 0.005 {
                    // Bail out at 5mm
                    break;
                }
            }
        }

        assert!(
            points.len() >= 2,
            "should've generated at least 2 points :("
        );
        self.branches.push(Branch {
            points: points,
            is_trunk: self.preset.leaf_depth > depth,
        });
    }

    /// Given the growing direction of the parent branch, calculates a growing
    /// direction to use for a new child branch.
    fn gen_branch_direction<R: Rng>(&self, rng: &mut R, parent_dir: Vector3f) -> Vector3f {
        // `branch_angle_deg` specifies the angle range in degrees
        let angle = range_sample(&self.preset.branch_angle_deg, rng);

        random_vec_with_angle(rng, parent_dir, angle)
    }

    fn create_trunk<R: Rng>(&mut self, rng: &mut R) {
        let trunk_diameter = range_sample(&self.preset.trunk_diameter, rng);
        let trunk_height = range_sample(&self.preset.trunk_height, rng);
        let mut trunk_diameter_top = range_sample(&self.preset.trunk_diameter_top, rng);
        let min_branch_height = range_sample(&self.preset.min_branch_height, rng) * trunk_height;
        let height_branchlength_dependence = self.preset.height_branchlength_dependence;

        // The trunk is supposed to get smaller as we go up, so just enforce that rule
        // here:
        trunk_diameter_top = trunk_diameter.min(trunk_diameter_top);

        debug!(
            "trunk diam {} to {}, height {}, branch start at {}",
            trunk_diameter, trunk_diameter_top, trunk_height, min_branch_height
        );

        let mut points = Vec::new();

        {
            let mut add_point = |height, diam| {
                let point = Point3f::new(0.0, 0.0, height);
                if height >= min_branch_height {
                    let branches = &[0, 1, 1, 1, 2, 2, 3, 3];
                    for _ in 0..(((*rng.choose(branches).unwrap()) as f32
                        * self.preset.branch_chance)
                        + 0.5) as usize
                    {
                        // Build a vector for the branch direction (Z is up)
                        let dir = self.gen_branch_direction(rng, Vector3f::new(0.0, 0.0, 1.0));
                        self.create_branch(
                            rng,
                            point,
                            dir,
                            1,
                            diam,
                            height_branchlength_dependence,
                        );
                    }
                }
            };

            let diam_start = Vector1::new(trunk_diameter);
            let diam_end = Vector1::new(trunk_diameter_top);

            // Split trunk in segments
            // FIXME Vary the segment direction like we do for normal branches
            let height_dependent_seg_count = (trunk_height * 2.0).max(2.0) as u16;
            for i in 0..height_dependent_seg_count + 1 {
                let height = i as f32 * trunk_height / height_dependent_seg_count as f32;
                let height_frac = height / trunk_height;
                let diam = diam_start.lerp(diam_end, height_frac);

                add_point(height, diam.x);
            }

            // Add top/bottom point to define the trunk
            points.push(ControlPoint {
                point: Point3f::new(0.0, 0.0, 0.0),
                diameter: trunk_diameter,
            });
            points.push(ControlPoint {
                point: Point3f::new(0.0, 0.0, trunk_height),
                diameter: trunk_diameter_top,
            });
            points.push(ControlPoint {
                point: Point3f::new(0.0, 0.0, trunk_height + 0.02 * trunk_height),
                diameter: trunk_diameter * trunk_diameter_top * 0.3,
            });
            points.push(ControlPoint {
                point: Point3f::new(0.0, 0.0, trunk_height + 0.03 * trunk_height),
                diameter: trunk_diameter * trunk_diameter_top * 0.05,
            });
        }

        self.branches.push(Branch {
            points: points,
            is_trunk: true,
        });

        debug!("generated tree with {} branches", self.branches.len());
    }

    /// Generates a random tree according to the stored parameters.
    ///
    /// The tree is returned as a list of branches for now.
    pub fn generate<R: Rng>(mut self, rng: &mut R) -> Tree {
        // Recursively create the tree and put all branches in a buffer.
        self.create_trunk(rng);

        Tree {
            branches: self.branches,
            trunk_color: Vector3f::new(
                range_sample(&self.preset.trunk_color.0, rng),
                range_sample(&self.preset.trunk_color.1, rng),
                range_sample(&self.preset.trunk_color.2, rng),
            ),
            leaf_color: Vector3f::new(
                range_sample(&self.preset.leaf_color.0, rng),
                range_sample(&self.preset.leaf_color.1, rng),
                range_sample(&self.preset.leaf_color.2, rng),
            ),
        }
    }

    pub fn new(plant_type: PlantType) -> Self {
        TreeGen {
            preset: plant_type.preset(),
            branches: Vec::new(),
        }
    }
}

/// Samples a random element from a range.
fn range_sample<T: SampleRange + cmp::PartialOrd + Copy, R: Rng>(
    range: &Range<T>,
    rng: &mut R,
) -> T {
    // Build a `rand` crate Range. We use `std`s Range for the cool `a..b` syntax ;)
    distributions::Range::new(range.start, range.end).ind_sample(rng)
}

/// Approximation of real-world distance of branch segments, depending on the
/// starting branch diameter.
fn segment_dist(segment_length: f32, segment_length2: f32, diameter: f32, depth: u16) -> f32 {
    if depth < 2 {
        diameter * segment_length
    } else {
        diameter * segment_length2
    }
}
