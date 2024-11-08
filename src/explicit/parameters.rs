// Min-Max
pub const MAX_N: usize = 1500; // Max total particles
pub const MAX_NEAR_N: usize = 1000; // Max nearing particles

// config
pub const DIM: usize = 3; // Dimension
pub const LENGTH: f64 = 1.0; // x-axis [m]
pub const WIDTH: f64 = 1.0; // y-axis [m]
pub const HEIGHT: f64 = 1.0; // z-xis [m]
pub const DX: f64 = 0.1;
pub const DY: f64 = 0.1;
pub const DZ: f64 = 0.1;

// Material information
#[derive(Debug)]
pub enum Material {
    Water,
    Air,
}
