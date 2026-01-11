use super::parameters::{DIM, DX, DY, DZ, MAX_N, NX, NY, NZ, Particle};
use anyhow::{Context, Result, bail};
use csv::ReaderBuilder;
use serde::Deserialize;

// Structure to hold air data
#[derive(Debug, Deserialize)]
struct AirSpace {
    x: f64,
    y: f64,
    z: f64,
}

// Structure to hold airfoil data
#[derive(Debug, Deserialize)]
struct Airfoil {
    x: f64,
    y: f64,
    z: f64,
}

// Function to read air space
fn read_air_space(file_path: &str) -> Result<Vec<AirSpace>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .context(format!("Failed to read file: {}", file_path))?;

    let mut air_space_data = Vec::new();

    for result in rdr.deserialize() {
        let record: AirSpace =
            result.map_err(|err| anyhow::anyhow!("Error deserializing air space data: {}", err))?;
        air_space_data.push(record);
    }

    Ok(air_space_data)
}

// Function to read airfoil data
fn read_airfoil_data(file_path: &str) -> Result<Vec<Airfoil>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .context(format!("Failed to read file: {}", file_path))?;

    let mut airfoil_data = Vec::new();

    for result in rdr.deserialize() {
        let record: Airfoil =
            result.map_err(|err| anyhow::anyhow!("Error deserializing airfoil data: {}", err))?;
        airfoil_data.push(record);
    }

    Ok(airfoil_data)
}

// Generate particles in a 3D grid
fn sim_model(
    particles: &mut [Particle<3>],
    airfoil_data: &[Airfoil],
    air_space_data: &[AirSpace],
) -> Result<usize> {
    let n = airfoil_data.len() + air_space_data.len();

    if n > MAX_N {
        bail!("Particle index is out of range: {} < {}", MAX_N, n);
    }

    for (i, airfoil) in airfoil_data.iter().enumerate() {
        if i < airfoil_data.len() - 1 {
            particles[i].x[0] = airfoil.x;
            particles[i].x[1] = airfoil.y;
            particles[i].x[2] = airfoil.z;
        }
    }

    for (i, air_space) in air_space_data.iter().enumerate() {
        let i2 = i + airfoil_data.len();

        if i2 < n {
            particles[i2].x[0] = air_space.x;
            particles[i2].x[1] = air_space.y;
            particles[i2].x[2] = air_space.z;
        }
    }

    // Assign volume evenly across all particles
    for particle in particles.iter_mut().take(n) {
        particle.volume /= n as f64;
    }

    Ok(n)
}

// Function to write particle coordinates to a CSV file
fn write_coordinates_to_csv(particles: &[Particle<3>]) -> Result<()> {
    let filename = "./results/model_particles.csv";
    let mut csv = String::new();

    // Write CSV header
    csv.push_str("num,x,y,z\n");

    // Write the coordinates of the particles
    for (i, particle) in particles.iter().enumerate() {
        let (x, y, z) = particle.axis();
        csv.push_str(&format!("{}, {x:.3},{y:.3},{z:.3}\n", i + 1));
    }

    // Write CSV file
    std::fs::write(filename, &csv).context(format!("Failed to write to file: {}", filename))?;

    Ok(())
}

// Templates: Box Fluid
fn make_box_model(particles: &mut [Particle<DIM>]) -> Result<usize> {
    // Particle counter, starts from 0
    let mut n = 0;

    for i in 0..=NX {
        for j in 0..=NY {
            for k in 0..=NZ {
                // Check if we exceed the maximum number of particles
                if n >= particles.len() {
                    bail!(
                        "Exceeded the maximum number of particles >= {}.",
                        particles.len()
                    );
                }

                // Set particle location
                particles[n].x[0] = i as f64 * DX; // x
                particles[n].x[1] = j as f64 * DY; // y
                particles[n].x[2] = k as f64 * DZ; // z

                n += 1; // Increment particle counter
            }
        }
    }

    for particle in particles.iter_mut().take(n) {
        particle.volume /= n as f64;
    }

    // Debug
    // write_coordinates_to_csv(&particles[0..n]).context("Failed to write particle coordinates")?;

    Ok(n)
}

/// # Errors
// Making simulation models
pub fn make_model(model: &str, particles: &mut [Particle<DIM>]) -> Result<usize> {
    if model == "csv" {
        // Read air_space.csv file
        let air_space_data =
            read_air_space("src/models/air_space.csv").context("Failed to read air space data")?;

        // Read airfoil data file
        let airfoil_data = read_airfoil_data("src/models/naca_2412_3d_airfoil.csv")
            .context("Failed to read airfoil data")?;

        // Setting model
        let n = sim_model(particles, &airfoil_data, &air_space_data)
            .context("Failed to create model")?;

        // Write particle coordinates to CSV
        write_coordinates_to_csv(&particles[0..n])
            .context("Failed to write particle coordinates")?;

        Ok(n)
    } else {
        // Default: Box
        make_box_model(particles)
    }
}
