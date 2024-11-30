use super::parameters::{Particle, DIM};
use anyhow::{bail, Ok, Result};

pub fn update_half_velocity(dt: f64, particles: &mut [Particle<DIM>]) -> Result<()> {
    for particle in particles.iter_mut() {
        for i in 0..DIM {
            // half increment
            particle.v[i] += 0.5 * particle.dvdt[i] * dt;

            if particle.v[i].is_nan() {
                bail!("None value is detected: v{}", i);
            }
        }
    }
    Ok(())
}

pub fn update_location(dt: f64, particles: &mut [Particle<DIM>]) -> Result<()> {
    for particle in particles.iter_mut() {
        for i in 0..DIM {
            // increment
            particle.x[i] += particle.v[i] * dt;

            if particle.x[i].is_nan() {
                bail!("None value is detected: x{}", i);
            }
        }
    }
    Ok(())
}
