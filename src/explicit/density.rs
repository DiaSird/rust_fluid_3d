use super::sph::Particle;
use anyhow::{bail, Ok, Result};

pub fn update_density(particles: &mut [Particle]) -> Result<()> {
    for particle in particles.iter_mut() {
        particle.rho += 100.0;
        if particle.rho < 0.0 {
            bail!("rho cannot be negative: {}", particle.rho);
        }
    }
    Ok(())
}
