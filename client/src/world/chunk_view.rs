use base::world::{World, ChunkIndex, Chunk, PillarSection, CHUNK_SIZE, HEX_OUTER_RADIUS, PILLAR_STEP_HEIGHT, HeightType};
use base::math::*;
use glium::index::PrimitiveType;
use glium::{self, DrawParameters, VertexBuffer, IndexBuffer};
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::backend::Facade;
use glium::texture::Texture2d;
use glium::uniforms::SamplerWrapFunction;
use glium::uniforms::MinifySamplerFilter;
use Camera;
use DayTime;
// use SimpleCull;
// use LOCATION;
use util::ToArr;
use world::ChunkRenderer;
use std::rc::Rc;
use base::world::ground::GroundMaterial;

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    pub offset: AxialPoint,
    renderer: Rc<ChunkRenderer>,
    /// Instance data buffer.
    // pillar_buf: VertexBuffer<Instance>,
    vertex_buf: VertexBuffer<Vertex>,
    index_buf: IndexBuffer<u32>,
}

impl ChunkView {
    /// Creates the graphical representation of given chunk at the given chunk
    /// offset
    pub fn from_chunk<F: Facade>(chunk: &Chunk,
                                 offset: AxialPoint,
                                 chunk_renderer: Rc<ChunkRenderer>,
                                 facade: &F)
                                 -> Self {
        let (raw_buf, raw_indices) = get_vertices(chunk);

        ChunkView {
            offset: offset,
            renderer: chunk_renderer,
            vertex_buf: VertexBuffer::new(facade, &raw_buf).unwrap(),
            index_buf: IndexBuffer::new(facade,
                                      PrimitiveType::TrianglesList,
                                      &raw_indices).unwrap(),
        }
    }

    pub fn draw_shadow<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let uniforms = uniform! {
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
            offset: self.offset.to_real().to_arr(),
        };

        let params = DrawParameters {
            depth: glium::Depth {
                write: true,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullCounterClockwise,
            multisampling: true,
            ..Default::default()
        };

        surface.draw(
            &self.vertex_buf,
            &self.index_buf,
            self.renderer.shadow_program(),
            &uniforms,
            &params).unwrap();
    }

    pub fn update<F: Facade>(&mut self, facade: &F, world: &World) {
        let chunk = world.chunk_at(ChunkIndex(self.offset)).unwrap();
        let (vbuf, ibuf) = get_vertices(&chunk);

        self.vertex_buf = VertexBuffer::new(facade, &vbuf).unwrap();
        self.index_buf = IndexBuffer::new(facade,
                                  PrimitiveType::TrianglesList,
                                  &ibuf).unwrap();

    }

