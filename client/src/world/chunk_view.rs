use base::math::*;
use base::world::ground::GroundMaterial;
use base::world::{
    Chunk,
    CHUNK_SIZE,
    ChunkIndex,
    HeightType,
    HEX_OUTER_RADIUS,
    HexPillar,
    PILLAR_STEP_HEIGHT,
    PillarSection,
    World,
};
use Camera;
use DayTime;
use glium::backend::Facade;
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::index::PrimitiveType;
use glium::texture::Texture2d;
use glium::uniforms::MinifySamplerFilter;
use glium::uniforms::SamplerWrapFunction;
use glium::{self, DrawParameters, IndexBuffer, VertexBuffer};
use std::rc::Rc;
use util::ToArr;
use world::ChunkRenderer;

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    offset: AxialPoint,
    renderer: Rc<ChunkRenderer>,
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

    /// Returns the chunks's offset.
    pub fn offset(&self) -> AxialPoint {
        self.offset
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

// ============================================================================
// ===== functions and types for vertex and index buffer creation
// ============================================================================
// Our hexagons are pointy topped and their position is described by an axial
// coordinate (q, r). Every axial point has a corresponding (floating point)
// point (x, y) in world coordinates.
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
// --- Names of corners and coordinates of sides:
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
//
//
// --------------------------------------------------------------------------
// Here are a few constants describing coordinates and vectors of edges or
// corners of a hexagon.

/// When the center of a hex pillar with the outer radius 1 sits in the origin,
/// the corners have the following coordinates. To get real world positions
/// multiply these values with `HEX_OUTER_RADIUS`.
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

/// Normal vectors pointing to the edges of the hexagon. To get real world
/// positions, multiply these values with `HEX_INNER_RADIUS`.
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
    [                       0.5, 1.0 ],
    [      (0.5 - SQRT_3 / 4.0), 0.75],
    [      (0.5 - SQRT_3 / 4.0), 0.25],
    [                       0.5, 0.0 ],
];

/// We add faces for sides to neighbors in these directions.
const SIDE_PROPAGATION_NEIGHBORS: &'static [EdgeDir] = &[
    EdgeDir::BottomRight,
    EdgeDir::Right,
    EdgeDir::TopRight,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EdgeDir {
    TopLeft,
    TopRight,
    Right,
    BottomRight,
    BottomLeft,
    Left,
}

impl EdgeDir {
    /// Index for lookup in the const arrays above.
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

