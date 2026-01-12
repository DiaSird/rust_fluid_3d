use super::parameters::{DIM, Particle};

pub fn cfl_dt(dt: f64, particles: &[Particle<DIM>], smooth_length: f64) -> f64 {
    // get max velocity
    let v_max: Option<f64> = particles
        .iter()
        // flat particle velocity
        .flat_map(|particle| particle.v.iter())
        // searching max velocity
        .fold(None, |max, &x| match max {
            Some(current_max) => Some(current_max.max(x)),
            None => Some(x),
        });

    // get max sound-velocity
    let max_sound_v: Option<f64> = particles.iter().fold(None, |max, particle| match max {
        Some(current_max) => Some(current_max.max(particle.sound_v)),
        None => Some(particle.sound_v),
    });

    // Calculate the new time step dt
    let new_dt = match (v_max, max_sound_v) {
        (Some(a), Some(b)) if (a + b) > 0.0 => 0.3 * smooth_length / (a + b),
        _ => dt,
    };

    // the CFL condition
    f64::min(dt, new_dt)
}
