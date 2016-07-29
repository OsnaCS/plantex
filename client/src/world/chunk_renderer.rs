use base::world;
use base::math::SQRT_3;
use std::f32::consts;
use world::chunk_view::Vertex;
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::{self, DrawParameters, IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use glium::texture::Texture2d;
use GameContext;
use std::rc::Rc;
use super::tex_generator;
use super::normal_converter;
use base::world::ground::GroundMaterial;

pub struct ChunkRenderer {
    /// Chunk shader
    program: Program,
    /// Shadow map shader
    shadow_program: Program,
    /// Vertex buffer for a single `HexPillar`, repeated, scaled and colored as
    /// needed to draw chunks.
    pillar_vbuf: VertexBuffer<Vertex>,
    /// Index Buffer for `pillar_vbuf`.
    pillar_ibuf: IndexBuffer<u32>,
    pub noise_sand: Texture2d,
    pub noise_snow: Texture2d,
    pub noise_grass: Texture2d,
    pub noise_stone: Texture2d,
    pub noise_dirt: Texture2d,
    pub noise_mulch: Texture2d,

    pub normal_sand: Texture2d,
    pub normal_snow: Texture2d,
    pub normal_grass: Texture2d,
    pub normal_stone: Texture2d,
    pub normal_dirt: Texture2d,
    pub normal_mulch: Texture2d,
    pub outline: HexagonOutline,
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

        let sand = tex_generator::create_texture_maps(GroundMaterial::Sand);
        let snow = tex_generator::create_texture_maps(GroundMaterial::Snow);
        let grass = tex_generator::create_texture_maps(GroundMaterial::Grass);
        let stone = tex_generator::create_texture_maps(GroundMaterial::Stone);
        let dirt = tex_generator::create_texture_maps(GroundMaterial::Dirt);
        let mulch = tex_generator::create_texture_maps(GroundMaterial::Mulch);

        // TODO: Maybe fix function return type instead of
        // calling create_sand(...).1
        // Are we only using the first value?
        ChunkRenderer {
            program: context.load_program("chunk_std").unwrap(),
            shadow_program: context.load_program("chunk_shadow").unwrap(),
            pillar_vbuf: VertexBuffer::new(context.get_facade(), &vertices).unwrap(),
            pillar_ibuf: IndexBuffer::new(context.get_facade(),
                                          PrimitiveType::TrianglesList,
                                          &indices)
                .unwrap(),
            noise_sand: Texture2d::new(context.get_facade(), sand.1).unwrap(),
            noise_snow: Texture2d::new(context.get_facade(), snow.1).unwrap(),
            noise_grass: Texture2d::new(context.get_facade(), grass.1).unwrap(),
            noise_stone: Texture2d::new(context.get_facade(), stone.1).unwrap(),
            noise_dirt: Texture2d::new(context.get_facade(), dirt.1).unwrap(),
            noise_mulch: Texture2d::new(context.get_facade(), mulch.1).unwrap(),

            normal_sand: Texture2d::new(context.get_facade(),
                                        normal_converter::convert(sand.0, 1.0))
                .unwrap(),
            normal_snow: Texture2d::new(context.get_facade(),
                                        normal_converter::convert(snow.0, 1.0))
                .unwrap(),
            normal_grass: Texture2d::new(context.get_facade(),
                                         normal_converter::convert(grass.0, 1.0))
                .unwrap(),
            normal_stone: Texture2d::new(context.get_facade(),
                                         normal_converter::convert(stone.0, 1.0))
                .unwrap(),
            normal_dirt: Texture2d::new(context.get_facade(),
                                        normal_converter::convert(dirt.0, 1.0))
                .unwrap(),
            normal_mulch: Texture2d::new(context.get_facade(),
                                         normal_converter::convert(mulch.0, 1.0))
                .unwrap(),
            outline: HexagonOutline::new(context),
        }
    }

    /// Gets a reference to the shared chunk shader.
    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn shadow_program(&self) -> &Program {
        &self.shadow_program
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

/// Calculate texture coordinates
fn tex_map(i: i32) -> (f32, f32) {
    match i {
        0 => (1.0 - (0.5 - SQRT_3 / 4.0), 0.25),
        1 => (1.0 - (0.5 - SQRT_3 / 4.0), 0.75),
        2 => (0.5, 1.0),
        3 => (0.5 - SQRT_3 / 4.0, 0.75),
        4 => (0.5 - SQRT_3 / 4.0, 0.25),
        5 => (0.5, 0.0),
        // TODO: ERROR HANDLING
        _ => (0.0, 0.0),
    }
}

/// Calculates the top face of the Hexagon and normals
fn get_top_hexagon_model(vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let cur_len = vertices.len() as u32;
    // Corner vertices
    for i in 0..6 {
        let (x, y) = hex_corner(world::HEX_OUTER_RADIUS, i);

        let (a, b) = tex_map(i);

        vertices.push(Vertex {
            position: [x, y, world::PILLAR_STEP_HEIGHT],
            normal: [0.0, 0.0, 1.0],
            radius: 1.0,
            tex_coords: [a, b],
        });
    }

    // Central Vertex
    vertices.push(Vertex {
        position: [0.0, 0.0, world::PILLAR_STEP_HEIGHT],
        normal: [0.0, 0.0, 1.0],
        radius: 0.0,
        tex_coords: [0.5, 0.5],
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

        let (a, b) = tex_map(i);

        vertices.push(Vertex {
            position: [x, y, 0.0],
            normal: [0.0, 0.0, -1.0],
            radius: 1.0,
            tex_coords: [a, b],
        });
    }

    vertices.push(Vertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        radius: 0.0,
        tex_coords: [0.5, 0.5],
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

    // TODO: tex_coords fix
    vertices.push(Vertex {
        position: [x1, y1, world::PILLAR_STEP_HEIGHT],
        normal: normal,
        radius: 0.0,
        tex_coords: [0.0, 2.0],
    });
    vertices.push(Vertex {
        position: [x1, y1, 0.0],
        normal: normal,
        radius: 0.0,
        tex_coords: [0.0, 0.0],
    });
    vertices.push(Vertex {
        position: [x2, y2, world::PILLAR_STEP_HEIGHT],
        normal: normal,
        radius: 0.0,
        tex_coords: [1.0, 2.0],
    });
    vertices.push(Vertex {
        position: [x2, y2, 0.0],
        normal: normal,
        radius: 0.0,
        tex_coords: [1.0, 0.0],
    });

    indices.append(&mut vec![cur_len + 0,
                             cur_len + 2,
                             cur_len + 1,
                             cur_len + 1,
                             cur_len + 2,
                             cur_len + 3]);
}