    /// The corresponding vector for the direction.
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

// --------------------------------------------------------------------------
// We have a few functions to create both buffers.

/// Generates a pair of vertex and index buffer which represent the given
/// `Chunk`.
///
/// This function (and the other helper functions) assume a few important
/// things about the given `Chunk`:
///
/// - all pillar sections within a pillar are sorted
/// - no two pillar sections within one pillar overlap
/// - `top > bottom` is true for all pillar sections
///
/// If the given chunk does not comply to these properties, this function could
/// do anything, including eating your laundry.
///
/// In the goemetry represented by the two returned buffers, there are no
/// inside facing faces with the exception of the "outer shell". In other
/// words: the buffer hold one or more closed goemetries which don't have
/// any faces inside. This, again, is not exactly true, because all faces
/// at height 0 are discarded, because the player will never see them anyway.
///
/// The buffers aren't minimal yet! There are still quite a few things that
/// could be optimized. For example, vertices of the top face could be shared
/// with the neighbor pillar under special circumstances. Another optimization
/// is a bit more ugly: we could connect side pieces with the same position and
/// orientation. Sadly this "creates" geometry inside our blobs of world.
fn get_vertices(chunk: &Chunk) -> (Vec<Vertex>, Vec<u32>) {
    // Make a crude guess how many vertices we will need. This assumes that the
    // chunk has at least one pillar section per pillar.
    //
    // Here we just account for the seven vertices/six triangles per top face.
    let minimal_vlen = (CHUNK_SIZE as usize).pow(2) * 7;
    let minimal_ilen = (CHUNK_SIZE as usize).pow(2) * 6 * 3;

    let mut vertices = Vec::with_capacity(minimal_vlen);
    let mut indices = Vec::with_capacity(minimal_ilen);

    Chunk::for_pillars_positions(|pos| {
        add_top_and_bottom_face(pos, &chunk[pos], &mut vertices, &mut indices);

        // We only do this in one direction to handle every edge only once.
        for &dir in SIDE_PROPAGATION_NEIGHBORS {
            connect_pillars(pos, dir, chunk, &mut vertices, &mut indices);
        }

        // Add sides to the outside if this pillar on the outer edge.
        add_outer_shell(pos, &chunk[pos], &mut vertices, &mut indices);
    });

    (vertices, indices)
}

/// Adds the top and bottom face for every section in a pillar, but completely
/// ignores all faces at height 0.
fn add_top_and_bottom_face(
    pos: AxialPoint,
    pillar: &HexPillar,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>
) {
    for sec in pillar.sections() {
        let ground = sec.ground.get_id();

        // Add top and bottom face
        let face_props = [
            (sec.top, [0.0, 0.0, 1.0], false),
            (sec.bottom, [0.0, 0.0, -1.0], true)
        ];

        for &(height, normal, rev) in &face_props {
            let prev_len = vertices.len() as u32;

            // We completely skip all faces at height 0
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

            // Iterate over all corners with position and uv coordinates
            let iter = NORM_CORNERS.iter()
                .map(|c| c * HEX_OUTER_RADIUS)
                .zip(CORNER_UV).enumerate();
            for (i, (corner, &uv)) in iter {
                let i = i as u32;
                let pos2d = pos.to_real() + corner;

                vertices.push(Vertex {
                    position: [pos2d.x, pos2d.y, height.to_real()],
                    normal: normal,
                    radius: 1.0,
                    tex_coords: uv,
                    material_color: sec.ground.get_color(),
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
}

/// If the given pillar is on the outer edge of the chunk, this function adds
/// faces to cover those sides.
fn add_outer_shell(
    pos: AxialPoint,
    pillar: &HexPillar,
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
        // Check if this neighbor is outside of the current chunk. If yes, we
        // need to add sides, otherwise we skip it.
        let neighbor_pos = pos + neighbor.axial_vec();
        let is_outer =
            neighbor_pos.q >= CHUNK_SIZE.into()
            || neighbor_pos.r >= CHUNK_SIZE.into()
            || neighbor_pos.q < 0
            || neighbor_pos.r < 0;

        if is_outer {
            for sec in pillar.sections() {
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

/// Given two pillars, this adds faces in between where necessary.
///
/// This sounds like a rather easy task, but I failed to come up with a very
/// easy solution. I think this is by far the most complicated algorithm of
/// all functions in this module. Detailed description inside the function.
fn connect_pillars(
    a_pos: AxialPoint,
    a_to_b: EdgeDir,
    chunk: &Chunk,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
) {
    // We'll call the two pillars 'a' and 'b'.
    let b_pos = a_pos + a_to_b.axial_vec();

    // If b isn't even inside this chunk, we will skip it. `add_outer_shell`
    // will repair those holes.
    let skip = b_pos.q >= CHUNK_SIZE.into()
        || b_pos.r >= CHUNK_SIZE.into()
        || b_pos.q < 0
        || b_pos.r < 0;
    if skip {
        return;
    }

    // Define shorter names and create an array which we can index easily later
    // on.
    let a = &chunk[a_pos];
    let b = &chunk[b_pos];
    let ab = [a, b];

    /// This is a small helper type to differentiate between the two pillars.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    enum Pillar {
        A,
        B,
    }

    impl Pillar {
        /// Returns the index that can be used to index the `ab` array above.
        fn idx(&self) -> usize {
            match *self {
                Pillar::A => 0,
                Pillar::B => 1,
            }
        }
    }

    // ----------------------------------------------------------------------
    // Two solve the problem at hand and add all necessary faces with the
    // correct position, orientation and material, we use three layers of
    // abstraction. Each one is an iterator over different items and the latter
    // two map over the previous one.

    // ----------------------------------------------------------------------
    /// This iterator takes two slices of `PillarSection`s and spits out a
    /// series of tuples representing a single "interesting" point:
    ///
    /// - `height`: height of the current point
    /// - `pillar`: what pillar (a or b) does this point belong to?
    /// - `a_idx`: the index of the section of a that is active at `height`
    /// - `b_idx`: the index of the section of b that is active at `height`
    /// - `is_top`: whether or not the current point is the top or bottom
    ///             of the section

    /// The yielded items are ordered ascendingly by the key `height`.
    ///
    /// *Note*: this iterator assumes that the slices of pillar sections are
    /// sorted, that there are no overlapping sections and that `bottom < top`
    /// is true for all sections.
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
                // save the section index that was active
                let sec_a_idx = self.a_idx;

                if self.within_a {
                    // we will now yield a's top
                    self.a_idx += 1;
                }
                self.within_a = !self.within_a;

                next_a.map(|h| (h, Pillar::A, sec_a_idx, self.b_idx, !self.within_a))
            } else {
                // save the section index that was active
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
    ///
    /// It uses the fact that we only need to add sides between points `2*n`
    /// and `2*n + 1` but never the other way around, if all points are sorted
    /// by height.
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
    // the final data required to add a side.
    let raw_intervals = PairUp {
        original: IntervalPoints::new(a.sections(), b.sections())
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

/// This function adds a side with from the height `bottom` to `top` with the
/// material `ground`. The side is added between the pillars at `offset`
/// (pillar 'a') and `offset + dir` (pillar 'b'). `normal_to_a` determines
/// what direction the side is facing: when it's `false`, it's facing 'b' (in
/// the `dir` direction), when it's `true`, it's facing 'a' (`-dir`). The
/// direction the side is facing determines the normal as well as the winding
/// order.
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
    let normal = EDGE_NORMALS[dir.idx()] * if normal_to_a { -1.0 } else  { 1.0 };
    let corner_cw = [
        (offset.to_real() + ca * HEX_OUTER_RADIUS, 0.25),
        (offset.to_real() + cb * HEX_OUTER_RADIUS, 0.75),
    ];

    // In order to have correctly stretched textures, we have to change one
    // texture coordinate depending on the height.
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
