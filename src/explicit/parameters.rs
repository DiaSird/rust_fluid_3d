// Min-Max
pub const MAX_N: usize = 1500; // Max total particles
pub const MAX_NEAR_N: usize = 200; // Max nearing particles
pub const MAX_NEAR_SUM: usize = MAX_N * MAX_NEAR_N;

// config
pub const DIM: usize = 3; // Dimension
pub const LENGTH: f64 = 1.0; // x-axis [m]
pub const WIDTH: f64 = 1.0; // y-axis [m]
pub const HEIGHT: f64 = 1.0; // z-xis [m]
pub const DX: f64 = 0.1;
pub const DY: f64 = 0.1;
pub const DZ: f64 = 0.1;

pub const NX: usize = (LENGTH / DX) as usize;
pub const NY: usize = (WIDTH / DY) as usize;
pub const NZ: usize = (HEIGHT / DZ) as usize;

// Material information
#[derive(Debug, PartialEq)]
pub enum Material {
    Water,
    Air,
}
