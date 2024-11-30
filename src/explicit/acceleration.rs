use super::{
    parameters::{NeighboringList as Neighbor, Particle, DIM},
    sph_utils::{SphDiff, Tensor},
};
use anyhow::{bail, Context, Ok, Result};
use nalgebra as na;

pub fn update_acceleration(
    particles: &mut [Particle<DIM>],
    neighbors: &mut [Neighbor<DIM>],
    diff_stress: &mut [Tensor<DIM>],
) -> Result<()> {
    // Total particles
    let n = particles.len();

    for (i, stress) in diff_stress.iter_mut().enumerate().take(n) {
        // Calculate div(stress)
        stress
            .sph_div(particles, neighbors, i)
            .context("Failed: div-stress in updating acceleration")?;

        // Navier-Stokes Eqs.
        for d in 0..DIM {
            particles[i].dvdt[d] = stress.div_tensor[d] / particles[i].rho;
        }

        let dvdt = na::Vector3::from(particles[i].dvdt);
        if dvdt.dot(&dvdt).is_nan() {
            bail!("dv/dt has nan on particle {}", i);
        }
    }

    Ok(())
}
