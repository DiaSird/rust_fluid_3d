use super::parameters::{Particle, DIM};
use anyhow::{Ok, Result};

pub fn boundary_condition(particles: &mut [Particle<DIM>]) -> Result<()> {
    for particle in particles.iter_mut() {
        if particle.x[0] < 0.1 {
            particle.v[0] = 5.0;
        }
    }
    Ok(())
}
