use super::{
    parameters::{DX, DY, DZ, NX, NY, NZ},
    sph::Particle,
};
use anyhow::{bail, Context, Ok, Result};

pub fn make_model(particles: &mut [Particle]) -> Result<usize> {
    // Particle counter, starts from 0
    let mut n = 0;

    for i in 0..=NX {
        for j in 0..=NY {
            for k in 0..=NZ {
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
    // write_coordinates_to_csv(&particles[0..n])?;
    Ok(n)
}

// Write only the particles created
#[allow(unused)]
fn write_coordinates_to_csv(particles: &[Particle]) -> Result<()> {
    let filename = "./results/particles_coordinates.csv";
    let mut csv = String::new();

    // CSV header
    csv.push_str("num,x,y,z\n");

    // Output coordinates of the particles created in `make_model`
    for (i, particle) in particles.iter().enumerate() {
        let (x, y, z) = particle.axis();
        csv.push_str(&format!("{}, {x:.3},{y:.3},{z:.3}\n", i + 1));
    }

    std::fs::write(filename, &csv).context("Failed to create CSV file")?;
    Ok(())
}
