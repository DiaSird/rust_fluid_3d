use super::{
    acceleration::update_acceleration,
    artificial_viscosity::update_artificial_viscosity,
    density::update_density,
    neighboring_lists::search_near_particles,
    smoothing::conservative_smoothing,
    sph_utils::{Tensor, Velocity},
    stress::update_stress,
    velocity::{update_half_velocity, update_location},
};
use anyhow::{Context, Ok, Result};
use utils::{
    bs_settings::boundary_condition,
    cfl_condition::cfl_dt,
    parameters::{Config, DIM, Fluid, Message, NeighboringList as Neighbor, Particle},
    rw_checkpoint,
    sim_models::make_model,
    write_csv::{display_result, write_result, write_sim_checkpoint},
};

/// SPH Main function
/// # Errors
/// MAX Particles < N
pub fn sph(config: Config) -> Result<()> {
    #[rustfmt::skip]
    let Config {
        max_n, max_near_n, model_scale, bc_pattern, u_lid,
        n_axis, smooth_length, cell_size, beta, cs_rate,
        dx, mut dt, out_step, max_step, restart_file, out_file,
        monitor_particle, log_report,
    } = config;

    // Initialize
    let mut time = 0.0;
    let water = Fluid::Water;
    let _ = Fluid::Air;

    let mut particles: Vec<Particle<DIM>>;
    let mut step: usize;

    // Model particles
    if let Some(file) = restart_file {
        // Load checkpoint
        let state = rw_checkpoint::load_checkpoint::<DIM, _>(&file)?;

        // restore next steps
        particles = state.particles;
        time = state.time;
        dt = state.dt;
        step = state.step + 1;

        let _log = format!(
            "Restarted from checkpoint {} at step {}, time {:.3} [ms]",
            file.display(),
            state.step,
            time * 1000.0
        );
        dbg!(&_log);
        // let _ = app.emit("simulation-log", log);
    } else {
        // Initialize
        particles = (0..max_n).map(|_| Particle::new(water)).collect();
        step = 1;
    }

    // Neighbor particles
    let mut neighbors: Vec<Neighbor<DIM>> = (0..max_n * max_near_n)
        .map(|_| Neighbor::default())
        .collect();

    // Gradient and div particles
    let mut diff_velocity: Vec<Velocity<DIM>> = (0..max_n).map(|_| Velocity::new()).collect();
    let mut diff_stress: Vec<Tensor<DIM>> = (0..max_n).map(|_| Tensor::new()).collect();

    // --- Initialing Simulation
    if let Some(log_report) = &log_report {
        log_report(utils::parameters::ParticleLog::LogInfo(
            Message::CreateModel,
        ));
    }

    // n: total particle numbers, k: total pair particles
    let n: usize = make_model("box", &mut particles)?;

    if let Some(log_report) = &log_report {
        log_report(utils::parameters::ParticleLog::LogInfo(Message::Search));
    }
    let k = search_near_particles(&mut particles[0..n], &mut neighbors)
        .context("Failed: searching near particles")?;

    if let Some(log_report) = &log_report {
        #[rustfmt::skip]
        display_result(monitor_particle, log_report, step, time, &particles[0..n]);
    }
    // write_result(step, &particles[0..n])?;

    // --- Simulation loop
    while step <= max_step {
        dt = cfl_dt(dt, &particles[0..n]);
        boundary_condition(&mut particles[0..n], bc_pattern, u_lid);

        #[rustfmt::skip]
        update_half_velocity(dt, &mut particles[0..n]).context("Failed: updating velocity")?;

        #[rustfmt::skip]
        update_location(dt, &mut particles[0..n]).context("Failed: updating velocity")?;

        #[rustfmt::skip]
        update_density(dt, &mut particles[0..n], &neighbors[0..k], &mut diff_velocity[0..n]).context("Failed: updating density")?;

        #[rustfmt::skip]
        update_artificial_viscosity(&mut particles[0..n], &neighbors[0..k]).context("Failed: updating artificial viscosity")?;

        #[rustfmt::skip]
        update_stress(&mut particles[0..n], &mut neighbors[0..k], &mut diff_velocity[0..n],).context("Failed: updating stress")?;

        #[rustfmt::skip]
        update_acceleration(&mut particles[0..n], &neighbors[0..k], &mut diff_stress[0..n]).context("Failed: updating acceleration")?;

        #[rustfmt::skip]
        update_half_velocity(dt, &mut particles[0..n]).context("Failed: updating velocity")?;

        #[rustfmt::skip]
        conservative_smoothing(&mut particles[0..n], &neighbors[0..k]).context("Failed: conservative smoothing")?;

        // Output
        if step.is_multiple_of(out_step) {
            if let Some(log_report) = &log_report {
                #[rustfmt::skip]
                display_result(monitor_particle, log_report, step, time, &particles[0..n]);
            }
            write_sim_checkpoint(&out_file, step, time, dt, n, &particles[0..n])?;
        }

        // Increment
        time += dt;
        step += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use utils::parameters::ParticleLog;

    use super::*;

    /// Test SPH on background
    #[test]
    fn test_sph() {
        let config = Config {
            restart_file: Some(std::path::PathBuf::from("sim_ckpt.bin")),
            log_report: Some(Box::new(log_report)),
            ..Default::default()
        };
        let result = sph(config);
        println!("{:?}", result);
    }

    fn log_report(pl: ParticleLog) {
        let ParticleLog::Info3 {
            monitor_particle,
            step,
            time,
            x,
            v,
            dvdt,
        } = pl
        else {
            return;
        };

        let log = format!(
            "------------------------------------------\n\
            Step {}, time = {:.3} [ms]\n\
                Particle: {}\n\
                (x, y, z) = {:.3}, {:.3}, {:.3}\n\
                (vx, vy, vz) = {:.3}, {:.3}, {:.3}\n\
                (ax, ay, az) = {:.3}, {:.3}, {:.3}\n\
            ------------------------------------------",
            step,
            time * 1000.0,
            monitor_particle,
            x[0],
            x[1],
            x[2],
            v[0],
            v[1],
            v[2],
            dvdt[0],
            dvdt[1],
            dvdt[2]
        );
        println!("{}", log);
    }
}
