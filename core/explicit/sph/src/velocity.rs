use rayon::prelude::*;
use utils::parameters::{DIM, Particle};

pub(crate) fn update_half_velocity(
    dt: f64,
    particles: &mut [Particle<DIM>],
) -> Result<(), utils::error::SimError> {
    particles.par_iter_mut().try_for_each(|particle| {
        for i in 0..DIM {
            // half increment
            particle.v[i] += 0.5 * particle.dvdt[i] * dt;
            if particle.v[i].is_nan() {
                return Err(utils::error::SimError::FailedUpdateVelocity);
                // bail!("None value is detected: v{}", i);
            }
        }
        Ok(())
    })
}

pub(crate) fn update_location(
    dt: f64,
    particles: &mut [Particle<DIM>],
) -> Result<(), utils::error::SimError> {
    particles.par_iter_mut().try_for_each(|particle| {
        for i in 0..DIM {
            // increment
            particle.x[i] += particle.v[i] * dt;
            if particle.x[i].is_nan() {
                return Err(utils::error::SimError::FailedUpdateLocation { i });
            }
        }
        Ok(())
    })
}
