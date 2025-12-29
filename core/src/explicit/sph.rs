// use crate::explicit::rw_checkpoint::{load_checkpoint, write_checkpoint};

use super::{
    acceleration::update_acceleration,
    artificial_viscosity::update_artificial_viscosity,
    bs_settings::boundary_condition,
    cfl_condition::cfl_dt,
    density::update_density,
    neighboring_lists::search_near_particles,
    parameters::{Fluid, NeighboringList as Neighbor, Particle, DIM, MAX_N, MAX_NEAR_SUM},
    rw_checkpoint,
    sim_models::make_model,
    smoothing::conservative_smoothing,
    sph_utils::{Tensor, Velocity},
    stress::update_stress,
    velocity::{update_half_velocity, update_location},
    write_csv::{display_result, write_result},
};
use anyhow::{Context, Ok, Result};

// SPH Main function
pub fn sph(
    mut dt: f64,
    out_step: usize,
    max_step: usize,
    restart_file: Option<&str>,
) -> Result<()> {
    // Initialize
    let mut time = 0.0;
    let water = Fluid::Water;
    let _ = Fluid::Air;

    let mut particles: Vec<Particle<DIM>>;
    let mut step: usize;

    // Model particles
    if let Some(file) = restart_file {
        // Load checkpoint
        let state = rw_checkpoint::load_checkpoint::<DIM>(file)?;

        // restore next steps
        particles = state.particles;
        time = state.time;
        dt = state.dt;
        step = state.step + 1;

        println!(
            "Restarted from checkpoint {} at step {}, time {:.3} [ms]",
            file,
            state.step,
            time * 1000.0
        );
    } else {
        // Initialize
        particles = (0..MAX_N).map(|_| Particle::new(water)).collect();
        step = 1;
    }

    // Neighbor particles
    // let mut particles: Vec<Particle<DIM>> = (0..MAX_N).map(|_| Particle::new(water)).collect();
    let mut neighbors: Vec<Neighbor<DIM>> = (0..MAX_NEAR_SUM).map(|_| Neighbor::new()).collect();

    // Gradient and div particles
    let mut diff_velocity: Vec<Velocity<DIM>> = (0..MAX_N).map(|_| Velocity::new()).collect();
    let mut diff_stress: Vec<Tensor<DIM>> = (0..MAX_N).map(|_| Tensor::new()).collect();

    // --- Initialing Simulation
    // n: total particle numbers, k: total pair particles
    let n: usize = make_model("box", &mut particles).context("Failed: model config")?;

    let k = search_near_particles(&mut particles[0..n], &mut neighbors)
        .context("Failed: searching near particles")?;

    display_result(step, time, &particles[0..n])?;
    write_result(step, &particles[0..n])?;

    // --- Simulation loop
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

        conservative_smoothing(&mut particles[0..n], &mut neighbors[0..k])
            .context("Failed: conservative smoothing")?;

        if step.is_multiple_of(out_step) {
            display_result(step, time, &particles[0..n])?;
            write_result(step, &particles[0..n])?;
            let state = rw_checkpoint::State {
                step,
                time,
                dt,
                n,
                particles: particles[0..n].to_vec(),
            };
            rw_checkpoint::write_checkpoint(
                "results/checkpoint.bin",
                // &format!("results/checkpoint_{:08}.bin", step),
                &state,
                1024 * 10000,
            )?;
        }

        time += dt;
        step += 1;
    }
    Ok(())
}
