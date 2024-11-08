use super::sph::Particle;
use anyhow::{bail, Ok, Result};

pub fn search_near_particles(particles: &mut [Particle]) -> Result<()> {
    for particle in particles.iter_mut() {
        if particle.volume < 0.0 {
            bail!("volume cannot be negative: {}", particle.volume);
        }
    }
    Ok(())
}
