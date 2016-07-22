use super::camera::*;
use super::event_manager::*;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{CursorState, ElementState, Event, MouseButton, VirtualKeyCode};

pub struct Ghost {
    cam: Camera,
    prev_mouse_pos: Option<(i32, i32)>,
    context: GlutinFacade,
}

impl Ghost {
    pub fn new(context: GlutinFacade) -> Self {
        Ghost {
            cam: Camera::default(),
            prev_mouse_pos: None,
            context: context,
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
                self.cam.move_forward(1.0);
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => {
                self.cam.move_backward(1.0);
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => {
                self.cam.move_left(1.0);
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => {
                self.cam.move_right(1.0);
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Space)) => {
                self.cam.move_up(1.0);
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
                    let x_diff = (x - prev_x);
                    let y_diff = (y - prev_y);
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
