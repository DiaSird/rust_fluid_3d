use super::{
    acceleration::update_acceleration,
    artificial_viscosity::update_artificial_viscosity,
    bs_settings::boundary_condition,
    cfl_condition::cfl_dt,
    density::update_density,
    neighboring_lists::search_near_particles,
    parameters::{Fluid, NeighboringList as Neighbor, Particle, DIM, MAX_N, MAX_NEAR_SUM},
    sim_models::make_model,
    sph_utils::{Tensor, Velocity},
    stress::update_stress,
    velocity::{update_half_velocity, update_location},
    write_csv::{display_result, write_result},
};
use anyhow::{Context, Ok, Result};

// SPH Main function
pub fn sph(mut dt: f64, out_step: usize, max_step: usize) -> Result<()> {
    // Initialize
    let mut time = 0.0;
    let water = Fluid::Water;
    let _ = Fluid::Air;

    // Model particles
    let mut particles: Vec<Particle<DIM>> = (0..MAX_N).map(|_| Particle::new(water)).collect();
    let mut neighbors: Vec<Neighbor<DIM>> = (0..MAX_NEAR_SUM).map(|_| Neighbor::new()).collect();

    // Gradient and div particles
    let mut diff_velocity: Vec<Velocity<DIM>> = (0..MAX_N).map(|_| Velocity::new()).collect();
    let mut diff_stress: Vec<Tensor<DIM>> = (0..MAX_N).map(|_| Tensor::new()).collect();

    // --- Initialing Simulation
    // n: total particle numbers, k: total pair particles
    let n: usize = make_model(&mut particles).context("Failed: model config")?;
    let k = search_near_particles(&mut particles[0..n], &mut neighbors)
        .context("Failed: searching near particles")?;

    // --- Simulation loop
    let mut step: usize = 1; // initial step
    while step <= max_step {
        dt = cfl_dt(dt, &mut particles[0..n]).context("Failed: CFL condition")?;
        boundary_condition(&mut particles[0..n]).context("Failed: boundary condition")?;

        update_half_velocity(dt, &mut particles[0..n]).context("Failed: updating velocity")?;
        update_location(dt, &mut particles[0..n]).context("Failed: updating velocity")?;

        update_density(
            dt,
            &mut particles[0..n],
            &mut neighbors[0..k],
            &mut diff_velocity[0..n],
        )
        .context("Failed: updating density")?;

        update_artificial_viscosity(&mut particles[0..n], &mut neighbors[0..k])
            .context("Failed: updating artificial viscosity")?;
        update_stress(
            &mut particles[0..n],
            &mut neighbors[0..k],
            &mut diff_velocity[0..n],
        )
        .context("Failed: updating stress")?;

        update_acceleration(
            &mut particles[0..n],
            &mut neighbors[0..k],
            &mut diff_stress[0..n],
        )
        .context("Failed: updating acceleration")?;
        update_half_velocity(dt, &mut particles[0..n]).context("Failed: updating velocity")?;

        if step % out_step == 0 {
            display_result(step, time, &mut particles[0..n])?;
        }

        if step % out_step == 0 {
            write_result(step, &mut particles[0..n])?;
        }

        time += dt;
        step += 1;
    }
    Ok(())
}
