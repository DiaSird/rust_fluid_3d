use crate::{
    error::{FailedWriteFileSnafu, SimError},
    parameters::{LogReporterFn, ParticleLog},
};

use super::parameters::{DIM, Particle};
use nalgebra as na;
use snafu::ResultExt as _;

/// # Errors
pub fn write_result(step: usize, particles: &[Particle<DIM>]) -> Result<(), SimError> {
    // Velocity
    write_velocity_to_csv(step, particles)?;
    // TODO: Energy and Temperature
    Ok(())
}

/// # Errors
#[allow(unused)]
pub fn write_velocity_to_csv(step: usize, particles: &[Particle<DIM>]) -> Result<(), SimError> {
    let filename = std::path::PathBuf::from(format!("./results/result_{}.csv", step));
    let mut csv = String::new();

    // CSV header
    csv.push_str("i,x,y,z,vx,vy,vz,ax,ay,az\n");

    // Output coordinates of the particles created in `make_model`
    for (i, particle) in particles.iter().enumerate() {
        let (x, y, z) = particle.axis();
        let (vx, vy, vz) = particles[i].velocity();
        let (ax, ay, az) = particles[i].accel();

        csv.push_str(&format!(
            "{i},{x:.3},{y:.3},{z:.3},{vx:.3},{vy:.3},{vz:.3},{ax:.3},{ay:.3},{az:.3}\n",
        ));
    }

    std::fs::write(&filename, &csv).with_context(|_| FailedWriteFileSnafu { path: filename })?;
    Ok(())
}

/// # Errors
pub fn display_result(monitor_particle: usize, status: &LogReporterFn, step: usize, time: f64, particles: &[Particle<DIM>]) {
    let x: [f64; DIM] = particles[monitor_particle].axis().into();
    let v: [f64; DIM] = particles[monitor_particle].velocity().into();
    let dvdt: [f64; DIM] = particles[monitor_particle].accel().into();

    let x = na::Vector3::from(x);
    let v = na::Vector3::from(v);
    let dvdt = na::Vector3::from(dvdt);

    #[rustfmt::skip]
    status(ParticleLog::Info3 { step, time, monitor_particle, x, v, dvdt });
}