    pub fn draw<S: glium::Surface>(&self,
                                   surface: &mut S,
                                   camera: &Camera,
                                   shadow_map: &Texture2d,
                                   depth_view_proj: &Matrix4<f32>,
                                   daytime: &DayTime,
                                   sun_dir: Vector3f) {
        let real_off = self.offset.to_real();
        let look_at2 = Vector2::new(camera.get_look_at_vector().x, camera.get_look_at_vector().y)
            .normalize();
        let pos2 = Point2::new(camera.position.x, camera.position.y);

        let player_to_chunk = real_off -
                              (pos2 + -look_at2 * CHUNK_SIZE.into() * HEX_OUTER_RADIUS * 2.8);
        if camera.get_look_at_vector().z.abs() < 0.6 && dot(player_to_chunk, look_at2) < 0.0 {
            return;
        }

        let uniforms = uniform! {
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
            shadow_map: shadow_map.sampled().wrap_function(SamplerWrapFunction::Clamp),
            depth_view_proj: depth_view_proj.to_arr(),
            sun_dir: sun_dir.to_arr(),
            offset: self.offset.to_real().to_arr(),
            cam_pos: camera.position.to_arr(),
            sun_color: daytime.get_sun_color().to_arr(),
            sky_light: daytime.get_sky_light().to_arr(),

            // Mipmapping and repeating the textures
            sand_texture:  self.renderer.noise_sand.sampled()
                .minify_filter(MinifySamplerFilter::NearestMipmapLinear)
                .wrap_function(SamplerWrapFunction::Repeat),
            snow_texture:  self.renderer.noise_snow.sampled()
                .minify_filter(MinifySamplerFilter::NearestMipmapLinear)
                .wrap_function(SamplerWrapFunction::Repeat),
            grass_texture: self.renderer.noise_grass.sampled()
                .minify_filter(MinifySamplerFilter::NearestMipmapLinear)
                .wrap_function(SamplerWrapFunction::Repeat),
            stone_texture: self.renderer.noise_stone.sampled()
                .minify_filter(MinifySamplerFilter::NearestMipmapLinear)
                .wrap_function(SamplerWrapFunction::Repeat),
            dirt_texture: self.renderer.noise_dirt.sampled()
                .minify_filter(MinifySamplerFilter::NearestMipmapLinear)
                .wrap_function(SamplerWrapFunction::Repeat),
            mulch_texture: self.renderer.noise_mulch.sampled()
                .minify_filter(MinifySamplerFilter::NearestMipmapLinear)
                .wrap_function(SamplerWrapFunction::Repeat),

            normal_sand: &self.renderer.normal_sand,
            normal_snow: &self.renderer.normal_snow,
            normal_grass: &self.renderer.normal_grass,
            normal_stone: &self.renderer.normal_stone,
            normal_dirt: &self.renderer.normal_dirt,
            normal_mulch: &self.renderer.normal_mulch,
        };
        let params = DrawParameters {
            depth: glium::Depth {
                write: true,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            point_size: Some(10.0),
            backface_culling: BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        surface.draw(
            &self.vertex_buf,
            &self.index_buf,
            self.renderer.program(),
            &uniforms,
            &params).unwrap();
    }
}


/// Vertex type used to render chunks (or hex pillars).
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub radius: f32,
    pub tex_coords: [f32; 2],
    pub material_color: [f32; 3],
    pub ground: i32,
}

implement_vertex!(Vertex, position, normal, radius, tex_coords, material_color, ground);

// Here are a few constants describing various properties of a hexagon in a
// grid. Our hexagons are pointy topped and their position is described by an
// axial coordinate (q, r). Every axial point has a corresponding (floating
// point) point (x, y) in world coordinates.
//
//  +--------------> q/x
//  |\  ⬢ ⬢ ⬢ ⬢ ⬢
//  | \  ⬢ ⬢ ⬢ ⬢ ⬢
//  |  \  ⬢ ⬢ ⬢ ⬢ ⬢
//  |   \  ⬢ ⬢ ⬢ ⬢ ⬢
//  V    V  ⬢ ⬢ ⬢ ⬢ ⬢
//  -y   -r
//
//
//
//     (q: 0, r: 1)     top      (q:1, r: 1)
//
//      top-left       _~^~_       top-right
//                  _~^     ^~_
//                ~^           ^~
//                |             |
// (q: -1, r: 0)  |             |       (q: 1, r: 0)
//                |             |
//                ^~_         _~^
//    bottom-left    ^~_   _~^   bottom-right
//   (q: -1, r: -1)     ^~^     (q: 0, r: -1)
//                    bottom

/// When the center of a hex pillar with the outer radius 1 sits in the origin
/// the corners have the following coordinates.
const NORM_CORNERS: &'static [Vector2f] = &[
    Vector2f { x: -SQRT_3 / 2.0, y:  0.5 }, // 0: top-left
    Vector2f { x:  0.0,          y:  1.0 }, // 1: top
    Vector2f { x:  SQRT_3 / 2.0, y:  0.5 }, // 2: top-right
    Vector2f { x:  SQRT_3 / 2.0, y: -0.5 }, // 3: bottom-right
    Vector2f { x:  0.0,          y: -1.0 }, // 4: bottom
    Vector2f { x: -SQRT_3 / 2.0, y: -0.5 }, // 5: bottom left
];

