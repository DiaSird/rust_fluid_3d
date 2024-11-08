use anyhow::{bail, Context, Ok, Result};

// Min-Max
const MAX_N: usize = 1000; // Max total particles
const MAX_NEAR_N: usize = 1000; // Max nearing particles

// config
const DIM: usize = 3; // Dimension
const LENGTH: f64 = 1.0; // x-axis [m]
const WIDTH: f64 = 1.0; // y-axis [m]
const HEIGHT: f64 = 1.0; // z-xis [m]

// Material information
#[derive(Debug)]
pub enum Material {
    Water,
    Air,
}

// Particle information
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
            volume: LENGTH * WIDTH * HEIGHT,
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

    pub fn check_density(&self) -> Result<()> {
        if self.rho == 0.0 {
            anyhow::bail!("Density is zero for material: {:?}", self.material);
        }
        Ok(())
    }
}


pub fn make_model() {}

pub fn search_near_particles() {}

pub fn update_density(particles: &mut [Particle]) -> Result<()> {
    for particle in particles.iter_mut() {
        particle.rho += 100.0;
        if particle.rho < 0.0 {
            bail!("rho cannot be negative: {}", particle.rho);
        }
    }
    Ok(())
}

pub fn update_acceleration() {}

pub fn update_velocity(particles: &mut [Particle]) -> Result<()> {
    for particle in particles.iter_mut() {
        if particle.v[0] < 0.0 {
            bail!("vx cannot be negative: {}", particle.v[0]);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // Initialize
    let mut time = 0.0;
    let dt = 0.001; // step time
    let water = Material::Water;
    let mut water_particles: [Particle; MAX_N] = std::array::from_fn(|_| Particle::new(&water));

    // Simulation Tests
    let mut step: usize = 1;
    let max_step = 100;

    while step <= max_step {
        update_density(&mut water_particles)
            .context("Error updating density for water particle.")?;
        update_velocity(&mut water_particles)
            .context("Error updating velocity for air particle.")?;

        dbg!(time);
        time += dt;
        step += 1;
    }

    dbg!(water_particles[100].rho);
    Ok(())
}
