use super::parameters::{NeighboringList, Particle, DIM};
use crate::explicit::parameters::MAX_N;
use anyhow::{Ok, Result};
use nalgebra as na;

pub fn sph_div(particles: &mut [Particle], neighbors: &mut [NeighboringList<DIM>]) -> Result<()> {
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

    Ok(())
}
