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
            Plant::Tree(Tree { ref branches, trunk_color, leaf_color }) => {
                for branch in branches {
                    let color = if branch.is_trunk { trunk_color } else { leaf_color };
                    gen_branch_buffer(&branch.points, &mut vertices, &mut indices, color);
                }
            }
        };

        PlantView {
            vertices: VertexBuffer::new(facade, &vertices).unwrap(),
            indices: IndexBuffer::new(facade,
                                      PrimitiveType::Patches { vertices_per_patch: 3 },
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
            multisampling: true,
            ..Default::default()
        };

        surface.draw(&self.vertices,
                  &self.indices,
                  &self.renderer.program(),
                  &uniforms,
                  &params)
            .unwrap();
    }

    pub fn draw<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let tess_level_inner = 10.0 as f32;
        let tess_level_outer = 10.0 as f32;

        let uniforms = uniform! {
            offset: self.pos.to_arr(),
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
            tess_level_inner: tess_level_inner,
            tess_level_outer: tess_level_outer,
            camera_pos: camera.position.to_arr(),
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

/// generates VertexBuffer and IndexBuffer for Plants
fn gen_branch_buffer(old_cps: &[ControlPoint],
                     vertices: &mut Vec<Vertex>,
                     indices: &mut Vec<u32>,
                     color: Vector3f) {
    let old_index_offset = vertices.len() as u32;
    let cps = {
        let mut cps = vec![old_cps[0]];
        cps.extend_from_slice(old_cps);
        cps.push(*old_cps.last().unwrap());
        cps
    };

    for window in cps.windows(3) {
        let prev_cp = window[0];
        let curr_cp = window[1];
        let next_cp = window[2];
        let dir = prev_cp.point - next_cp.point;

        for curr_point in &get_points_from_vector(dir) {
            vertices.push(Vertex {
                position: (curr_cp.point + curr_point * curr_cp.diameter).to_arr(),
                color: color.to_arr(),
                normal: curr_point.to_arr(),
            });
        }
    }


    for offset in 0..(old_cps.len() as u32) - 1 {
        let offset = offset * 3;
        let segment_indices = [0, 1, 3, 4, 3, 1, 1, 2, 4, 5, 4, 2, 2, 0, 5, 3, 5, 0];

        indices.extend(segment_indices.into_iter().map(|i| i + offset + old_index_offset));
    }

    let vert_len = vertices.len() as u32;
    indices.extend_from_slice(&[vert_len - 3, vert_len - 2, vert_len - 1]);
}

/// generates 3 normalized vectors perpendicular to the given vector
fn get_points_from_vector(vector: Vector3f) -> [Vector3f; 3] {
    let ortho = random_vec_with_angle(&mut seeded_rng(0x2651aa465abded, (), ()),
                                      vector.normalize(),
                                      90.0);
    let rot = Basis3::from_axis_angle(vector.normalize(), Deg::new(120.0).into());
    let v0 = rot.rotate_vector(ortho);
    let v1 = rot.rotate_vector(v0);

    [ortho.normalize(), v0.normalize(), v1.normalize()]
}
