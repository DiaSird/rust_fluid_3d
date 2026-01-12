use rayon::prelude::*;
use utils::{
    error::{SimError, check_nan_to_error},
    parameters::{DIM, Particle},
};

pub(crate) fn update_half_velocity(
    dt: f64,
    particles: &mut [Particle<DIM>],
) -> Result<(), SimError> {
    particles.par_iter_mut().try_for_each(|particle| {
        for i in 0..DIM {
            // half increment
            particle.v[i] += 0.5 * particle.dvdt[i] * dt;
            check_nan_to_error(i, particle.v[i])?;
        }
        Ok(())
    })
}

pub(crate) fn update_location(dt: f64, particles: &mut [Particle<DIM>]) -> Result<(), SimError> {
    particles.par_iter_mut().try_for_each(|particle| {
        for i in 0..DIM {
            // increment
            particle.x[i] += particle.v[i] * dt;
            check_nan_to_error(i, particle.x[i])?;
        }
        Ok(())
    })
}
