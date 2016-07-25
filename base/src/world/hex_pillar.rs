use super::{GroundMaterial, HeightType};
use prop;
use std::slice::Iter;
/// Represents one pillar of hexgonal shape in the game world.
///
/// A pillar consists of multiple sections (each of which has a material) and
/// optionally props (plants, objects, ...).
#[derive(Clone, Default, Debug)]
pub struct HexPillar {
    sections: Vec<PillarSection>,
    props: Vec<Prop>,
}

impl HexPillar {
    pub fn new(sections: Vec<PillarSection>, props: Vec<Prop>) -> Self {
        HexPillar {
            sections: sections,
            props: props,
        }
    }

    /// Returns an iterator of this pillar's sections.
    pub fn sections(&self) -> Iter<PillarSection> {
        self.sections.iter()
    }

    /// Returns an iterator of this pillar's props.
    pub fn props(&self) -> Iter<Prop> {
        self.props.iter()
    }
}

/// Represents one section of a hex pillar.
#[derive(Clone, Debug)]
pub struct PillarSection {
    pub ground: GroundMaterial,
    pub bottom: HeightType,
    pub top: HeightType,
}

impl PillarSection {
    /// Creates a new pillar section and asserts `bottom < top`.
    pub fn new(ground: GroundMaterial, bottom: HeightType, top: HeightType) -> Self {
        assert!(bottom < top, "attempt to create an invalid pillar section");

        PillarSection {
            ground: ground,
            bottom: bottom,
            top: top,
        }
    }
}

/// A prop in a hex pillar
#[derive(Clone, Debug)]
pub struct Prop {
    /// The height/baseline at which the prop starts
    pub baseline: HeightType,
    /// The actual prop data
    pub prop: PropType,
}

/// Represents one of many different prop types
#[derive(Clone, Debug)]
pub enum PropType {
    Plant(prop::Plant),
}
