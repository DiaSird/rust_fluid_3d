use super::parameters::Particle;
use anyhow::{bail, Ok, Result};

pub fn update_velocity(particles: &mut [Particle]) -> Result<()> {
    for particle in particles.iter_mut() {
        if particle.v[0] < 0.0 {
            bail!("vx cannot be negative: {}", particle.v[0]);
        }
    }
    Ok(())
}
