use glium::backend::Facade;
use glium::{Depth, DepthTest, DrawParameters, Program, Surface, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use Camera;
use render::ToArr;
use base::math::*;
use base::prop::Plant;
use world::chunk_view::Vertex;

/// Graphical representation of a 'base::Plant'
pub struct PlantView {
    branches: Vec<VertexBuffer<Vertex>>,
    /// program links vertex and fragment shader together
    program: Program,
    pos: Point3f,
}

impl PlantView {
    pub fn from_plant<F: Facade>(pos: Point3f, plant: &Plant, facade: &F) -> Self {
        // FIXME this is just stupid, don't recompile the shader for every plant
        let prog = Program::from_source(facade,
                                        include_str!("plant_dummy.vert"),
                                        include_str!("plant_dummy.frag"),
                                        None)
            .unwrap();

        // FIXME handle other plant types
        let mut verts = 0;
        let branches = match *plant {
            Plant::Tree { ref branches } => {
                branches.iter()
                    .map(|branch| {
                        let mut vertices = Vec::new();
                        for p in branch.points.iter().map(|cp| cp.point) {
                            verts += 1;
                            vertices.push(Vertex {
                                position: [p.x, p.y, p.z + 50.0],
                                color: [0.0, 1.0, 0.0],
                            });
                        }

                        VertexBuffer::new(facade, &vertices).unwrap()
                    })
                    .collect()
            }
        };

        info!("{} verts -> {:?}", verts, pos);

        PlantView {
            branches: branches,
            program: prog,
            pos: pos,
        }
    }

    pub fn draw<S: Surface>(&self, surface: &mut S, camera: &Camera) {
        let uniforms = uniform! {
            offset: [self.pos.x, self.pos.y],
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
        };

        let params = DrawParameters {
            depth: Depth {
                write: true,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            ..Default::default()
        };

        for vbuf in &self.branches {
            surface.draw(vbuf,
                      &NoIndices(PrimitiveType::LineStrip),
                      &self.program,
                      &uniforms,
                      &params)
                .unwrap();
        }
    }
}
