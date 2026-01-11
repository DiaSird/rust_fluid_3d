mod boundary_condition;
mod config;
mod consts;
mod particle_neighbors;
mod particle_status;
mod particles;

pub use boundary_condition::BoundaryCondition;
pub use config::{Config, ModelScale, Resolution};
pub use consts::*;
pub use particle_neighbors::NeighboringList;
pub use particle_status::{LogReporterFn, Message, ParticleLog};
pub use particles::Particle;

use nalgebra::{self as na};

// Material information
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Fluid {
    Water,
    Air,
}

/// type arias
pub(crate) type Vector<const DIM: usize> = na::SVector<f64, DIM>;
pub(crate) type Matrix<const DIM: usize> =
    na::Matrix<f64, na::Const<DIM>, na::Const<DIM>, na::ArrayStorage<f64, DIM, DIM>>;
pub type BC = BoundaryCondition;
