use base::world::{self, Chunk, ChunkIndex};
use base::prop::Plant;
use base::math::*;
use glium::backend::Facade;
use glium::{self, DepthTest, DrawParameters, LinearBlendingFactor};
use glium::draw_parameters::BlendingFunction;
use glium::texture::Texture2d;
use Camera;
use SimpleCull;
use std::collections::HashMap;
use std::rc::Rc;
use view::PlantRenderer;
use world::ChunkRenderer;
use world::HexagonOutline;
use GameContext;
use util::ToArr;
use DayTime;

pub use world::chunk_view::ChunkView;
pub use view::PlantView;

/// Graphical representation of the `base::World`.
pub struct WorldView {
    chunks: HashMap<ChunkIndex, ChunkView>,
    chunk_renderer: Rc<ChunkRenderer>,
    plant_renderer: Rc<PlantRenderer>,

    pub outline: HexagonOutline,

    plant_views: HashMap<usize, PlantView>,

    plant_list: Vec<Plant>,
}

impl WorldView {
    pub fn new(context: Rc<GameContext>, plant_list: Vec<Plant>) -> Self {
        let plant_renderer = Rc::new(PlantRenderer::new(context.clone()));
        let chunk_renderer = Rc::new(ChunkRenderer::new(context.clone()));

        WorldView {
            chunks: HashMap::new(),
            chunk_renderer: chunk_renderer,
            plant_renderer: plant_renderer,
            outline: HexagonOutline::new(context),
            plant_views: HashMap::new(),
            plant_list: plant_list,
        }
    }

    pub fn refresh_chunk<F: Facade>(&mut self, chunk_pos: ChunkIndex, chunk: &Chunk, facade: &F) {
        self.chunks.insert(chunk_pos,
                           ChunkView::from_chunk(chunk,
                                                 AxialPoint::new(chunk_pos.0.q *
                                                                 (1 * world::CHUNK_SIZE as i32),
                                                                 chunk_pos.0.r *
                                                                 (1 * world::CHUNK_SIZE as i32)),
                                                 self.chunk_renderer.clone(),
                                                 facade));

        for (pillar_pos, pillar) in chunk.pillars() {
            for prop in pillar.props() {
                let plant_index = prop.plant_index;
                let plant = &self.plant_list[plant_index];
                let real_pos = pillar_pos.to_real();

                let real_chunk_pos = (chunk_pos.0 * world::CHUNK_SIZE as i32).to_real();

                self.plant_views
                    .entry(plant_index)
                    .or_insert(PlantView::from_plant(plant, self.plant_renderer.clone(), facade))
                    .add_instance_from_pos(chunk_pos,
                                           Point3f::new(real_chunk_pos.x + real_pos.x,
                                                        real_chunk_pos.y + real_pos.y,
                                                        prop.baseline.units() as f32 *
                                                        world::PILLAR_STEP_HEIGHT));
            }
        }


    }

    pub fn get_chunk_view(&self, index: &ChunkIndex) -> Option<&ChunkView> {
        self.chunks.get(&index)
    }

    pub fn get_chunk_view_mut(&mut self, index: &ChunkIndex) -> Option<&mut ChunkView> {
        self.chunks.get_mut(&index)
    }

    pub fn remove_chunk(&mut self, chunk_pos: ChunkIndex) {
        self.chunks.remove(&chunk_pos);
        for plant_view in self.plant_views.values_mut() {
            plant_view.remove_instance_at_pos(chunk_pos);
        }
    }

    pub fn draw_shadow<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        for chunkview in self.chunks.values() {
            chunkview.draw_shadow(surface, camera);
        }

        for plantview in self.plant_views.values() {
            plantview.draw_shadow(surface, camera);
        }

    }

    pub fn draw<S: glium::Surface>(&self,
                                   surface: &mut S,
                                   camera: &Camera,
                                   shadow_map: &Texture2d,
                                   depth_view_proj: &Matrix4<f32>,
                                   daytime: &DayTime,
                                   sun_dir: Vector3f,
                                   frustum: &SimpleCull) {
        for chunkview in self.chunks.values() {
            chunkview.draw(surface,
                           camera,
                           shadow_map,
                           depth_view_proj,
                           daytime,
                           sun_dir,
                           frustum);
        }
        if self.outline.display {
            // Draw outline
            let outline_params = DrawParameters {
                depth: glium::Depth {
                    write: false,
                    test: DepthTest::Overwrite,
                    ..Default::default()
                },
                blend: glium::Blend {
                    color: BlendingFunction::Addition {
                        source: LinearBlendingFactor::SourceAlpha,
                        destination: LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    ..Default::default()
                },
                ..Default::default()
            };

            let outline_uniforms = uniform! {
              outline_pos: self.outline.position().to_arr(),
              proj_matrix: camera.proj_matrix().to_arr(),
              view_matrix: camera.view_matrix().to_arr()
            };

            surface.draw(self.outline.vertices(),
                      self.outline.indices(),
                      self.outline.program(),
                      &outline_uniforms,
                      &outline_params)
                .unwrap();
        }


        for plantview in self.plant_views.values() {
            plantview.draw(surface,
                           camera,
                           shadow_map,
                           depth_view_proj,
                           daytime,
                           sun_dir);
        }
    }
}
