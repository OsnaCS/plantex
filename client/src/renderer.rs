use world::WorldView;
use glium::Surface;
use super::{Camera, GameContext};
use std::rc::Rc;

use glium::texture::texture2d::Texture2d;
use glium::texture::UncompressedFloatFormat;
use glium::texture::MipmapsOption;
use glium::texture::{DepthFormat, DepthTexture2d};
use glium::framebuffer::{MultiOutputFrameBuffer, SimpleFrameBuffer};
use glium::VertexBuffer;
use glium::{IndexBuffer, Program};
use base::math::*;
use glium::index::PrimitiveType;

pub struct Renderer {
    context: Rc<GameContext>,
}

impl Renderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        Renderer { context: context }
    }


    /// Is called once every main loop iteration
    pub fn render(&self, world_view: &WorldView, camera: &Camera) -> Result<(), ()> {
        let screen_width = 1280;
        let screen_heigth = 720;

        let quad_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                    UncompressedFloatFormat::F32F32F32F32,
                                                    MipmapsOption::NoMipmap,
                                                    screen_width,
                                                    screen_heigth)
            .unwrap();

        let depth_texture = DepthTexture2d::empty_with_format(self.context.get_facade(),
                                                              DepthFormat::F32,
                                                              MipmapsOption::NoMipmap,
                                                              screen_width,
                                                              screen_heigth)
            .unwrap();

        let output = &[("color", &quad_tex)];

        let mut hdr_buffer = MultiOutputFrameBuffer::with_depth_buffer(self.context.get_facade(),
                                                                       output.iter().cloned(),
                                                                       &depth_texture)
            .unwrap();

        hdr_buffer.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        world_view.draw(&mut hdr_buffer, camera);


        let mut target = self.context.get_facade().draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);


        let ortho_mat: Matrix4<f32> = ortho(0.0,
                                            screen_width as f32,
                                            0.0,
                                            screen_heigth as f32,
                                            -1.0,
                                            1.0);

        let light_texture = Texture2d::empty_with_format(self.context.get_facade(),
                                                         UncompressedFloatFormat::F32F32F32F32,
                                                         MipmapsOption::NoMipmap,
                                                         screen_width,
                                                         screen_heigth)
            .unwrap();


        let mut light_buffer = SimpleFrameBuffer::with_depth_buffer(self.context.get_facade(),
                                                                    &light_texture,
                                                                    &depth_texture)
            .unwrap();
        world_view.draw(&mut light_buffer, camera);


        let uniforms = uniform! {
            matrix: Into::<[[f32;4];4]>::into(ortho_mat),
            decal_texture: &quad_tex,
            light_texture: &light_texture
        };

        let quad_vertex_buffer = {
            #[derive(Copy, Clone)]
            struct Vertex {
                position: [f32; 4],
                texcoord: [f32; 2],
            }

            implement_vertex!(Vertex, position, texcoord);

            VertexBuffer::new(self.context.get_facade(),
                              &[Vertex {
                                    position: [0.0, 0.0, 0.0, 1.0],
                                    texcoord: [0.0, 0.0],
                                },
                                Vertex {
                                    position: [1280.0, 0.0, 0.0, 1.0],
                                    texcoord: [1.0, 0.0],
                                },
                                Vertex {
                                    position: [1280.0, 720.0, 0.0, 1.0],
                                    texcoord: [1.0, 1.0],
                                },
                                Vertex {
                                    position: [0.0, 720.0, 0.0, 1.0],
                                    texcoord: [0.0, 1.0],
                                }])
                .unwrap()
        };


        let composit_program = Program::from_source(self.context.get_facade(),
                                                    include_str!("renderer_hdr_vertexshader.vert"),
                                                    include_str!("renderer_hdr_fragmentshader.\
                                                                  frag"),
                                                    None)
            .unwrap();

        let quad_index_buffer = IndexBuffer::new(self.context.get_facade(),
                                                 PrimitiveType::TrianglesList,
                                                 &[0u16, 1, 2, 0, 2, 3])
            .unwrap();


        target.draw(&quad_vertex_buffer,
                  &quad_index_buffer,
                  &composit_program,
                  &uniforms,
                  &Default::default())
            .unwrap();


        target.finish().unwrap();



        //



        // let mut target = self.context.get_facade().draw();
        // target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        //
        // world_view.draw(&mut target, camera);
        //
        // target.finish().unwrap();
        //
        Ok(())
    }
}
