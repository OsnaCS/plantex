use base::world::{self, Chunk, ChunkIndex};
use base::prop::Plant;
use base::math::*;
use glium::backend::Facade;
use glium::texture::Texture2d;
use glium;
use Camera;
use std::collections::HashMap;
use std::rc::Rc;
use view::PlantRenderer;
use world::ChunkRenderer;
use GameContext;

pub use world::chunk_view::ChunkView;
pub use view::PlantView;

/// Graphical representation of the `base::World`.
pub struct WorldView {
    chunks: HashMap<ChunkIndex, ChunkView>,
    chunk_renderer: Rc<ChunkRenderer>,
    plant_renderer: Rc<PlantRenderer>,
    plant_views: HashMap<ChunkIndex, Vec<PlantView>>,
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
                                                 self.plant_renderer.clone(),
                                                 facade));

        for (pillar_pos, pillar) in chunk.pillars() {
            for prop in pillar.props() {
                let plant = &self.plant_list[prop.plant_index];
                let real_pos = pillar_pos.to_real();
                let real_chunk_pos = (chunk_pos.0 * world::CHUNK_SIZE as i32).to_real();

                self.plant_views
                    .entry(chunk_pos)
                    .or_insert(Vec::new())
                    .push(PlantView::from_plant(Point3f::new(real_chunk_pos.x + real_pos.x,
                                                             real_chunk_pos.y + real_pos.y,
                                                             prop.baseline.units() as f32 *
                                                             world::PILLAR_STEP_HEIGHT),
                                                &plant,
                                                self.plant_renderer.clone(),
                                                facade));
            }
        }
    }

    pub fn remove_chunk(&mut self, chunk_pos: ChunkIndex) {
        self.chunks.remove(&chunk_pos);
        self.plant_views.remove(&chunk_pos);
    }

    pub fn draw_shadow<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        for chunkview in self.chunks.values() {
            chunkview.draw_shadow(surface, camera);
        }
    }

    pub fn draw<S: glium::Surface>(&self,
                                   surface: &mut S,
                                   camera: &Camera,
                                   shadow_map: &Texture2d,
                                   depth_view_proj: &Matrix4<f32>,
                                   sun_dir: Vector3f) {
        for (idx, chunkview) in &self.chunks {
            if idx.0 != AxialPoint::new(0, 0) {
                continue;
            }
            chunkview.draw(surface, camera, shadow_map, depth_view_proj, sun_dir);
        }

        // for plantview_vec in self.plant_views.values() {
        //     for plantview in plantview_vec {
        //         plantview.draw(surface, camera);
        //     }
        // }
    }
}
