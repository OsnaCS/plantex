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
use glium::{Blend, BlendingFunction, IndexBuffer, LinearBlendingFactor, Program, VertexBuffer};
use glium::index::PrimitiveType;
use glium::backend::Facade;
use glium::framebuffer::ToColorAttachment;
use glium::backend::glutin_backend::GlutinFacade;
use glium;
use glium::uniforms::{MagnifySamplerFilter, SamplerWrapFunction};
use glium::draw_parameters::DrawParameters;


// ===================================================================
//                       CONFIGURATION VALUES
// ===================================================================

// ========================  THE SHADOW  =============================

const SHADOW_MAP_SIZE: u32 = 2048;
const SHADOW_ORTHO_WIDTH: f32 = 100.0;
const SHADOW_ORTHO_HEIGHT: f32 = 100.0;
const SHADOW_ORTHO_NEAR: f32 = 100.0;
const SHADOW_ORTHO_FAR: f32 = 600.0;


// ===========================  BLOOM  ===============================
// Bloom state
// 0: Disable Bloom
// 1: Enable Bloom
// 2: Show only Bloom Map
const BLOOM_STATE: i8 = 1;
// number of times the light texture will be blured.
// each iteration contains one horizontal and one vertical blur
const BLOOM_ITERATION: u8 = 6;
// Divisor to downsize blur texture
// Increase to decrease bloom texture size DEFAULT: 2
const BLUR_TEXTURE_DIVISOR: u32 = 2;
const EXPOSURE_THRESHOLD: f32 = 0.5;

// ===================  AUTOMATIC BRIGHTNESS ADAPTION  ===============

// The following values define how well you can adapt to brightness / darkness.
// The adaption of the eye is clamped between these values.
const EYE_OPEN: f32 = 5.2;  //increase to allow to see better in the dark       DEFAULT:3.2
const EYE_CLOSED: f32 = 0.0008;  //decrease to allow to see brighter areas better  DEFAULT:0.8

// The following values define how much the exposure value will be drawn to
// a given ("optimal") value.
const OPTIMAL_EXPOSURE: f32 = 0.5;  // optimal Value that exposure should reach.    DEFAULT: 0.5
const WE_WANT_OPTIMAL: f32 = 0.7;  // Agressiveness of exposure correction in [0;1] DEFAULT: 0.7

// Speed of eye adaption. Lower values result in longer time needed
// to adapt to different light conditions. Set to 1 to test without adaption
// effect.
const ADAPTION_SPEED_BRIGHT_DARK: f32 = 0.25;  //adaption speed from bright to dark DEFAULT:0.25
const ADAPTION_SPEED_DARK_BRIGHT: f32 = 0.06; //adaption speed from dark to bright  DEFAULT:0.06






// TODO: WE_WANT_OPTIMAL kram anpassen, Nächte dunkler, Brightness adaption
// aggressiver





pub struct Renderer {
    context: Rc<GameContext>,
    /// Screen-sized texture the scene is rendered into and then post-processed.
    quad_tex: Texture2d,
    /// Depth texture used by the normal render.
    depth_texture: DepthTexture2d,
    /// Depth texture rendered to from sun perspective.
    shadow_map: Texture2d,
    shadow_horz_blur: Texture2d,
    shadow_motion_blur: Texture2d,
    /// Unnecessary dummy depth texture used when rendering the shadow map.
    shadow_depth: DepthTexture2d,
    /// Render the shadow map to the screen instead of the world.
    shadow_debug: bool,
    shadow_debug_program: Program,
    shadow_blend_program: Program,
    // Vertexbuffer of screenquad
    quad_vertex_buffer: VertexBuffer<Vertex>,
    // Indexbuffer of screenquad
    quad_index_buffer: IndexBuffer<u16>,
    // Screen resolution
    resolution: (u32, u32),
    // filter to create bloom light texture
    bloom_filter_tex: Texture2d,
    // texture for the horizontal bloom blur
    bloom_horz_tex: Texture2d,
    // texture for the vertical bloom blur
    bloom_vert_tex: Texture2d,
    // texture for blending the bloom light texture with the quad texture
    bloom_blend_tex: Texture2d,
    // shader programs for tonemapping
    tonemapping_program: Program,
    // shader programs for bloom light texture
    bloom_filter_program: Program,
    // shader programs for bloom blur texture
    bloom_blur_program: Program,
    // shader programs for blending the bloom light texture with the quad texture
    bloom_blend_program: Program,
    // shader programs for shrinking the texture
    adaption_shrink_program: Program,
    // shader programs to transform texture into greyscale for calculating avg. exposure
    relative_luminance_program: Program,
    // Vector of Textures used to downscale the szene to 1 pixel size for calc. avg. exposure
    lum_texs: Vec<Texture2d>,
    // last average luminance
    last_lum: f32,
    // current exposure level
    exposure: f32,
    lum_relative_tex: Texture2d,
}

