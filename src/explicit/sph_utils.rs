use super::parameters::{NeighboringList as Neigh, Particle, DIM};
use anyhow::Result;

// -- Traits --
pub trait SphInterface {
    fn get_value(&self, particles: &mut [Particle<DIM>], neigh: &mut [Neigh<DIM>]);
    fn sph_std(&self, particles: &mut [Particle<DIM>], neigh: &mut [Neigh<DIM>]) -> Result<()>;
    fn sph_grad(&self, particles: &mut [Particle<DIM>], neigh: &mut [Neigh<DIM>]) -> Result<()>;
    fn sph_div(&self, particles: &mut [Particle<DIM>], neigh: &mut [Neigh<DIM>]) -> Result<()>;
}

// -- Structs --
pub struct Velocity;

impl SphInterface for Velocity {
    fn get_value(&self, _particles: &mut [Particle<DIM>], _neigh: &mut [Neigh<DIM>]) {}

    fn sph_std(&self, _particles: &mut [Particle<DIM>], _neigh: &mut [Neigh<DIM>]) -> Result<()> {
        Ok(())
    }

    fn sph_grad(&self, _particles: &mut [Particle<DIM>], _neigh: &mut [Neigh<DIM>]) -> Result<()> {
        Ok(())
    }

    fn sph_div(&self, _particles: &mut [Particle<DIM>], _neigh: &mut [Neigh<DIM>]) -> Result<()> {
        Ok(())
    }
}
