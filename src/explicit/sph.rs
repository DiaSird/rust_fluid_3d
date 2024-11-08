use super::parameters::{Material, DIM, HEIGHT, LENGTH, MAX_N, WIDTH};
use crate::explicit::{
    acceleration::update_acceleration, density::update_density,
    neighoring_lists::search_near_particles, sim_models::make_model, velocity::update_velocity,
};
use anyhow::{Context, Ok, Result};

// Particle information
#[derive(Debug, PartialEq)]
pub struct Particle<'a> {
    // SPH parameters
    pub volume: f64, // [m^3]

    // physical quantity for fluid equations
    pub rho: f64,               // density [kg/m^3]
    pub visco: f64,             // viscosity [Pa*s]
    pub x: [f64; DIM],          // location vector [m]
    pub v: [f64; DIM],          // velocity [m/s]
    pub dvdt: [f64; DIM],       // acceleration [m/s^2]
    pub e: f64,                 // total energy [J]
    pub dedt: f64,              // power [J/s]
    pub material: &'a Material, // Material type (Water, Air, etc.)
}

impl<'a> Particle<'a> {
    pub fn new(material: &'a Material) -> Self {
        let (rho, visco) = match material {
            Material::Water => (1000.0, 0.001),
            Material::Air => (1.225, 0.0000181),
        };

        // a new particle
        Particle {
            volume: LENGTH * WIDTH * HEIGHT / MAX_N as f64,
            rho,
            visco,
            x: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
            dvdt: [0.0, 0.0, 0.0],
            e: 0.0,
            dedt: 0.0,
            material,
        }
    }
}

// SPH Main function
pub fn sph(dt: f64, max_step: usize) -> Result<()> {
    // Initialize
    let mut time = 0.0;
    let water = Material::Water;
    let mut water_particles: [Particle; MAX_N] = std::array::from_fn(|_| Particle::new(&water));

    // Simulation Tests
    let n = make_model(&mut water_particles).context("Error model config for water particle.")?;
    search_near_particles(&mut water_particles[0..n])
        .context("Error searching near particles for water particle.")?;

    let mut step: usize = 1;
    while step <= max_step {
        update_density(&mut water_particles[0..n])
            .context("Error updating density for water particle.")?;
        update_acceleration(&mut water_particles[0..n])
            .context("Error updating acceleration for water particle.")?;
        update_velocity(&mut water_particles[0..n])
            .context("Error updating velocity for air particle.")?;

        dbg!(time);
        time += dt;
        step += 1;
    }

    dbg!(water_particles[100].rho);
    Ok(())
}
