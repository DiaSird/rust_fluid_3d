use super::sph_utils::{SphDiff, Tensor};
use nalgebra as na;
use rayon::prelude::*;
use utils::{
    error::{SimError, check_nan_to_error},
    parameters::{DIM, NeighboringList as Neighbor, Particle},
};

pub(crate) fn update_acceleration(
    particles: &mut [Particle<DIM>],
    neighbors: &[Neighbor<DIM>],
    diff_stress: &mut [Tensor<DIM>],
) -> Result<(), SimError> {
    let n = particles.len();

    // Thread-local buffer for dv/dt
    let dvdt_buf: Vec<na::Vector3<f64>> = vec![na::Vector3::zeros(); n];
    let dvdt_buf = std::sync::Arc::new(std::sync::RwLock::new(dvdt_buf));

    diff_stress
        .par_iter_mut()
        .enumerate()
        .try_for_each(|(i, stress)| -> Result<(), SimError> {
            // Calculate div(stress)
            stress.sph_div(particles, neighbors, i)?;

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
        check_nan_to_error(i, dv.dot(dv))?;
        particles[i].dvdt = *dv;
    }

    Ok(())
}
