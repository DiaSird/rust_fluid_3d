use super::parameters::{BC, DIM, ModelScale, Particle, Resolution};
use rayon::prelude::*;

/// # Errors
pub fn boundary_condition(
    particles: &mut [Particle<DIM>],
    pattern: BC,
    u_lid: f64,
    model_scale: ModelScale,
    resolution: Resolution,
    smooth_length: f64,
) {
    let ModelScale {
        length,
        width,
        height: _,
    } = model_scale;
    let Resolution { dx: _, dy, dz: _ } = resolution;

    match pattern {
        BC::CavityFlow => cavity_flow(particles, u_lid, model_scale, resolution, smooth_length),
        BC::PoiseuilleFlow => poiseuille_flow(particles, u_lid, width, dy),
        BC::PeriodicFlow => periodic_flow(particles, length),
        BC::LidDrivenCavity => lid_driven_cavity(particles, u_lid, width, smooth_length),
    }
}

/// Cavity flow
pub fn cavity_flow(
    particles: &mut [Particle<DIM>],
    u_lid: f64,
    model_scale: ModelScale,
    dx: Resolution,
    smooth_length: f64,
) {
    let ModelScale { length, width, height } = model_scale;
    let Resolution { dx, dy, dz } = dx;

    particles.par_iter_mut().for_each(|p| {
        let x = p.x[0];
        let y = p.x[1];
        let z = p.x[2];

        if y > width - smooth_length {
            p.v[0] = u_lid;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        } else if !(dx..=length - dx).contains(&x) || y < dy || !(dz..=height - dz).contains(&z) {
            p.v[0] = 0.0;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        }
    });
}

/// Poiseuille Flow
pub fn poiseuille_flow(particles: &mut [Particle<DIM>], u_lid: f64, width: f64, dy: f64) {
    particles.par_iter_mut().for_each(|p| {
        let y = p.x[1];

        p.v[0] = 4.0 * u_lid * y * (width - y) / (width * width);
        p.v[1] = 0.0;
        p.v[2] = 0.0;

        // no-slip
        if !(dy..=width - dy).contains(&y) {
            p.v[0] = 0.0;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        }
    });
}

/// Periodic flow
pub fn periodic_flow(particles: &mut [Particle<DIM>], length: f64) {
    let x_max = length;
    particles.par_iter_mut().for_each(|p| {
        if p.x[0] < 0.0 {
            p.x[0] += x_max;
        } else if p.x[0] > x_max {
            p.x[0] -= x_max;
        }
    });
}

// Lid-driven cavity
pub fn lid_driven_cavity(particles: &mut [Particle<DIM>], u_lid: f64, width: f64, smooth_length: f64) {
    particles.par_iter_mut().for_each(|p| {
        let y = p.x[1];
        if y > width - smooth_length {
            p.v[0] = u_lid;
        } else {
            p.v[0] = 0.0;
        }
        p.v[1] = 0.0;
        p.v[2] = 0.0;
    });
}
