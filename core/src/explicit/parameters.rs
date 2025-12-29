// -------------------------------------------------------
//  COnstant and Global parameters
// -------------------------------------------------------
use nalgebra::{self as na, SimdComplexField};
use serde::{Deserialize, Serialize};

// Max parameters
// pub const MAX_N: usize = 1400;  // Max total particles
/// Max total particles
pub const MAX_N: usize = 60000;
/// Max nearing particles
pub const MAX_NEAR_N: usize = 100;
pub const MAX_NEAR_SUM: usize = MAX_N * MAX_NEAR_N;

// SPH parameters
/// Smooth length [m]
// pub const SMOOTH_LENGTH: f64 = 0.1;
pub const SMOOTH_LENGTH: f64 = 0.0324;
/// CLL Cell size[m]
pub const CELL_SIZE: f64 = 2.0 * SMOOTH_LENGTH;
/// artificial viscosity
// pub const BETA: f64 = 1.0;
pub const BETA: f64 = 0.3;
/// conservative smoothing rate *100 [%]
pub const CS_RATE: f64 = 0.05;

// MOdel config
// Dimension
pub const DIM: usize = 3;
// x-axis [m]
pub const LENGTH: f64 = 0.5;
// y-axis [m]
pub const WIDTH: f64 = 0.5;
// z-xis [m]
pub const HEIGHT: f64 = 0.5;

// Resolution
// x-axis [m]
// pub const DX: f64 = 0.1;
pub const DX: f64 = 0.027;
// y-axis [m]
// pub const DY: f64 = 0.1;
pub const DY: f64 = 0.027;
// z-axis [m]
// pub const DZ: f64 = 0.1;
pub const DZ: f64 = 0.027;

pub const NX: usize = (LENGTH / DX) as usize;
pub const NY: usize = (WIDTH / DY) as usize;
pub const NZ: usize = (HEIGHT / DZ) as usize;

// Material information
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Fluid {
    Water,
    Air,
}

/// type arias
type Vector<const DIM: usize> =
    na::Matrix<f64, na::Const<DIM>, na::U1, na::ArrayStorage<f64, DIM, 1>>;

type Matrix<const DIM: usize> =
    na::Matrix<f64, na::Const<DIM>, na::Const<DIM>, na::ArrayStorage<f64, DIM, DIM>>;

// Particle information
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

// SPH Neighboring List
#[derive(Debug, PartialEq)]
pub struct NeighboringList<const DIM: usize> {
    pub i: usize, // pair i
    pub j: usize, // pair j
    pub w: f64,
    pub dwdr: Vector<DIM>,
}

impl<const DIM: usize> NeighboringList<DIM> {
    pub fn new() -> Self {
        Self {
            i: 0,
            j: 0,
            w: 0.0,
            dwdr: Vector::zeros(),
        }
    }

    pub fn kernel_axis3(&self) -> (f64, f64, f64) {
        let dwdr1 = self.dwdr[0];
        let dwdr2 = self.dwdr[1];
        let dwdr3 = self.dwdr[2];
        (dwdr1, dwdr2, dwdr3)
    }
}