/// Groups together two corner coordinates of a specific edge.
const EDGE_CORNERS_TO_NEIGHBOR: &'static [(Vector2f, Vector2f)] = &[
    (NORM_CORNERS[0], NORM_CORNERS[1]), // q:  0, r:  1, top-left
    (NORM_CORNERS[1], NORM_CORNERS[2]), // q:  1, r:  1, top-right
    (NORM_CORNERS[2], NORM_CORNERS[3]), // q:  1, r:  0, right
    (NORM_CORNERS[3], NORM_CORNERS[4]), // q:  0, r: -1, bottom-right
    (NORM_CORNERS[4], NORM_CORNERS[5]), // q: -1, r: -1, bottom-left
    (NORM_CORNERS[5], NORM_CORNERS[0]), // q: -1, r:  0, left
];

const EDGE_NORMALS: &'static [Vector2f] = & [
    Vector2f { x: -0.5, y:  SQRT_3 / 2.0 }, // 0: top-left
    Vector2f { x:  0.5, y:  SQRT_3 / 2.0 }, // 1: top-right
    Vector2f { x:  1.0, y:  0.0          }, // 2: right
    Vector2f { x:  0.5, y: -SQRT_3 / 2.0 }, // 3: bottom-right
    Vector2f { x: -0.5, y: -SQRT_3 / 2.0 }, // 4: bottom-left
    Vector2f { x: -1.0, y:  0.0          }, // 5: left
];

/// UV texture coordinates for all corners
const CORNER_UV: &'static [[f32; 2]] = &[
    [1.0 - (0.5 - SQRT_3 / 4.0), 0.25],
    [1.0 - (0.5 - SQRT_3 / 4.0), 0.75],
    [0.5, 1.0],
    [0.5 - SQRT_3 / 4.0, 0.75],
    [0.5 - SQRT_3 / 4.0, 0.25],
    [0.5, 0.0],
];

/// We add faces for sides to neighbors in these directions
const SIDE_PROPAGATION_NEIGHBORS: &'static [EdgeDir] = &[
    EdgeDir::BottomRight,
    EdgeDir::Right,
    EdgeDir::TopRight,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
enum CornerDir {
    TopLeft,
    Top,
    TopRight,
    BottomRight,
    Bottom,
    BottomLeft,
}

// impl CornerDir {
//     /// Index for array lookup
//     fn idx(&self) -> usize {
//         match *self {
//             CornerDir::TopLeft => 0,
//             CornerDir::Top => 1,
//             CornerDir::TopRight => 2,
//             CornerDir::BottomRight => 3,
//             CornerDir::Bottom => 4,
//             CornerDir::BottomLeft => 5,
//         }
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
enum EdgeDir {
    TopLeft,
    TopRight,
    Right,
    BottomRight,
    BottomLeft,
    Left,
}

impl EdgeDir {
    /// Index for array lookup
    fn idx(&self) -> usize {
        match *self {
            EdgeDir::TopLeft => 0,
            EdgeDir::TopRight => 1,
            EdgeDir::Right => 2,
            EdgeDir::BottomRight => 3,
            EdgeDir::BottomLeft => 4,
            EdgeDir::Left => 5,
        }
    }

    fn axial_vec(&self) -> AxialVector {
        match *self {
            EdgeDir::TopLeft        => AxialVector::new( 0,  1),
            EdgeDir::TopRight       => AxialVector::new( 1,  1),
            EdgeDir::Right          => AxialVector::new( 1,  0),
            EdgeDir::BottomRight    => AxialVector::new( 0, -1),
            EdgeDir::BottomLeft     => AxialVector::new(-1, -1),
            EdgeDir::Left           => AxialVector::new(-1,  0),
        }
    }
}

fn add_outer_shell(
    pos: AxialPoint,
    chunk: &Chunk,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
) {
    let neighbors = [
        EdgeDir::TopLeft,
        EdgeDir::TopRight,
        EdgeDir::Right,
        EdgeDir::BottomRight,
        EdgeDir::BottomLeft,
        EdgeDir::Left,
    ];

    for &neighbor in &neighbors {
        let neighbor_pos = pos + neighbor.axial_vec();
        let is_outer =
            neighbor_pos.q >= CHUNK_SIZE.into()
            || neighbor_pos.r >= CHUNK_SIZE.into()
            || neighbor_pos.q < 0
            || neighbor_pos.r < 0;

        if is_outer {
            // if neighbor == EdgeDir::BottomLeft;

            for sec in chunk[pos].sections() {
                add_side(
                    sec.bottom,
                    sec.top,
                    sec.ground,
                    false,
                    neighbor,
                    pos,
                    vertices,
                    indices,
                );
            }
        }
    }
}

