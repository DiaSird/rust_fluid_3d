use crate::sph_utils::{SphDiff, Velocity};
use rayon::prelude::*;
use utils::{
    error::SimError,
    parameters::{DIM, NeighboringList as Neighbor, Particle},
};

pub(crate) fn update_density(
    dt: f64,
    particles: &mut [Particle<DIM>],
    neighbors: &[Neighbor<DIM>],
    diff_velocity: &mut [Velocity<DIM>],
) -> Result<(), SimError> {
    // Total particles
    let n = particles.len();

    // Calculate div(velocity)
    diff_velocity[..n]
        .par_iter_mut()
        .enumerate()
        .try_for_each(|(i, v)| v.sph_div(particles, neighbors, i))?;

    // update: rho = -rho * div(velocity) * dt
    particles[..n]
        .par_iter_mut()
        .zip(diff_velocity[..n].par_iter())
        .for_each(|(p, v)| {
            p.rho += -p.rho * v.div_v * dt;
        });

    Ok(())
}
