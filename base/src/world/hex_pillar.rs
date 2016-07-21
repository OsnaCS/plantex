use super::{GroundMaterial, HeightType};
use prop;

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
    /// Creates a dummy pillar for early testing. FIXME: remove
    pub fn dummy() -> Self {
        HexPillar {
            sections: vec![PillarSection::new(GroundMaterial::Dirt, HeightType(0), HeightType(50))],
            props: vec![Prop {
                            baseline: HeightType(50),
                            prop: PropType::Plant(prop::Plant {
                                height: 5.0,
                                stem_width: 0.5,
                            }),
                        }],
        }
    }

    pub fn from_height(height: HeightType) -> Self {
        HexPillar {
            sections: vec![PillarSection::new(GroundMaterial::Dirt, HeightType(0), height)],
            props: vec![Prop {
                            baseline: HeightType(50),
                            prop: PropType::Plant(prop::Plant {
                                height: 5.0,
                                stem_width: 0.5,
                            }),
                        }],
        }
    }

    /// Returns a slice of this pillar's sections.
    pub fn sections(&self) -> &[PillarSection] {
        &self.sections
    }

    /// Returns a slice of this pillar's props.
    pub fn props(&self) -> &[Prop] {
        &self.props
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
