use crate::parameters::{LogReporterFn, ParticleLog};

use super::parameters::{DIM, Particle};
use super::rw_checkpoint;
use anyhow::{Context, Ok, Result};
use nalgebra as na;

/// # Errors
pub fn write_result(step: usize, particles: &[Particle<DIM>]) -> Result<()> {
    // Velocity
    write_velocity_to_csv(step, particles)?;
    // TODO: Energy and Temperature
    Ok(())
}

/// # Errors
pub fn write_velocity_to_csv(step: usize, particles: &[Particle<DIM>]) -> Result<()> {
    let filename = format!("./results/result_{}.csv", step);
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

    std::fs::write(filename, &csv).context("Failed to create CSV file")?;
    Ok(())
}

/// # Errors
/// let log = format!(
///     "------------------------------------------\n\
///  Step {}, time = {:.3} [ms]\n\
///      Particle: {}\n\
///      (x, y, z) = {:.3}, {:.3}, {:.3}\n\
///      (vx, vy, vz) = {:.3}, {:.3}, {:.3}\n\
///      (ax, ay, az) = {:.3}, {:.3}, {:.3}\n\
/// ------------------------------------------",
///     step,
///     time * 1000.0,
///     i,
///     x,
///     y,
///     z,
///     vx,
///     vy,
///     vz,
///     ax,
///     ay,
///     az
/// );
pub fn display_result(
    monitor_particle: usize,
    status: &LogReporterFn,
    step: usize,
    time: f64,
    particles: &[Particle<DIM>],
) {
    let x: [f64; DIM] = particles[monitor_particle].axis().into();
    let v: [f64; DIM] = particles[monitor_particle].velocity().into();
    let dvdt: [f64; DIM] = particles[monitor_particle].accel().into();

    let x = na::Vector3::from(x);
    let v = na::Vector3::from(v);
    let dvdt = na::Vector3::from(dvdt);

    #[rustfmt::skip]
    status(ParticleLog::Info3 { step, time, monitor_particle, x, v, dvdt });
}

/// # Errors
pub fn write_sim_checkpoint(
    out_file: impl AsRef<std::path::Path>,
    step: usize,
    time: f64,
    dt: f64,
    n: usize,
    particles: &[Particle<DIM>],
) -> Result<()> {
    let state = rw_checkpoint::State {
        step,
        time,
        dt,
        n,
        particles: particles[0..n].to_vec(),
    };
    rw_checkpoint::write_checkpoint(out_file, &state, 1024 * 10000)?;
    Ok(())
}
