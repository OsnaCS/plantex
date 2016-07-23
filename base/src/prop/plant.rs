use math::{Point3f, Vector3f};

/// Contains all types of plants we can generate.
#[derive(Clone, Debug)]
pub enum Plant {
    /// A parameterized tree-like structure.
    Tree {
        /// The list of branches representing this tree.
        branches: Vec<Branch>, // FIXME: No texture representation, can't use glium.
    },
}

#[derive(Clone, Debug)]
pub struct Branch {
    /// At least 2 points describing the branch.
    ///
    /// The branch extends through all `ControlPoint`s in order.
    pub points: Vec<ControlPoint>,
    /// The color of this branch.
    ///
    /// Currently, a branch has a uniform color. The vector holds elements in
    /// range `0..1` representing the RGB color channels.
    pub color: Vector3f,
}

#[derive(Clone, Debug)]
pub struct ControlPoint {
    /// The location of this point in model coordinates (relative to the tree
    /// position).
    pub point: Point3f,
    /// The diameter of the branch at this point.
    pub diameter: f32,
}
