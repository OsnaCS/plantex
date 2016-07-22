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

    pub fn poll_events(&self, mut handlers: Vec<&mut EventHandler>) -> EventResponse {

        for ev in self.context.poll_events() {
            for i in 0..handlers.len() {
                let response = handlers[i].handle_event(&ev);
                match response {
                    EventResponse::NotHandled |
                    EventResponse::Continue => (),
                    EventResponse::Break => break,
                    EventResponse::Quit => return EventResponse::Quit,
                }
            }
        }
        // Just for the sake of return value
        EventResponse::NotHandled
    }
}

pub trait EventHandler {
    fn handle_event(&mut self, e: &Event) -> EventResponse;
}

/// Handler that handles the closing of the window
pub struct CloseHandler;

/// handle_event function of CloseHandler
/// Windows can be closed by clicking the 'X' on the upper edge of the window
/// or by pressing 'Escape'
impl EventHandler for CloseHandler {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        match *e {
            Event::Closed => EventResponse::Quit,
            Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => EventResponse::Quit,
            _ => EventResponse::NotHandled,
        }
    }
}