impl Renderer {
    pub fn new(context: Rc<GameContext>) -> Self {

        // FIXME The index buffer is useless, switch to using `NoIndices` and
        // `TriangleStrip`
        let ibuf = IndexBuffer::new(context.get_facade(),
                                    PrimitiveType::TrianglesList,
                                    &[0u16, 1, 2, 0, 2, 3])
            .unwrap();


        let shadow_map = Texture2d::empty_with_format(context.get_facade(),
                                                      UncompressedFloatFormat::F32F32,
                                                      MipmapsOption::NoMipmap,
                                                      SHADOW_MAP_SIZE,
                                                      SHADOW_MAP_SIZE)
            .unwrap();

        let shadow_horz_blur = Texture2d::empty_with_format(context.get_facade(),
                                                            UncompressedFloatFormat::F32F32,
                                                            MipmapsOption::NoMipmap,
                                                            SHADOW_MAP_SIZE,
                                                            SHADOW_MAP_SIZE)
            .unwrap();

        let shadow_motion_blur = Texture2d::empty_with_format(context.get_facade(),
                                                              UncompressedFloatFormat::F32F32,
                                                              MipmapsOption::NoMipmap,
                                                              SHADOW_MAP_SIZE,
                                                              SHADOW_MAP_SIZE)
            .unwrap();
        shadow_motion_blur.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

        let shadow_depth =
            DepthTexture2d::empty(context.get_facade(), SHADOW_MAP_SIZE, SHADOW_MAP_SIZE)
            .unwrap();

        let lum_texs = initialize_luminosity(context.get_facade());

        let last_lum = 2.0;
        let exposure = 2.0;


        let tonemapping_program = context.load_program("tonemapping").unwrap();
        let bloom_filter_program = context.load_program("bloom_filter").unwrap();
        let bloom_blur_program = context.load_program("bloom_blur").unwrap();
        let bloom_blend_program = context.load_program("bloom_blending").unwrap();
        let shadow_debug_program = context.load_program("shadow_debug").unwrap();
        let shadow_blend_program = context.load_program("blend").unwrap();
        let adaption_shrink_program = context.load_program("adaption_shrink").unwrap();
        let relative_luminance_program = context.load_program("relative_luminance").unwrap();

        let mut this = Renderer {
            context: context.clone(),
            quad_tex: Texture2d::empty(context.get_facade(), 1, 1).unwrap(),
            tonemapping_program: tonemapping_program,
            depth_texture: DepthTexture2d::empty(context.get_facade(), 1, 1).unwrap(),
            shadow_map: shadow_map,
            shadow_horz_blur: shadow_horz_blur,
            shadow_motion_blur: shadow_motion_blur,
            shadow_depth: shadow_depth,
            shadow_debug: env::var("SHADOW_DEBUG").is_ok(),
            shadow_debug_program: shadow_debug_program,
            shadow_blend_program: shadow_blend_program,
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
            last_lum: last_lum,
            exposure: exposure,
            lum_relative_tex: Texture2d::empty(context.get_facade(), 1, 1).unwrap(),
            relative_luminance_program: relative_luminance_program,
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
        let mut shadow_target = try!(SimpleFrameBuffer::with_depth_buffer(self.context
                                                                              .get_facade(),
                                                                          &self.shadow_map,
                                                                          &self.shadow_depth));
        shadow_target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

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

        // Blur the shadow map to get soft shadows

        // Blur in horizontal direction into a new frame buffer:
        let mut blur_horz_buffer = try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                               self.shadow_horz_blur
                                                                   .to_color_attachment()));

