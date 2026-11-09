mod motor;
mod rocket;
mod simulation;

use motor::SolidMotor;
use rocket::Rocket;
use simulation::FlightState;

fn main() {
    println!("=== Launching Simulated Rocket Flight ===");

    let motor = SolidMotor::new(0.060, 0.050, 2.0, 15.0);
    let rocket = Rocket::new(0.350, 0.45, 0.050, motor);

    let mut state = FlightState::new();
    let dt = 0.01; // 100 Hz simulation rate (10ms steps)

    let mut max_alt = 0.0;
    let mut apogee_time = 0.0;
    let mut max_vel = 0.0;
    let mut max_accel = 0.0;
    let mut burnout_detected = false;

    // Run simulation loop
    while state.time_s < 20.0 {
        state.step(&rocket, dt);

        // Track peak stats
        if state.altitude_m > max_alt {
            max_alt = state.altitude_m;
            apogee_time = state.time_s;
        }
        if state.velocity_m_s > max_vel {
            max_vel = state.velocity_m_s;
        }
        if state.acceleration_m_s2 > max_accel {
            max_accel = state.acceleration_m_s2;
        }

        // Detect motor burnout
        if !burnout_detected && state.time_s >= 2.0 {
            burnout_detected = true;
            println!(
                "[EVENT] T+{:>4.2}s: Motor Burnout! Alt: {:>5.1}m, Vel: {:>5.1} m/s",
                state.time_s, state.altitude_m, state.velocity_m_s
            );
        }

        // Print telemetry every 0.5 seconds
        if (state.time_s * 100.0).round() as i64 % 50 == 0 && state.time_s <= apogee_time + 0.1 {
            println!(
                "T+{:>4.2}s | Alt: {:>6.1} m | Vel: {:>5.1} m/s | Accel: {:>5.1} m/s^2",
                state.time_s, state.altitude_m, state.velocity_m_s, state.acceleration_m_s2
            );
        }

        // Break once it starts falling back down past apogee
        if burnout_detected && state.velocity_m_s <= 0.0 {
            println!(
                "[EVENT] T+{:>4.2}s: APOGEE REACHED! Peak Altitude: {:>5.1} m",
                apogee_time, max_alt
            );
            break;
        }
    }

    println!("\n=== Flight Summary ===");
    println!(
        "Max Acceleration: {:>5.1} m/s^2 ({:>4.1} Gs)",
        max_accel,
        max_accel / 9.80665
    );
    println!(
        "Max Velocity:     {:>5.1} m/s ({:>5.1} km/h)",
        max_vel,
        max_vel * 3.6
    );
    println!("Apogee Altitude:  {:>5.1} meters", max_alt);
    println!("Time to Apogee:   {:>5.2} seconds", apogee_time);
}
