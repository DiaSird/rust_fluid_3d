use super::parameters::{NeighboringList as Neigh, Particle, DIM};
use anyhow::{bail, Result};
use nalgebra as na;

// -- Traits --
// Standard
pub trait _SphStd {
    fn sph_std(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neigh<DIM>],
        i: usize,
    ) -> Result<()>;
}

// Differential
pub trait SphDiff {
    fn _sph_grad(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neigh<DIM>],
        i: usize,
    ) -> Result<()>;
    fn sph_div(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neigh<DIM>],
        i: usize,
    ) -> Result<()>;
}

// -- Structs --
// Traits are used these only structs.
#[derive(Debug, PartialEq)]
pub struct Velocity<const D: usize> {
    pub grad_v: na::Matrix3<f64>,
    pub div_v: f64,
}

impl<const D: usize> Velocity<D> {
    pub fn new() -> Self {
        Velocity {
            grad_v: na::Matrix3::zeros(),
            div_v: 0.0,
        }
    }
}

impl<const D: usize> SphDiff for Velocity<D> {
    fn _sph_grad(
        &mut self,
        _particles: &mut [Particle<DIM>],
        _neighbors: &mut [Neigh<DIM>],
        _i: usize,
    ) -> Result<()> {
        Ok(())
    }

    // Todo: Generic Vector (velocity -> vector3)
    fn sph_div(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neigh<DIM>],
        i: usize,
    ) -> Result<()> {
        if i >= particles.len() {
            bail!(
                "Exceeded the maximum number of particles. {}",
                particles.len()
            );
        }

        let start: usize = match i {
            0 => 0,
            _ => particles[i - 1].pair + 1,
        };
        let end: usize = particles[i].pair;

        // sph referred to neighboring list
        // for pair in start..=end {
        for neigh in neighbors.iter().take(end + 1).skip(start) {
            let j = neigh.j;
            let vi = na::Vector3::from(particles[i].v);
            let vj = na::Vector3::from(particles[j].v);
            let dwdr = na::Vector3::from(neigh.dwdr);

            let volume_i = particles[i].volume;
            let volume_j = particles[j].volume;

            self.div_v += (vi - vj).dot(&dwdr) * volume_j;
            self.div_v += (vj - vi).dot(&-dwdr) * volume_i;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct StressTemp<const D: usize> {
    pub grad_stress: na::Matrix3<f64>,
    pub div_stress: [f64; D],
}

pub struct Stress;

impl SphDiff for Stress {
    fn _sph_grad(
        &mut self,
        _particles: &mut [Particle<DIM>],
        _neighbors: &mut [Neigh<DIM>],
        _i: usize,
    ) -> Result<()> {
        Ok(())
    }

    fn sph_div(
        &mut self,
        _particles: &mut [Particle<DIM>],
        _neighbors: &mut [Neigh<DIM>],
        _i: usize,
    ) -> Result<()> {
        Ok(())
    }
}
