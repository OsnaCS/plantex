use glium::backend::Context;
use Camera;
use render::ToArr;
use base::math::*;
use base::world::{HexPillar, PropType};
use world::chunk::Vertex;
use std::rc::Rc;
use glium;

/// Graphical representation of a 'base::Plant'
pub struct PlantView {
    /// contains the vertices for vertex shader
    vertices: glium::VertexBuffer<Vertex>,
    /// program links vertex and fragment shader together
    program: glium::Program,
    plants_pos: Vec<Point2f>,
    /// Context for IndexBuffer
    con: Rc<Context>,
}

impl PlantView {
    /// Creates the graphical representation of given plant
    /// at the given plant offset
    pub fn from_dummy_plant<F>(hex_vec: &[HexPillar], pillars_pos: Vec<Point2f>, facade: &F) -> Self
        where F: glium::backend::Facade
    {
        // get height and width for the plant from given Prop
        let (width, height) = match hex_vec[0].props()[0].prop {
            PropType::Plant(p) => (p.stem_width, p.height),
        };

        // Context for the draw method
        // to fill the IndexBuffer
        let c = facade.get_context().clone();

        let buffer = vec![Vertex {
                              position: [0.0, 1.0 * width, 0.0],
                              color: [0.545, 0.27, 0.075],
                          },
                          Vertex {
                              position: [0.0, 1.0 * width, 1.0 * height],
                              color: [0.545, 0.27, 0.075],
                          },
                          Vertex {
                              position: [1.0 * width, 1.0 * width, 1.0 * height],
                              color: [0.545, 0.27, 0.075],
                          },
                          Vertex {
                              position: [1.0 * width, 1.0 * width, 0.0],
                              color: [0.545, 0.27, 0.075],
                          },
                          Vertex {
                              position: [1.0 * width, 0.0, 1.0 * height],
                              color: [0.545, 0.27, 0.075],
                          },
                          Vertex {
                              position: [1.0 * width, 0.0, 0.0],
                              color: [0.545, 0.27, 0.075],
                          },
                          Vertex {
                              position: [0.0, 0.0, 0.0],
                              color: [0.545, 0.27, 0.075],
                          },
                          Vertex {
                              position: [0.0, 0.0, 1.0 * height],
                              color: [0.545, 0.27, 0.075],
                          }];

        let vbuf = glium::VertexBuffer::new(facade, &buffer).unwrap();
        let prog = glium::Program::from_source(facade,
                                               include_str!("plant_dummy.vert"),
                                               include_str!("plant_dummy.frag"),
                                               None)
            .unwrap();

        PlantView {
            vertices: vbuf,
            program: prog,
            plants_pos: pillars_pos,
            con: c,
        }
    }

    pub fn draw<S>(&self, surface: &mut S, camera: &Camera)
        where S: glium::Surface
    {
        // initialise the IndexBuffer
        let index: [u32; 30] = [1, 0, 2, 2, 0, 3, 2, 3, 4, 4, 3, 5, 4, 5, 6, 4, 6, 7, 7, 6, 0, 7,
                                0, 1, 1, 4, 7, 4, 1, 2];
        let ibuf = glium::index::IndexBuffer::new(&self.con,
                                                  glium::index::PrimitiveType::TrianglesList,
                                                  &index)
            .unwrap();

        for plant_pos in &self.plants_pos {
            let uniforms = uniform! {
                offset: [plant_pos.x, plant_pos.y],
                proj_matrix: camera.proj_matrix().to_arr(),
                view_matrix: camera.view_matrix().to_arr(),
            };

            surface.draw(&self.vertices,
                      &ibuf,
                      &self.program,
                      &uniforms,
                      &Default::default())
                .unwrap();
        }

    }
}