fn get_vertices(chunk: &Chunk) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    Chunk::for_pillars_positions(|pos| {
        for sec in chunk[pos].sections() {
            let ground = sec.ground.get_id();

            // Add top and bottom face
            for &(height, normal, rev) in &[(sec.top, [0.0, 0.0, 1.0], false), (sec.bottom, [0.0, 0.0, -1.0], true)] {
                let prev_len = vertices.len() as u32;

                // we skip all bottom faces at 0 completely
                if height.units() == 0 {
                    continue;
                }

                // Add center point
                vertices.push(Vertex {
                    position: [pos.to_real().x, pos.to_real().y, height.to_real()],
                    normal: normal,
                    radius: 0.0,
                    tex_coords: [0.5, 0.5],
                    material_color: sec.ground.get_color(),
                    ground: ground,
                });

                let iter = NORM_CORNERS.iter().map(|c| c * HEX_OUTER_RADIUS).zip(CORNER_UV).enumerate();
                for (i, (corner, &uv)) in iter {
                    let i = i as u32;
                    let pos2d = pos.to_real() + corner;

                    vertices.push(Vertex {
                        position: [pos2d.x, pos2d.y, height.to_real()],
                        normal: normal,
                        radius: 1.0,
                        tex_coords: uv,
                        material_color: sec.ground.get_color(),
                        // material_color: tmp_col,
                        ground: ground,
                    });

                    if rev {
                        indices.push(prev_len);
                        indices.push(prev_len + ((i + 1) % 6) + 1);
                        indices.push(prev_len + i + 1);
                    } else {
                        indices.push(prev_len);
                        indices.push(prev_len + i + 1);
                        indices.push(prev_len + ((i + 1) % 6) + 1);
                    }
                }
            }
        }

        for &dir in SIDE_PROPAGATION_NEIGHBORS {
            connect_pillars(pos, dir, chunk, &mut vertices, &mut indices);
        }

        add_outer_shell(pos, chunk, &mut vertices, &mut indices);
    });

    (vertices, indices)
}