        for _ in 0..2 {
            blur_horz_buffer.clear_color(0.0, 0.0, 0.0, 1.0);


            try!(blur_horz_buffer.draw(&self.quad_vertex_buffer,
                                       &self.quad_index_buffer,
                                       &self.bloom_blur_program,
                                       &uniform! {
                                            image: self.shadow_map.sampled()
                                              .wrap_function(SamplerWrapFunction::Clamp),
                                            horizontal: true,
                                        },
                                       &Default::default()));

            // ...then blur in vertical direction into the normal shadow map
            try!(shadow_target.draw(&self.quad_vertex_buffer,
                                    &self.quad_index_buffer,
                                    &self.bloom_blur_program,
                                    &uniform! {
                                        image: self.shadow_horz_blur.sampled()
                                          .wrap_function(SamplerWrapFunction::Clamp),
                                        horizontal: false,
                                    },
                                    &Default::default()));
        }


        // Blend the motion blur texture over the whole scene
        let uniforms = uniform! {
            image: &self.shadow_motion_blur,
        };
        let params = DrawParameters {
            blend: Blend {
                color: BlendingFunction::Addition {
                    source: LinearBlendingFactor::ConstantAlpha,
                    destination: LinearBlendingFactor::OneMinusConstantAlpha,
                },
                alpha: BlendingFunction::AlwaysReplace,
                constant_value: (0.0, 0.0, 0.0, 0.05),
            },
            ..Default::default()
        };

        try!(shadow_target.draw(&self.quad_vertex_buffer,
                                &self.quad_index_buffer,
                                &self.shadow_blend_program,
                                &uniforms,
                                &params));


        // Copy final shadow map to motion blur texture
        shadow_target.fill(&self.shadow_motion_blur.as_surface(),
                           MagnifySamplerFilter::Nearest);

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
        let depth_mvp = try!(self.render_shadow_map(world_view, sun.position(), 
            camera.position));

        // ===================================================================
        // Rendering into HDR framebuffer
        // ===================================================================
        {
            let mut hdr_buffer = try!(SimpleFrameBuffer::with_depth_buffer(self.context
                                                                               .get_facade(),
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

        }
        // ===================================================================
        //                      Brightness Adaption Calls
        // ===================================================================


        let adapt = try!(self.adapt_brightness());
        if adapt >= self.last_lum {
            self.last_lum = (1.0 - ADAPTION_SPEED_DARK_BRIGHT) * self.last_lum +
                            ADAPTION_SPEED_DARK_BRIGHT * adapt;
        } else {
            self.last_lum = (1.0 - ADAPTION_SPEED_BRIGHT_DARK) * self.last_lum +
                            ADAPTION_SPEED_BRIGHT_DARK * adapt
        }
        info!("last_lum {}", self.last_lum);

        self.exposure = (1.0 - WE_WANT_OPTIMAL) * self.last_lum +
                        WE_WANT_OPTIMAL * OPTIMAL_EXPOSURE;
        info!("exp: {}", self.exposure);



        // ===================================================================
        //                                  Bloom
        // ===================================================================

        if BLOOM_STATE != 0 {
            try!(self.bloom());
        }

        // ===================================================================
        //                                 Tonemapping
        // ===================================================================


        let decal_texture = match BLOOM_STATE {
            0 => &self.quad_tex,
            2 => &self.bloom_vert_tex,
            _ => &self.bloom_blend_tex,
        };

        let uniforms = uniform! {
            decal_texture: decal_texture,
            exposure: self.exposure,
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

        let ffff = UncompressedFloatFormat::F32F32F32F32;
        self.quad_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                     ffff,
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
                                                             ffff,
                                                             MipmapsOption::NoMipmap,
                                                             self.resolution.0,
                                                             self.resolution.1)
            .unwrap();


        self.bloom_horz_tex =
            Texture2d::empty_with_format(self.context.get_facade(),
                                         ffff,
                                         MipmapsOption::NoMipmap,
                                         (self.resolution.0) / BLUR_TEXTURE_DIVISOR,
                                         (self.resolution.1) / BLUR_TEXTURE_DIVISOR)
                .unwrap();

        self.bloom_vert_tex =
            Texture2d::empty_with_format(self.context.get_facade(),
                                         ffff,
                                         MipmapsOption::NoMipmap,
                                         (self.resolution.0) / BLUR_TEXTURE_DIVISOR,
                                         (self.resolution.1) / BLUR_TEXTURE_DIVISOR)
                .unwrap();

        self.bloom_blend_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                            ffff,
                                                            MipmapsOption::NoMipmap,
                                                            self.resolution.0,
                                                            self.resolution.1)
            .unwrap();

        self.lum_relative_tex = Texture2d::empty_with_format(self.context.get_facade(),
                                                             UncompressedFloatFormat::F32,
                                                             MipmapsOption::NoMipmap,
                                                             self.resolution.0,
                                                             self.resolution.1)
            .unwrap();
    }


