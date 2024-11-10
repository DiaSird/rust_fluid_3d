use super::parameters::{NeighboringList, Particle, DIM};
use crate::explicit::parameters::MAX_N;
use anyhow::Result;
use nalgebra as na;

pub fn update_density(
    dt: f64,
    particles: &mut [Particle<DIM>],
    neighbors: &mut [NeighboringList<DIM>],
) -> Result<()> {
    let mut div_vi = vec![0.0; MAX_N];
    let mut div_vj = vec![0.0; MAX_N];

    for neigh in neighbors.iter_mut() {
        let vi = na::Vector3::from(particles[neigh.i].v);
        let vj = na::Vector3::from(particles[neigh.j].v);
        let dwdr = na::Vector3::from(neigh.dwdr);

        let volume_i = particles[neigh.i].volume;
        let volume_j = particles[neigh.j].volume;

        div_vi[neigh.i] += (vi - vj).dot(&dwdr) * volume_j;
        div_vj[neigh.j] += (vj - vi).dot(&-dwdr) * volume_i;
    }

    for neigh in neighbors.iter_mut() {
        particles[neigh.i].rho += -particles[neigh.i].rho * div_vi[neigh.i] * dt;
        particles[neigh.j].rho += -particles[neigh.j].rho * div_vi[neigh.j] * dt;
    }

    // dbg!(-particles[0].rho * div_vi[0] * dt);

    Ok(())
}
