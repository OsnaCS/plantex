use math::{Point3f, Vector3f};

/// Contains all types of plants we can generate.
#[derive(Clone, Debug)]
pub enum Plant {
    /// A parameterized tree-like structure.
    Tree(Tree),
}

#[derive(Clone, Debug)]
pub struct Tree {
    /// The list of branches representing this tree.
    pub branches: Vec<Branch>,
    /// Color for all trunks defined by is_trunk in `Branch`
    /// The vector holds elements in range `0..1` representing the RGB
    /// color channels.
    pub trunk_color: Vector3f,
    /// Color for all remaining `Branch`es
    /// The vector holds elements in range `0..1` representing the RGB
    /// color channels.
    pub leaf_color: Vector3f,
}

#[derive(Clone, Debug)]
pub struct Branch {
    /// At least 2 points describing the branch.
    ///
    /// The branch extends through all `ControlPoint`s in order.
    pub points: Vec<ControlPoint>,

    /// Tells whether trunk_color or leaf_color should be used for this branch.
    pub is_trunk: bool,
}

#[derive(Clone, Debug)]
pub struct ControlPoint {
    /// The location of this point in model coordinates (relative to the tree
    /// position).
    pub point: Point3f,
    /// The diameter of the branch at this point.
    pub diameter: f32,
}
