use super::event_manager::*;
use glium::glutin::{ElementState, Event, VirtualKeyCode};
use std::f32::consts;
use base::math::*;

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
const PLUS_TIME_SPEED: f32 = 200.0;


impl Default for DayTime {
    fn default() -> DayTime {
        DayTime {
            time_year: 0,
            time_day: 0,
            time_on_day: 0.0,
            speed: DEFAULT_TIME_SPEED,
        }
    }
}

// 360 == Mittag
const DAY_LENGTH: f32 = 720.0;
const YEAR_LENGTH: u32 = 12;


// lengthens the day by a static offset
const DAY_LENGTHER: f32 = 0.0;

// Distance of the path of the sun to the player
const SUN_DISTANCE: f32 = 300.0;

impl DayTime {
    pub fn set_time(&mut self, time_year: u32, time_day: u32, time_on_day: f32) {
        self.time_year = time_year;
        self.time_day = time_day;
        self.time_on_day = time_on_day;
        self.speed = DEFAULT_TIME_SPEED;
    }

    // Bei tag sind die RGBWerte bei 3000, leicht rötlich
    // Bei nacht sind sie 0
    pub fn get_ambient_color(&self) -> Vector3f {
        let mut vec = Vector3f::new();

        let mut offset_factor = 1.1;
        let mut max = 3000 / offset_factor;

        let mut factor = if (self.time_on_day <= (DAY_LENGTH / 2.0)) {
            max * (self.time_on_day / (DAY_LENGTH / 2.0))
        } else {
            max - (max * ((self.time_on_day - DAY_LENGTH / 2) / (DAY_LENGTH / 2.0)))
        };

        vec.x = offset_factor * factor;
        vec.y = factor;
        vec.z = factor;
    }

    // Bei tag ist die Helligkeit 100, leicht bläulich
    // Bei nacht ist die Helligkeit 0
    pub fn get_sky_light(&self) -> Vector3f {
        let mut vec = Vector3f::new();

        let mut offset_factor = 1.1;
        let mut max = 100 / offset_factor;

        let mut factor = if (self.time_on_day <= (DAY_LENGTH / 2.0)) {
            max * (self.time_on_day / (DAY_LENGTH / 2.0))
        } else {
            max - (max * ((self.time_on_day - DAY_LENGTH / 2) / (DAY_LENGTH / 2.0)))
        };

        vec.x = factor;
        vec.y = factor;
        vec.z = offset_factor * factor;
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

    /// Updates time with the use of `delta` as additionally passed time
    /// `DAY_LENGTH` defines the length of a day in real-life seconds
    /// `YEAR_LENGTH` defines the length of a year in `DAY_LENGTH`s
    pub fn update(&mut self, delta: f32) {
        // Output of Time
        debug!("Year: {} Day: {} Time: {}",
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


    /// returns the position of the sun corresponding to time
    /// only mid summer
    pub fn get_sun_position(&self) -> Point3f {

        let half_year = YEAR_LENGTH as f32 / 2.0;
        let half_day = DAY_LENGTH as f32 / 2.0;

        let theta;
        let phi;

        let mut month_diff = self.time_day as f32 - half_year;
        if month_diff < 0.0 {
            month_diff *= -1.0
        }

        if self.time_on_day < half_day {
            // pre noon
            // sun rising
            theta = consts::PI - consts::PI * (self.time_on_day / half_day);
            phi = 0.0;
        } else {
            // after noon
            // sun going down
            theta = consts::PI * ((self.time_on_day - half_day) / half_day);
            phi = consts::PI;
        }

        // for debugging
        // info!("THETA: {} PHI: {}", theta, phi);

        // returns sun position in cartesian coordinates
        // uses `YEAR_LENGTH` and the current day of the year (month) to influence the
        // path the sun moves on
        let pos =
            Vector3f::new(theta.sin() * phi.cos(),
                          theta.sin() * phi.sin() + month_diff / (0.75 * YEAR_LENGTH as f32),
                          theta.cos() - month_diff / (0.75 * YEAR_LENGTH as f32) + DAY_LENGTHER)
                .normalize() * SUN_DISTANCE;

        Point3f::new(pos.x, pos.y, pos.z)
    }

    /// returns the Vector3f for the directional sunlight
    pub fn get_sun_light_vector(&self) -> Vector3f {
        Vector3f::new(0.0, 0.0, 0.0) - self.get_sun_position().to_vec().normalize()
    }
}

/// Handler to speed up time with use of '+' key
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
