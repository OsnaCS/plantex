use base::world;
use std::f32::consts;
use world::chunk_view::Vertex;
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use GameContext;
use std::rc::Rc;
use base::math::*;
use Camera;
use util::ToArr;

pub struct ChunkRenderer {
    /// Chunk shader
    program: Program,
    /// Vertex buffer for a single `HexPillar`, repeated, scaled and colored as
    /// needed to draw chunks.
    pillar_vbuf: VertexBuffer<Vertex>,
    /// Index Buffer for `pillar_vbuf`.
    pillar_ibuf: IndexBuffer<u32>,
}

impl ChunkRenderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        get_top_hexagon_model(&mut vertices, &mut indices);
        get_bottom_hexagon_model(&mut vertices, &mut indices);
        get_side_hexagon_model(4, 5, &mut vertices, &mut indices);
        get_side_hexagon_model(1, 2, &mut vertices, &mut indices);
        get_side_hexagon_model(5, 0, &mut vertices, &mut indices);
        get_side_hexagon_model(0, 1, &mut vertices, &mut indices);
        get_side_hexagon_model(3, 4, &mut vertices, &mut indices);
        get_side_hexagon_model(2, 3, &mut vertices, &mut indices);

        ChunkRenderer {
            program: context.load_program("chunk_std").unwrap(),
            pillar_vbuf: VertexBuffer::new(context.get_facade(), &vertices).unwrap(),
            pillar_ibuf: IndexBuffer::new(context.get_facade(),
                                          PrimitiveType::TrianglesList,
                                          &indices)
                .unwrap(),
        }
    }

    /// Gets a reference to the shared chunk shader.
    pub fn program(&self) -> &Program {
        &self.program
    }

    /// Gets the `VertexBuffer` to use for drawing a pillar
    pub fn pillar_vertices(&self) -> &VertexBuffer<Vertex> {
        &self.pillar_vbuf
    }

    /// Gets the `IndexBuffer` to use for drawing a pillar
    pub fn pillar_indices(&self) -> &IndexBuffer<u32> {
        &self.pillar_ibuf
    }
}

pub struct HexagonOutline {
    program: Program,
    vbuf: VertexBuffer<Vertex>,
    ibuf: IndexBuffer<u32>,
    pub pos: Vector3f,
    pub display: bool,
}

impl HexagonOutline {
    pub fn new(context: Rc<GameContext>) -> Self {
        // Initialize HexagonOutline
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        get_top_hexagon_model(&mut vertices, &mut indices);
        get_bottom_hexagon_model(&mut vertices, &mut indices);
        get_side_hexagon_model(4, 5, &mut vertices, &mut indices);
        get_side_hexagon_model(1, 2, &mut vertices, &mut indices);
        get_side_hexagon_model(5, 0, &mut vertices, &mut indices);
        get_side_hexagon_model(0, 1, &mut vertices, &mut indices);
        get_side_hexagon_model(3, 4, &mut vertices, &mut indices);
        get_side_hexagon_model(2, 3, &mut vertices, &mut indices);

        HexagonOutline {
            program: context.load_program("outline").unwrap(),
            vbuf: VertexBuffer::new(context.get_facade(), &vertices).unwrap(),
            ibuf: IndexBuffer::new(context.get_facade(), PrimitiveType::LinesList, &indices)
                .unwrap(),
            pos: Vector3f::new(0.0, 0.0, 50.0),
            display: false,
        }
    }

    pub fn position(&self) -> &Vector3f {
        &self.pos
    }

    pub fn vertices(&self) -> &VertexBuffer<Vertex> {
        &self.vbuf
    }

    pub fn indices(&self) -> &IndexBuffer<u32> {
        &self.ibuf
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}



/// Calculates one Point-coordinates of a Hexagon
fn hex_corner(size: f32, i: i32) -> (f32, f32) {
    let angle_deg = 60.0 * (i as f32) + 30.0;
    let angle_rad = (consts::PI / 180.0) * angle_deg;

    (size * angle_rad.cos(), size * angle_rad.sin())
}

/// Calculates the top face of the Hexagon and normals
fn get_top_hexagon_model(vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let cur_len = vertices.len() as u32;
    for i in 0..6 {
        let (x, y) = hex_corner(world::HEX_OUTER_RADIUS, i);

        vertices.push(Vertex {
            position: [x, y, world::PILLAR_STEP_HEIGHT],
            normal: [0.0, 0.0, 1.0],
        });
    }

    vertices.push(Vertex {
        position: [0.0, 0.0, world::PILLAR_STEP_HEIGHT],
        normal: [0.0, 0.0, 1.0],
    });

    indices.append(&mut vec![cur_len + 0,
                             cur_len + 6,
                             cur_len + 1,
                             cur_len + 5,
                             cur_len + 6,
                             cur_len + 0,
                             cur_len + 4,
                             cur_len + 6,
                             cur_len + 5,
                             cur_len + 3,
                             cur_len + 6,
                             cur_len + 4,
                             cur_len + 2,
                             cur_len + 6,
                             cur_len + 3,
                             cur_len + 1,
                             cur_len + 6,
                             cur_len + 2]);
}

/// Calculates the bottom face of the Hexagon and the normals
fn get_bottom_hexagon_model(vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let cur_len = vertices.len() as u32;
    for i in 0..6 {
        let (x, y) = hex_corner(world::HEX_OUTER_RADIUS, i);

        vertices.push(Vertex {
            position: [x, y, 0.0],
            normal: [0.0, 0.0, -1.0],
        });
    }

    vertices.push(Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, -1.0],
    });
    indices.append(&mut vec![cur_len + 1,
                             cur_len + 6,
                             cur_len + 0,
                             cur_len + 0,
                             cur_len + 6,
                             cur_len + 5,
                             cur_len + 5,
                             cur_len + 6,
                             cur_len + 4,
                             cur_len + 4,
                             cur_len + 6,
                             cur_len + 3,
                             cur_len + 3,
                             cur_len + 6,
                             cur_len + 2,
                             cur_len + 2,
                             cur_len + 6,
                             cur_len + 1]);
}

/// Calculates the sides of the Hexagon and normals
fn get_side_hexagon_model(ind1: i32,
                          ind2: i32,
                          vertices: &mut Vec<Vertex>,
                          indices: &mut Vec<u32>) {
    let cur_len = vertices.len() as u32;
    let (x1, y1) = hex_corner(world::HEX_OUTER_RADIUS, ind1);
    let (x2, y2) = hex_corner(world::HEX_OUTER_RADIUS, ind2);
    let normal = [y1 + y2, x1 + x2, 0.0];

    vertices.push(Vertex {
        position: [x1, y1, world::PILLAR_STEP_HEIGHT],
        normal: normal,
    });
    vertices.push(Vertex {
        position: [x1, y1, 0.0],
        normal: normal,
    });
    vertices.push(Vertex {
        position: [x2, y2, world::PILLAR_STEP_HEIGHT],
        normal: normal,
    });
    vertices.push(Vertex {
        position: [x2, y2, 0.0],
        normal: normal,
    });

    indices.append(&mut vec![cur_len + 0,
                             cur_len + 2,
                             cur_len + 1,
                             cur_len + 1,
                             cur_len + 2,
                             cur_len + 3]);
}
