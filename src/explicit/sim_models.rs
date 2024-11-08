use super::{
    parameters::{DX, DY, DZ, HEIGHT, LENGTH, WIDTH},
    sph::Particle,
};
use anyhow::{bail, Context, Ok, Result};
use std::fs::File;
use std::io::Write;

pub fn make_model(particles: &mut [Particle]) -> Result<usize> {
    let nx: usize = (LENGTH / DX) as usize;
    let ny: usize = (WIDTH / DY) as usize;
    let nz: usize = (HEIGHT / DZ) as usize;

    // Particle counter, starts from 0
    let mut n = 0;

    for i in 0..=nx {
        for j in 0..=ny {
            for k in 0..=nz {
                // Check if we exceed the maximum number of particles
                if n >= particles.len() {
                    bail!("Exceeded the maximum number of particles.");
                }

                // Set particle location
                particles[n].x[0] = i as f64 * DX; // x
                particles[n].x[1] = j as f64 * DY; // y
                particles[n].x[2] = k as f64 * DZ; // z

                n += 1; // Increment particle counter
            }
        }
    }
    write_coordinates_to_csv(&particles[0..n])?;
    Ok(n)
}

// Write only the particles created
fn write_coordinates_to_csv(particles: &[Particle]) -> Result<()> {
    let filename = "./results/particles_coordinates.csv";
    let mut file = File::create(filename).context("Failed to create CSV file")?;

    // CSV header
    writeln!(file, "num,x,y,z").context("Failed to write header to CSV")?;

    // Output coordinates of the particles created in `make_model`
    for (i, particle) in particles.iter().enumerate() {
        writeln!(
            file,
            "{},{:.3},{:.3},{:.3}",
            i + 1,
            particle.x[0],
            particle.x[1],
            particle.x[2]
        )
        .context("Failed to write particle coordinates to CSV")?;
    }

    Ok(())
}
