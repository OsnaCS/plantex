use super::camera::*;
use super::event_manager::*;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{CursorState, ElementState, Event, MouseButton, VirtualKeyCode};

pub struct Ghost {
    cam: Camera,
    context: GlutinFacade,
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    mouselock: bool,
}



impl Ghost {
    pub fn new(context: GlutinFacade) -> Self {
        Ghost {
            cam: Camera::default(),
            context: context,
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
            mouselock: false,
        }
    }
    pub fn update(&mut self) {
        if self.forward {
            self.cam.move_forward(1.0);
        }
        if self.backward {
            self.cam.move_backward(1.0);
        }
        if self.left {
            self.cam.move_left(1.0);
        }
        if self.right {
            self.cam.move_right(1.0);
        }
        if self.up {
            self.cam.move_up(1.0);
        }
        if self.down {
            self.cam.move_down(1.0);
        }
    }

    pub fn get_camera(&self) -> Camera {
        self.cam
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

            Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                if !self.mouselock {
                    self.mouselock = true;
                    if let Some(window) = self.context.get_window() {
                        let res = window.set_cursor_state(CursorState::Hide);
                        warn!("{:?} a", res);
                    } else {
                        warn!("Failed to obtain window from facade");
                    }
                } else if self.mouselock {
                    self.mouselock = false;

                    if let Some(window) = self.context.get_window() {
                        let res = window.set_cursor_state(CursorState::Normal);
                        warn!("{:?} b", res);
                    } else {
                        warn!("Failed to obtain window from facade");
                    }
                }

                EventResponse::Continue
            }

            Event::MouseMoved(x, y) => {
                if self.mouselock {
                    if let Some(window) = self.context.get_window() {
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
