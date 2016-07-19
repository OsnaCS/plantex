use super::{GroundMaterial, HeightType};


#[derive(Clone, Default, Debug)]
pub struct HexPillar {
    sections: Vec<PillarSection>,
    props: Vec<Prop>,
}

impl HexPillar {
    pub fn sections(&self) -> &[PillarSection] {
        &self.sections
    }

    pub fn props(&self) -> &[Prop] {
        &self.props
    }
}

// from < to
#[derive(Clone, Debug)]
pub struct PillarSection {
    ground: GroundMaterial,
    from: HeightType,
    to: HeightType,
}

#[derive(Clone, Debug)]
pub struct Prop {
    baseline: HeightType,
    prop: PropType,
}

#[derive(Clone, Debug)]
pub enum PropType {
    Plant,
}
