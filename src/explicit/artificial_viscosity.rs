use super::parameters::{NeighboringList as Neighbor, Particle, BETA, DIM, SMOOTH_LENGTH};
use anyhow::{Ok, Result};
use nalgebra::{self as na, SimdComplexField};

pub fn update_artificial_viscosity(
    particles: &mut [Particle<DIM>],
    neighbors: &mut [Neighbor<DIM>],
) -> Result<()> {
    for neigh in neighbors.iter_mut() {
        let rhoij = 0.5 * (particles[neigh.i].rho + particles[neigh.j].rho);
        let cij = 0.5 * (particles[neigh.i].sound_v + particles[neigh.j].sound_v);

        let vij = na::Vector3::from(particles[neigh.i].v) - na::Vector3::from(particles[neigh.j].v);
        let xij = na::Vector3::from(particles[neigh.i].x) - na::Vector3::from(particles[neigh.j].x);

        let v_dot_x = vij.dot(&xij);
        let coef = v_dot_x / (xij.dot(&xij) + (0.1 * SMOOTH_LENGTH).simd_powf(2.0));

        if v_dot_x < 0.0 {
            let identity: na::Matrix3<f64> = na::Matrix3::identity();
            let mut coef_i = -BETA * cij * coef + BETA * coef.simd_powf(2.0);
            let mut coef_j = -BETA * cij * coef + BETA * coef.simd_powf(2.0);

            coef_i /= rhoij;
            coef_j /= rhoij;

            particles[neigh.i].stress -= coef_i * identity;
            particles[neigh.j].stress -= coef_j * identity;
        }
    }

    Ok(())
}
