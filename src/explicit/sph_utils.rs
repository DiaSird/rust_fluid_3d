use super::parameters::{NeighboringList as Neighbor, Particle, DIM};
use anyhow::{bail, Result};
use nalgebra::{self as na, SimdComplexField};

// -- Traits --
// Standard sph
pub trait _SphStd {
    fn sph_std(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neighbor<DIM>],
        i: usize,
    ) -> Result<()>;
}

// Differential sph
pub trait SphDiff {
    fn _sph_grad(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neighbor<DIM>],
        i: usize,
    ) -> Result<()>;
    fn sph_div(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neighbor<DIM>],
        i: usize,
    ) -> Result<()>;
}

// -- Structs --
// Note: Traits are used these only structs.
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
        _neighbors: &mut [Neighbor<DIM>],
        _i: usize,
    ) -> Result<()> {
        Ok(())
    }

    // Todo: Generic Vector (velocity -> vector3)
    fn sph_div(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neighbor<DIM>],
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

        // sph referred to neighboring list (pair: start -> end)
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

// Note: Only working on DIM = 3
#[derive(Debug, PartialEq)]
pub struct Tensor<const D: usize> {
    pub div_tensor: [f64; D],
}

impl Tensor<DIM> {
    pub fn new() -> Self {
        Tensor {
            div_tensor: [0.0; DIM],
        }
    }
}

impl SphDiff for Tensor<DIM> {
    fn _sph_grad(
        &mut self,
        _particles: &mut [Particle<DIM>],
        _neighbors: &mut [Neighbor<DIM>],
        _i: usize,
    ) -> Result<()> {
        Ok(())
    }

    fn sph_div(
        &mut self,
        particles: &mut [Particle<DIM>],
        neighbors: &mut [Neighbor<DIM>],
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

        // sph referred to neighboring list (pair: start -> end)
        for neigh in neighbors.iter().take(end + 1).skip(start) {
            let j = neigh.j;
            let mut tensor_i = na::Matrix3::from(particles[i].stress);
            let mut tensor_j = na::Matrix3::from(particles[j].stress);

            let dwdr = na::Vector3::from(neigh.dwdr);
            let volume_i = particles[i].volume;
            let volume_j = particles[j].volume;

            tensor_i /= particles[i].rho.simd_powf(2.0);
            tensor_j /= particles[j].rho.simd_powf(2.0);

            let dot_i = particles[i].rho * (tensor_i + -tensor_j) * dwdr;
            let dot_j = particles[j].rho * (tensor_j + tensor_i) * (-dwdr);

            for d in 0..DIM {
                self.div_tensor[d] += dot_i[d] * volume_j;
                self.div_tensor[d] += dot_j[d] * volume_i;
            }
        }

        Ok(())
    }
}
