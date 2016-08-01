use glium::backend::Facade;
use glium::{self, BackfaceCullingMode, DepthTest, DrawParameters, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use Camera;
use util::ToArr;
use base::math::*;
use base::gen::seeded_rng;
use base::prop::plant::{Plant, Tree};
use std::rc::Rc;
use super::PlantRenderer;
use base::prop::plant::ControlPoint;

/// Graphical representation of a 'base::Plant'
pub struct PlantView {
    vertices: VertexBuffer<Vertex>,
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
        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        match *plant {
            Plant::Tree(Tree { ref branches, branch_color }) => {
                for branch in branches {
                    for i in 1..branch.points.len() {
                        get_vertices_for_branch(&branch.points[i - 1],
                                                &branch.points[i],
                                                &mut vertices,
                                                &mut indices,
                                                branch_color);
                    }
                }
            }
        };

        PlantView {
            vertices: VertexBuffer::new(facade, &vertices).unwrap(),
            indices: IndexBuffer::new(facade,
                                      PrimitiveType::Patches { vertices_per_patch: 6 },
                                      &indices)
                .unwrap(),
            renderer: renderer,
            pos: pos,
        }
    }

    pub fn draw_shadow<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
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
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        surface.draw(&self.vertices,
                  &self.indices,
                  &self.renderer.shadow_program(),
                  &uniforms,
                  &params)
            .unwrap();
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let tess_level_inner = 5.0 as f32;
        let tess_level_outer = 5.0 as f32;

        let uniforms = uniform! {
            offset: self.pos.to_arr(),
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
            tess_level_inner: tess_level_inner ,
            tess_level_outer: tess_level_outer,
        };

        let params = DrawParameters {
            depth: glium::Depth {
                write: true,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullCounterClockwise,
            // polygon_mode: PolygonMode::Line,
            ..Default::default()
        };

        surface.draw(&self.vertices,
                  &self.indices,
                  &self.renderer.program(),
                  &uniforms,
                  &params)
            .unwrap();
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

    let to_end = end.point - start.point;
    let ortho = get_points_from_vector(to_end);

    let mut cur_len = vertices.len() as u32;

    // Bottom
    for vec in ortho.iter() {
        vertices.push(Vertex {
            position: (start.point + vec * start.diameter).to_arr(),
            normal: (-to_end).normalize().to_arr(),
            color: color.to_arr(),
        });
    }
    indices.append(&mut vec![cur_len + 0,
                             cur_len + 1,
                             cur_len + 2,
                             cur_len + 2,
                             cur_len + 3,
                             cur_len + 0]);

    // Top
    cur_len = vertices.len() as u32;


    for vec in ortho.iter() {
        vertices.push(Vertex {
            position: (end.point + vec * end.diameter).to_arr(),
            normal: to_end.normalize().to_arr(),
            color: color.to_arr(),
        });
    }
    indices.append(&mut vec![cur_len + 0,
                             cur_len + 3,
                             cur_len + 2,
                             cur_len + 2,
                             cur_len + 1,
                             cur_len + 0]);

    side(vertices, indices, color, start, end, ortho[0], ortho[1]);
    side(vertices, indices, color, start, end, ortho[1], ortho[2]);
    side(vertices, indices, color, start, end, ortho[2], ortho[3]);
    side(vertices, indices, color, start, end, ortho[3], ortho[0]);
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
    vertices.extend_from_slice(&[Vertex {
                                     position: (end.point + first_normal * end.diameter).to_arr(),
                                     normal: (first_normal + second_normal).normalize().to_arr(),
                                     color: color.to_arr(),
                                 },
                                 Vertex {
                                     position: (end.point + second_normal * end.diameter).to_arr(),
                                     normal: (first_normal + second_normal).normalize().to_arr(),
                                     color: color.to_arr(),
                                 },
                                 Vertex {
                                     position: (start.point + first_normal * start.diameter)
                                         .to_arr(),
                                     normal: (first_normal + second_normal).normalize().to_arr(),
                                     color: color.to_arr(),
                                 },
                                 Vertex {
                                     position: (start.point + second_normal * start.diameter)
                                         .to_arr(),
                                     normal: (first_normal + second_normal).normalize().to_arr(),
                                     color: color.to_arr(),
                                 }]);

    indices.extend_from_slice(&[cur_len + 2,
                                cur_len + 0,
                                cur_len + 1,
                                cur_len + 2,
                                cur_len + 1,
                                cur_len + 3]);
}

/// generates 3 normalized vectors  perpendicular to the given vector
fn get_points_from_vector(vector: Vector3f) -> [Vector3f; 4] {
    let ortho = random_vec_with_angle(&mut seeded_rng(0x2651aa465abded, (), ()), vector, 90.0);
    let rot = Basis3::from_axis_angle(vector, Deg::new(90.0).into());
    let v0 = rot.rotate_vector(ortho);
    let v1 = rot.rotate_vector(v0);
    let v2 = rot.rotate_vector(v1);

    [ortho.normalize(), v0.normalize(), v1.normalize(), v2.normalize()]
}
