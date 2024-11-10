use super::{
    parameters::{NeighboringList, Particle, DIM},
    sph_utils::Velocity,
};
use crate::explicit::sph_utils::SphDiff;
use anyhow::{Context, Result};

pub fn update_density(
    dt: f64,
    particles: &mut [Particle<DIM>],
    neighbors: &mut [NeighboringList<DIM>],
    velocity: &mut [Velocity<DIM>],
) -> Result<()> {
    // Total particles
    let n = particles.len();

    // Calculate div(velocity)
    for (i, v) in velocity.iter_mut().enumerate().take(n) {
        v.sph_div(particles, neighbors, i)
            .context("Failed: div-v")?;
    }

    // update: rho = -rho * div(velocity) * dt
    for (i, v) in velocity.iter_mut().enumerate().take(n) {
        particles[i].rho += -particles[i].rho * v.div_v * dt;
        // dbg!(particles[i].rho);
    }

    Ok(())
}
