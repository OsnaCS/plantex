use base::world::{CHUNK_SIZE, Chunk, ChunkIndex, HexPillar, PropType, World};
use base::math::*;
use glium::{self, DrawParameters, VertexBuffer};
use glium::draw_parameters::{BackfaceCullingMode, DepthTest};
use glium::backend::Facade;
use glium::texture::Texture2d;
use glium::uniforms::SamplerWrapFunction;
use glium::uniforms::MinifySamplerFilter;
use Camera;
use DayTime;
use SimpleCull;
use LOCATION;
use util::ToArr;
use view::{PlantRenderer, PlantView};
use world::ChunkRenderer;
use std::rc::Rc;
use base::world::ground::GroundMaterial;

/// Graphical representation of the `base::Chunk`.
pub struct ChunkView {
    offset: AxialPoint,
    renderer: Rc<ChunkRenderer>,
    plant_renderer: Rc<PlantRenderer>,
    pillars: Vec<PillarView>,
    /// Instance data buffer.
    pillar_buf: VertexBuffer<Instance>,
    // save corner positions for draw
    corner_ps: Vec<Point3f>,
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

        let mut sections = Vec::new();
        let mut pillars = Vec::new();
        let mut i = 0;
        let mut vec = Vec::new();
        {
            // scope so we can borrow mut vec
            let mut c = |x, pos: Point2f, z| {
                if x == 0 || x == CHUNK_SIZE || x == (CHUNK_SIZE * CHUNK_SIZE - CHUNK_SIZE) ||
                   x == (CHUNK_SIZE * CHUNK_SIZE - 1) {
                    vec.push(Point3f::new(pos.x, pos.y, z));
                }
            };
            for (axial, pillar) in chunk.pillars() {
                let pos = offset.to_real() + axial.to_real();
                let height = 150.;
                // save if corner (assume fixed location of corners)
                c(i, pos, height);
                c(i, pos, 0.);
                i += 1;
                pillars.push(PillarView::from_pillar(pillar, plant_renderer.clone(), facade));
                for section in pillar.sections() {
                    let g = match section.ground {
                        GroundMaterial::Grass => 1,
                        GroundMaterial::Sand => 2,
                        GroundMaterial::Snow => 3,
                        GroundMaterial::Dirt => 4,
                        GroundMaterial::Stone => 5,
                        GroundMaterial::JungleGrass => 1,
                        GroundMaterial::Mulch => 7,
                        GroundMaterial::Debug => 8,
                    };
                    sections.push(Instance {
                        material_color: section.ground.get_color(),
                        ground: g,
                        offset: [pos.x, pos.y, section.bottom.to_real()],
                        height: (section.top.units() - section.bottom.units()) as f32,
                    });
                }
            }
        }

        ChunkView {
            offset: offset,
            plant_renderer: plant_renderer,
            renderer: chunk_renderer,
            pillars: pillars,
            pillar_buf: VertexBuffer::dynamic(facade, &sections).unwrap(),
            corner_ps: vec,
        }
    }

    pub fn draw_shadow<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        let uniforms = uniform! {
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

        surface.draw((self.renderer.pillar_vertices(), self.pillar_buf.per_instance().unwrap()),
                  self.renderer.pillar_indices(),
                  self.renderer.shadow_program(),
                  &uniforms,
                  &params)
            .unwrap();

        for pillar in &self.pillars {
            for plant in &pillar.plants {
                plant.draw_shadow(surface, camera);
            }
        }
    }

    pub fn update<F: Facade>(&mut self, facade: &F, world: &mut World) {
        let chunk = world.chunk_at(ChunkIndex(self.offset));

        if chunk.is_none() {
            return;
        }

        let mut sections = Vec::new();
        let mut pillars = Vec::new();

        for (axial, pillar) in chunk.unwrap().pillars() {
            let pos = self.offset.to_real() + axial.to_real();
            pillars.push(PillarView::from_pillar(pillar, self.plant_renderer.clone(), facade));
            for section in pillar.sections() {
                let g = match section.ground {
                    GroundMaterial::Grass => 1,
                    GroundMaterial::Sand => 2,
                    GroundMaterial::Snow => 3,
                    GroundMaterial::Dirt => 4,
                    GroundMaterial::Stone => 5,
                    GroundMaterial::JungleGrass => 1,
                    GroundMaterial::Mulch => 7,
                    GroundMaterial::Debug => 8,
                };
                sections.push(Instance {
                    ground: g,
                    material_color: section.ground.get_color(),
                    offset: [pos.x, pos.y, section.bottom.to_real()],
                    height: (section.top.units() - section.bottom.units()) as f32,
                });
            }
        }
        self.pillar_buf = VertexBuffer::dynamic(facade, &sections).unwrap();
    }

    pub fn draw<S: glium::Surface>(&self,
                                   surface: &mut S,
                                   camera: &Camera,
                                   shadow_map: &Texture2d,
                                   depth_view_proj: &Matrix4<f32>,
                                   daytime: &DayTime,
                                   sun_dir: Vector3f,
                                   frustum: &SimpleCull) {

        // such skill much wow :D
        let mut i = 0;
        let mut c = || {
            let a = i;
            i += 1;
            &self.corner_ps[a]
        };
        let corner = [c(), c(), c(), c(), c(), c(), c(), c()];

        let render = match frustum.is_vis(corner) {
            LOCATION::Outside => 0,
            LOCATION::Inside => 1,
            LOCATION::Intersect => 1,
        };
        if render == 0 {
            return;
        }
        let uniforms = uniform! {
            proj_matrix: camera.proj_matrix().to_arr(),
            view_matrix: camera.view_matrix().to_arr(),
            shadow_map: shadow_map.sampled().wrap_function(SamplerWrapFunction::Clamp),
            depth_view_proj: depth_view_proj.to_arr(),
            sun_dir: sun_dir.to_arr(),
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

        // Draw hexagons
        surface.draw((self.renderer.pillar_vertices(), self.pillar_buf.per_instance().unwrap()),
                  self.renderer.pillar_indices(),
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
}

implement_vertex!(Vertex, position, normal, radius, tex_coords);

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

pub struct PillarView {
    plants: Vec<PlantView>,
}

impl PillarView {
    // fn from_pillar<F: Facade>( pos: Point2f,
    fn from_pillar<F: Facade>(pillar: &HexPillar,
                              plant_renderer: Rc<PlantRenderer>,
                              facade: &F)
                              -> PillarView {
        PillarView {
            plants: pillar.props()
                .iter()
                .map(|prop| {
                    match prop.prop {
                        PropType::Plant(ref plant) => {
                            PlantView::from_plant(plant, plant_renderer.clone(), facade)
                        }
                    }
                })
                .collect(),
        }
    }
}
