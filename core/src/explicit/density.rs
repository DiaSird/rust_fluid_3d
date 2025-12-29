use super::{
    parameters::{DIM, NeighboringList as Neighbor, Particle},
    sph_utils::Velocity,
};
use crate::explicit::sph_utils::SphDiff;
use anyhow::{Context, Result};
use rayon::prelude::*;

pub fn update_density(
    dt: f64,
    particles: &mut [Particle<DIM>],
    neighbors: &[Neighbor<DIM>],
    diff_velocity: &mut [Velocity<DIM>],
) -> Result<()> {
    // Total particles
    let n = particles.len();

    // Calculate div(velocity)
    diff_velocity[..n]
        .par_iter_mut()
        .enumerate()
        .try_for_each(|(i, v)| {
            v.sph_div(particles, neighbors, i)
                .context("Failed: div-v in updating density")
        })?;

    // update: rho = -rho * div(velocity) * dt
    particles[..n]
        .par_iter_mut()
        .zip(diff_velocity[..n].par_iter())
        .for_each(|(p, v)| {
            p.rho += -p.rho * v.div_v * dt;
        });

    Ok(())
}
