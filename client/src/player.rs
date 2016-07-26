use super::camera::*;
use super::event_manager::*;
use GameContext;
use glium::glutin::{CursorState, ElementState, Event, MouseButton, VirtualKeyCode};
use base::math::*;
use base::world::*;
use std::rc::Rc;
use super::world_manager::*;

const VELOCITY: f32 = 1.5;
const LOW_VELOCITY: f32 = 1.0;
const FAST_VELOCITY: f32 = 10.0;
pub struct Player {
    cam: Camera,
    context: Rc<GameContext>,
    world_manager: WorldManager,
    walk_vel: Vector2<f32>,
    dz: f32,
    gravity: f32,
    is_jumping: bool,
    is_falling: bool,
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    mouselock: bool,
    speed: bool,
}

impl Player {
    pub fn new(context: Rc<GameContext>, world_manager: WorldManager) -> Self {
        Player {
            cam: Camera {
                position: Point3::new(15.0, 10.0, 50.0),
                phi: -0.27,
                theta: 2.6,
            },
            world_manager: world_manager,
            context: context,
            gravity: 0.005,
            dz: 0.0,
            walk_vel: Vector2::new(0.0, 0.0),
            is_jumping: false,
            is_falling: false,
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
            mouselock: false,
            speed: false,
        }
    }

    pub fn get_height(&self) -> f32 {
        let world = self.world_manager.get_world();
        let real_pos = Point2f::new(self.cam.position[0], self.cam.position[1]);
        let axial_pos = AxialPoint::from_real(real_pos);
        let argument = PillarIndex(axial_pos);
        let pillar = world.pillar_at(argument);

        if pillar.is_some() {
            let sect = pillar.unwrap().sections();
            sect[0].top.to_real()
        } else {
            0.0
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
    }

    pub fn update(&mut self, delta: f32) {
        let height = self.get_height() + 1.0;

        if !self.is_jumping && !self.is_falling {
            if height < self.cam.position[2] {
                self.is_falling = true;
            } else {
                self.cam.position[2] = height;
            }
        }

        if self.cam.position[2] > height && !self.is_jumping {
            self.is_falling = true;
        }

        if self.is_jumping {
            self.is_falling = true;
        }
        if self.is_falling {
            self.dz = self.dz - self.gravity;

            if self.dz < -0.2 {
                self.dz = -0.2;
            }
            self.cam.move_up(self.dz);
        }
        if self.cam.position[2] < height {
            self.is_falling = false;
            self.is_jumping = false;
            self.dz = 0.0;
            self.cam.position[2] = height;
        }
        if self.forward && !self.backward {
            if self.speed {
                if self.walk_vel[1] < FAST_VELOCITY {
                    self.walk_vel[1] += 0.1;
                }
            }
            if self.walk_vel[1] < VELOCITY {
                self.walk_vel[1] += 0.1;
            }
            self.cam.move_forward(self.walk_vel[1] * delta);
        }
        if self.backward && !self.forward {
            if self.walk_vel[1] < LOW_VELOCITY {
                self.walk_vel[1] += 0.1;
            }
            self.cam.move_backward(self.walk_vel[1] * delta);
        }
        if self.left && !self.right {
            if self.walk_vel[0] < LOW_VELOCITY {
                self.walk_vel[0] += 0.1;
            }
            self.cam.move_left(self.walk_vel[0] * delta);
        }
        if !self.left && self.right {
            if self.walk_vel[0] < LOW_VELOCITY {
                self.walk_vel[0] += 0.1;
            }
            self.cam.move_right(self.walk_vel[0] * delta);
        }
    }
}

impl EventHandler for Player {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        match *e {
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => {
                self.forward = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::W)) => {
                self.walk_vel[1] = 0.0;
                self.forward = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                self.backward = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::S)) => {
                self.walk_vel[1] = 0.0;
                self.backward = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                self.left = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::A)) => {
                self.walk_vel[0] = 0.0;
                self.left = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                self.right = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::D)) => {
                self.walk_vel[0] = 0.0;
                self.right = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Space)) => {
                self.jump();
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Space)) => {
                self.up = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::LControl)) => {
                self.down = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::LControl)) => {
                self.down = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::LShift)) => {
                self.speed = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::LShift)) => {
                if self.walk_vel[1] > VELOCITY {
                    self.walk_vel[1] = VELOCITY;
                }
                self.speed = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::F)) => {
                self.is_falling = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::F)) => {
                self.is_falling = false;
                EventResponse::Continue
            }
            Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                if !self.mouselock {
                    self.mouselock = true;
                    if let Some(window) = self.context.get_facade().get_window() {
                        window.set_cursor_state(CursorState::Hide)
                            .expect("failed to set cursor state");
                    } else {
                        warn!("Failed to obtain window from facade");
                    }
                } else if self.mouselock {
                    self.mouselock = false;

                    if let Some(window) = self.context.get_facade().get_window() {
                        window.set_cursor_state(CursorState::Normal)
                            .expect("failed to set cursor state");
                    } else {
                        warn!("Failed to obtain window from facade");
                    }
                }

                EventResponse::Continue
            }

            Event::MouseMoved(x, y) => {
                if self.mouselock {
                    if let Some(window) = self.context.get_facade().get_window() {
                        // Possibility of mouse being outside of window without it resetting to the
                        // middle?
                        if let Some(middle) = window.get_inner_size_pixels() {
                            let middle_x = (middle.0 as i32) / 2;
                            let middle_y = (middle.1 as i32) / 2;
                            let x_diff = x - middle_x;
                            let y_diff = y - middle_y;
                            self.cam.change_dir(y_diff as f32 / 300.0, -x_diff as f32 / 300.0);
                            window.set_cursor_position(middle_x as i32, middle_y as i32)
                                .expect("setting cursor position failed");
                        }
                    }
                }
                EventResponse::Continue
            }

            _ => EventResponse::NotHandled,
        }
    }
}
