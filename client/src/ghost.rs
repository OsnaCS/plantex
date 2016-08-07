use super::camera::*;
use super::GameContext;
use super::event_manager::*;
use glium::glutin::{CursorState, ElementState, Event, MouseButton, VirtualKeyCode};
use std::rc::Rc;

pub struct Ghost {
    cam: Camera,
    context: Rc<GameContext>,
    speed: f32,
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    mouselock: bool,
}

// Speed per second
const DEFAULT_SPEED: f32 = 12.0;
const SHIFT_SPEED: f32 = 60.0;



impl Ghost {
    pub fn new(context: Rc<GameContext>) -> Self {
        Ghost {
            cam: Camera::new(context.get_config().resolution.aspect_ratio()),
            context: context,
            speed: DEFAULT_SPEED,
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
            mouselock: false,
        }
    }
    pub fn update(&mut self, delta: f32) {
        let factored_speed = self.speed * delta;

        if self.forward {
            self.cam.move_forward(factored_speed);
        }
        if self.backward {
            self.cam.move_backward(factored_speed);
        }
        if self.left {
            self.cam.move_left(factored_speed);
        }
        if self.right {
            self.cam.move_right(factored_speed);
        }
        if self.up {
            self.cam.move_up(factored_speed);
        }
        if self.down {
            self.cam.move_down(factored_speed);
        }
    }

    pub fn get_camera(&self) -> Camera {
        self.cam
    }
    pub fn set_camera(&mut self, cam: Camera) {
        self.cam = cam;
    }
}


/// **Implements Controls**
/// *W => FORWARD
/// *A => LEFT
/// *S => BACKWARDS
/// *D => RIGHT
/// *'Space' => UP
/// *'LControl' => DOWN (only accepts one keystroke (cannot hold LControl to go
/// down))
/// *'C' => DOWN (as a replacement for 'LControl')

/// MouseMovement for changing the direction the camera looks
impl EventHandler for Ghost {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        match *e {
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => {
                self.forward = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::W)) => {
                self.forward = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                self.backward = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::S)) => {
                self.backward = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                self.left = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::A)) => {
                self.left = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                self.right = true;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::D)) => {
                self.right = false;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Space)) => {
                self.up = true;
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
                self.speed = SHIFT_SPEED;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::LShift)) => {
                self.speed = DEFAULT_SPEED;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::G)) => {
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
