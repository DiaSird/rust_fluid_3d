use super::parameters::{NeighboringList as Neighbor, Particle, CS_RATE, DIM, MAX_N};
use anyhow::{Ok, Result};
use nalgebra as na;

struct CsValue {
    /// SPH Velocity [m/s]
    pub velocity: na::Vector3<f64>,
    /// SPH Cauthy stress [Pa]
    pub stress: na::Matrix3<f64>,
}

impl CsValue {
    pub fn new() -> Self {
        Self {
            velocity: na::Vector3::zeros(),
            stress: na::Matrix3::zeros(),
        }
    }
}

pub fn conservative_smoothing(
    particles: &mut [Particle<DIM>],
    neighbors: &mut [Neighbor<DIM>],
) -> Result<()> {
    let mut coef = vec![0.0; MAX_N];
    let mut cs_value: Vec<CsValue> = (0..MAX_N).map(|_| CsValue::new()).collect();

    neighbors.iter().for_each(|Neighbor { i, j, w, .. }| {
        let coef_i = w * particles[*i].volume;
        let coef_j = w * particles[*j].volume;

        coef[*i] += coef_i;
        coef[*j] += coef_j;

        let vi = na::Vector3::from(particles[*i].v);
        let vj = na::Vector3::from(particles[*j].v);

        let stress_i = na::Matrix3::from(particles[*i].stress);
        let stress_j = na::Matrix3::from(particles[*j].stress);

        cs_value[*i].velocity += coef_i * vi;
        cs_value[*j].velocity += coef_j * vj;

        cs_value[*i].stress += coef_i * stress_i;
        cs_value[*j].stress += coef_j * stress_j;
    });

    for (i, particle) in particles.iter_mut().enumerate() {
        particle.v = (1.0 - CS_RATE) * particle.v + CS_RATE * cs_value[i].velocity / coef[i];
        particle.stress =
            (1.0 - CS_RATE) * particle.stress + CS_RATE * cs_value[i].stress / coef[i];
    }

    Ok(())
}
