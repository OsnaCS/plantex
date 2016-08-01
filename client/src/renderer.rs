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
use glium::framebuffer::{MultiOutputFrameBuffer, SimpleFrameBuffer};
use glium::{IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use glium::backend::Facade;
use glium::framebuffer::ToColorAttachment;

pub struct Renderer {
    context: Rc<GameContext>,
    quad_tex: Texture2d,
    depth_texture: DepthTexture2d,
    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_index_buffer: IndexBuffer<u16>,
    resolution: (u32, u32),
    bloom_filter_tex: Texture2d,
    bloom_horz_tex: Texture2d,
    bloom_vert_tex: Texture2d,
    bloom_blend_tex: Texture2d,
    tonemapping_program: Program,
    bloom_filter_program: Program,
    bloom_blur_program: Program,
    bloom_blend_program: Program,
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


        let bloom_filter_tex = Texture2d::empty_with_format(context.get_facade(),
                                                            UncompressedFloatFormat::F32F32F32F32,
                                                            MipmapsOption::NoMipmap,
                                                            resolution.0,
                                                            resolution.1)
            .unwrap();



        let bloom_horz_tex = Texture2d::empty_with_format(context.get_facade(),
                                                          UncompressedFloatFormat::F32F32F32F32,
                                                          MipmapsOption::NoMipmap,
                                                          resolution.0,
                                                          resolution.1)
            .unwrap();

        let bloom_vert_tex = Texture2d::empty_with_format(context.get_facade(),
                                                          UncompressedFloatFormat::F32F32F32F32,
                                                          MipmapsOption::NoMipmap,
                                                          resolution.0,
                                                          resolution.1)
            .unwrap();



        let bloom_blend_tex = Texture2d::empty_with_format(context.get_facade(),
                                                           UncompressedFloatFormat::F32F32F32F32,
                                                           MipmapsOption::NoMipmap,
                                                           resolution.0,
                                                           resolution.1)
            .unwrap();


        let tonemapping_program = context.load_program("tonemapping").unwrap();
        let bloom_filter_program = context.load_program("bloom_filter").unwrap();
        let bloom_blur_program = context.load_program("bloom_blur").unwrap();
        let bloom_blend_program = context.load_program("bloom_blending").unwrap();


        Renderer {
            context: context.clone(),
            quad_tex: quad_tex_temp,
            depth_texture: depth_texture,
            quad_vertex_buffer: Renderer::create_vertex_buf(context.get_facade()),
            quad_index_buffer: ibuf,
            resolution: resolution,
            bloom_filter_tex: bloom_filter_tex,
            bloom_horz_tex: bloom_horz_tex,
            bloom_vert_tex: bloom_vert_tex,
            bloom_blend_tex: bloom_blend_tex,
            tonemapping_program: tonemapping_program,
            bloom_filter_program: bloom_filter_program,
            bloom_blur_program: bloom_blur_program,
            bloom_blend_program: bloom_blend_program,
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
        let mut hdr_buffer =
            try!(MultiOutputFrameBuffer::with_depth_buffer(self.context.get_facade(),
                                                           output.iter().cloned(),
                                                           &self.depth_texture));

        hdr_buffer.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);


        world_view.draw(&mut hdr_buffer, camera);

        sky_view.draw_skydome(&mut hdr_buffer, camera);
        sun.draw_sun(&mut hdr_buffer, camera);
        weather.draw(&mut hdr_buffer, camera);

        // ===================================================================
        // Creating the Bloom framebuffer
        // ===================================================================


        // =======================  light texture  ===========================

        // Bloom state
        // 0: Disable Bloom
        // 1: Enable Bloom
        // 2: Show only Bloom Map
        let bloom_state = 1;

        let uniforms = uniform! {
            decal_texture: &self.quad_tex,
            exposure: 1.0f32
        };


        let mut bloom_buffer = try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                           self.bloom_filter_tex
                                                               .to_color_attachment()));

        bloom_buffer.clear_color(0.0, 0.0, 0.0, 1.0);


        try!(bloom_buffer.draw(&self.quad_vertex_buffer,
                               &self.quad_index_buffer,
                               &self.bloom_filter_program,
                               &uniforms,
                               &Default::default()));



        // ============================  blur  ===============================

        let mut bloom_blur_horz_buffer = try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                                     self.bloom_horz_tex
                                                                         .to_color_attachment()));

        bloom_blur_horz_buffer.clear_color(0.0, 0.0, 0.0, 1.0);


        let mut bloom_blur_vert_buffer = try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                                     self.bloom_vert_tex
                                                                         .to_color_attachment()));
        bloom_blur_vert_buffer.clear_color(0.0, 0.0, 0.0, 1.0);

        let mut uniforms_horz_blur = uniform! {
            //for the first iteration: Use the bloom quad texture, from second iteration on this
            // will change to bloom_vert_tex
            image: &self.bloom_filter_tex,
            horizontal: true,
        };
        let uniforms_vert_blur = uniform! {
            image: &self.bloom_horz_tex,
            horizontal: false,
        };


        let mut first_iteration = true; //to know when we need to switch uniforms_horz_blur
        let mut horizontal = true;      //to switch between horizontal and vertical blur

        for _ in 0..10 {
            if horizontal {
                try!(bloom_blur_horz_buffer.draw(&self.quad_vertex_buffer,
                                                 &self.quad_index_buffer,
                                                 &self.bloom_blur_program,
                                                 &uniforms_horz_blur,
                                                 &Default::default()));
            } else {
                try!(bloom_blur_vert_buffer.draw(&self.quad_vertex_buffer,
                                                 &self.quad_index_buffer,
                                                 &self.bloom_blur_program,
                                                 &uniforms_vert_blur,
                                                 &Default::default()));
            }
            if first_iteration {
                uniforms_horz_blur = uniform! {
                    image: &self.bloom_vert_tex,
                    horizontal: true,
                };
                first_iteration = false;
            }
            horizontal = !horizontal;
        }

        // ==========================  blending  =============================

        let mut bloom_blend_buffer = try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                                 self.bloom_blend_tex
                                                                     .to_color_attachment()));

        bloom_blend_buffer.clear_color(0.0, 0.0, 0.0, 1.0);


        let u_blend_tex =
            if horizontal { &self.bloom_vert_tex } else { &self.bloom_horz_tex };

        let uniforms = uniform! {
            bloom_tex: u_blend_tex,
            world_tex: &self.quad_tex,
        };


        try!(bloom_blend_buffer.draw(&self.quad_vertex_buffer,
                                     &self.quad_index_buffer,
                                     &self.bloom_blend_program,
                                     &uniforms,
                                     &Default::default()));


        // ===================================================================
        // Tonemapping
        // ===================================================================


        let decal_texture = match bloom_state {
            0 => &self.quad_tex,
            2 => u_blend_tex,
            _ => &self.bloom_blend_tex,
        };

        let uniforms = uniform! {
            decal_texture: decal_texture,
            exposure: 1.0f32,
        };


        let mut target = self.context.get_facade().draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        try!(target.draw(&self.quad_vertex_buffer,
                         &self.quad_index_buffer,
                         &self.tonemapping_program,
                         &uniforms,
                         &Default::default()));

        try!(target.finish());

        Ok(())
    }

    fn create_vertex_buf<F: Facade>(facade: &F) -> VertexBuffer<Vertex> {

        VertexBuffer::new(facade,
                          &[Vertex {
                                in_position: [-1.0, -1.0, 0.0, 1.0],
                                in_texcoord: [0.0, 0.0],
                            },
                            Vertex {
                                in_position: [1.0, -1.0, 0.0, 1.0],
                                in_texcoord: [1.0, 0.0],
                            },
                            Vertex {
                                in_position: [1.0, 1.0, 0.0, 1.0],
                                in_texcoord: [1.0, 1.0],
                            },
                            Vertex {
                                in_position: [-1.0, 1.0, 0.0, 1.0],
                                in_texcoord: [0.0, 1.0],
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


        self.bloom_filter_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                             UncompressedFloatFormat::F32F32F32F32,
                                                             MipmapsOption::NoMipmap,
                                                             self.resolution.0,
                                                             self.resolution.1)
            .unwrap();


        self.bloom_horz_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                           UncompressedFloatFormat::F32F32F32F32,
                                                           MipmapsOption::NoMipmap,
                                                           self.resolution.0,
                                                           self.resolution.1)
            .unwrap();

        self.bloom_vert_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                           UncompressedFloatFormat::F32F32F32F32,
                                                           MipmapsOption::NoMipmap,
                                                           self.resolution.0,
                                                           self.resolution.1)
            .unwrap();

        self.bloom_blend_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                            UncompressedFloatFormat::F32F32F32F32,
                                                            MipmapsOption::NoMipmap,
                                                            self.resolution.0,
                                                            self.resolution.1)
            .unwrap();
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    in_position: [f32; 4],
    in_texcoord: [f32; 2],
}

implement_vertex!(Vertex, in_position, in_texcoord);
