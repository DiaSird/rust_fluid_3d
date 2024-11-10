use super::parameters::{Particle, DIM};
use anyhow::{bail, Ok, Result};

pub fn update_acceleration(particles: &mut [Particle<DIM>]) -> Result<()> {
    for particle in particles.iter_mut() {
        if particle.dvdt[0] < 0.0 {
            bail!("ax cannot be negative: {}", particle.dvdt[0]);
        }
    }
    Ok(())
}
