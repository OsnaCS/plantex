#![allow(unused_imports)]


use base::world::{Chunk, HexPillar, PillarSection, PropType, CHUNK_SIZE, HEX_OUTER_RADIUS, HeightType};
use base::math::*;
use glium::index::PrimitiveType;
use glium::{self, DrawParameters, VertexBuffer, IndexBuffer, Program,};
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::backend::Facade;
use glium::texture::Texture2d;
use glium::uniforms::SamplerWrapFunction;
use glium::uniforms::MinifySamplerFilter;
use Camera;
use util::ToArr;
use view::{PlantRenderer, PlantView};
use world::ChunkRenderer;
use std::rc::Rc;
use base::world::ground::GroundMaterial;
use std::cmp::{min, max};

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    renderer: Rc<ChunkRenderer>,
    /// Instance data buffer.
    // pillar_buf: VertexBuffer<Instance>,
    vertex_buf: VertexBuffer<Vertex>,
    index_buf: IndexBuffer<u32>,
    offset: AxialPoint,
}

impl ChunkView {
    /// Creates the graphical representation of given chunk at the given chunk
    /// offset
    pub fn from_chunk<F: Facade>(chunk: &Chunk,
                                 offset: AxialPoint,
                                 chunk_renderer: Rc<ChunkRenderer>,
                                 plant_renderer: Rc<PlantRenderer>,
                                 facade: &F)
                                 -> Self {

        // let mut sections = Vec::new();

        // for (axial, pillar) in chunk.pillars() {
        //     let pos = offset.to_real() + axial.to_real();
        //     for section in pillar.sections() {
        //         let g = match section.ground {
        //             GroundMaterial::Grass => 1,
        //             GroundMaterial::Sand => 2,
        //             GroundMaterial::Snow => 3,
        //             GroundMaterial::Dirt => 4,
        //             GroundMaterial::Stone => 5,
        //             GroundMaterial::JungleGrass => 1,
        //             GroundMaterial::Mulch => 7,
        //             GroundMaterial::Debug => 8,
        //         };
        //         sections.push(Instance {
        //             material_color: section.ground.get_color(),
        //             ground: g,
        //             offset: [pos.x, pos.y, section.bottom.to_real()],
        //             height: (section.top.units() - section.bottom.units()) as f32,
        //         });
        //     }
        // }

        let (raw_buf, raw_indices) = if offset != AxialPoint::new(0, 0) {
            (Vec::new(), Vec::new())
        } else {
            Self::get_vertices(chunk)
        };

        // let raw_indices = [0, 1, 2];

        // println!("{:?} -> {:?}", offset, offset.to_real());

        ChunkView {
            renderer: chunk_renderer,
            // pillar_buf: VertexBuffer::dynamic(facade, &sections).unwrap(),
            vertex_buf: VertexBuffer::new(facade, &raw_buf).unwrap(),
            index_buf: IndexBuffer::new(facade,
                                      PrimitiveType::TrianglesList,
                                      &raw_indices).unwrap(),
            offset: offset,
            // index_buf: ::glium::index::NoIndices(PrimitiveType::Points),
        }
    }

