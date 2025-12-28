use super::{
    parameters::{NeighboringList as Neighbor, Particle, DIM},
    sph_utils::{SphDiff, Tensor},
};
use anyhow::{bail, Context, Result};
use nalgebra as na;
use rayon::prelude::*;

pub fn update_acceleration(
    particles: &mut [Particle<DIM>],
    neighbors: &mut [Neighbor<DIM>],
    diff_stress: &mut [Tensor<DIM>],
) -> Result<()> {
    let n = particles.len();

    // Thread-local buffer for dv/dt
    let dvdt_buf: Vec<na::Vector3<f64>> = vec![na::Vector3::zeros(); n];
    let dvdt_buf = std::sync::Arc::new(std::sync::RwLock::new(dvdt_buf));

    diff_stress
        .par_iter_mut()
        .enumerate()
        .try_for_each(|(i, stress)| -> Result<()> {
            // Calculate div(stress)
            stress.sph_div(particles, neighbors, i).with_context(|| {
                format!(
                    "Failed: div-stress in updating acceleration for particle {}",
                    i
                )
            })?;

            let dvdt = na::Vector3::from(stress.div_tensor) / particles[i].rho;

            // store into thread-safe buffer
            {
                let mut buf = dvdt_buf.write().unwrap();
                buf[i] = dvdt;
            }

            Ok(())
        })?;

    // merge buffer into particles
    let buf = dvdt_buf.read().unwrap();
    for (i, dv) in buf.iter().enumerate() {
        if dv.dot(dv).is_nan() {
            bail!("dv/dt has NaN on particle {}", i);
        }
        particles[i].dvdt = (*dv).into();
    }

    Ok(())
}
