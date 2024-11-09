use super::parameters::{Fluid, DIM, MAX_N};
use crate::explicit::{
    acceleration::update_acceleration,
    bs_settings::boundary_condition,
    density::update_density,
    neighoring_lists::search_near_particles,
    parameters::{NeighboringList, Particle, MAX_NEAR_SUM},
    sim_models::make_model,
    stress::update_stress,
    velocity::update_velocity,
};
use anyhow::{Context, Ok, Result};

// SPH Main function
pub fn sph(dt: f64, max_step: usize) -> Result<()> {
    // Initialize
    let mut time = 0.0;
    let water = Fluid::Water;
    let air = Fluid::Air;
    dbg!(air);

    // let mut particles: [Particle; MAX_N] = std::array::from_fn(|_| Particle::new(water));
    // let mut neigh_lists: Vec<NeighboringList<DIM>> = vec![NeighboringList::new(); MAX_NEAR_SUM];
    let mut particles: Vec<Particle> = (0..MAX_N).map(|_| Particle::new(water)).collect();
    let mut neigh_lists: Vec<NeighboringList<DIM>> =
        (0..MAX_NEAR_SUM).map(|_| NeighboringList::new()).collect();

    // Simulation Tests
    let n: usize = make_model(&mut particles).context("Failed: model config")?;
    let k = search_near_particles(&mut particles[0..n], &mut neigh_lists)
        .context("Failed: searching near particles")?;

    let mut step: usize = 1;
    while step <= max_step {
        boundary_condition(&mut particles[0..n]).context("Failed: boundary condition")?;

        update_density(dt, &mut particles[0..n], &mut neigh_lists[0..k])
            .context("Failed: updating density")?;
        update_stress(&mut particles[0..n], &mut neigh_lists[0..k])
            .context("Failed: updating stress")?;
        update_acceleration(&mut particles[0..n]).context("Failed: updating acceleration")?;
        update_velocity(&mut particles[0..n]).context("Failed: updating velocity")?;

        if step % 10 == 0 {
            dbg!(time);
        }

        time += dt;
        step += 1;
    }
    Ok(())
}
