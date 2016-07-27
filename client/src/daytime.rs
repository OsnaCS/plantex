use super::event_manager::*;
use glium::glutin::{ElementState, Event, VirtualKeyCode};

#[derive(Debug)]
pub struct DayTime {
    time_year: u32,
    time_day: u32,
    time_on_day: f32,
    speed: f32,
}

// `DEFAULT_TIME_SPEED` is 1.0 so the time goes at normal speed
// `PLUS_TIME_SPEED` is the factor with which the time is sped up, when the
// speed-up key is pressed
const DEFAULT_TIME_SPEED: f32 = 1.0;
const PLUS_TIME_SPEED: f32 = 1000.0;


impl Default for DayTime {
    fn default() -> DayTime {
        DayTime {
            time_year: 0,
            time_day: 0,
            time_on_day: 0.0,
            speed: 1.0,
        }
    }
}

impl DayTime {
    pub fn set_time(&mut self, time_year: u32, time_day: u32, time_on_day: f32) {
        self.time_year = time_year;
        self.time_day = time_day;
        self.time_on_day = time_on_day;
        self.speed = DEFAULT_TIME_SPEED;
    }

    pub fn get_time_year(&self) -> u32 {
        self.time_year
    }

    pub fn get_time_day(&self) -> u32 {
        self.time_day
    }

    pub fn get_time_on_day(&self) -> f32 {
        self.time_on_day
    }

    // Updates time with the use of `delta` as additionally passed time
    // `DAY_LENGTH` defines the length of a day in real-life seconds
    // `YEAR_LENGTH` defines the length of a year in `DAY_LENGTH`s
    pub fn update(&mut self, delta: f32) {
        const DAY_LENGTH: f32 = 720.0;
        const YEAR_LENGTH: u32 = 12;

        // Output of Time
        info!("Year: {} Day: {} Time: {}",
              self.time_year,
              self.time_day,
              self.time_on_day);

        // Checks if one day has passed
        self.time_on_day += delta * self.speed;
        if (self.time_on_day) >= DAY_LENGTH {
            self.time_on_day -= DAY_LENGTH; // Removes one day from time_on_day
            self.time_day += 1;
            if (self.time_day) >= YEAR_LENGTH {
                self.time_day = 0;
                self.time_year += 1;
            }
        }

    }
}

// Handler to speed up time with use of key
impl EventHandler for DayTime {
    fn handle_event(&mut self, e: &Event) -> EventResponse {
        match *e {
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Add)) => {
                self.speed = PLUS_TIME_SPEED;
                EventResponse::Continue
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Add)) => {
                self.speed = DEFAULT_TIME_SPEED;
                EventResponse::Continue
            }
            _ => EventResponse::NotHandled,
        }

    }
}
