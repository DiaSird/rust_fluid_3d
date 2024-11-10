use super::{
    parameters::{Fluid, DIM, MAX_N},
    sph_utils::Velocity,
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

    // Particles for models
    let mut particles: Vec<Particle<DIM>> = (0..MAX_N).map(|_| Particle::new(water)).collect();
    let mut neighbors: Vec<NeighboringList<DIM>> =
        (0..MAX_NEAR_SUM).map(|_| NeighboringList::new()).collect();
    let mut velocity: Vec<Velocity<DIM>> = (0..MAX_N).map(|_| Velocity::new()).collect();

    // Simulation Tests
    let n: usize = make_model(&mut particles).context("Failed: model config")?;
    let k = search_near_particles(&mut particles[0..n], &mut neighbors)
        .context("Failed: searching near particles")?;

    // let mut i = 0;
    // for v in velocity.iter_mut() {
    //     if i < n {
    //         v.sph_grad(&mut particles[0..n], &mut neighbors[0..k], i)
    //             .context("Failed: grad-v")?;
    //         v.sph_div(&mut particles[0..n], &mut neighbors[0..k], i)
    //             .context("Failed: div-v")?;
    //         i += 1;
    //         dbg!(v.div_v);
    //     }
    // }

    let mut step: usize = 1;
    while step <= max_step {
        boundary_condition(&mut particles[0..n]).context("Failed: boundary condition")?;

        update_density(
            dt,
            &mut particles[0..n],
            &mut neighbors[0..k],
            &mut velocity[0..n],
        )
        .context("Failed: updating density")?;
        update_stress(&mut particles[0..n], &mut neighbors[0..k])
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