    pub fn get_vertices(chunk: &Chunk) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        Chunk::for_pillars_positions(|pos| {
            let tmp_col = [pos.q as f32 / (CHUNK_SIZE  as f32), pos.r as f32 / (CHUNK_SIZE  as f32), 0.0];

            for sec in chunk[pos].sections() {
                let ground = sec.ground.get_id();

                // Add top and bottom face
                for &(height, rev) in &[(sec.top, false), (sec.bottom, true)] {
                    let prev_len = vertices.len() as u32;
                    // we skip all bottom faces at 0 completely
                    if height.units() == 0 {
                        continue;
                    }

                    // Add center point
                    vertices.push(Vertex {
                        position: [pos.to_real().x, pos.to_real().y, height.to_real()],
                        normal: [0.0, 0.0, 1.0],
                        radius: 0.0,
                        tex_coords: [0.5, 0.5],
                        // material_color: sec.ground.get_color(),
                        material_color: tmp_col,
                        ground: ground,
                    });

                    // Add all points of top face
                    let iter = NORM_CORNERS.iter().map(|c| c * HEX_OUTER_RADIUS).zip(CORNER_UV).enumerate();
                    for (i, (corner, &uv)) in iter {
                        let i = i as u32;
                        let pos2D = pos.to_real() + corner;

                        vertices.push(Vertex {
                            position: [pos2D.x, pos2D.y, height.to_real()],
                            normal: [0.0, 0.0, 1.0],
                            radius: 1.0,
                            tex_coords: uv,
                            // material_color: sec.ground.get_color(),
                            material_color: tmp_col,
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


                // // Add faces to neighbor *chunks*. We don't know how to render

                // // Add faces to all neighbors in the directions:
                // // - positive q, r stays the same
                // // - positive r, q stays the same
                // // - positive q and negative r (s stays the same)
                // for (&neighbor_pos, &side) in neighbors.iter().zip(&norm_corners_to_neighbor) {
                //     // Check if the neighbor even exists in the current chunk,
                //     // if not, proceed with the next.
                //     let skip = neighbor_pos.q >= CHUNK_SIZE.into()
                //         || neighbor_pos.r >= CHUNK_SIZE.into()
                //         || neighbor_pos.r < 0;
                //     if skip {
                //         continue;
                //     }


                //     // Find the neighbor sections we need to interact with. Our
                //     // section `sec` has `top` and `bottom`. We can classify other
                //     // sections (`other`) into three different classes:
                //     // _________________________________________________
                //     //                      +
                //     //                      |
                //     //                      +
                //     //                                   +
                //     //  sec.top -------+                 |
                //     //                 |          +      +
                //     //                 |          |
                //     //                 |          |
                //     //                 |          +      +
                //     //  sec.bottom ----+                 |
                //     //                      +            +
                //     //                      |
                //     //                      +
                //     // ________________________________________________
                //     //                      ^     ^      ^
                //     // Completely outside --+     |      |
                //     // (a)                        |      |
                //     //    Completely inside (b) --+      |
                //     //                                   |
                //     //                           Partially inside (c)
                //     //
                //     // - (a): we won't interact with those (ignore)
                //     // - (b) and (c): luckily we don't really need to distinguish
                //     //   between those two cases. More about this in a comment
                //     //   further down
                //     let fitting_neighbor_sections = chunk[neighbor_pos]
                //         .sections()
                //         .iter()
                //         .filter(|other| {
                //             sec.top > other.bottom && sec.bottom < other.top
                //         });
                //     let mut at_least_one_fitted = false;

                //     for other in fitting_neighbor_sections {
                //         at_least_one_fitted = true;

                //         // Regardless of being case (a) or (b) we need to draw
                //         // two faces: from top to top and bottom to bottom.
                //         // The normal, `material_color` and `ground` depend on
                //         // what pillar-end is higher.

                //         // TODO: normal is dummy value!

                //         // Draw top-top-face
                //         if sec.top != other.top {
                //             add_side(
                //                 false,
                //                 (sec.top, sec.ground),
                //                 (other.top, other.ground),
                //                 [pos.to_real() + side.0, pos.to_real() + side.1],
                //                 &mut vertices,
                //                 &mut indices
                //             );
                //         }

                //         // Draw bottom-bottom-face
                //         if sec.bottom != other.bottom {
                //             add_side(
                //                 true,
                //                 (sec.bottom, sec.ground),
                //                 (other.bottom, other.ground),
                //                 [pos.to_real() + side.0, pos.to_real() + side.1],
                //                 &mut vertices,
                //                 &mut indices
                //             );
                //         }
                //         // if sec.bottom != other.bottom {
                //         //     let (normal, color, ground) = if sec.bottom < other.bottom {
                //         //         // Material of `sec` is used, normals point to `other`
                //         //         ([1.0, 0.0, 0.0], sec.ground.get_color(), sec.ground)
                //         //     } else {
                //         //         // Material of `other` is used, normals point to `sec`
                //         //         ([1.0, 0.0, 0.0], other.ground.get_color(), other.ground)
                //         //     };
                //         //     let lower = min(sec.bottom, sec.bottom);
                //         //     let higher = max(sec.bottom, sec.bottom);

                //         //     let prev_len = vertices.len() as u32;

                //         //     // lower two vertices
                //         //     for &z in &[lower, higher] {
                //         //         for xy in [side.0, side.1].iter().map(|s| pos.to_real() + s) {
                //         //             vertices.push(Vertex {
                //         //                 position: [xy.x, xy.y, z.to_real()],
                //         //                 normal: normal,
                //         //                 radius: 0.0,
                //         //                 tex_coords: [0.0, 0.0],
                //         //                 material_color: color,
                //         //                 ground: ground.get_id(),
                //         //             });
                //         //         }
                //         //     }

                //         //     indices.extend_from_slice(&[
                //         //         prev_len, prev_len + 2, prev_len + 1,
                //         //         prev_len + 3, prev_len + 2, prev_len + 1,
                //         //     ]);
                //         // }
                //     }

                //     if !at_least_one_fitted {
                //         // add_side(
                //         //     false,
                //         //     (sec.top, sec.ground),
                //         //     (sec.bottom, sec.ground),
                //         //     [pos.to_real() + side.0, pos.to_real() + side.1],
                //         //     &mut vertices,
                //         //     &mut indices
                //         // );
                //     }
                // }
            }

            for &dir in SIDE_PROPAGATION_NEIGHBORS {
                connect_pillars(pos, dir, chunk, &mut vertices, &mut indices);
            }
        });

        (vertices, indices)
    }

    pub fn draw_shadow<S: glium::Surface>(&self, _surface: &mut S, _camera: &Camera) {
        // let uniforms = uniform! {
        //     proj_matrix: camera.proj_matrix().to_arr(),
        //     view_matrix: camera.view_matrix().to_arr(),
        // };
        // let params = DrawParameters {
        //     depth: glium::Depth {
        //         write: true,
        //         test: DepthTest::IfLess,
        //         ..Default::default()
        //     },
        //     backface_culling: BackfaceCullingMode::CullClockwise,
        //     multisampling: true,
        //     ..Default::default()
        // };

        // surface.draw((self.renderer.pillar_vertices(), self.pillar_buf.per_instance().unwrap()),
        //           self.renderer.pillar_indices(),
        //           self.renderer.shadow_program(),
        //           &uniforms,
        //           &params)
        //     .unwrap();

        // for pillar in &self.pillars {
        //     for plant in &pillar.plants {
        //         plant.draw_shadow(surface, camera);
        //     }
        // }
    }

    pub fn draw<S: glium::Surface>(&self,
                                   surface: &mut S,
                                   camera: &Camera,
                                   shadow_map: &Texture2d,
                                   depth_view_proj: &Matrix4<f32>,
                                   sun_dir: Vector3f) {
        let uniforms = uniform! {
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
            shadow_map: shadow_map.sampled().wrap_function(SamplerWrapFunction::Clamp),
            depth_view_proj: depth_view_proj.to_arr(),
            sun_dir: sun_dir.to_arr(),
            offset: self.offset.to_real().to_arr(),
            offset_ax: [self.offset.q, self.offset.r],

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

        // surface.draw((self.renderer.pillar_vertices(), self.pillar_buf.per_instance().unwrap()),
        //           self.renderer.pillar_indices(),
        //           self.renderer.program(),
        //           &uniforms,
        //           &params)
        //     .unwrap();
        surface.draw(&self.vertex_buf,
                    &self.index_buf,
                    // &::glium::index::NoIndices(PrimitiveType::Points),
                  // self.renderer.pillar_indices(),
                  self.renderer.program(),
                  &uniforms,
                  &params)
            .unwrap();
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

/// Instance data for each pillar section.
#[derive(Debug, Copy, Clone)]
pub struct Instance {
    ground: i32,
    /// Material color.
    material_color: [f32; 3],
    /// Offset in world coordinates.
    offset: [f32; 3],
    /// Pillar height.
    height: f32,
}

implement_vertex!(Instance, material_color, offset, ground, height);

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
//  y     r
//
//
//
//     (q: 0, r: -1)    top      (q:1, r: -1)
//
//      top-left       _~^~_       top-right
//                  _~^     ^~_
//                ~^           ^~
//                |             |
// (q: -1, r: 0)  |             |       (q: 1, r: 0)
//                |             |
//                ^~_         _~^
//    bottom-left    ^~_   _~^   bottom-right
//   (q: -1, r: 1)      ^~^     (q: 0, r: 1)
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
    (NORM_CORNERS[0], NORM_CORNERS[1]), // q:  0, r: -1, top-left
    (NORM_CORNERS[1], NORM_CORNERS[2]), // q:  1, r: -1, top-right
    (NORM_CORNERS[2], NORM_CORNERS[3]), // q:  1, r:  0, right
    (NORM_CORNERS[3], NORM_CORNERS[4]), // q:  0, r:  1, bottom-right
    (NORM_CORNERS[4], NORM_CORNERS[5]), // q: -1, r:  1, bottom-left
    (NORM_CORNERS[5], NORM_CORNERS[0]), // q: -1, r:  0, left
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

impl CornerDir {
    /// Index for array lookup
    fn idx(&self) -> usize {
        match *self {
            CornerDir::TopLeft => 0,
            CornerDir::Top => 1,
            CornerDir::TopRight => 2,
            CornerDir::BottomRight => 3,
            CornerDir::Bottom => 4,
            CornerDir::BottomLeft => 5,
        }
    }
}

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
            EdgeDir::TopLeft => AxialVector::new(0, -1),
            EdgeDir::TopRight => AxialVector::new(1, -1),
            EdgeDir::Right => AxialVector::new(1,  0),
            EdgeDir::BottomRight => AxialVector::new(0,  1),
            EdgeDir::BottomLeft => AxialVector::new(1,  1),
            EdgeDir::Left => AxialVector::new(1,  0),
        }
    }
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

    // println!("--------------------");
    // println!("a_to_b: {:?}", a_to_b);
    // println!("self: {:?}", a_pos);
    // println!("neighbor: {:?}", neighbor_pos);
    // println!("-------");

    let a = &chunk[a_pos];
    let b = &chunk[neighbor_pos];

    // println!("a: {:#?}", a.sections());
    // println!("b: {:#?}", b.sections());

    // let mut a_sections = a.sections().iter().peekable();
    // let mut b_sections = b.sections().iter().peekable();

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

    let raw_intervals = PairUp {
        original: IntervalPoints::new(a.sections(), b.sections()) //.inspect(|x| println!("-- {:?}", x))
    };
    let sides = raw_intervals.map(|(lower, upper)| {
        let (l_height, l_pillar, l_sec_a_idx, l_sec_b_idx, l_is_top) = lower;
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
        add_side_new(
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


    // macro_rules! pair_iter {
    //     ($a:expr, $b:expr) => (::std::iter::once($a).chain(::std::iter::once($b)))
    // }

    // use std::iter::Peekable;
    // struct Scanline<I: Iterator> {
    //     a: Peekable<I>,
    //     b: Peekable<I>,
    // }

    // impl<I> Iterator for Scanline<I> where I: Iterator<Item=(HeightType, GroundMaterial)> {
    //     type Item = (HeightType, GroundMaterial);
    //     fn next(&mut self) -> Option<Self::Item> {
    //         let choose_a = match (self.a.peek(), self.b.peek()) {
    //             (Some(a), Some(b)) => {
    //                 if a.0 < b.0 {
    //                     Some(true)
    //                 } else {
    //                     Some(false)
    //                 }
    //             }
    //             (None, Some(_)) => Some(false),
    //             (Some(_), None) => Some(true),
    //             _ => None,
    //         };

    //         match choose_a {
    //             Some(true) => self.a.next(),
    //             Some(false) => self.b.next(),
    //             None => None,
    //         }
    //     }
    // }

    // let flat_a: Vec<_> = a.sections()
    //     .iter()
    //     .flat_map(|s| pair_iter!((s.bottom, s.ground), (s.top, s.ground)))
    //     .collect();
    // let flat_b: Vec<_> = b.sections()
    //     .iter()
    //     .flat_map(|s| pair_iter!((s.bottom, s.ground), (s.top, s.ground)))
    //     .collect();

    // let interesting_points = Scanline {
    //     a: flat_a.into_iter().peekable(),
    //     b: flat_b.into_iter().peekable(),
    // };

    // let mut scanline = Vec::with_capacity(
    //     (a.sections().len() + b.sections().len()) * 2
    // );
    // for a in a.sections() {
    //     scanline.push()
    // }


    // let mut start_height = HeightType::from_units(0);
    // let mut start_ground = GroundMaterial::Debug;
    // let mut need_side = false;
    // for (p_height, p_ground) in interesting_points {
    //     println!("--> {:?}", p_height);
    //     need_side = !need_side;
    //     if need_side {
    //         start_height = p_height;
    //         start_ground = p_ground;
    //     } else {
    //         if start_height == p_height {
    //             continue;
    //         }
    //         println!("add side from {} to {}", start_height.units(), p_height.units());
    //         add_side_new(
    //             start_height,
    //             p_height,
    //             start_ground,
    //             false,
    //             a_to_b,
    //             a_pos,
    //             vertices,
    //             indices,
    //         );
    //     }
    // }

}


fn add_side_new(
    bottom: HeightType,
    top: HeightType,
    ground: GroundMaterial,
    normal_to_a: bool,
    dir: EdgeDir,
    offset: AxialPoint,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
) {
    // TODO: normal is dummy
    let prev_len = vertices.len() as u32;
    let corners = EDGE_CORNERS_TO_NEIGHBOR[dir.idx()];
    // println!("offset {:?}", offset);
    // println!("corners: {:?}", corners);
    for &z in &[bottom, top] {
        for xy in [corners.0, corners.1].iter().map(|c| offset.to_real() + c) {
            // println!("vertex at {:?}", (xy.x, xy.y, z.to_real()));
            let zz = (z.to_real() - bottom.to_real()) / (top.to_real() - bottom.to_real());

            vertices.push(Vertex {
                position: [xy.x, xy.y, z.to_real()],
                normal: [0.0, 0.0, 0.0],
                radius: 0.0,
                tex_coords: [0.0, 0.0],
                // material_color: color,
                // material_color: [xy.x - pos.to_real().x, xy.y - pos.to_real().y, zz],
                material_color: [0.0, 0.0, zz],
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

// fn add_side(
//     normal_to_upper: bool,
//     (a_height, a_ground): (HeightType, GroundMaterial),
//     (b_height, b_ground): (HeightType, GroundMaterial),
//     corner_pos_cw: [Point2f; 2],
//     vertices: &mut Vec<Vertex>,
//     indices: &mut Vec<u32>,
// ) {
//     // TODO: normal is dummy
//     let (normal, ground) = if (a_height > b_height) ^ normal_to_upper {
//         ([1.0, 0.0, 0.0], a_ground)
//     } else {
//         ([1.0, 0.0, 0.0], b_ground)
//     };
//     let lower = min(a_height, b_height);
//     let higher = max(a_height, b_height);

//     let prev_len = vertices.len() as u32;

//     for &z in &[lower, higher] {
//         for xy in &corner_pos_cw {
//             let zz = (z.to_real() - lower.to_real()) / (higher.to_real() - lower.to_real());

//             vertices.push(Vertex {
//                 position: [xy.x, xy.y, z.to_real()],
//                 normal: normal,
//                 radius: 0.0,
//                 tex_coords: [0.0, 0.0],
//                 // material_color: color,
//                 // material_color: [xy.x - pos.to_real().x, xy.y - pos.to_real().y, zz],
//                 material_color: [0.0, 0.0, zz],
//                 ground: ground.get_id(),
//             });
//         }
//     }

//     if (a_height > b_height) ^ normal_to_upper {
//         indices.extend_from_slice(&[
//             prev_len, prev_len + 2, prev_len + 1,
//             prev_len + 3, prev_len + 1, prev_len + 2,
//         ]);
//     } else {
//         indices.extend_from_slice(&[
//             prev_len, prev_len + 1, prev_len + 2,
//             prev_len + 3, prev_len + 2, prev_len + 1,
//         ]);
//     };
// }
