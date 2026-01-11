// -------------------------------------------------------
//  Constant parameters
// -------------------------------------------------------
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
