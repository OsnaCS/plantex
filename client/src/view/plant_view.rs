use glium::backend::Facade;
use glium::{self, BackfaceCullingMode, DepthTest, DrawParameters, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use Camera;
use util::ToArr;
use base::math::*;
use base::gen::seeded_rng;
use base::prop::Plant;
use std::rc::Rc;
use super::PlantRenderer;
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
    pub normal: [f32; 3],
}

implement_vertex!(Vertex, position, color, normal);

/// generates Vertexbuffer and indexbuffer for a branch
fn get_vertices_for_branch(start: &ControlPoint,
                           end: &ControlPoint,
                           vertices: &mut Vec<Vertex>,
                           indices: &mut Vec<u32>,
                           color: Vector3f) {

    let to_end: Vector3f = Vector3f::new(end.point.x - start.point.x,
                                         end.point.y - start.point.y,
                                         end.point.z - start.point.z);
    let ortho = get_points_from_vector(to_end);

    let mut cur_len = vertices.len() as u32;

    // Bottom
    for vec in ortho.iter() {
        vertices.push(Vertex {
            position: [start.point.x + vec.x * start.diameter,
                       start.point.y + vec.y * start.diameter,
                       start.point.z + vec.z * start.diameter],
            normal: (-to_end).normalize().to_arr(),
            color: color.to_arr(),
        });
    }
    indices.append(&mut vec![cur_len + 2, cur_len + 0, cur_len + 1]);

    // Top
    cur_len = vertices.len() as u32;


    for vec in ortho.iter() {
        vertices.push(Vertex {
            position: [end.point.x + vec.x * end.diameter,
                       end.point.y + vec.y * end.diameter,
                       end.point.z + vec.z * end.diameter],
            normal: to_end.normalize().to_arr(),
            color: color.to_arr(),
        });
    }
    indices.append(&mut vec![cur_len + 1, cur_len + 0, cur_len + 2]);

    side(vertices, indices, color, start, end, ortho[0], ortho[1]);
    side(vertices, indices, color, start, end, ortho[1], ortho[2]);
    side(vertices, indices, color, start, end, ortho[2], ortho[0]);
}

/// Creates Vertexbuffer and IndexBuffer for a Side of the plants
fn side(vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        color: Vector3f,
        start: &ControlPoint,
        end: &ControlPoint,
        first_normal: Vector3f,
        second_normal: Vector3f) {
    let cur_len = vertices.len() as u32;

    vertices.push(Vertex {
        position: [end.point.x + first_normal.x * end.diameter,
                   end.point.y + first_normal.y * end.diameter,
                   end.point.z + first_normal.z * end.diameter],
        normal: (first_normal + second_normal).normalize().to_arr(),
        color: color.to_arr(),
    });

    vertices.push(Vertex {
        position: [end.point.x + second_normal.x * end.diameter,
                   end.point.y + second_normal.y * end.diameter,
                   end.point.z + second_normal.z * end.diameter],
        normal: (first_normal + second_normal).normalize().to_arr(),
        color: color.to_arr(),
    });

    vertices.push(Vertex {
        position: [start.point.x + first_normal.x * start.diameter,
                   start.point.y + first_normal.y * start.diameter,
                   start.point.z + first_normal.z * start.diameter],
        normal: (first_normal + second_normal).normalize().to_arr(),
        color: color.to_arr(),
    });

    vertices.push(Vertex {
        position: [start.point.x + second_normal.x * start.diameter,
                   start.point.y + second_normal.y * start.diameter,
                   start.point.z + second_normal.z * start.diameter],
        normal: (first_normal + second_normal).normalize().to_arr(),
        color: color.to_arr(),
    });

    indices.append(&mut vec![cur_len + 2,
                             cur_len + 0,
                             cur_len + 1,
                             cur_len + 2,
                             cur_len + 1,
                             cur_len + 3]);
}

/// generates 3 normalized vectors  perpendicular to the given vector
fn get_points_from_vector(vector: Vector3f) -> [Vector3f; 3] {
    let ortho = random_vec_with_angle(&mut seeded_rng(0x2651aa465abded, (), ()), vector, 90.0);
    let rot = Basis3::from_axis_angle(vector, Deg::new(120.0).into());
    let v0 = rot.rotate_vector(ortho);
    let v1 = rot.rotate_vector(v0);

    [ortho.normalize(), v0.normalize(), v1.normalize()]
}
