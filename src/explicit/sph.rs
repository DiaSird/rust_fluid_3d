use super::{
    parameters::{Fluid, DIM, MAX_N},
    sph_utils::{SphInterface, Velocity},
};
use crate::explicit::{
    acceleration::update_acceleration,
    bs_settings::boundary_condition,
    density::update_density,
    neighboring_lists::search_near_particles,
    parameters::{NeighboringList, Particle, MAX_NEAR_SUM},
    sim_models::make_model,
    stress::update_stress,
    velocity::update_velocity,
};
use anyhow::{Context, Ok, Result};

// SPH Main function
pub fn sph(dt: f64, out_step: usize, max_step: usize) -> Result<()> {
    // Initialize
    let mut time = 0.0;
    let water = Fluid::Water;
    let _ = Fluid::Air;

    let mut particles: Vec<Particle<DIM>> = (0..MAX_N).map(|_| Particle::new(water)).collect();
    let mut neigh_lists: Vec<NeighboringList<DIM>> =
        (0..MAX_NEAR_SUM).map(|_| NeighboringList::new()).collect();

    // Simulation Tests
    let n: usize = make_model(&mut particles).context("Failed: model config")?;
    let k = search_near_particles(&mut particles[0..n], &mut neigh_lists)
        .context("Failed: searching near particles")?;

    let velocity = Velocity;
    velocity.get_value(&mut particles[0..n], &mut neigh_lists[0..k]);
    velocity
        .sph_div(&mut particles[0..n], &mut neigh_lists[0..k])
        .context("Failed: div-v")?;
    velocity
        .sph_std(&mut particles[0..n], &mut neigh_lists[0..k])
        .context("Failed: std-v")?;
    velocity
        .sph_grad(&mut particles[0..n], &mut neigh_lists[0..k])
        .context("Failed: grad-v")?;
    velocity
        .sph_div(&mut particles[0..n], &mut neigh_lists[0..k])
        .context("Failed: div-v")?;

    let mut step: usize = 1;
    while step <= max_step {
        boundary_condition(&mut particles[0..n]).context("Failed: boundary condition")?;

        update_density(dt, &mut particles[0..n], &mut neigh_lists[0..k])
            .context("Failed: updating density")?;
        update_stress(&mut particles[0..n], &mut neigh_lists[0..k])
            .context("Failed: updating stress")?;
        update_acceleration(&mut particles[0..n]).context("Failed: updating acceleration")?;
        update_velocity(&mut particles[0..n]).context("Failed: updating velocity")?;

        if step % out_step == 0 {
            dbg!(time);
        }

        time += dt;
        step += 1;
    }
    Ok(())
}
