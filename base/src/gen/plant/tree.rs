//! Generates random trees and tree-like plants.

use prop::plant::{Branch, ControlPoint, Tree};
use math::*;
use rand::Rng;
use rand::distributions::range::SampleRange;
use rand::distributions::{self, IndependentSample};
use std::ops::Range;
use std::cmp;
use gen::world::biome::Biome;


/// Parameters for the tree generator.
#[derive(Debug)]
pub struct Preset {
    name: &'static str,
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
    /// Range of angles to use for rotation of new segments.
    ///
    /// The higher the angle, the more "twisted" branches appear.
    branch_segment_angle: Range<f32>,
    /// Range of segment counts for branches.
    ///
    /// Together with `branch_segment_length`, this defines the overall branch
    /// length.
    branch_segment_count: Range<u32>,
    /// Range of branch colors for the whole tree, as distinct ranges for R, G
    /// and B.
    branch_color: (Range<f32>, Range<f32>, Range<f32>),
    /// for conifer trees the branches become smaller with height
    conifer: bool,
}

pub const PRESETS: &'static [Preset] =
    &[Preset {
          name: "'Regular' Tree",
          trunk_diameter: 0.3..0.5,
          trunk_height: 3.0..6.0,
          trunk_diameter_top: 0.2..0.4,
          min_branch_height: 0.4..0.6,
          branch_chance: 0.6,
          branch_diameter_factor: 0.3..0.5,
          branch_angle_deg: 70.0..110.0,
          branch_diam_reduction: 0.75..0.85,
          branch_segment_length: 11.25..11.26,
          branch_segment_angle: 5.0..15.0,
          branch_segment_count: 1..4,
          branch_color: (0.3..0.33, 0.9..0.99, 0.0..0.02),
          conifer: false, // height_branchlength_dependence: |_: f32| 1.0,
      },
      Preset {
          name: "Shrub",
          trunk_diameter: 0.05..0.15,
          trunk_height: 0.5..1.5,
          trunk_diameter_top: 0.6..0.60001,
          min_branch_height: 0.4..0.6,
          branch_chance: 10.0,
          branch_diameter_factor: 0.3..0.5,
          branch_angle_deg: 60.0..100.0,
          branch_diam_reduction: 0.70..0.80,
          branch_segment_length: 11.25..11.26,
          branch_segment_angle: 15.0..20.0,
          branch_segment_count: 1..4,
          branch_color: (0.9..0.99, 0.1..0.11, 0.0..0.02),
          conifer: false, // height_branchlength_dependence: |_: f32| 1.0,
      },
      Preset {
          name: "Cactus",
          trunk_diameter: 0.6..0.60001,
          trunk_height: 2.0..4.0,
          trunk_diameter_top: 0.6..0.60001,
          min_branch_height: 0.05..0.1,
          branch_chance: 5.0,
          branch_diameter_factor: 0.1..0.15,
          branch_angle_deg: 90.0..90.00001,
          branch_diam_reduction: 0.90..0.95,
          branch_segment_length: 2.0..4.0,
          branch_segment_angle: 0.0..0.00001,
          branch_segment_count: 1..2,
          branch_color: (0.3..0.59, 0.75..0.88, 0.08..0.15),
          conifer: false, // height_branchlength_dependence: |_: f32| 1.0,
      },
      Preset {
          name: "Jungle Tree",
          trunk_diameter: 1.0..2.0,
          trunk_height: 17.0..21.0,
          trunk_diameter_top: 0.6..1.0,
          min_branch_height: 0.6..0.7,
          branch_chance: 1.2,
          branch_diameter_factor: 0.3..0.5,
          branch_angle_deg: 80.0..100.0,
          branch_diam_reduction: 0.5..0.75,
          branch_segment_length: 11.25..11.26,
          branch_segment_angle: 10.0..13.0,
          branch_segment_count: 3..4,
          branch_color: (0.3..0.33, 0.1..0.11, 0.9..0.99),
          conifer: false, // height_branchlength_dependence: |_: f32| 1.0,
      },
      Preset {
          name: "Clump Of Grass",
          trunk_diameter: 0.03..0.8,
          trunk_height: 0.3..0.8,
          trunk_diameter_top: 0.03..0.8,
          min_branch_height: 0.1..0.3,
          branch_chance: 12.0,
          branch_diameter_factor: 0.3..0.5,
          branch_angle_deg: 60.0..100.0,
          branch_diam_reduction: 0.70..0.80,
          branch_segment_length: 8.0..10.0,
          branch_segment_angle: 25.0..30.0,
          branch_segment_count: 1..4,
          branch_color: (0.1..0.25, 0.6..0.8, 0.0..0.06),
          conifer: false, // height_branchlength_dependence: |_: f32| 1.0,
      },
      Preset {
          name: "Conifer",
          trunk_diameter: 0.5..0.8,
          trunk_height: 5.0..8.0,
          trunk_diameter_top: 0.3..0.5,
          min_branch_height: 0.1..0.2,
          branch_chance: 2.0,
          branch_diameter_factor: 0.3..0.5,
          branch_angle_deg: 70.0..110.0,
          branch_diam_reduction: 0.75..0.85,
          branch_segment_length: 11.25..11.26,
          branch_segment_angle: 1.0..2.0,
          branch_segment_count: 1..4,
          // dark green
          branch_color: (0.13..0.18, 0.2..0.22, 0.05..0.09),
          conifer: true, // height_branchlength_dependence: |height: f32| 1.0 - 0.125 * height,
      }];

