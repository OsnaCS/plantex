use world::WorldView;
use glium::Surface;
use super::{Camera, GameContext};
use view::Sun;
use view::SkyView;
use std::rc::Rc;
use std::error::Error;
use super::weather::Weather;
use glium::texture::texture2d::Texture2d;
use glium::texture::{DepthFormat, DepthTexture2d, MipmapsOption, UncompressedFloatFormat};
use glium::framebuffer::MultiOutputFrameBuffer;
use glium::{IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use glium::backend::Facade;

pub struct Renderer {
    context: Rc<GameContext>,
    quad_tex: Texture2d,
    depth_texture: DepthTexture2d,
    tonemapping_program: Program,
    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_index_buffer: IndexBuffer<u16>,
    resolution: (u32, u32),
}

impl Renderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        let resolution = context.get_facade().get_framebuffer_dimensions();
        let quad_tex_temp = Texture2d::empty_with_format(context.get_facade(),
                                                         UncompressedFloatFormat::F32F32F32F32,
                                                         MipmapsOption::NoMipmap,
                                                         resolution.0,
                                                         resolution.1)
            .unwrap();

        let depth_texture = DepthTexture2d::empty_with_format(context.get_facade(),
                                                              DepthFormat::F32,
                                                              MipmapsOption::NoMipmap,
                                                              resolution.0,
                                                              resolution.1)
            .unwrap();

        let ibuf = IndexBuffer::new(context.get_facade(),
                                    PrimitiveType::TrianglesList,
                                    &[0u16, 1, 2, 0, 2, 3])
            .unwrap();

        Renderer {
            context: context.clone(),
            quad_tex: quad_tex_temp,
            tonemapping_program: context.load_program("tonemapping").unwrap(),
            depth_texture: depth_texture,
            quad_vertex_buffer: Renderer::create_vertex_buf(context.get_facade()),
            quad_index_buffer: ibuf,
            resolution: resolution,
        }
    }


    /// Is called once every main loop iteration
    pub fn render(&mut self,
                  world_view: &WorldView,
                  camera: &Camera,
                  sun: &Sun,
                  weather: &mut Weather,
                  sky_view: &SkyView)
                  -> Result<(), Box<Error>> {
        // ===================================================================
        // check dimensions
        // ===================================================================
        let new_res = self.context.get_facade().get_framebuffer_dimensions();
        if self.resolution != new_res {
            self.render_update();
        }

        // ===================================================================
        // Rendering into HDR framebuffer
        // ===================================================================
        let output = &[("color", &self.quad_tex)];
        let mut hdr_buffer = MultiOutputFrameBuffer::with_depth_buffer(self.context.get_facade(),
                                                                       output.iter().cloned(),
                                                                       &self.depth_texture)
            .unwrap();
        hdr_buffer.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        world_view.draw(&mut hdr_buffer, camera);

        sky_view.draw_skydome(&mut hdr_buffer, camera);
        sun.draw_sun(&mut hdr_buffer, camera);
        weather.draw(&mut hdr_buffer, camera);

        // ===================================================================
        // Tonemapping
        // ===================================================================
        let mut target = self.context.get_facade().draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let uniforms = uniform! {
            decal_texture: &self.quad_tex,
            exposure: 1.0f32
        };

        target.draw(&self.quad_vertex_buffer,
                  &self.quad_index_buffer,
                  &self.tonemapping_program,
                  &uniforms,
                  &Default::default())
            .unwrap();

        try!(target.finish());

        Ok(())
    }

    fn create_vertex_buf<F: Facade>(facade: &F) -> VertexBuffer<Vertex> {

        VertexBuffer::new(facade,
                          &[Vertex {
                                position: [-1.0, -1.0, 0.0, 1.0],
                                texcoord: [0.0, 0.0],
                            },
                            Vertex {
                                position: [1.0, -1.0, 0.0, 1.0],
                                texcoord: [1.0, 0.0],
                            },
                            Vertex {
                                position: [1.0, 1.0, 0.0, 1.0],
                                texcoord: [1.0, 1.0],
                            },
                            Vertex {
                                position: [-1.0, 1.0, 0.0, 1.0],
                                texcoord: [0.0, 1.0],
                            }])
            .unwrap()
    }

    // render update is called when a dimensions resize of the facade occures
    // recreates the 2D textures
    fn render_update(&mut self) {
        self.resolution = self.context.get_facade().get_framebuffer_dimensions();
        info!("time to resize framebuffer with dimensions {},{}",
              self.resolution.0,
              self.resolution.1);

        self.quad_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                     UncompressedFloatFormat::F32F32F32F32,
                                                     MipmapsOption::NoMipmap,
                                                     self.resolution.0,
                                                     self.resolution.1)
            .unwrap();

        self.depth_texture = DepthTexture2d::empty_with_format(self.context.get_facade(),
                                                               DepthFormat::F32,
                                                               MipmapsOption::NoMipmap,
                                                               self.resolution.0,
                                                               self.resolution.1)
            .unwrap();
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 4],
    texcoord: [f32; 2],
}

implement_vertex!(Vertex, position, texcoord);
