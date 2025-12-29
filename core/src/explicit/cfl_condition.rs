use super::parameters::{Particle, DIM, SMOOTH_LENGTH};
use anyhow::{Ok, Result};

pub fn cfl_dt(mut dt: f64, particles: &[Particle<DIM>]) -> Result<f64> {
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

    let abs_v = match (v_max, max_sound_v) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    };

    // Calculate the new time step dt
    let new_dt = match abs_v {
        Some(sum_v) => 0.3 * SMOOTH_LENGTH / sum_v,
        None => dt,
    };

    // the CFL condition
    dt = f64::min(dt, new_dt);

    Ok(dt)
}
