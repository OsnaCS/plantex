use super::camera::*;
use super::event_manager::*;
use super::GameContext;
use glium::glutin::{CursorState, ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent, KeyboardInput};
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
                self.forward = true;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::W),
                ..
            }, .. } => {
                self.forward = false;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::S),
                ..
            }, .. } => {
                self.backward = true;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::S),
                ..
            }, .. } => {
                self.backward = false;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::A),
                ..
            }, .. } => {
                self.left = true;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::A),
                ..
            }, .. } => {
                self.left = false;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::D),
                ..
            }, .. } => {
                self.right = true;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::D),
                ..
            }, .. } => {
                self.right = false;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Space),
                ..
            }, .. } => {
                self.up = true;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::Space),
                ..
            }, .. } => {
                self.up = false;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::LControl),
                ..
            }, .. } => {
                self.down = true;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::LControl),
                ..
            }, .. } => {
                self.down = false;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::LShift),
                ..
            }, .. } => {
                self.speed = SHIFT_SPEED;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::LShift),
                ..
            }, .. } => {
                self.speed = DEFAULT_SPEED;
                EventResponse::Continue
            }
            WindowEvent::KeyboardInput { input: KeyboardInput {
                state: ElementState::Released,
                virtual_keycode: Some(VirtualKeyCode::G),
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

            WindowEvent::MouseMoved { position: (x, y), .. } => {
                if self.mouselock {
                    let window = self.context.get_facade().gl_window();
                    // Possibility of mouse being outside of window without it resetting to the
                    // middle?
                    if let Some(middle) = window.get_inner_size_pixels() {
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
