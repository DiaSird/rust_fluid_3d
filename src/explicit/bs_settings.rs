use super::parameters::{Particle, DIM, DX, DY, DZ, HEIGHT, LENGTH, SMOOTH_LENGTH, WIDTH};
use anyhow::{Ok, Result};

const U_LID: f64 = 5.0;

pub fn boundary_condition(particles: &mut [Particle<DIM>]) -> Result<()> {
    let pattern: usize = 1;
    match pattern {
        1 => cavity_flow(particles),
        2 => poiseuille_flow(particles),
        3 => periodic_flow(particles),
        4 => lid_driven_cavity(particles),
        _ => Ok(()),
    }
}

/// Cavity flow
pub fn cavity_flow(particles: &mut [Particle<DIM>]) -> Result<()> {
    for p in particles.iter_mut() {
        let x = p.x[0];
        let y = p.x[1];
        let z = p.x[2];

        if y > WIDTH - SMOOTH_LENGTH {
            p.v[0] = U_LID;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        } else if x < DX || x > LENGTH - DX || y < DY || z < DZ || z > HEIGHT - DZ {
            p.v[0] = 0.0;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        }
    }
    Ok(())
}

/// Poiseuille Flow
pub fn poiseuille_flow(particles: &mut [Particle<DIM>]) -> Result<()> {
    for p in particles.iter_mut() {
        let y = p.x[1];

        p.v[0] = 4.0 * U_LID * y * (WIDTH - y) / (WIDTH * WIDTH);
        p.v[1] = 0.0;
        p.v[2] = 0.0;

        // no-slip
        if y < DY || y > WIDTH - DY {
            p.v[0] = 0.0;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        }
    }
    Ok(())
}

/// Periodic flow
pub fn periodic_flow(particles: &mut [Particle<DIM>]) -> Result<()> {
    let x_max = LENGTH;
    for p in particles.iter_mut() {
        if p.x[0] < 0.0 {
            p.x[0] += x_max;
        } else if p.x[0] > x_max {
            p.x[0] -= x_max;
        }
    }
    Ok(())
}

// Lid-driven cavity
pub fn lid_driven_cavity(particles: &mut [Particle<DIM>]) -> Result<()> {
    for p in particles.iter_mut() {
        let y = p.x[1];
        if y > WIDTH - SMOOTH_LENGTH {
            p.v[0] = U_LID;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        } else {
            p.v[0] = 0.0;
            p.v[1] = 0.0;
            p.v[2] = 0.0;
        }
    }
    Ok(())
}
