use glium::glutin::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};

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
    events_loop: EventsLoop,
}

impl EventManager {
    pub fn new(events_loop: EventsLoop) -> Self {
        EventManager { events_loop }
    }

    pub fn poll_events(&mut self, mut handlers: Vec<&mut dyn EventHandler>) -> EventResponse {
        let mut quit = false;
        self.events_loop.poll_events(|ev| {
            for i in 0..handlers.len() {
                let response = handlers[i].handle_event(&ev);
                match response {
                    EventResponse::NotHandled | EventResponse::Continue => (),
                    EventResponse::Break => break,
                    EventResponse::Quit => quit = true,
                }
            }
        });

        if quit {
            EventResponse::Quit
        } else {
            EventResponse::NotHandled
        }
    }
}

pub trait EventHandler {
    fn handle_event(&mut self, e: &Event) -> EventResponse;
}

/// Handler that handles the closing of the window
pub struct CloseHandler;

/// handle_event function of CloseHandler

/// Windows can be closed by:

/// *clicking the 'X' on the upper edge of the window

/// *pressing 'Escape'
impl EventHandler for CloseHandler {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        let e = match e {
            Event::WindowEvent { event, .. } => event,
            _ => return EventResponse::NotHandled,
        };

        match e {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => EventResponse::Quit,
            _ => EventResponse::NotHandled,
        }
    }
}
