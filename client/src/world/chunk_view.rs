use base::world::{self, Chunk, HexPillar, PillarSection, PropType};
use base::math::*;
use glium::{self, DrawParameters, VertexBuffer};
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::backend::Facade;
use Camera;
use util::ToArr;
use view::{PlantRenderer, PlantView};
use world::ChunkRenderer;
use std::rc::Rc;

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    renderer: Rc<ChunkRenderer>,
    pillars: Vec<PillarView>,
    /// Instance data buffer.
    pillar_buf: VertexBuffer<Instance>,
}

impl ChunkView {
    /// Creates the graphical representation of given chunk at the given chunk
    /// offset
    pub fn from_chunk<F: Facade>(chunk: &Chunk,
                                 offset: AxialPoint,
                                 chunk_renderer: Rc<ChunkRenderer>,
                                 plant_renderer: Rc<PlantRenderer>,
                                 facade: &F)
                                 -> Self {


        let mut sections = Vec::new();
        let mut pillars = Vec::new();
        for q in 0..world::CHUNK_SIZE * world::CHUNK_SIZE {
            let pos = offset.to_real() +
                      AxialVector::new((q / world::CHUNK_SIZE).into(),
                                       (q % world::CHUNK_SIZE).into())
                .to_real();
            let pillar = &chunk.pillars()[q as usize];

            pillars.push(PillarView::from_pillar(pos, pillar, plant_renderer.clone(), facade));

            for section in pillar.sections() {
                sections.push(Instance {
                    material_color: section.ground.get_color(),
                    offset: [pos.x, pos.y, section.bottom.to_real()],
                    height: (section.top.units() - section.bottom.units()) as f32,
                });
            }
        }

        ChunkView {
            renderer: chunk_renderer,
            pillars: pillars,
            pillar_buf: VertexBuffer::dynamic(facade, &sections).unwrap(),
        }
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let uniforms = uniform! {
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
        };
        let params = DrawParameters {
            depth: glium::Depth {
                write: true,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        surface.draw((self.renderer.pillar_vertices(), self.pillar_buf.per_instance().unwrap()),
                  self.renderer.pillar_indices(),
                  self.renderer.program(),
                  &uniforms,
                  &params)
            .unwrap();

        for pillar in &self.pillars {
            for plant in &pillar.plants {
                plant.draw(surface, camera);
            }
        }
    }
}


/// Vertex type used to render chunks (or hex pillars).
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

/// Instance data for each pillar section.
#[derive(Debug, Copy, Clone)]
pub struct Instance {
    /// Material color.
    material_color: [f32; 3],
    /// Offset in world coordinates.
    offset: [f32; 3],
    /// Pillar height.
    height: f32,
}

implement_vertex!(Instance, material_color, offset, height);

pub struct PillarView {
    pos: Point2f,
    sections: Vec<PillarSection>,
    plants: Vec<PlantView>,
}

impl PillarView {
    fn from_pillar<F: Facade>(pos: Point2f,
                              pillar: &HexPillar,
                              plant_renderer: Rc<PlantRenderer>,
                              facade: &F)
                              -> PillarView {
        PillarView {
            pos: pos,
            sections: pillar.sections().to_vec(),
            plants: pillar.props()
                .iter()
                .map(|prop| {
                    match prop.prop {
                        PropType::Plant(ref plant) => {
                            let pos = Point3f::new(pos.x, pos.y, prop.baseline.to_real());
                            PlantView::from_plant(pos, plant, plant_renderer.clone(), facade)
                        }
                    }
                })
                .collect(),
        }
    }
}
