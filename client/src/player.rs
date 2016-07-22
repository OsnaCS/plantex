use super::camera::*;
use super::event_manager::*;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{CursorState, ElementState, Event, MouseButton, VirtualKeyCode};
use base::math::*;
use std::time::{Duration, Instant};
use std::thread::sleep;

pub struct Player {
    cam: Camera,
    // velocity: f32,
    prev_mouse_pos: Option<(i32, i32)>,
    context: GlutinFacade,
    delta_x: f32,
    vel_z: f32,
    delta_y: f32,
    delta_z: f32,
    dz: f32,
    gravity: f32,
    is_jumping: bool,
    is_moving: bool,
    tick_count: f32,
}

impl Player {
    pub fn new(context: GlutinFacade) -> Self {
        Player {
            cam: Camera {
                position: Point3::new(0.0, 0.0, 50.0),
                phi: -0.27,
                theta: 2.6,
            },
            prev_mouse_pos: None,
            context: context,
            delta_x: 0.0,
            delta_y: 0.0,
            delta_z: 0.0,
            gravity: 0.005,
            vel_z: 0.0,
            dz: 0.0,
            is_jumping: false,
            is_moving: false,
            tick_count: 1.0,
        }
    }

    pub fn get_camera(&self) -> Camera {
        self.cam
    }
    pub fn jump(&mut self) {

        if self.is_jumping == false {
            self.dz = 0.11;
            self.is_jumping = true;
        }


        // if self.delta_x == 0.0 && self.delta_y == 0.0 {
        //     // Jump vertically because sideways motion is zero
        //     let now = Instant::now();

        //     tick_count = 1.0;
        //     while 1.0 > 0.999999 {

        //         let mut dz: f32 = 1.0;

        //         self.cam.move_up(20.0);
        //         // self.cam.move_up(dz);
        //         sleep(Duration::new(1, 0));
        //         tick_count += 1.0;
        //         println!("dz: {}", dz);
        //     }
        //     // self.cam.move_down(self.cam.position[2] - 1.0);
        // }
    }

    pub fn update(&mut self, delta: f32) {
        // let mut tick_count = self.tick_count;
        // phyiscal calulcations and update cam pos
        // let mut dz: f32 = (10.0 * tick_count * 0.1) -
        //         ((self.gravity / 2.0) * (tick_count * 0.1) * (tick_count *
        //         0.1));
        // println!("cam position: {:?}", self.cam.position);
        if self.is_jumping == true {
            if self.cam.position[2] >= 50.0 {
                self.dz = self.dz - self.gravity;
                // let mut dz = (1.5 * self.tick_count * 0.007) -
                //              ((self.gravity) * (self.tick_count * 0.007) *
                //               (self.tick_count * 0.007));
                // if dz < -0.03 {
                //     dz = -0.03;
                // }
                println!("dz: {:?}", self.dz);
                if self.dz < -0.2 {
                    self.dz = -0.2;
                }
                self.cam.move_up(self.dz);
                self.tick_count += 1.0;
                // self.cam.position[2] += self.delta_z - self.gravity;
                if self.delta_z > -100.0 {
                    self.delta_z -= 1.0;
                }
            } else {
                self.tick_count = 1.0;
                self.is_jumping = false;
                self.cam.position[2] = 1.0;
            }

        }

        // if self.is_moving {
        //     self.get_curr_motion();
        // }
    }

    // pub fn get_curr_motion() -> (f32, f32) {
    //     (self.vel_x, self.vel_y)
    // }
    // pub fn get_jumping_vel(&self) -> f32 {

    //     self.delta_z = self.delta_z

    // }
    // pub fn move(&self) {

    // }
}

impl EventHandler for Player {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        match *e {
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => {
                self.cam.move_forward(0.1);
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                self.cam.move_backward(0.1);
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                self.cam.move_left(0.1);

                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                self.cam.move_right(0.1);
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Space)) => {

                // if !self.is_jumping() {
                self.jump();
                //  self.cam.move_down(0.1);
                EventResponse::Continue
            }

            // X only for the fact, that you cannot hold LControl to go down,
            // because holding the key only counts as one keystroke
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::C)) => {
                self.cam.move_down(1.0);
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::LControl)) => {
                self.cam.move_down(1.0);
                EventResponse::Continue
            }

            Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                info!("clicked");
                if let Some(window) = self.context.get_window() {
                    let res = window.set_cursor_state(CursorState::Grab);
                    warn!("{:?}", res);
                } else {
                    warn!("Failed to obtain window from facade");
                }
                EventResponse::Continue
            }
            Event::MouseMoved(x, y) => {

                if let Some((prev_x, prev_y)) = self.prev_mouse_pos {
                    let x_diff = x - prev_x;
                    let y_diff = y - prev_y;
                    info!("x = {}, y = {}", x_diff, y_diff);
                    self.cam.change_dir(y_diff as f32 / 300.0, x_diff as f32 / 300.0);
                }

                self.prev_mouse_pos = Some((x, y));

                EventResponse::Continue
            }

            _ => EventResponse::NotHandled,
        }
    }
}
