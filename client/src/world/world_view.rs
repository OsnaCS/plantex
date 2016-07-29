use base::world::{self, Chunk, ChunkIndex, World};
use base::math::*;
use glium::backend::Facade;
use glium::texture::DepthTexture2d;
use glium;
use Camera;
use std::collections::HashMap;
use std::rc::Rc;
use view::PlantRenderer;
use world::ChunkRenderer;
use world::HexagonOutline;
use GameContext;
use util::ToArr;

pub use world::chunk_view::ChunkView;
pub use view::PlantView;

/// Graphical representation of the `base::World`.
pub struct WorldView {
    chunks: HashMap<ChunkIndex, ChunkView>,
    chunk_renderer: Rc<ChunkRenderer>,
    plant_renderer: Rc<PlantRenderer>,
    pub outline: HexagonOutline,
}

impl WorldView {
    pub fn from_world(world: &World, context: Rc<GameContext>) -> Self {
        let plant_renderer = Rc::new(PlantRenderer::new(context.clone()));
        let chunk_renderer = Rc::new(ChunkRenderer::new(context.clone()));

        let mut chunks = HashMap::new();
        for chunkkey in world.chunks.keys() {
            // FIX: 1 is HEX_OUTER_RADIUS, but thats a float
            chunks.insert(*chunkkey,
                          ChunkView::from_chunk(world.chunks.get(chunkkey).unwrap(),
                                                AxialPoint::new(chunkkey.0.q *
                                                                (1 * world::CHUNK_SIZE as i32),
                                                                chunkkey.0.r *
                                                                (1 * world::CHUNK_SIZE as i32)),
                                                chunk_renderer.clone(),
                                                plant_renderer.clone(),
                                                context.get_facade()));
        }

        WorldView {
            chunks: chunks,
            chunk_renderer: chunk_renderer,
            plant_renderer: plant_renderer,
            outline: HexagonOutline::new(context),
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
    }

    pub fn remove_chunk(&mut self, chunk_pos: ChunkIndex) {
        self.chunks.remove(&chunk_pos);
    }

    pub fn draw_shadow<S: glium::Surface>(&self, surface: &mut S, camera: &Camera) {
        for chunkview in self.chunks.values() {
            chunkview.draw_shadow(surface, camera);
        }
    }

    pub fn draw<S: glium::Surface>(&self,
                                   surface: &mut S,
                                   camera: &Camera,
                                   shadow_map: &DepthTexture2d,
                                   depth_view_proj: &Matrix4<f32>,
                                   sun_dir: Vector3f) {
        for chunkview in self.chunks.values() {
            chunkview.draw(surface, camera, shadow_map, depth_view_proj, sun_dir);
        }
        if self.outline.display {
            // Draw outline
            let outline_params = DrawParameters {
                depth: glium::Depth {
                    write: true,
                    test: DepthTest::IfLess,
                    ..Default::default()
                },
                ..Default::default()
            };

            // println!("DRAW: {:?}", self.outline.position().to_arr());
            let outline_uniforms = uniform! {
              outline_pos: self.outline.position().to_arr(),
              proj_matrix: camera.proj_matrix().to_arr(),
              view_matrix: camera.view_matrix().to_arr(),
              transformation: [
                  [1.5, 0.0, 0.0, 0.0],
                  [0.0, 1.5, 0.0, 0.0],
                  [0.0, 0.0, 1.5, 0.0],
                  [0.0, 0.0, 0.0, 1.0f32]
              ],
            };

            surface.draw(self.outline.vertices(),
                      self.outline.indices(),
                      self.outline.program(),
                      &outline_uniforms,
                      &outline_params)
                .unwrap();
        }
    }
}
