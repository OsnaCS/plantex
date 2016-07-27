use base::world::{Chunk, HexPillar, PropType};
use base::math::*;
use glium::{self, DrawParameters, VertexBuffer};
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::backend::Facade;
use Camera;
use util::ToArr;
use view::{PlantRenderer, PlantView};
use world::ChunkRenderer;
use std::rc::Rc;
use glium::texture::texture2d::Texture2d;

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    renderer: Rc<ChunkRenderer>,
    pillars: Vec<PillarView>,
    /// Instance data buffer.
    pillar_buf: VertexBuffer<Instance>,
    texture: Texture2d,
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

        for (axial, pillar) in chunk.pillars() {
            let pos = offset.to_real() + axial.to_real();
            pillars.push(PillarView::from_pillar(pos, pillar, plant_renderer.clone(), facade));
            for section in pillar.sections() {
                sections.push(Instance {
                    material_color: section.ground.get_color(),
                    offset: [pos.x, pos.y, section.bottom.to_real()],
                    height: (section.top.units() - section.bottom.units()) as f32,
                });
            }
        }

        let data = vec![
            vec![(0u8, 0u8, 0u8), (0u8, 0u8, 255u8), (0u8, 0u8, 255u8)],
            vec![(0u8, 0u8, 0u8), (0u8, 0u8, 255u8), (0u8, 0u8, 255u8)],
            vec![(255u8, 0u8, 0u8), (0u8, 255u8, 0u8), (0u8, 255u8, 0u8)],
        ];

        let tex = Texture2d::new(facade, data).unwrap();

        ChunkView {
            renderer: chunk_renderer,
            pillars: pillars,
            pillar_buf: VertexBuffer::dynamic(facade, &sections).unwrap(),
            texture: tex,
        }
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {



        let uniforms = uniform! {
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
            my_texture: self.texture.sampled().wrap_function(::glium::uniforms::SamplerWrapFunction::Clamp),
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
    pub tex_coord: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coord);

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
    plants: Vec<PlantView>,
}

impl PillarView {
    fn from_pillar<F: Facade>(pos: Point2f,
                              pillar: &HexPillar,
                              plant_renderer: Rc<PlantRenderer>,
                              facade: &F)
                              -> PillarView {
        PillarView {
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