fn connect_pillars(
    a_pos: AxialPoint,
    a_to_b: EdgeDir,
    chunk: &Chunk,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
) {
    let neighbor_pos = a_pos + a_to_b.axial_vec();

    let skip = neighbor_pos.q >= CHUNK_SIZE.into()
        || neighbor_pos.r >= CHUNK_SIZE.into()
        || neighbor_pos.q < 0
        || neighbor_pos.r < 0;
    if skip {
        return;
    }

    let a = &chunk[a_pos];
    let b = &chunk[neighbor_pos];

    let ab = [a, b];

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    enum Pillar {
        A,
        B,
    }

    impl Pillar {
        fn idx(&self) -> usize {
            match *self {
                Pillar::A => 0,
                Pillar::B => 1,
            }
        }
    }


    // ----------------------------------------------------------------------
    /// This iterator takes two slices of `PillarSection`s and spits out a
    /// series of tuples `(height, pillar, is_top)`. The yielded items are
    /// ordered ascendingly by the key `height`. Each item knows from what
    /// pillar the height is from and if that height was the top or bottom
    /// of a pillar section.
    ///
    /// *Note*: this iterator assumes that the slices of pillar sections are
    /// sorted, that there are no overlapping sections and that bottom < top is
    /// true for all sections.
    struct IntervalPoints<'c> {
        a: &'c [PillarSection],
        b: &'c [PillarSection],
        a_idx: usize,
        b_idx: usize,
        /// `true` means that `a.bottom` was already yielded
        within_a: bool,
        /// `true` means that `b.bottom` was already yielded
        within_b: bool,
    }

    impl<'c> IntervalPoints<'c> {
        fn new(a: &'c [PillarSection], b: &'c [PillarSection]) -> Self {
            IntervalPoints {
                a: a,
                b: b,
                a_idx: 0,
                b_idx: 0,
                within_a: false,
                within_b: false,
            }
        }
    }

    impl<'c> Iterator for IntervalPoints<'c> {
        type Item = (HeightType, Pillar, usize, usize, bool);
        fn next(&mut self) -> Option<Self::Item> {
            // Determine the next height that would be yielded from a/b.
            let next_a = self.a.get(self.a_idx).map(|sec| {
                if self.within_a {
                    sec.top
                } else {
                    sec.bottom
                }
            });
            let next_b = self.b.get(self.b_idx).map(|sec| {
                if self.within_b {
                    sec.top
                } else {
                    sec.bottom
                }
            });

            // If both pillars can still yield height points, choose the one
            // with the smaller height. Otherwise choose the one thats
            // available.
            let yield_from_a = match (next_a, next_b) {
                (Some(ha), Some(hb)) => ha < hb,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                // we will yield `a` anyway, because it is `None`
                (None, None) => true,
            };

            if yield_from_a {
                let sec_a_idx = self.a_idx;
                if self.within_a {
                    // we will now yield a's top
                    self.a_idx += 1;
                }
                self.within_a = !self.within_a;

                next_a.map(|h| (h, Pillar::A, sec_a_idx, self.b_idx, !self.within_a))
            } else {
                let sec_b_idx = self.b_idx;
                if self.within_b {
                    // we will now yield b's top
                    self.b_idx += 1;
                }
                self.within_b = !self.within_b;

                next_b.map(|h| (h, Pillar::B, self.a_idx, sec_b_idx, !self.within_b))
            }
        }
    }

    // ----------------------------------------------------------------------
    /// This iterator takes another iterator and groups together two yielded
    /// items from the original iterator in a pair. *Note*: it assumes that the
    /// original iterator yields an even number of items.
    struct PairUp<I: Iterator> {
        original: I,
    }

    impl<I: Iterator> Iterator for PairUp<I> {
        type Item = (I::Item, I::Item);

        fn next(&mut self) -> Option<Self::Item> {
            self.original.next().map(|first| (
                first,
                self.original.next()
                    .expect("original iterator yielded an odd number of items"),
            ))
        }
    }

    // ----------------------------------------------------------------------
    // Here we combine both iterators and create a third one by applying a
    // rather complicated map function. This third and final iterator yields
    // the final data to add a side.
    let raw_intervals = PairUp {
        original: IntervalPoints::new(a.sections(), b.sections()) //.inspect(|x| println!("-- {:?}", x))
    };
    let sides = raw_intervals.map(|(lower, upper)| {
        let (l_height, l_pillar,          _ , l_sec_b_idx, l_is_top) = lower;
        let (u_height, u_pillar, u_sec_a_idx, u_sec_b_idx, u_is_top) = upper;

        let (pillar, sec_idx, normal_to_a) = match ((l_pillar, l_is_top), (u_pillar, u_is_top)) {
            // It's impossible to have two consecutive interval points of the
            // same type: the height list is sorted and there should be a top
            // in between two bottoms of one pillar.
            ((Pillar::A, false), (Pillar::A, false)) => unreachable!(),
            ((Pillar::A,  true), (Pillar::A,  true)) => unreachable!(),
            ((Pillar::B, false), (Pillar::B, false)) => unreachable!(),
            ((Pillar::B,  true), (Pillar::B,  true)) => unreachable!(),

            // These are the cases where both interval points are from the same
            // pillar. This means that either the two sections do not intersect
            // at all in the given interval or that one section is a super-
            // interval of two other ones:
            //
            // ---+
            //    | +---
            //    | | b
            //    | +---
            //  a |           <-- (b.top, b.bottom)
            //    | +---
            //    | | b
            //    | +---
            // ---+
            //
            //      +---
            // ---+ |
            //  a | |
            // ---+ |
            //      | b       <-- (a.top, a.bottom)
            // ---+ |
            //  a | |
            // ---+ |
            //      +---
            ((Pillar::A, false), (Pillar::A, true)) => (Pillar::A, u_sec_a_idx, false),
            ((Pillar::B, false), (Pillar::B, true)) => (Pillar::B, u_sec_b_idx, true),
            ((Pillar::A, true), (Pillar::A, false)) => (Pillar::B, u_sec_b_idx, true),
            ((Pillar::B, true), (Pillar::B, false)) => (Pillar::A, u_sec_a_idx, false),

            // ---+
            //    |         <-- (b.top, a.top)       [1]  (=> A)
            //  a | +---
            //    | |       <-- (a.bottom, b.top)    [2]
            // ---+ | b
            //      |       <-- (b.bottom, a.bottom) [3]  (=> B)
            //      +---
            //
            // Cases [1] and [3] are possible, case [2] is not: the iterator
            // always pairs up two consecutive height values (which are sorted)
            // and thus it's impossible to get such a pair.
            ((Pillar::B,  true), (Pillar::A,  true)) => (Pillar::A, u_sec_a_idx, false),
            ((Pillar::A, false), (Pillar::B,  true)) => unreachable!(),
            ((Pillar::B, false), (Pillar::A, false)) => (Pillar::B, l_sec_b_idx, true),

            //      +---
            //      |       <-- (a.top, b.top)       [1]  (=> B)
            // ---+ | b
            //    | |       <-- (b.bottom, a.top)    [2]
            //  a | +---
            //    |         <-- (a.bottom, b.bottom) [3]  (=> A)
            // ---+
            //
            // The same as above but with a and b switched.
            ((Pillar::A,  true), (Pillar::B,  true)) => (Pillar::B, l_sec_b_idx, true),
            ((Pillar::B, false), (Pillar::A,  true)) => unreachable!(),
            ((Pillar::A, false), (Pillar::B, false)) => (Pillar::A, u_sec_a_idx, false),

            // ---+
            //  a |
            // ---+
            //              <-- impossible
            //      +---
            //      | b
            //      +---
            //
            // The pairing argument from above applies here too: it's impossible
            // to get the displayed interval at this point.
            ((Pillar::A, true), (Pillar::B, false)) => unreachable!(),
            ((Pillar::B, true), (Pillar::A, false)) => unreachable!(),
        };

        (l_height, u_height, pillar, sec_idx, normal_to_a)
    });

    for (lower, upper, pillar, sec_idx, normal_to_a) in sides {
        add_side(
            lower,
            upper,
            ab[pillar.idx()].sections()[sec_idx].ground,
            normal_to_a,
            a_to_b,
            a_pos,
            vertices,
            indices,
        );
    }
}


