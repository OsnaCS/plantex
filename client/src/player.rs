use super::camera::*;
use super::event_manager::*;
use super::world_manager::*;
use base::math::*;
use base::world::*;
use glium::glutin::{CursorState, ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent, KeyboardInput};
use std::f32;
use std::rc::Rc;
use GameContext;

const GRAVITY: f32 = 9.81;

/// Represents a `Player` in the world, the `Player` can move up, right, down
/// left, right with w, a, s, d, jump with space and speed with shift
pub struct Player {
    cam: Camera,
    context: Rc<GameContext>,
    world_manager: WorldManager,
    acceleration: Vector3f,
    velocity: Vector3f,
    timer_velx: f32,
    timer_vely: f32,
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
            timer_velx: 1.0,
            timer_vely: 1.0,
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            mouselock: false,
            shift_speed: 1.0,
            step_size: 1.0,
        }
    }

    // Return the `Pillar` at the given `Vector2`
    pub fn get_ground_height_at(
        &mut self,
        add_vec: Vector2f,
    ) -> (Option<f32>, Option<f32>, Option<f32>) {
        let mut height = 0.0;
        let mut above = 0.0;
        let world = self.world_manager.get_world();
        let real_pos = Point2f::new(
            self.cam.position.x + add_vec.x,
            self.cam.position.y + add_vec.y,
        );
        let pillar_index = PillarIndex(AxialPoint::from_real(real_pos));
        let vec_len = world
            .pillar_at(pillar_index)
            .map(|pillar| pillar.sections().len())
            .unwrap_or(0);

        let pillar_vec = world
            .pillar_at(pillar_index)
            .map(|pillar| pillar.sections());

        if pillar_vec.is_some() {
            let new_pillar_vec = pillar_vec.unwrap();

            if vec_len == 1 {
                height = new_pillar_vec[0].top.to_real();
                above = f32::INFINITY;
            } else {
                for i in 0..vec_len {
                    if i != vec_len - 1 {
                        if new_pillar_vec[i].top.to_real() < self.cam.position.z
                            && self.cam.position.z < new_pillar_vec[i + 1].bottom.to_real()
                        {
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

    /// Update the `Player` after every iteration
    pub fn update(&mut self, delta: f32) {
        // Get current pillar floor (`height`) and the ceiling (`above`)
        let height =
            (self.get_ground_height_at(Vector2 { x: 0.0, y: 0.0 }).0).unwrap_or(0.0) + 1.75;
        let above = (self.get_ground_height_at(Vector2 { x: 0.0, y: 0.0 }).1).unwrap_or(0.0);

        // Calculate `angle` for the next `Pillar`
        let angle: f32 = (60.0 as f32).to_radians();

        // Move the Player forward or backward with the acceleration and delta
        // (1.0 - (-((self.timer_vel * delta) / (1.0))).exp()) -> this is a formula
        // that calculates a
        // number from 0 to 1. So the player has a maximum velocity
        if self.acceleration.x != 0.0 {
            self.velocity.x = self.acceleration.x
                * delta
                * delta
                * self.shift_speed
                * (1.0 - (-((self.timer_velx * delta) / (1.0))).exp());
        }
        // Reduce x velocity every tick
        else {
            if self.velocity.x > 0.0001 || self.velocity.x < -0.0001 {
                self.velocity.x /= 1.4;
            } else {
                self.velocity.x = 0.0;
            }
        }
        // Reset the timer for the exp-function
        if self.timer_velx != 100.0 {
            self.timer_velx += 1.0;
        }
        if self.acceleration.x == 0.0 {
            self.timer_velx = 1.0;
        }

        // Move the `Player` left and right with the acceleration and delta
        if self.acceleration.y != 0.0 {
            self.velocity.y = self.acceleration.y
                * delta
                * delta
                * (1.0 - (-((self.timer_vely * delta) / (1.0))).exp());
        } else {
            if self.velocity.y > 0.0001 || self.velocity.y < -0.0001 {
                self.velocity.y /= 1.4;
            } else {
                self.velocity.y = 0.0;
            }
        }
        // Reset the timer for the exp-function
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

        // Let the `Player` jump with the given start-velocity
        if self.velocity.z != 0.0 {
            self.velocity.z += (-delta * GRAVITY) * 0.2;

            if self.cam.position.z + self.velocity.z > above {
                self.velocity.z = 0.0;
            } else {
                self.cam.move_up(self.velocity.z);
            }
            // Needed: Update to reflect multiple level pillars
            if self.cam.position.z + self.velocity.z < height {
                self.velocity.z = 0.0;
            }
        }

        // Check if the `Player` is higher than the actual `Pillar` on which he is
        // standing and let him fall on that
        if self.cam.position.z > height && self.velocity.z == 0.0 {
            self.velocity.z += (-delta * GRAVITY) / 16.0;
            if self.cam.position.z + self.velocity.z < height {
                self.velocity.z = 0.0;
            }
            self.cam.move_down(self.velocity.z);
        }

        // Calculate the vectors arround the `Player` in the direction where he is
        // moving
        let vec_0 = Vector2 {
            x: self.cam.phi.cos(),
            y: self.cam.phi.sin(),
        };
        let vec_60 = Vector2 {
            x: vec_0.x * angle.cos() + vec_0.y * angle.sin(),
            y: vec_0.x * (-(angle.sin())) + vec_0.y * angle.cos(),
        };
        let vec_120 = Vector2 {
            x: vec_60.x * angle.cos() + vec_60.y * angle.sin(),
            y: vec_60.x * (-(angle.sin())) + vec_60.y * angle.cos(),
        };
        let vec_180 = Vector2 {
            x: vec_120.x * angle.cos() + vec_120.y * angle.sin(),
            y: vec_120.x * (-(angle.sin())) + vec_120.y * angle.cos(),
        };
        let vec_240 = Vector2 {
            x: vec_180.x * angle.cos() + vec_180.y * angle.sin(),
            y: vec_180.x * (-(angle.sin())) + vec_180.y * angle.cos(),
        };
        let vec_300 = Vector2 {
            x: vec_240.x * angle.cos() + vec_240.y * angle.sin(),
            y: vec_240.x * (-(angle.sin())) + vec_240.y * angle.cos(),
        };

        // Return the six `Pillar`s around the `Player`
        let pillar_0 = (
            vec_0,
            self.get_ground_height_at(vec_0).0.unwrap_or(0.0),
            self.get_ground_height_at(vec_0).2.unwrap_or(0.0),
        );
        let pillar_60 = (
            vec_60,
            self.get_ground_height_at(vec_60).0.unwrap_or(0.0),
            self.get_ground_height_at(vec_60).2.unwrap_or(0.0),
        );
        let pillar_120 = (
            vec_120,
            self.get_ground_height_at(vec_120).0.unwrap_or(0.0),
            self.get_ground_height_at(vec_120).2.unwrap_or(0.0),
        );
        let pillar_180 = (
            vec_180,
            self.get_ground_height_at(vec_180).0.unwrap_or(0.0),
            self.get_ground_height_at(vec_180).2.unwrap_or(0.0),
        );
        let pillar_240 = (
            vec_240,
            self.get_ground_height_at(vec_240).0.unwrap_or(0.0),
            self.get_ground_height_at(vec_240).2.unwrap_or(0.0),
        );
        let pillar_300 = (
            vec_300,
            self.get_ground_height_at(vec_300).0.unwrap_or(0.0),
            self.get_ground_height_at(vec_300).2.unwrap_or(0.0),
        );

        // Collison-detection
        // Move forward, compare the front `Pillar` and the two side `Pillar`s
        if (self.velocity.x > 0.001
            && (self
                .get_ground_height_at(Vector2 {
                    x: vec_0.x - 0.2,
                    y: vec_0.y - 0.1,
                })
                .0
                .unwrap_or(0.0)
                > height + self.step_size
                || self
                    .get_ground_height_at(Vector2 {
                        x: vec_0.x + 0.2,
                        y: vec_0.y + 0.1,
                    })
                    .0
                    .unwrap_or(0.0)
                    > height + self.step_size
                || pillar_0.1 > height + self.step_size))
            || (pillar_0.2 < 3.0)
        {
            if (self.velocity.y > 0.001 && pillar_60.1 > height + self.step_size)
                || pillar_60.2 < 3.0
            {
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            }
            if (self.velocity.y < -0.001 && pillar_300.1 > height + self.step_size)
                || pillar_300.2 < 3.0
            {
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            } else {
                self.velocity.x = 0.0;
            }
        }

        // Move right, compare the front `Pillar`
        if (self.velocity.y > 0.001
            && (pillar_60.1 > height + self.step_size || pillar_120.1 > height + self.step_size))
            || (pillar_60.2 < 3.0 || pillar_120.2 < 3.0)
        {
            self.velocity.y = 0.0;
        }

        // Move backward, compare the `Pillar` behind and the two side `Pillar`s
        if (self.velocity.x < -0.001
            && (self
                .get_ground_height_at(Vector2 {
                    x: vec_180.x - 0.2,
                    y: vec_180.y - 0.1,
                })
                .0
                .unwrap_or(0.0)
                > height + self.step_size
                || self
                    .get_ground_height_at(Vector2 {
                        x: vec_180.x + 0.2,
                        y: vec_180.y + 0.1,
                    })
                    .0
                    .unwrap_or(0.0)
                    > height + self.step_size
                || pillar_180.1 > height + self.step_size))
            || pillar_180.2 < 3.0
        {
            if (self.velocity.y > 0.001 && pillar_120.1 > height + self.step_size)
                || pillar_120.2 < 3.0
            {
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            }
            if (self.velocity.y < -0.001 && pillar_240.1 > height + self.step_size)
                || pillar_240.2 < 3.0
            {
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
            } else {
                self.velocity.x = 0.0;
            }
        }

        // Move right, compare the front `Pillar`
        if (self.velocity.y < -0.001
            && (pillar_240.1 > height + self.step_size || pillar_300.1 > height + self.step_size))
            || (pillar_240.2 < 3.0 || pillar_300.2 < 3.0)
        {
            self.velocity.y = 0.0;
        }

        if self.cam.position.z < height
            && self.velocity.z == 0.0
            && height - self.cam.position.z < 3.0
        {
            self.cam.position.z = height;
        }

        // Move the `Player` with the given velocity
        self.cam.move_forward(self.velocity.x);
        self.cam.move_right(self.velocity.y);
    }
}
/// `EventHandler` for the `Player`
impl EventHandler for Player {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        let e = match e {
            Event::WindowEvent { event, .. } => event,
            _ => return EventResponse::NotHandled,
        };

        match e {
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::W),
                ..
            }, .. } => {
                self.acceleration.x = 60.5;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::W),
                ..
            }, .. } => {
                self.acceleration.x = 0.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::S),
                ..
            }, .. } => {
                self.acceleration.x = -50.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::S),
                ..
            }, .. } => {
                self.acceleration.x = 0.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::A),
                ..
            }, .. } => {
                self.acceleration.y = -50.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::A),
                ..
            }, .. } => {
                self.acceleration.y = 0.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::D),
                ..
            }, .. } => {
                self.acceleration.y = 50.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::D),
                ..
            }, .. } => {
                self.acceleration.y = 0.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Space),
                ..
            }, .. } => {
                if self.velocity.z == 0.0 {
                    self.velocity.z = 0.7;
                }
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::Space),
                ..
            }, .. } => {
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::LControl),
                ..
            }, .. } => {
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::LControl),
                ..
            }, .. } => {
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::LShift),
                ..
            }, .. } => {
                self.shift_speed = 2.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::LShift),
                ..
            }, .. } => {
                self.shift_speed = 1.0;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::F),
                ..
            }, .. } => {
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::F),
                ..
            }, .. } => {
                EventResponse::Continue
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                if !self.mouselock {
                    self.mouselock = true;
                    self.context
                        .get_facade()
                        .gl_window()
                        .set_cursor_state(CursorState::Hide)
                        .expect("failed to set cursor state");
                } else if self.mouselock {
                    self.mouselock = false;

                    self.context
                        .get_facade()
                        .gl_window()
                        .set_cursor_state(CursorState::Normal)
                        .expect("failed to set cursor state");
                }

                EventResponse::Continue
            }

            WindowEvent::CursorMoved { position: (x, y), .. } => {
                if self.mouselock {
                    let window = self.context.get_facade().gl_window();
                    // Possibility of mouse being outside of window without it resetting to the
                    // middle?
                    if let Some(middle) = window.get_inner_size() {
                        let middle_x = (middle.0 as f64) / 2.0;
                        let middle_y = (middle.1 as f64) / 2.0;
                        let x_diff = x - middle_x;
                        let y_diff = y - middle_y;
                        self.cam
                            .change_dir(y_diff as f32 / 300.0, -x_diff as f32 / 300.0);
                        window
                            .set_cursor_position(middle_x as i32, middle_y as i32)
                            .expect("setting cursor position failed");
                    }
                }
                EventResponse::Continue
            }

            _ => EventResponse::NotHandled,
        }
    }
}
