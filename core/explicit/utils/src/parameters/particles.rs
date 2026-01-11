use crate::parameters::{
    Fluid, Matrix, Vector,
    consts::{HEIGHT, LENGTH, WIDTH},
};
use nalgebra::SimdComplexField;

// Particle information
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Particle<const DIM: usize> {
    // SPH parameters
    pub pair: usize, // pair numbers per one particles
    pub volume: f64, // [m^3]

    // physical quantity for fluid
    /// initial density [kg/m^3]
    pub rho0: f64,
    /// density [kg/m^3]
    pub rho: f64,
    /// viscosity [Pa*s]
    pub viscosity: f64,
    /// sound velocity [m/s]
    pub sound_v: f64,
    /// location vector [m]
    pub x: Vector<DIM>,
    /// velocity [m/s]
    pub v: Vector<DIM>,
    /// Cauthy stress [Pa]
    pub stress: Matrix<DIM>,
    /// acceleration [m/s^2]
    pub dvdt: Vector<DIM>,
    /// total energy [J]
    pub e: f64,
    /// power [J/s]
    pub dedt: f64,
    /// Temperature [K]
    pub temperature: f64,
    /// Fluid type (Water, Air, etc.)
    pub fluid: Fluid,
}

impl<const DIM: usize> Particle<DIM> {
    pub fn new(fluid: Fluid) -> Self {
        // Initial temperature and sound speed
        let temperature: f64 = 273.15 + 20.0;
        let sound_air = 0.6_f64.mul_add(temperature - 273.15, 331.3);
        let sound_water = 0.057_f64.mul_add(
            -(temperature - 273.15).simd_powf(2.0),
            5.04_f64.mul_add(temperature - 273.15, 1402.4),
        );

        // Fluid properties
        let (rho, viscosity, sound_v) = match fluid {
            Fluid::Water => (1000.0, 0.001, sound_water),
            // Fluid::Water => (1000.0, 1.0, sound_water),
            Fluid::Air => (1.225, 0.0000181, sound_air),
        };

        // initial value
        let rho0 = rho;

        // set a new particle
        Self {
            pair: 0,
            volume: LENGTH * WIDTH * HEIGHT,
            rho0,
            rho,
            viscosity,
            sound_v,
            x: Vector::<DIM>::zeros(),
            v: Vector::<DIM>::zeros(),
            stress: Matrix::<DIM>::zeros(),
            dvdt: Vector::<DIM>::zeros(),
            e: 0.0,
            dedt: 0.0,
            temperature,
            fluid,
        }
    }

    pub fn axis(&self) -> (f64, f64, f64) {
        let x = self.x[0];
        let y = self.x[1];
        let z = self.x[2];
        (x, y, z)
    }

    pub fn velocity(&self) -> (f64, f64, f64) {
        let vx = self.v[0];
        let vy = self.v[1];
        let vz = self.v[2];
        (vx, vy, vz)
    }

    pub fn accel(&self) -> (f64, f64, f64) {
        let ax = self.dvdt[0];
        let ay = self.dvdt[1];
        let az = self.dvdt[2];
        (ax, ay, az)
    }
}
