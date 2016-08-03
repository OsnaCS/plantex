use player::Player;
use ghost::Ghost;
use super::camera::*;
use super::event_manager::*;
use glium::glutin::{ElementState, Event, VirtualKeyCode};

/// Switch between `ghost` and `player` cameras with `G` key
pub struct ControlSwitcher {
    player: Player,
    ghost: Ghost,
    is_ghost: bool,
}


impl ControlSwitcher {
    pub fn new(player: Player, ghost: Ghost) -> Self {
        ControlSwitcher {
            player: player,
            ghost: ghost,
            is_ghost: true,
        }
    }
    /// Return current `Camera`
    pub fn get_camera(&self) -> Camera {
        if self.is_ghost { self.ghost.get_camera() } else { self.player.get_camera() }
    }

    /// Run camera `update` function
    pub fn update(&mut self, delta: f32) {
        if self.is_ghost {
            self.ghost.update(delta);
        } else {
            self.player.update(delta);
        }
    }

    /// Switch current camera between `ghost` and `player`
    /// Return to original location of `player`
    pub fn switch_cam(&mut self) {
        if self.is_ghost {
            self.player.set_camera(self.ghost.get_camera());
            self.is_ghost = false;
        } else {
            self.ghost.set_camera(self.player.get_camera());
            self.is_ghost = true;
        }

    }
}

/// Listen for `G`
impl EventHandler for ControlSwitcher {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        match *e {

            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::G)) => {
                self.switch_cam();
                EventResponse::Continue
            }

            _ => {
                if self.is_ghost {
                    self.ghost.handle_event(e)
                } else {
                    self.player.handle_event(e)
                }
            }
        }
    }
}