// pub const usize: REGULAR_TREE = 0;
// pub const usize: SHRUB = 1;
// pub const usize: CACTUS = 2;
// pub const usize: JUNGLE_TREE = 3;
// pub const usize: CLUMP_OF_GRASS = 4;
// pub const usize: CONIFER = 5;

const GRASS_LAND_PRESET: &'static [&'static Preset] = &[&PRESETS[0], &PRESETS[1], &PRESETS[4]];
const DESERT_PRESET: &'static [&'static Preset] = &[&PRESETS[2]];
const SNOW_PRESET: &'static [&'static Preset] = &[&PRESETS[5]];
const FOREST_PRESET: &'static [&'static Preset] = &[&PRESETS[0], &PRESETS[5]];
const RAIN_FOREST_PRESET: &'static [&'static Preset] =
    &[&PRESETS[0], &PRESETS[0], &PRESETS[0], &PRESETS[1], &PRESETS[2], &PRESETS[3]];
const SAVANNA_PRESET: &'static [&'static Preset] = &[&PRESETS[1]];
const STONE_PRESET: &'static [&'static Preset] = &[&PRESETS[5]];
const DEBUG_PRESET: &'static [&'static Preset] = &[&PRESETS[3]];

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
                             parent_diam: f32,
                             conifer: bool) {
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

                // FIXME Make branch spawn chance configurable
                if rng.gen_weighted_bool(depth * 3) {
                    // Build a vector for the branch direction (Z is up)
                    let dir = self.gen_branch_direction(rng, dir);
                    self.create_branch(rng, point, dir, depth + 1, diam, conifer);
                }

                points.push(ControlPoint {
                    point: point,
                    diameter: diam,
                });
            };

            // In a loop, get the length of the next segment from the current diameter.
            for _ in 0..segment_count {
                let length = if conifer {
                    (1.0 - 0.125 * start.z) * segment_dist(segment_length, diam)
                    // self.preset.height_branchlength_dependence(start.z) *
                    // segment_dist(segment_length, diam)
                } else {
                    segment_dist(segment_length, diam)
                };
                diam *= diam_factor;

                add_point(length, diam);

                if diam < 0.005 {
                    // Bail out at 5mm
                    break;
                }
            }
        }

        assert!(points.len() >= 2,
                "should've generated at least 2 points :(");
        self.branches.push(Branch { points: points });
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
        let conifer = self.preset.conifer;

        // The trunk is supposed to get smaller as we go up, so just enforce that rule
        // here:
        trunk_diameter_top = trunk_diameter.min(trunk_diameter_top);

        debug!("trunk diam {} to {}, height {}, branch start at {}",
               trunk_diameter,
               trunk_diameter_top,
               trunk_height,
               min_branch_height);

        let mut points = Vec::new();

        {
            let mut add_point = |height, diam| {
                let point = Point3f::new(0.0, 0.0, height);
                if height >= min_branch_height {
                    let branches = &[0, 1, 1, 1, 2, 2, 3, 3];
                    for _ in 0..(((*rng.choose(branches).unwrap()) as f32 *
                                  self.preset.branch_chance) +
                                 0.5) as usize {
                        // Build a vector for the branch direction (Z is up)
                        let dir = self.gen_branch_direction(rng, Vector3f::new(0.0, 0.0, 1.0));
                        self.create_branch(rng, point, dir, 1, diam, conifer);
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
        }

        self.branches.push(Branch { points: points });

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
            branch_color: Vector3f::new(range_sample(&self.preset.branch_color.0, rng),
                                        range_sample(&self.preset.branch_color.1, rng),
                                        range_sample(&self.preset.branch_color.2, rng)),
        }
    }

    pub fn rand<R: Rng>(rng: &mut R, biome: Biome) -> Self {
        // Select a random preset that we'll use

        let matched_preset = match biome {
            Biome::GrassLand => GRASS_LAND_PRESET,
            Biome::Desert => DESERT_PRESET,
            Biome::Snow => SNOW_PRESET,
            Biome::Forest => FOREST_PRESET,
            Biome::RainForest => RAIN_FOREST_PRESET,
            Biome::Savanna => SAVANNA_PRESET,
            Biome::Stone => STONE_PRESET,
            Biome::Debug => DEBUG_PRESET,
        };
        let preset = rng.choose(matched_preset).unwrap().clone();

        TreeGen {
            preset: preset,
            branches: Vec::new(),
        }



    }
}


/// Samples a random element from a range.
fn range_sample<T: SampleRange + cmp::PartialOrd + Copy, R: Rng>(range: &Range<T>,
                                                                 rng: &mut R)
                                                                 -> T {
    // Build a `rand` crate Range. We use `std`s Range for the cool `a..b` syntax ;)
    distributions::Range::new(range.start, range.end).ind_sample(rng)
}

/// Approximation of real-world distance of branch segments, depending on the
/// starting branch diameter.
fn segment_dist(segment_length: f32, diameter: f32) -> f32 {
    diameter * segment_length
}
