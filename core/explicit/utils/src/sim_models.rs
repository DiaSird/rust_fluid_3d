use crate::{
    error::{FailedFeatureReadFileSnafu, FailedWriteFileSnafu, SimError},
    parameters::{DIM, ModelScale, Particle, Resolution},
};
use csv::ReaderBuilder;
use serde::Deserialize;
use snafu::ResultExt as _;

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
fn read_air_space(file_path: &std::path::Path) -> Result<Vec<AirSpace>, SimError> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .with_context(|_| FailedFeatureReadFileSnafu {
            path: file_path.to_path_buf(),
        })?;

    let mut air_space_data = Vec::new();

    for result in rdr.deserialize() {
        let record: AirSpace = result.with_context(|_| FailedFeatureReadFileSnafu {
            path: file_path.to_path_buf(),
        })?;
        air_space_data.push(record);
    }

    Ok(air_space_data)
}

// Function to read airfoil data
fn read_airfoil_data(file_path: &std::path::Path) -> Result<Vec<Airfoil>, SimError> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .with_context(|_| FailedFeatureReadFileSnafu {
            path: file_path.to_path_buf(),
        })?;

    let mut airfoil_data = Vec::new();
    for result in rdr.deserialize() {
        let record: Airfoil = result.with_context(|_| FailedFeatureReadFileSnafu {
            path: file_path.to_path_buf(),
        })?;
        airfoil_data.push(record);
    }

    Ok(airfoil_data)
}

// Generate particles in a 3D grid
fn sim_model(
    particles: &mut [Particle<3>],
    airfoil_data: &[Airfoil],
    air_space_data: &[AirSpace],
) -> Result<usize, SimError> {
    let n = airfoil_data.len() + air_space_data.len();

    if n > particles.len() {
        return Err(SimError::ExceededMaxNumber {
            n,
            max_n: particles.len(),
        });
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
fn write_coordinates_to_csv(particles: &[Particle<3>]) -> Result<(), SimError> {
    let filename = std::path::Path::new("./results/model_particles.csv");
    let mut csv = String::new();

    // Write CSV header
    csv.push_str("num,x,y,z\n");

    // Write the coordinates of the particles
    for (i, particle) in particles.iter().enumerate() {
        let (x, y, z) = particle.axis();
        csv.push_str(&format!("{}, {x:.3},{y:.3},{z:.3}\n", i + 1));
    }

    // Write CSV file
    std::fs::write(filename, &csv).with_context(|_| FailedWriteFileSnafu {
        path: filename.to_path_buf(),
    })?;

    Ok(())
}

// Templates: Box Fluid
fn make_box_model(
    particles: &mut [Particle<DIM>],
    model_scale: &ModelScale,
    resolution: &Resolution,
) -> Result<usize, SimError> {
    // Particle counter, starts from 0
    let mut n = 0;

    let (nx, ny, nz) = (
        (model_scale.length / resolution.dx) as usize,
        (model_scale.width / resolution.dy) as usize,
        (model_scale.height / resolution.dz) as usize,
    );

    for i in 0..=nx {
        for j in 0..=ny {
            for k in 0..=nz {
                // Check if we exceed the maximum number of particles
                if n >= particles.len() {
                    return Err(SimError::ExceededMaxNumber {
                        n,
                        max_n: particles.len(),
                    });
                }

                // Set particle location
                particles[n].x[0] = i as f64 * resolution.dx; // x
                particles[n].x[1] = j as f64 * resolution.dy; // y
                particles[n].x[2] = k as f64 * resolution.dz; // z

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
pub fn make_model(
    model: &str,
    particles: &mut [Particle<DIM>],
    model_scale: &ModelScale,
    resolution: &Resolution,
) -> Result<usize, SimError> {
    if model == "csv" {
        // Read air_space.csv file
        let csv_path = std::path::Path::new("src/models/air_space.csv");
        let air_space_data = read_air_space(csv_path)?;

        // Read airfoil data file
        let csv_path = std::path::Path::new("src/models/naca_2412_3d_airfoil.csv");
        let airfoil_data = read_airfoil_data(csv_path)?;

        // Setting model
        let n = sim_model(particles, &airfoil_data, &air_space_data)?;

        // Write particle coordinates to CSV
        write_coordinates_to_csv(&particles[0..n])?;

        Ok(n)
    } else {
        // Default: Box
        make_box_model(particles, model_scale, resolution)
    }
}
