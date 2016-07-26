use glium::backend::Facade;
use glium::{self, BackfaceCullingMode, DepthTest, DrawParameters, IndexBuffer, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use Camera;
use util::ToArr;
use base::math::*;
use base::prop::Plant;
use std::rc::Rc;
use super::PlantRenderer;
use std::f32::consts;
use base::prop::plant::ControlPoint;

/// Graphical representation of a 'base::Plant'
pub struct PlantView {
    branches: Vec<VertexBuffer<Vertex>>,
    indices: IndexBuffer<u32>,
    renderer: Rc<PlantRenderer>,
    pos: Point3f,
}

impl PlantView {
    pub fn from_plant<F: Facade>(pos: Point3f,
                                 plant: &Plant,
                                 renderer: Rc<PlantRenderer>,
                                 facade: &F)
                                 -> Self {
        // FIXME handle other plant types
        let mut verts = 0;
        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        let branches = match *plant {
            Plant::Tree { ref branches } => {
                branches.iter()
                    .map(|branch| {

                        for i in 1..branch.points.len() {
                            get_vertices_for_branch(&branch.points[i - 1],
                                                    &branch.points[i],
                                                    &mut vertices,
                                                    &mut indices,
                                                    branch.color)
                        }
                        VertexBuffer::new(facade, &vertices).unwrap()
                    })
                    .collect()
            }
        };

        debug!("{} verts -> {:?}", verts, pos);

        PlantView {
            branches: branches,
            indices: IndexBuffer::new(facade, PrimitiveType::TrianglesList, &indices).unwrap(),
            renderer: renderer,
            pos: pos,
        }
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let uniforms = uniform! {
            offset: self.pos.to_arr(),
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

        for vbuf in &self.branches {
            surface.draw(vbuf,
                      &self.indices,
                      &self.renderer.program(),
                      &uniforms,
                      &params)
                .unwrap();
        }
    }
}

/// Vertex type used to render plants/trees.
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

fn triangle_corner(size: f32, i: i32) -> (f32, f32) {
    let angle_deg = 120.0 * (i as f32);
    let angle_rad = (consts::PI / 180.0) * angle_deg;

    (size * angle_rad.cos(), size * angle_rad.sin())
}

fn get_vertices_for_branch(start: &ControlPoint,
                           end: &ControlPoint,
                           vertices: &mut Vec<Vertex>,
                           indices: &mut Vec<u32>,
                           color: Vector3f)
                           -> () {

    let cur_len = vertices.len() as u32;
    for i in 0..3 {
        let (x, y) = triangle_corner(end.diameter, i);
        vertices.push(Vertex {
            position: [end.point.x + x, end.point.y + y, end.point.z],
            color: color.to_arr(),
        });
    }
    for i in 0..3 {
        let (x, y) = triangle_corner(start.diameter, i);
        vertices.push(Vertex {
            position: [start.point.x + x, start.point.y + y, start.point.z],
            color: color.to_arr(),
        });
    }


    indices.append(&mut vec![cur_len + 2,
                             cur_len + 1,
                             cur_len + 0,
                             cur_len + 5,
                             cur_len + 4,
                             cur_len + 3,
                             cur_len + 0,
                             cur_len + 1,
                             cur_len + 3,
                             cur_len + 1,
                             cur_len + 4,
                             cur_len + 3,
                             cur_len + 5,
                             cur_len + 2,
                             cur_len + 0,
                             cur_len + 5,
                             cur_len + 0,
                             cur_len + 3,
                             cur_len + 4,
                             cur_len + 1,
                             cur_len + 2,
                             cur_len + 4,
                             cur_len + 2,
                             cur_len + 5]);
}
