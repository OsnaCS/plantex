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
    timer_velx: f32,
    timer_vely: f32,
    mouselock: bool,
    shift_speed: f32,
    step_size: f32,
    move_z: f32,
}

impl Player {
    pub fn new(context: Rc<GameContext>, world_manager: WorldManager) -> Self {
        Player {
            cam: Camera {
                position: Point3::new(15.0, 10.0, 50.0),
                phi: -0.27,
                theta: 2.6,
                aspect_ratio: context.get_config().resolution.aspect_ratio(),
            },
            world_manager: world_manager,
            context: context,
            timer_jump: 1.0,
            timer_velx: 1.0,
            timer_vely: 1.0,
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            mouselock: false,
            shift_speed: 1.0,
            step_size: 1.0,
            move_z: 0.0,
        }
    }

    /// Gets the actual `Height` of the `HexPillar`
    pub fn get_ground_height(&mut self, add_velo: bool) -> (Option<f32>, Option<f32>) {
        let mut height = 0.0;
        let mut above = 0.0;
        let world = self.world_manager.get_world();
        let mut real_pos = Point2f::new(self.cam.position.x, self.cam.position.y);
        if add_velo {
            real_pos.x += self.velocity.x;
            real_pos.y += self.velocity.y;
        }
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
                            above = new_pillar_vec[i + 1].bottom.to_real();
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
    pub fn get_ground_height_x(&mut self,
                               add_vec: Vector2f)
                               -> (Option<f32>, Option<f32>, Option<f32>) {
        let mut height = 0.0;
        let mut above = 0.0;
        let world = self.world_manager.get_world();
        let mut real_pos = Point2f::new(self.cam.position.x + add_vec.x,
                                        self.cam.position.y + add_vec.y);
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
                            above = new_pillar_vec[i + 1].bottom.to_real();
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

        (Some(height), Some(above), Some(above - height))
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

        // Get current pillar floor (`height`) and the ceiling (`above`)
        let height = (self.get_ground_height(false).0).unwrap_or(0.0) + 1.75;
        let above = (self.get_ground_height(false).1).unwrap_or(0.0);

        // Calculates the vectors arround the player, wehre he is moving at
        let vec_front = Vector2 {
            x: self.cam.phi.cos() * 1.5,
            y: self.cam.phi.sin() * 1.5,
        };
        let vec_right = ((Vector3 {
                    x: vec_front.x,
                    y: vec_front.y,
                    z: 0.0,
                })
                .cross(Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                }))
            .truncate();;
        let vec_back = Vector2 {
            x: -vec_front.x,
            y: -vec_front.y,
        };
        let vec_left = Vector2 {
            x: -vec_right.x,
            y: -vec_right.y,
        };
        let vec_front_right = Vector2 {
            x: vec_front.x + vec_right.x,
            y: vec_front.y + vec_right.y,
        };
        let vec_front_left = Vector2 {
            x: vec_front.x + vec_left.x,
            y: vec_front.y + vec_left.y,
        };
        let vec_back_right = Vector2 {
            x: vec_back.x + vec_right.x,
            y: vec_back.y + vec_right.y,
        };
        let vec_back_left = Vector2 {
            x: vec_back.x + vec_left.x,
            y: vec_back.y + vec_left.y,
        };


        // Move the Player forward or backward with the acceleration and delta
        // (1.0 - (-((self.timer_vel * delta) / (1.0))).exp()) -> this is a formula
        // that calculates a
        // number from 0 to 1. So the player has a maximum velocity
        if self.acceleration.x != 0.0 {
            self.velocity.x = self.acceleration.x * delta * delta * self.shift_speed *
                              (1.0 - (-((self.timer_velx * delta) / (1.0))).exp());

        }
        // Reduce x velocity every tick
        else {
            if self.velocity.x > 0.0001 || self.velocity.x < -0.0001 {
                self.velocity.x /= 1.4;
            } else {
                self.velocity.x = 0.0;
            }
        }

        if self.timer_velx != 100.0 {
            self.timer_velx += 1.0;
        }
        if self.acceleration.x == 0.0 {
            self.timer_velx = 1.0;
        }

        // Move the Player left and right with the acceleration and delta
        if self.acceleration.y != 0.0 {
            self.velocity.y = self.acceleration.y * delta * delta *
                              (1.0 - (-((self.timer_vely * delta) / (1.0))).exp());
        } else {
            if self.velocity.y > 0.0001 || self.velocity.y < -0.0001 {
                self.velocity.y /= 1.4;
            } else {
                self.velocity.y = 0.0;
            }
        }
        if self.timer_vely != 100.0 {
            self.timer_vely += 1.0;
        }
        if self.acceleration.y == 0.0 {
            self.timer_vely = 1.0;
        }
        // Reduce y velocity every tick
        else {
            if self.velocity.y > 0.0001 || self.velocity.y < -0.0001 {
                self.velocity.y /= 1.4;
            } else {
                self.velocity.y = 0.0;
            }

        }
        // Let the player jump with the given start-velocity
        if self.velocity.z != 0.0 {

            self.velocity.z += (-delta * GRAVITY) * 0.2;
            self.timer_jump += 1.0;

            if self.cam.position.z + self.velocity.z > above {
                self.velocity.z = 0.0;
                self.timer_jump = 1.0;
                // self.move_z = 0.0;
            } else {
                self.cam.move_up(self.velocity.z);
            }
            // Needed: Update to reflect multiple level pillars
            if self.cam.position.z < height {
                self.velocity.z = 0.0;
                self.timer_jump = 1.0;
                // self.move_z = 0.0;
            }
        }

        // Checks if the `Player` is higher than the actual `Player` on wich he is
        // standing and let
        // him fall on that
        if self.cam.position.z > height && self.velocity.z == 0.0 {
            self.velocity.z += (-delta * GRAVITY) / 16.0;


            // moveZ = -((self.timer_jump * self.timer_jump * delta * delta * GRAVITY) /
            // 16.0);
            self.timer_jump += 1.0;
            if self.cam.position.z < height {
                self.timer_jump = 1.0;
                // self.move_z = 0.0;
            }
            self.cam.move_down(self.velocity.z);
        }
        let mut height_set = true;
        if (self.velocity.x > 0.0001 &&
            self.get_ground_height_x(vec_front).0.unwrap_or(0.0) > height + self.step_size) ||
           self.get_ground_height_x(vec_front).2.unwrap_or(0.0) < 3.0 {
            self.velocity.x = 0.0;
            height_set = false;
        } else if (self.velocity.x < -0.0001 &&
            self.get_ground_height_x(vec_back).0.unwrap_or(0.0) > height + self.step_size) ||
           self.get_ground_height_x(vec_back).2.unwrap_or(0.0) < 3.0 {
            self.velocity.x = 0.0;
            height_set = false;
        } else if (self.velocity.y > 0.0001 &&
            self.get_ground_height_x(vec_right).0.unwrap_or(0.0) > height + self.step_size) ||
           self.get_ground_height_x(vec_right).2.unwrap_or(0.0) < 3.0 {
            self.velocity.y = 0.0;
            height_set = false;
        } else if (self.velocity.y < -0.0001 &&
            self.get_ground_height_x(vec_left).0.unwrap_or(0.0) > height + self.step_size) ||
           self.get_ground_height_x(vec_left).2.unwrap_or(0.0) < 3.0 {
            self.velocity.y = 0.0;
            height_set = false;
        }
        println!("velocity:------{:?}", self.velocity);
        println!("hoehe:---------{:?}", self.cam.position.z);
        println!("-------------");
        println!("-------------");

        // if self.velocity.x > 0.0001 && self.velocity.y > 0.0001 &&
        // self.get_ground_height_x(vec_front_right).0.unwrap_or(0.0) > height +
        // self.step_size ||
        //    self.get_ground_height_x(vec_front_right).2.unwrap_or(0.0) < 3.0 {
        //     self.velocity.x = 0.0;
        //     self.velocity.y = 0.0;
        //     height_set = false;
        // } else if self.velocity.x < -0.0001 && self.velocity.y < -0.0001 &&
        // self.get_ground_height_x(vec_back_left).0.unwrap_or(0.0) > height +
        // self.step_size ||
        //    self.get_ground_height_x(vec_back_left).2.unwrap_or(0.0) < 3.0 {
        //     self.velocity.x = 0.0;
        //     self.velocity.y = 0.0;
        //     height_set = false;
        // } else if self.velocity.y > 0.0001 && self.velocity.x < -0.0001 &&
        // self.get_ground_height_x(vec_back_right).0.unwrap_or(0.0) > height +
        // self.step_size ||
        //    self.get_ground_height_x(vec_back_right).2.unwrap_or(0.0) < 3.0 {
        //     self.velocity.y = 0.0;
        //     self.velocity.x = 0.0;
        //     height_set = false;
        // } else if self.velocity.y < -0.0001 && self.velocity.x > 0.0001 &&
        // self.get_ground_height_x(vec_front_left).0.unwrap_or(0.0) > height +
        // self.step_size ||
        //    self.get_ground_height_x(vec_front_left).2.unwrap_or(0.0) < 3.0 {
        //     self.velocity.y = 0.0;
        //     self.velocity.x = 0.0;
        //     height_set = false;
        // }

        if self.cam.position.z < height && self.velocity.z == 0.0 && height_set {
            self.cam.position.z = height;
        }

        self.cam.move_forward(self.velocity.x);
        self.cam.move_right(self.velocity.y);





        // println!("Velocity.x {:?} Velocity.y {:?}",
        //          self.velocity.x,
        //          self.velocity.y);
        // let mut next_height_x = (self.get_ground_height_x(true).0).unwrap_or(0.0) +
        // 1.75;
        // let mut next_height_y = (self.get_ground_height_y(true).0).unwrap_or(0.0) +
        // 1.75;
        // // let delta = next_height_x - height;
        // let diff = next_height_x - height;
        // if self.velocity.z == 0.0 && self.cam.position.z < height && diff > 3.0 {
        //     self.cam.position.z = height;
        // }
        // // let next_above = (self.get_ground_height_x(true).1).unwrap_or(0.0) + 1.75;
        // println!("height {:?} next_height {:?}", height, next_height_x);
        // if next_height_x > self.step_size + height || next_height_y > self.step_size
        // + height {
        //     println!("Height error ---------------x{:?}", next_height_x);
        //     println!("Height error ---------------y{:?}", next_height_y);
        //     self.velocity.x = 0.0;
        //     self.velocity.y = 0.0;
        // } else {
        //     self.cam.move_forward(self.velocity.x);

        //     self.cam.move_right(self.velocity.y);
        // }
        // next_height_x = 0.0;
        // next_height_y = 0.0;




        // if next_height_y > self.step_size + height {
        //     println!("Height error ---------------y {:?}", next_height_y);
        //     self.velocity.y = 0.0;
        // } else {
        //     // self.cam.move_forward(self.velocity.x);
        //     self.cam.move_right(self.velocity.y);
        // }
    }
}
/// `EventHandler` for the `Player`
impl EventHandler for Player {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        match *e {
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => {
                self.acceleration.x = 100.5;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::W)) => {
                self.acceleration.x = 0.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                self.acceleration.x = -80.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::S)) => {
                self.acceleration.x = 0.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                self.acceleration.y = -80.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::A)) => {
                self.acceleration.y = 0.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                self.acceleration.y = 80.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::D)) => {
                self.acceleration.y = 0.0;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Space)) => {
                self.velocity.z = 0.7;
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
