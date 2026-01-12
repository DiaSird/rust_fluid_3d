use nalgebra::{self as na, SimdComplexField};
use utils::{
    error::{check_nan_to_error, SimError},
    parameters::{NeighboringList as Neighbor, Particle, DIM},
};

// -- Traits --
// Standard fdm
pub(crate) trait _FdmStd {
    type Error;

    fn fdm_std(
        &mut self,
        particles: &[Particle<DIM>],
        neighbors: &[Neighbor<DIM>],
        i: usize,
    ) -> Result<(), Self::Error>;
}

// Differential sph
pub(crate) trait SphDiff {
    type Error;

    fn _fdm_grad(
        &mut self,
        particles: &[Particle<DIM>],
        neighbors: &[Neighbor<DIM>],
        i: usize,
    ) -> Result<(), Self::Error>;

    fn fdm_div(
        &mut self,
        particles: &[Particle<DIM>],
        neighbors: &[Neighbor<DIM>],
        i: usize,
    ) -> Result<(), Self::Error>;
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
        Self {
            grad_v: na::Matrix3::zeros(),
            div_v: 0.0,
        }
    }
}

impl<const D: usize> SphDiff for Velocity<D> {
    type Error = SimError;

    fn _fdm_grad(
        &mut self,
        _particles: &[Particle<DIM>],
        _neighbors: &[Neighbor<DIM>],
        _i: usize,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    // Todo: Generic Vector (velocity -> vector3)
    fn fdm_div(
        &mut self,
        particles: &[Particle<DIM>],
        neighbors: &[Neighbor<DIM>],
        i: usize,
    ) -> Result<(), SimError> {
        let start: usize = match i {
            0 => 0,
            _ => particles[i - 1].pair + 1,
        };
        let end: usize = particles[i].pair;

        // fdm referred to neighboring list (pair: start -> end)
        for neigh in neighbors.iter().take(end + 1).skip(start) {
            let j = neigh.j;
            let vi = na::Vector3::from(particles[i].v);
            let vj = na::Vector3::from(particles[j].v);
            let dwdr = na::Vector3::from(neigh.dwdr);

            let volume_i = particles[i].volume;
            let volume_j = particles[j].volume;

            self.div_v += (vi - vj).dot(&dwdr) * volume_j;
            self.div_v += (vj - vi).dot(&-dwdr) * volume_i;

            check_nan_to_error(0, self.div_v)?;
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
    pub const fn new() -> Self {
        Tensor {
            div_tensor: [0.0; DIM],
        }
    }
}

impl FdmDiff for Tensor<DIM> {
    type Error = SimError;

    fn _fdm_grad(
        &mut self,
        _particles: &[Particle<DIM>],
        _neighbors: &[Neighbor<DIM>],
        _i: usize,
    ) -> Result<(), Self::Error> {
        // No-need to impl
        Ok(())
    }

    fn fdm_div(
        &mut self,
        particles: &[Particle<DIM>],
        neighbors: &[Neighbor<DIM>],
        i: usize,
    ) -> Result<(), Self::Error> {
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
                check_nan_to_error(d, self.div_tensor[d])?;
            }
        }

        Ok(())
    }
}
