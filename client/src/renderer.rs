use base::math::*;
use world::WorldView;
use glium::Surface;
use super::{Camera, GameContext};
use view::Sun;
use view::SkyView;
use std::rc::Rc;
use std::error::Error;
use std::env;
use super::weather::Weather;
use glium::texture::texture2d::Texture2d;
use glium::texture::{DepthFormat, DepthTexture2d, MipmapsOption, UncompressedFloatFormat};
use glium::framebuffer::SimpleFrameBuffer;
use glium::{IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use glium::backend::Facade;
use glium::framebuffer::ToColorAttachment;
use glium::backend::glutin_backend::GlutinFacade;
use glium;

const SHADOW_MAP_SIZE: u32 = 2048;
const SHADOW_ORTHO_WIDTH: f32 = 200.0;
const SHADOW_ORTHO_HEIGHT: f32 = 200.0;
const SHADOW_ORTHO_NEAR: f32 = 100.0;
const SHADOW_ORTHO_FAR: f32 = 600.0;

pub struct Renderer {
    context: Rc<GameContext>,
    /// Screen-sized texture the scene is rendered into and then post-processed.
    quad_tex: Texture2d,
    /// Depth texture used by the normal render.
    depth_texture: DepthTexture2d,
    /// Depth texture rendered to from sun perspective.
    shadow_map: DepthTexture2d,
    /// Render the shadow map to the screen instead of the world.
    shadow_debug: bool,
    shadow_debug_program: Program,
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
    adaption_shrink_program: Program,
    lum_texs: Vec<Texture2d>,
}

impl Renderer {
    pub fn new(context: Rc<GameContext>) -> Self {

        // FIXME The index buffer is useless, switch to using `NoIndices` and
        // `TriangleStrip`
        let ibuf = IndexBuffer::new(context.get_facade(),
                                    PrimitiveType::TrianglesList,
                                    &[0u16, 1, 2, 0, 2, 3])
            .unwrap();


        let shadow_map = DepthTexture2d::empty_with_format(context.get_facade(),
                                                           DepthFormat::I16,
                                                           MipmapsOption::NoMipmap,
                                                           SHADOW_MAP_SIZE,
                                                           SHADOW_MAP_SIZE)
            .unwrap();

        let lum_texs = initialize_luminosity(context.get_facade());

        let tonemapping_program = context.load_program("tonemapping").unwrap();
        let bloom_filter_program = context.load_program("bloom_filter").unwrap();
        let bloom_blur_program = context.load_program("bloom_blur").unwrap();
        let bloom_blend_program = context.load_program("bloom_blending").unwrap();
        let shadow_debug_program = context.load_program("shadow_debug").unwrap();
        let adaption_shrink_program = context.load_program("adaption_shrink").unwrap();

        let mut this = Renderer {
            context: context.clone(),
            quad_tex: Texture2d::empty(context.get_facade(), 1, 1).unwrap(),
            tonemapping_program: tonemapping_program,
            depth_texture: DepthTexture2d::empty(context.get_facade(), 1, 1).unwrap(),
            shadow_map: shadow_map,
            shadow_debug: env::var("SHADOW_DEBUG").is_ok(),
            shadow_debug_program: shadow_debug_program,
            quad_vertex_buffer: Renderer::create_vertex_buf(context.get_facade()),
            quad_index_buffer: ibuf,
            resolution: (0, 0),
            bloom_filter_tex: Texture2d::empty(context.get_facade(), 1, 1).unwrap(),
            bloom_horz_tex: Texture2d::empty(context.get_facade(), 1, 1).unwrap(),
            bloom_vert_tex: Texture2d::empty(context.get_facade(), 1, 1).unwrap(),
            bloom_blend_tex: Texture2d::empty(context.get_facade(), 1, 1).unwrap(),
            bloom_filter_program: bloom_filter_program,
            bloom_blur_program: bloom_blur_program,
            bloom_blend_program: bloom_blend_program,
            adaption_shrink_program: adaption_shrink_program,
            lum_texs: lum_texs,
        };

        // Create all textures with correct screen size
        this.render_update();
        this
    }

    /// Renders `world_view` into the renderer's shadow map from the
    /// perspective of the sun, whose position is `sun_pos`.
    ///
    /// Returns the MVP matrix used.
    fn render_shadow_map(&mut self,
                         world_view: &WorldView,
                         sun_pos: Point3f,
                         camera: Point3f)
                         -> Result<Matrix4<f32>, Box<Error>> {
        let mut shadow_target = try!(SimpleFrameBuffer::depth_only(self.context
                                                                       .get_facade(),
                                                                   &self.shadow_map));
        shadow_target.clear_depth(1.0);

        // Render the world from the perspective of the sun.
        let mut cam_pos = camera.to_vec();
        cam_pos.z = 70.0;
        let mut sun_cam = Camera::new_from_vector(sun_pos + cam_pos,
                                                  -sun_pos.to_vec(),
                                                  SHADOW_ORTHO_WIDTH / SHADOW_ORTHO_HEIGHT);
        // Set an orthographic projection matrix.
        sun_cam.set_proj_matrix(ortho(-SHADOW_ORTHO_WIDTH / 2.0,
                                      SHADOW_ORTHO_WIDTH / 2.0,
                                      -SHADOW_ORTHO_HEIGHT / 2.0,
                                      SHADOW_ORTHO_HEIGHT / 2.0,
                                      SHADOW_ORTHO_NEAR,
                                      SHADOW_ORTHO_FAR));
        world_view.draw_shadow(&mut shadow_target, &sun_cam);

        Ok(sun_cam.proj_matrix() * sun_cam.view_matrix())
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
        // Creating shadow map
        // ===================================================================
        let depth_mvp = try!(self.render_shadow_map(world_view, sun.position(), camera.position));

        // ===================================================================
        // Rendering into HDR framebuffer
        // ===================================================================
        let mut hdr_buffer = try!(SimpleFrameBuffer::with_depth_buffer(self.context.get_facade(),
                                                                       &self.quad_tex,
                                                                       &self.depth_texture));
        hdr_buffer.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        if self.shadow_debug {
            let mut target = self.context.get_facade().draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            try!(target.draw(&self.quad_vertex_buffer,
                             &self.quad_index_buffer,
                             &self.shadow_debug_program,
                             &uniform!{
                          decal_texture: &self.shadow_map,
                      },
                             &Default::default()));
            try!(target.finish());
            return Ok(());
        }

        let sun_dir = (-sun.position().to_vec()).normalize();
        world_view.draw(&mut hdr_buffer,
                        camera,
                        &self.shadow_map,
                        &depth_mvp,
                        sun_dir);
        sky_view.draw_skydome(&mut hdr_buffer, camera);
        sun.draw_sun(&mut hdr_buffer, camera);
        weather.draw(&mut hdr_buffer, camera);


        // ===================================================================
        //                  Brightness Adaption Data Structures
        // ===================================================================

        let avg_luminance = try!(self.adapt_brightness());

        let exposure = (2.0 * avg_luminance) as f32;


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
            exposure: exposure,
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
            exposure: exposure,
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


    // ===================================================================
    //                         Brightness Adaption
    // ===================================================================

    fn adapt_brightness(&self) -> Result<f32, Box<Error>> {
        let mut adaption_buffers: Vec<SimpleFrameBuffer> = Vec::with_capacity(10);

        let mut image = &self.quad_tex;

        for i in 0..10 {
            adaption_buffers.push(try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                              self.lum_texs[i]
                                                                  .to_color_attachment())));


            if i != 0 {
                image = &self.lum_texs[i - 1];
            }

            let uniforms = uniform!{
                image: image,
            };

            try!(adaption_buffers[i].draw(&self.quad_vertex_buffer,
                                          &self.quad_index_buffer,
                                          &self.adaption_shrink_program,
                                          &uniforms,
                                          &Default::default()));

        }


        // Read only pixel in the lowest level texture from lum_texs.
        // never change a working system. Yeah, it's that complicated.
        let buf: Vec<Vec<(f32, f32, f32, f32)>> = self.lum_texs
            .last()
            .unwrap()
            .main_level()
            .first_layer()
            .into_image(None)
            .unwrap()
            .raw_read(&glium::Rect {
                left: 0,
                bottom: 0,
                width: 1,
                height: 1,
            });

        let pixel = buf[0][0];

        Ok(Vector3f::new(pixel.0, pixel.1, pixel.2).dot(Vector3f::new(0.2126, 0.7152, 0.0722)))
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    in_position: [f32; 4],
    in_texcoord: [f32; 2],
}

implement_vertex!(Vertex, in_position, in_texcoord);





// ===================================================================
//                  Brightness Adaption Data Structures
// ===================================================================


fn initialize_luminosity(facade: &GlutinFacade) -> Vec<Texture2d> {
    let mut lum: Vec<Texture2d> = Vec::with_capacity(10);
    for i in 0..10 {
        lum.push(Texture2d::empty_with_format(facade,
                                              UncompressedFloatFormat::F32F32F32F32,
                                              MipmapsOption::NoMipmap,
                                              (2 as u32).pow((9 - i) as u32),
                                              (2 as u32).pow((9 - i) as u32))
            .unwrap());
    }
    lum
}
