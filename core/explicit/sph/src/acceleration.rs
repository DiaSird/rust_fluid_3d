use super::sph_utils::{SphDiff, Tensor};
use anyhow::{Context, Result, bail};
use nalgebra as na;
use rayon::prelude::*;
use utils::parameters::{DIM, NeighboringList as Neighbor, Particle};

pub(crate) fn update_acceleration(
    particles: &mut [Particle<DIM>],
    neighbors: &[Neighbor<DIM>],
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
                #[allow(clippy::unwrap_used)]
                let mut buf = dvdt_buf.write().unwrap();
                buf[i] = dvdt;
            }

            Ok(())
        })?;

    // merge buffer into particles
    #[allow(clippy::unwrap_used, clippy::unwrap_in_result)]
    for (i, dv) in dvdt_buf.read().unwrap().iter().enumerate() {
        if dv.dot(dv).is_nan() {
            bail!("dv/dt has NaN on particle {}", i);
        }
        particles[i].dvdt = *dv;
    }

    Ok(())
}
