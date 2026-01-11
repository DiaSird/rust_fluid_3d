use super::parameters::{BC, DIM, DX, DY, DZ, HEIGHT, LENGTH, Particle, SMOOTH_LENGTH, WIDTH};
use rayon::prelude::*;

/// # Errors
pub fn boundary_condition(particles: &mut [Particle<DIM>], pattern: BC, u_lid: f64) {
    match pattern {
        BC::CavityFlow => cavity_flow(particles, u_lid),
        BC::PoiseuilleFlow => poiseuille_flow(particles, u_lid),
        BC::PeriodicFlow => periodic_flow(particles),
        BC::LidDrivenCavity => lid_driven_cavity(particles, u_lid),
    }
}

/// Cavity flow
pub fn cavity_flow(particles: &mut [Particle<DIM>], u_lid: f64) {
    particles.par_iter_mut().for_each(|p| {
        let x = p.x[0];
        let y = p.x[1];
        let z = p.x[2];

        if y > WIDTH - SMOOTH_LENGTH {
            p.v[0] = u_lid;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        } else if !(DX..=LENGTH - DX).contains(&x) || y < DY || !(DZ..=HEIGHT - DZ).contains(&z) {
            p.v[0] = 0.0;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        }
    });
}

/// Poiseuille Flow
pub fn poiseuille_flow(particles: &mut [Particle<DIM>], u_lid: f64) {
    particles.par_iter_mut().for_each(|p| {
        let y = p.x[1];

        p.v[0] = 4.0 * u_lid * y * (WIDTH - y) / (WIDTH * WIDTH);
        p.v[1] = 0.0;
        p.v[2] = 0.0;

        // no-slip
        if !(DY..=WIDTH - DY).contains(&y) {
            p.v[0] = 0.0;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        }
    });
}

/// Periodic flow
pub fn periodic_flow(particles: &mut [Particle<DIM>]) {
    let x_max = LENGTH;
    particles.par_iter_mut().for_each(|p| {
        if p.x[0] < 0.0 {
            p.x[0] += x_max;
        } else if p.x[0] > x_max {
            p.x[0] -= x_max;
        }
    });
}

// Lid-driven cavity
pub fn lid_driven_cavity(particles: &mut [Particle<DIM>], u_lid: f64) {
    particles.par_iter_mut().for_each(|p| {
        let y = p.x[1];
        if y > WIDTH - SMOOTH_LENGTH {
            p.v[0] = u_lid;
        } else {
            p.v[0] = 0.0;
        }
        p.v[1] = 0.0;
        p.v[2] = 0.0;
    });
}
