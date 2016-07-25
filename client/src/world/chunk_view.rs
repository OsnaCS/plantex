use base::world::{self, Chunk, HexPillar, PillarSection, PropType};
use base::math::*;
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::backend::Facade;
use glium::index::PrimitiveType;
use Camera;
use util::ToArr;
use std::collections::VecDeque;
use std::f32::consts;
use world::plant_view::PlantView;

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    vertices: VertexBuffer<Vertex>,
    program: Program,
    pillars: Vec<PillarView>,
    index_buffer: IndexBuffer<u32>,
}

/// Calculates one Point-coordinates of a Hexagon
fn hex_corner(size: f32, i: i32) -> (f32, f32) {
    let angle_deg = 60.0 * (i as f32) + 30.0;
    let angle_rad = (consts::PI / 180.0) * angle_deg;

    (size * angle_rad.cos(), size * angle_rad.sin())
}

impl ChunkView {
    /// Creates the graphical representation of given chunk at the given chunk
    /// offset
    pub fn from_chunk<F: Facade>(chunk: &Chunk, offset: AxialPoint, facade: &F) -> Self {

        // Create one hexagon for this chunk
        let mut hexagon_vertex_buffer = VecDeque::new();
        for i in 0..6 {
            let (x, y) = hex_corner(world::HEX_OUTER_RADIUS, i);

            hexagon_vertex_buffer.push_front(Vertex {
                position: [x, y, world::PILLAR_STEP_HEIGHT],
            });
            hexagon_vertex_buffer.push_back(Vertex { position: [x, y, 0.0] });

        }

        // convert to vector
        let final_buffer: Vec<_> = hexagon_vertex_buffer.into_iter().collect();


        let vbuf = VertexBuffer::new(facade, &final_buffer).unwrap();
        let prog = Program::from_source(facade,
                                        include_str!("chunk_std.vert"),
                                        include_str!("chunk_std.frag"),
                                        None)
            .unwrap();

        let mut pillars = Vec::new();
        for q in 0..world::CHUNK_SIZE * world::CHUNK_SIZE {
            let pos = offset.to_real() +
                      AxialVector::new((q / world::CHUNK_SIZE).into(),
                                       (q % world::CHUNK_SIZE).into())
                .to_real();

            // Every element is reachable because `Chunk` generates
            // a filled vector of `HexPillar` which changed in an Iterator
            let pillar = match chunk.pillars().next() {
                Some(p) => p,
                _ => unreachable!(),
            };
            pillars.push(PillarView::from_pillar(pos, pillar, facade));
        }

        // Indecies
        let raw_index_buffer = [5, 0, 1, 2, 5, 1, 4, 5, 2, 3, 4, 2 /* TOP */, 6, 7, 8, 8, 9,
                                6, 9, 11, 6, 9, 10, 11 /* BOTTOM */, 6, 5, 4, 7, 6, 4, 6, 0,
                                5, 11, 0, 6, 10, 1, 0, 11, 10, 0, 9, 2, 1, 10, 9, 1, 8, 3, 2, 9,
                                8, 2, 7, 4, 3, 8, 7, 3 /* Body */];


        let ibuf = IndexBuffer::new(facade, PrimitiveType::TrianglesList, &raw_index_buffer)
            .unwrap();

        ChunkView {
            vertices: vbuf,
            program: prog,
            pillars: pillars,
            index_buffer: ibuf,
        }
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        for pillar in &self.pillars {
            for section in &pillar.sections {
                let height = section.top.units() - section.bottom.units();

                let uniforms = uniform! {
                    height: height as f32,
                    offset: [pillar.pos.x, pillar.pos.y, section.bottom.to_real()],
                    proj_matrix: camera.proj_matrix().to_arr(),
                    view_matrix: camera.view_matrix().to_arr(),
                    material_color: section.ground.get_color(),
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

                surface.draw(&self.vertices,
                          &self.index_buffer,
                          &self.program,
                          &uniforms,
                          &params)
                    .unwrap();
            }

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
}

implement_vertex!(Vertex, position);

pub struct PillarView {
    pos: Point2f,
    sections: Vec<PillarSection>,
    plants: Vec<PlantView>,
}

impl PillarView {
    fn from_pillar<F: Facade>(pos: Point2f, pillar: &HexPillar, facade: &F) -> PillarView {
        PillarView {
            pos: pos,
            sections: pillar.sections().as_slice().to_vec(),
            plants: pillar.props()
                .map(|prop| {
                    match prop.prop {
                        PropType::Plant(ref plant) => {
                            let pos = Point3f::new(pos.x, pos.y, prop.baseline.to_real());
                            PlantView::from_plant(pos, plant, facade)
                        }
                    }
                })
                .collect(),
        }
    }
}
