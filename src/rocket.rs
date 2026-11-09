use crate::motor::SolidMotor;
use std::f64::consts::PI;

pub const GRAVITY: f64 = 9.80665; // m/s^2
pub const AIR_DENSITY_SEA_LEVEL: f64 = 1.225; // kg/m^3

/// Represents the physical rocket airframe carrying a motor.
pub struct Rocket {
    pub dry_mass_kg: f64,
    pub drag_coefficient: f64,
    pub diameter_m: f64,
    pub motor: SolidMotor,
}

impl Rocket {
    pub fn new(
        dry_mass_kg: f64,
        drag_coefficient: f64,
        diameter_m: f64,
        motor: SolidMotor,
    ) -> Self {
        Self {
            dry_mass_kg,
            drag_coefficient,
            diameter_m,
            motor,
        }
    }

    /// Frontal cross-sectional area: A = π * r^2
    pub fn cross_sectional_area(&self) -> f64 {
        let radius = self.diameter_m / 2.0;
        PI * radius * radius
    }

    /// Total mass of the rocket at any given time (airframe + motor casing + remaining fuel).
    pub fn total_mass_at(&self, time_s: f64) -> f64 {
        self.dry_mass_kg + self.motor.current_mass_at(time_s)
    }

    /// Computes aerodynamic drag force in Newtons.
    /// Notice the sign: opposes velocity!
    pub fn drag_force(&self, velocity_m_s: f64) -> f64 {
        let area = self.cross_sectional_area();
        // -0.5 * rho * |v| * v * Cd * A
        -0.5 * AIR_DENSITY_SEA_LEVEL
            * velocity_m_s.abs()
            * velocity_m_s
            * self.drag_coefficient
            * area
    }

    /// Net vertical force acting on the rocket in Newtons (Up is positive).
    pub fn net_force(&self, time_s: f64, velocity_m_s: f64) -> f64 {
        let mass = self.total_mass_at(time_s);
        let thrust = self.motor.thrust_at(time_s);
        let gravity_force = mass * GRAVITY;
        let drag = self.drag_force(velocity_m_s);

        thrust - gravity_force + drag
    }

    /// Acceleration = F_net / mass (m/s^2)
    pub fn acceleration(&self, time_s: f64, velocity_m_s: f64) -> f64 {
        let mass = self.total_mass_at(time_s);
        self.net_force(time_s, velocity_m_s) / mass
    }
}
