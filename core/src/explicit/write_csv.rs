use super::parameters::{DIM, Particle};
use super::rw_checkpoint;
use anyhow::{Context, Ok, Result};

pub fn write_result(step: usize, particles: &[Particle<DIM>]) -> Result<()> {
    // Velocity
    write_velocity_to_csv(step, particles)?;
    // TODO: Energy and Temperature
    Ok(())
}

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

pub fn display_result(step: usize, time: f64, particles: &[Particle<DIM>]) -> Result<()> {
    // let i: usize = 10;
    let i: usize = 1000;
    let (x, y, z) = particles[i].axis();
    let (vx, vy, vz) = particles[i].velocity();
    let (ax, ay, az) = particles[i].accel();

    println!("------------------------------------------");
    println!("Step {}, time = {:.3} [ms]", step, time * 1000.0);
    println!("    Particle: {}", i);
    println!("      (x, y, z) = {:.3}, {:.3}, {:.3}", x, y, z);
    println!("      (vx, vy, vz) = {:.3}, {:.3}, {:.3}", vx, vy, vz);
    println!("      (ax, ay, az) = {:.3}, {:.3}, {:.3}", ax, ay, az);
    println!("------------------------------------------");

    Ok(())
}

pub fn write_sim_checkpoint(
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
    rw_checkpoint::write_checkpoint(
        "results/checkpoint.bin",
        // &format!("results/checkpoint_{:08}.bin", step),
        &state,
        1024 * 10000,
    )?;
    Ok(())
}
