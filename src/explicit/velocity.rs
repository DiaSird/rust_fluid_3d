use super::parameters::{Particle, DIM};
use anyhow::{bail, Context, Ok, Result};

pub fn update_velocity(particles: &mut [Particle<DIM>]) -> Result<()> {
    for particle in particles.iter_mut() {
        if particle.v[0] < 0.0 {
            bail!("vx cannot be negative: {}", particle.v[0]);
        }
    }
    write_velocity_to_csv(particles)
}

pub fn write_velocity_to_csv(particles: &[Particle<DIM>]) -> Result<()> {
    let filename = "./results/result.csv";
    let mut csv = String::new();

    // CSV header
    csv.push_str("i,x,y,z,vx,vy,vz\n");

    // Output coordinates of the particles created in `make_model`
    for (i, particle) in particles.iter().enumerate() {
        let (x, y, z) = particle.axis();
        let (vx, vy, vz) = (particle.v[0], particle.v[1], particle.v[2]);

        csv.push_str(&format!(
            "{i}{x:.3},{y:.3},{z:.3},{vx:.3},{vy:.3},{vz:.3}\n",
        ));
    }

    std::fs::write(filename, &csv).context("Failed to create CSV file")?;
    Ok(())
}
