use super::camera::*;
use super::event_manager::*;
use GameContext;
use glium::glutin::{CursorState, ElementState, Event, MouseButton, VirtualKeyCode};
use base::math::*;
use base::world::*;
use std::rc::Rc;
use super::world_manager::*;
use std::f32;


const GRAVITY: f32 = 9.81;

/// Represents a `Player` in the world, the `Player` can move up, right, down
/// left, right with w, a, s, d, jump with space and speed with shift
pub struct Player {
    cam: Camera,
    context: Rc<GameContext>,
    world_manager: WorldManager,
    acceleration: Vector3f,
    velocity: Vector3f,
    timer_jump: f32,
    timer_vel: f32,
    mouselock: bool,
    shift_speed: f32,
    step_size: f32,
}

impl Player {
    pub fn new(context: Rc<GameContext>, world_manager: WorldManager) -> Self {
        Player {
            cam: Camera::new(context.get_config().resolution.aspect_ratio()),
            world_manager: world_manager,
            context: context,
            timer_jump: 1.0,
            timer_vel: 1.0,
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            mouselock: false,
            shift_speed: 1.0,
            step_size: 2.5,
        }
    }
    /// Gets the actual `Height` of the `HexPillar`
    pub fn get_ground_height(&mut self) -> (Option<f32>, Option<f32>) {
        let mut height = 0.0;
        let mut above = 0.0;
        let world = self.world_manager.get_world();
        let real_pos = Point2f::new(self.cam.position.x, self.cam.position.y);
        let pillar_index = PillarIndex(AxialPoint::from_real(real_pos));
        let vec_len =
            world.pillar_at(pillar_index).map(|pillar| pillar.sections().len()).unwrap_or(0);

        let pillar_vec = world.pillar_at(pillar_index).map(|pillar| pillar.sections());

        if pillar_vec.is_some() {
            let new_pillar_vec = pillar_vec.unwrap();

            if vec_len == 1 {
                height = new_pillar_vec[0].top.to_real();
                above = f32::INFINITY;
            } else {
                for i in 0..vec_len {
                    if i != vec_len - 1 {
                        if new_pillar_vec[i].top.to_real() < self.cam.position.z &&
                           self.cam.position.z < new_pillar_vec[i + 1].bottom.to_real() {
                            height = new_pillar_vec[i].top.to_real();
                            above = new_pillar_vec[i + 1].bottom.to_real();;
                            break;
                        } else {
                            continue;
                        }
                    } else {
                        height = new_pillar_vec[i].top.to_real();
                        above = f32::INFINITY;
                        break;
                    }
                }
            }
        }

        (Some(height), Some(above))
    }

    /// Getter method for the `Camera`
    pub fn get_camera(&self) -> Camera {
        self.cam
    }
    pub fn set_camera(&mut self, cam: Camera) {
        self.cam = cam;
    }

    /// Updates the `Player` after every iteration
    pub fn update(&mut self, delta: f32) {


        let height = (self.get_ground_height().0).unwrap_or(0.0) + 1.75;
        let above = (self.get_ground_height().1).unwrap_or(0.0) + 1.75;
        // Moves the Player forward or backward with the acceleration and delta
        // (1.0 - (-((self.timer_vel * delta) / (1.0))).exp()) -> this is a formula
        // that calculates a
        // number from 0 to 1. So the player has a maximum velocity
        if self.acceleration.x != 0.0 {
            self.velocity.x = self.acceleration.x * delta * delta * self.shift_speed *
                              (1.0 - (-((self.timer_vel * delta) / (1.0))).exp());
        } else {
            if self.velocity.x > 0.5 {
                self.velocity.x /= 1.1;
            } else {
                self.velocity.x = 0.0;
            }
        }

        if self.timer_vel != 100.0 {
            self.timer_vel += 1.0;
        }
        if self.acceleration.x == 0.0 {
            self.timer_vel = 1.0;
        }
        // Moves the Player left and right with the acceleration and delta
        if self.acceleration.y != 0.0 {
            self.velocity.y = self.acceleration.y * delta * delta *
                              (1.0 - (-((self.timer_vel * delta) / (1.0))).exp());

            if self.timer_vel != 100.0 {
                self.timer_vel += 1.0;
            }
            if self.acceleration.y == 0.0 {
                self.timer_vel = 1.0;
            }
        } else {
            if self.velocity.y > 0.5 {
                self.velocity.y /= 1.1;
            } else {
                self.velocity.y = 0.0;
            }
        }

        // Let the player jump with the given start-velocity
        if self.velocity.z != 0.0 {
            let velz = self.velocity.z * self.timer_jump * delta -
                       self.timer_jump * self.timer_jump * delta * delta * GRAVITY;
            self.timer_jump += 1.0;
            if self.cam.position.z + velz > above {
                self.velocity.z = 0.0;
                self.timer_jump = 1.0;
            } else {
                self.cam.move_up(velz);
            }
            // Needed: Update to reflect multiple level pillars
            if self.cam.position.z < height {
                self.velocity.z = 0.0;
                self.timer_jump = 1.0;
            }
        }

        // Checks if the `Player` is higher than the actual `Player` on wich he is
        // standing and let
        // him fall on that
        if self.cam.position.z > height && self.velocity.z == 0.0 {
            self.cam
                .move_down((self.timer_jump * self.timer_jump * delta * delta * GRAVITY) / 16.0);
            self.timer_jump += 1.0;
            if self.cam.position.z < height {
                self.timer_jump = 1.0;
            }
        }
        if height > self.cam.position.z + self.step_size {
            // self.velocity.x = 0.0;
            // self.velocity.y = 0.0;
        } else {
            // Places the `Player` on the actual `HexPillar` if the position of the
            // `Player` is less than
            // the `HexPillar`
            if self.velocity.z == 0.0 && self.cam.position.z < height {
                self.cam.position.z = height;
            }
        }

        self.cam.move_forward(self.velocity.x);
        self.cam
            .move_right(self.velocity.y);


    }
}
/// `EventHandler` for the `Player`
impl EventHandler for Player {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        match *e {
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => {
                self.acceleration.x = 500.5;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::W)) => {
                self.acceleration.x = 0.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                self.acceleration.x = -400.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::S)) => {
                self.acceleration.x = 0.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                self.acceleration.y = -400.5;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::A)) => {
                self.acceleration.y = 0.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                self.acceleration.y = 400.5;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::D)) => {
                self.acceleration.y = 0.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Space)) => {
                self.velocity.z = 6.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Space)) => {
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::LControl)) => {
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::LControl)) => {
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::LShift)) => {
                self.shift_speed = 2.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::LShift)) => {
                self.shift_speed = 1.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::F)) => {
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::F)) => {
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
