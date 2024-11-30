use super::{
    parameters::{NeighboringList as Neighbor, Particle, DIM},
    sph_utils::Velocity,
};
use anyhow::{Ok, Result};
use nalgebra as na;

// For water
pub fn tait_eq(particle: &mut Particle<DIM>) -> f64 {
    let gamma = 7.0; // parameter of Tait eq.
    let b = particle.sound_v.powf(2.0) / gamma; // parameter of Tait eq.
    let rho_ratio = particle.rho / particle.rho0;

    // Pressure using Tait equation
    let p = particle.rho0 * b * (rho_ratio.powf(gamma) - 1.0);
    p
}

pub fn static_stress(particles: &mut [Particle<DIM>]) {
    let identity: na::Matrix3<f64> = na::Matrix3::identity();

    for particle in particles.iter_mut() {
        // p = p * identity matrix
        let p = -tait_eq(particle) * &identity;
        particle.stress += p;
    }
}

pub fn viscosity_stress(
    particles: &mut [Particle<DIM>],
    neighbors: &mut [Neighbor<DIM>],
    diff_velocity: &mut [Velocity<DIM>],
) {
    // Total particles and identity matrix
    let n = particles.len();
    let identity: na::Matrix3<f64> = na::Matrix3::identity();

    for neigh in neighbors.iter_mut() {
        // Velocity gradient
        let mut grad_vi = na::Matrix3::zeros();
        let mut grad_vj = na::Matrix3::zeros();

        let vi = na::Vector3::from(particles[neigh.i].v);
        let vj = na::Vector3::from(particles[neigh.j].v);
        let dwdr = na::Vector3::from(neigh.dwdr);

        let volume_i = particles[neigh.i].volume;
        let volume_j = particles[neigh.j].volume;

        grad_vi += (vi - vj) * dwdr.transpose() * volume_j;
        grad_vj += (vj - vi) * dwdr.transpose() * volume_i;

        // Viscosity stress: grad(v) + grad(v)^T
        particles[neigh.i].stress = grad_vi + grad_vi.transpose();
        particles[neigh.j].stress = grad_vj + grad_vj.transpose();
    }

    // // Calculate grad(velocity)
    // for (i, v) in diff_velocity.iter_mut().enumerate().take(n) {
    //     v.sph_grad(particles, neighbors, i)
    //         .context("Failed: grad-v in updating stress")?;
    // }

    // Viscosity stress
    for (i, v) in diff_velocity.iter_mut().enumerate().take(n) {
        // Viscosity stress
        particles[i].stress += -&identity * v.div_v * 2.0 / 3.0;
        particles[i].stress *= particles[i].visco;
    }
}

pub fn update_stress(
    particles: &mut [Particle<DIM>],
    neighbors: &mut [Neighbor<DIM>],
    diff_velocity: &mut [Velocity<DIM>],
) -> Result<()> {
    // Compute viscosity stress
    viscosity_stress(particles, neighbors, diff_velocity);

    // Add static stress
    static_stress(particles);

    Ok(())
}
