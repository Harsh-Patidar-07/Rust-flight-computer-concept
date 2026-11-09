use crate::rocket::Rocket;

/// Represents the instantaneous physical state of the rocket in 1D flight.
#[derive(Debug, Clone, Copy)]
pub struct FlightState {
    pub time_s: f64,
    pub altitude_m: f64,
    pub velocity_m_s: f64,
    pub acceleration_m_s2: f64,
}

impl FlightState {
    pub fn new() -> Self {
        Self {
            time_s: 0.0,
            altitude_m: 0.0,
            velocity_m_s: 0.0,
            acceleration_m_s2: 0.0,
        }
    }

    /// Advances the flight state by one small time step (dt).
    pub fn step(&mut self, rocket: &Rocket, dt: f64) {
        // 1. Calculate acceleration from current forces
        let raw_accel = rocket.acceleration(self.time_s, self.velocity_m_s);

        // If sitting on the pad and thrust < weight, pad holds rocket up (no negative accel into ground)
        if self.altitude_m <= 0.0 && raw_accel < 0.0 {
            self.acceleration_m_s2 = 0.0;
            self.velocity_m_s = 0.0;
            self.altitude_m = 0.0;
        } else {
            self.acceleration_m_s2 = raw_accel;
            // 2. Integrate velocity
            self.velocity_m_s += self.acceleration_m_s2 * dt;
            // 3. Integrate altitude
            self.altitude_m += self.velocity_m_s * dt;

            // Ground collision check
            if self.altitude_m < 0.0 {
                self.altitude_m = 0.0;
                self.velocity_m_s = 0.0;
                self.acceleration_m_s2 = 0.0;
            }
        }

        self.time_s += dt;
    }
}