    // ===================================================================
    //                         Brightness Adaption
    // ===================================================================

    fn adapt_brightness(&self) -> Result<f32, Box<Error>> {

        let mut rel_luminance_buffer = try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                                   self.lum_relative_tex
                                                                       .to_color_attachment()));

        let uniforms = uniform!{
            image: &self.quad_tex,
        };



        try!(rel_luminance_buffer.draw(&self.quad_vertex_buffer,
                                       &self.quad_index_buffer,
                                       &self.relative_luminance_program,
                                       &uniforms,
                                       &Default::default()));



        let mut adaption_buffers: Vec<SimpleFrameBuffer> = Vec::with_capacity(10);

        let mut image = &self.lum_relative_tex;

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
        // never change a working system.
        let buf: Vec<Vec<f32>> = self.lum_texs
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

        // let avg_luminance = Vector3f::new(pixel.0, pixel.1, pixel.2)
        //    .dot(Vector3f::new(0.2126, 0.7152, 0.0722));
        let avg_luminance = buf[0][0];


        // info!("lum: {:?}", buf);

        // the exposure level is inversely propotional to the avg. luminance.
        // log2 is necessary to adapt more for the lower than for the higher values.
        // (This is still WIP and will be changed in the next version.)
        // The +1 in the argument of the log is necessary because many color values
        // are <1 and would result in a negative result.
        let adapted_luminance = (1.0 / avg_luminance)
            .min(EYE_OPEN)
            .max(EYE_CLOSED);
        Ok(adapted_luminance)
    }


    fn bloom(&mut self) -> Result<(), Box<Error>> {
        // =======================  light texture  ===========================
        // create texture containing only bright areas, everything else will be black

        let uniforms = uniform! {
            decal_texture: &self.quad_tex,
            bloom_threshhold: self.exposure / EXPOSURE_THRESHOLD,
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
        // ping pong blur between the horizontal and the vertical blur buffer.

        let mut bloom_blur_horz_buffer = try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                                     self.bloom_horz_tex
                                                                        .to_color_attachment()));

        bloom_blur_horz_buffer.clear_color(0.0, 0.0, 0.0, 1.0);


        let mut bloom_blur_vert_buffer = try!(SimpleFrameBuffer::new(self.context.get_facade(),
                                                                     self.bloom_vert_tex
                                                                        .to_color_attachment()));
        bloom_blur_vert_buffer.clear_color(0.0, 0.0, 0.0, 1.0);
        let mut uniforms_horz_blur = uniform! {
            //for the first iteration: Use the bloom quad texture as source of light map,
            // from second iteration on this will change to bloom_vert_tex
            image: self.bloom_filter_tex.sampled().wrap_function(SamplerWrapFunction::Clamp),
            horizontal: true,
        };
        let uniforms_vert_blur = uniform! {
            image: self.bloom_horz_tex.sampled().wrap_function(SamplerWrapFunction::Clamp),
            horizontal: false,
        };

        let mut first_iteration = true; //to know when we need to switch uniforms_horz_blur
        let mut horizontal = true;      //to switch between horizontal and vertical blur

        for _ in 0..BLOOM_ITERATION {
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
                    image: self.bloom_vert_tex.sampled()
                    .wrap_function(SamplerWrapFunction::Clamp),
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

        // after 2x BLOOM_ITERATION we can rely on bloom_vert_tex bein the blur output
        let uniforms = uniform! {
            bloom_tex: self.bloom_vert_tex.sampled().wrap_function(SamplerWrapFunction::Clamp),
            world_tex: self.quad_tex.sampled().wrap_function(SamplerWrapFunction::Clamp),
        };

        try!(bloom_blend_buffer.draw(&self.quad_vertex_buffer,
                                     &self.quad_index_buffer,
                                     &self.bloom_blend_program,
                                     &uniforms,
                                     &Default::default()));

        Ok(())
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
    let mut lum = Vec::with_capacity(10);
    for i in 0..10 {
        lum.push(Texture2d::empty_with_format(facade,
                                              UncompressedFloatFormat::F32,
                                              MipmapsOption::NoMipmap,
                                              (2u32).pow((9 - i)),
                                              (2u32).pow((9 - i)))
            .unwrap());
    }
    lum
}
