use glium;
use Camera;
use render::ToArr;
use base::math::*;
use base::world::{HexPillar, PropType};
use world::chunk::Vertex;

///Graphical representation of a "base::Plant"
pub struct PlantView {
    //contains the vertices for vertex shader
    vertices: glium::VertexBuffer<Vertex>,
    //program links vertex and fragment shader together
    program: glium::Program,
    plants_pos: Vec<Point2f>,
}

impl PlantView {
    ///Creates the graphical representation of given plant
    ///at the given plant offset
    pub fn from_dummy_plant<F>(hex_vec: &[HexPillar], pillars_pos: Vec<Point2f>,
            facade: &F) -> Self where F: glium::backend::Facade
    {
        //get height and width for the plant from given Prop
        let props = hex_vec[0].props();
        let _prop_ = &props[0];
        let p_type = &_prop_.prop;
        let width: f32;
        let height: f32;
        match p_type {
            &PropType::Plant(p) => { width = p.stem_width; height = p.height },
        }

        let buffer = vec![Vertex {
                            position: [0.0, 1.0*width, 0.0],
                            color: [0.545, 0.27, 0.075],
                           },
                           Vertex {
                                position: [0.0, 1.0*width, 1.0*height],
                                color: [0.545, 0.27, 0.075],
                           },
                            Vertex {
                                position: [1.0*width, 1.0*width, 1.0*height],
                                color: [0.545, 0.27, 0.075],
                            },
                            Vertex {
                                position: [1.0*width, 1.0*width, 0.0],
                                color: [0.545, 0.27, 0.075],
                            },
                            Vertex {
                                position: [1.0*width, 0.0, 1.0*height],
                                color: [0.545, 0.27, 0.075],
                            },
                            Vertex {
                                position: [1.0*width, 0.0, 0.0],
                                color: [0.545, 0.27, 0.075],
                            },
                            Vertex {
                                position: [0.0, 0.0, 0.0],
                                color: [0.545, 0.27, 0.075],
                            },
                            Vertex {
                                position: [0.0, 0.0, 1.0*height],
                                color: [0.545, 0.27, 0.075],
                            }];

        let vbuf = glium::VertexBuffer::new(facade, &buffer).unwrap();
        let prog = glium::Program::from_source(facade,
                                            include_str!("plant_dummy.vert"),
                                            include_str!("plant_dummy.frag"),
                                            None).unwrap();
        PlantView {
            vertices: vbuf,
            program: prog,
            plants_pos: pillars_pos,
        }
    }

    pub fn draw<S>(&self, surface: &mut S, camera: &Camera)
        where S: glium::Surface
    {
        let ibuf = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
