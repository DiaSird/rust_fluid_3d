// -------------------------------------------------------
//  COnstant and Global parameters
// -------------------------------------------------------
use nalgebra as na;

// Max parameters
pub const MAX_N: usize = 1400; // Max total particles
pub const MAX_NEAR_N: usize = 100; // Max nearing particles
pub const MAX_NEAR_SUM: usize = MAX_N * MAX_NEAR_N;

// SPH parameters
pub const SMOOTH_LENGTH: f64 = 0.1; // [m]
pub const CELL_SIZE: f64 = 2.0 * SMOOTH_LENGTH; // [m]
pub const BETA: f64 = 1.0;

// MOdel config
pub const DIM: usize = 3; // Dimension
pub const LENGTH: f64 = 1.0; // x-axis [m]
pub const WIDTH: f64 = 1.0; // y-axis [m]
pub const HEIGHT: f64 = 1.0; // z-xis [m]

// Resolution
pub const DX: f64 = 0.1; // x-axis [m]
pub const DY: f64 = 0.1; // y-axis [m]
pub const DZ: f64 = 0.1; // z-axis [m]

pub const NX: usize = (LENGTH / DX) as usize;
pub const NY: usize = (WIDTH / DY) as usize;
pub const NZ: usize = (HEIGHT / DZ) as usize;

// Material information
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Fluid {
    Water,
    Air,
}

// Particle information
#[derive(Debug, PartialEq)]
pub struct Particle<const D: usize> {
    // SPH parameters
    pub pair: usize, // pair numbers per one particles
    pub volume: f64, // [m^3]

    // physical quantity for fluid
    pub rho0: f64,                // initial density [kg/m^3]
    pub rho: f64,                 // density [kg/m^3]
    pub visco: f64,               // viscosity [Pa*s]
    pub sound_v: f64,             // sound velocity [m/s]
    pub x: [f64; DIM],            // location vector [m]
    pub v: [f64; DIM],            // velocity [m/s]
    pub stress: na::Matrix3<f64>, // Cauthy stress [Pa]
    pub dvdt: [f64; DIM],         // acceleration [m/s^2]
    pub e: f64,                   // total energy [J]
    pub dedt: f64,                // power [J/s]
    pub temperature: f64,         // Temperature [K]
    pub fluid: Fluid,             // Fluid type (Water, Air, etc.)
}

impl<const D: usize> Particle<D> {
    pub fn new(fluid: Fluid) -> Self {
        // Initial temperature and sound speed
        let temperature: f64 = 273.15 + 20.0;
        let sound_air = 331.3 + (0.6 * (temperature - 273.15));
        let sound_water =
            1402.4 + 5.04 * (temperature - 273.15) - 0.057 * (temperature - 273.15).powf(2.0);

        // Fluid properties
        let (rho, visco, sound_v) = match fluid {
            Fluid::Water => (1000.0, 0.001, sound_water),
            Fluid::Air => (1.225, 0.0000181, sound_air),
        };

        // initial value
        let rho0 = rho;

        // set a new particle
        Particle {
            pair: 0,
            volume: LENGTH * WIDTH * HEIGHT,
            rho0,
            rho,
            visco,
            sound_v,
            x: [0.0, 0.0, 0.0],
            v: [0.0, 0.0, 0.0],
            stress: na::Matrix3::zeros(),
            dvdt: [0.0, 0.0, 0.0],
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
pub struct NeighboringList<const D: usize> {
    pub i: usize, // pair i
    pub j: usize, // pair j
    pub w: f64,
    pub dwdr: [f64; D],
}

impl<const D: usize> NeighboringList<D> {
    pub fn new() -> Self {
        NeighboringList {
            i: 0,
            j: 0,
            w: 0.0,
            dwdr: [0.0; D],
        }
    }

    pub fn kernel_axis3(&self) -> (f64, f64, f64) {
        let dwdr1 = self.dwdr[0];
        let dwdr2 = self.dwdr[1];
        let dwdr3 = self.dwdr[2];
        (dwdr1, dwdr2, dwdr3)
    }
}