fn add_side(
    bottom: HeightType,
    top: HeightType,
    ground: GroundMaterial,
    normal_to_a: bool,
    dir: EdgeDir,
    offset: AxialPoint,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
) {
    let prev_len = vertices.len() as u32;
    let (ca, cb) = EDGE_CORNERS_TO_NEIGHBOR[dir.idx()];
    let normal = EDGE_NORMALS[dir.idx()];
    let corner_cw = [
        (offset.to_real() + ca, 0.25),
        (offset.to_real() + cb, 0.75),
    ];

    let v_bottom = 0.5 * (top.to_real() - bottom.to_real()) / PILLAR_STEP_HEIGHT;

    for &(z, v) in &[(bottom, v_bottom), (top, 0.0)] {
        for &(xy, u) in &corner_cw {
            vertices.push(Vertex {
                position: [xy.x, xy.y, z.to_real()],
                normal: [normal.x, normal.y, 0.0],
                radius: 0.0,
                tex_coords: [u, v],
                material_color: ground.get_color(),
                ground: ground.get_id(),
            });
        }
    }

    if normal_to_a {
        indices.extend_from_slice(&[
            prev_len, prev_len + 2, prev_len + 1,
            prev_len + 3, prev_len + 1, prev_len + 2,
        ]);
    } else {
        indices.extend_from_slice(&[
            prev_len, prev_len + 1, prev_len + 2,
            prev_len + 3, prev_len + 2, prev_len + 1,
        ]);
    };
}
