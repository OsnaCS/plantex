use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{Event, VirtualKeyCode};

/// Every event receiver has to return a response for each event received.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventResponse {
    /// The event was not handled at all
    NotHandled,
    /// The event was handled but should be forwarded to other receivers, too
    Continue,
    /// The event was handled and should *not* be forwarded to other receivers
    Break,
    /// In response to the event, the program should terminate
    Quit,
}

pub struct EventManager {
    context: GlutinFacade,
}

impl EventManager {
    pub fn new(context: GlutinFacade) -> Self {
        EventManager { context: context }
    }

    pub fn poll_events(&self) -> EventResponse {
        for ev in self.context.poll_events() {
            match ev {
                Event::Closed => return EventResponse::Quit,
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => {
                    return EventResponse::Quit;
                }
                _ => (),
            }
        }

        EventResponse::NotHandled
    }
}
