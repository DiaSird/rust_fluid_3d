use super::parameters::{NeighboringList, Particle, DIM};
use anyhow::{Ok, Result};
use nalgebra as na;

// For water
pub fn tait_eq(particle: &mut Particle) -> f64 {
    let gamma = 7.0; // parameter of Tait eq.
    let c0 = 1483.18; // sound velocity [m/s]
    let b = (c0 * c0) / gamma; // parameter of Tait eq.
    let rho_ratio = particle.rho / particle.rho0;

    // Pressure using Tait equation
    particle.rho0 * b * (rho_ratio.powf(gamma) - 1.0)
}

pub fn viscosity_stress(particles: &mut [Particle], neighbors: &mut [NeighboringList<DIM>]) {
    for neigh in neighbors.iter_mut() {
        let mut grad_vi = na::Matrix3::zeros();
        let mut grad_vj = na::Matrix3::zeros();

        let vi = na::Vector3::from(particles[neigh.i].v);
        let vj = na::Vector3::from(particles[neigh.j].v);
        let dwdr = na::Vector3::from(neigh.dwdr);

        let volume_i = particles[neigh.i].volume;
        let volume_j = particles[neigh.j].volume;

        grad_vi += (vi - vj) * dwdr.transpose() * volume_j;
        grad_vj += (vj - vi) * dwdr.transpose() * volume_i;
        // dbg!(grad_vi);
    }

    // for particle in particles.iter_mut() {
    //     let visco_stress: na::Matrix3<f64> = na::Matrix3::zeros();
    // }
}

pub fn update_stress(
    particles: &mut [Particle],
    neighbors: &mut [NeighboringList<DIM>],
) -> Result<()> {
    let identity: na::Matrix3<f64> = na::Matrix3::identity();

    viscosity_stress(particles, neighbors);

    for particle in particles.iter_mut() {
        let p = -tait_eq(particle) * identity;

        if p[0] < 0.0 {
            dbg!(p);
        }
    }
    Ok(())
}
