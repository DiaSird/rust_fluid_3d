use sph::sph;
use tauri::{AppHandle, Emitter};
use utils::parameters::{BC, Config, ModelScale, Resolution};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct GUIConfig {
    /// Max particles
    pub max_n: usize,
    pub max_near_n: usize,
    pub max_near_sum: usize,

    // Model size
    pub model_scale: ModelScale,

    // Boundary Condition
    pub bc_pattern: BC,
    pub u_lid: f64,

    // SPH parameters
    pub n_axis: usize,
    pub smooth_length: f64,
    pub cell_size: f64,
    pub beta: f64,
    pub cs_rate: f64,

    // Resolution
    pub dx: Resolution,

    // Time stepping
    pub dt: f64,
    pub out_step: usize,
    pub max_step: usize,

    // Checkpoint file to restart
    pub restart_file: Option<std::path::PathBuf>,

    // Monitoring and log report
    pub monitor_particle: usize,
}

#[tauri::command]
pub(crate) async fn run_simulation(app: AppHandle, config: GUIConfig) -> Result<(), String> {
    let _ = app.emit("simulation-log", "Simulation started.");
    // let _ = app.emit("simulation-log", format!("config = {:?}", config));

    let _ = std::fs::create_dir_all("./results");
    let checkpoint_path = "results/checkpoint.bin";
    let _restart_file = match std::path::Path::new(checkpoint_path).exists() {
        true => Some(checkpoint_path),
        false => None,
    };

    let config = Config {
        max_n: config.max_n,
        max_near_n: config.max_near_n,
        model_scale: config.model_scale,
        bc_pattern: config.bc_pattern,
        u_lid: config.u_lid,
        n_axis: config.n_axis,
        smooth_length: config.smooth_length,
        cell_size: config.cell_size,
        beta: config.beta,
        cs_rate: config.cs_rate,
        dx: config.dx,
        dt: config.dt,
        out_step: config.out_step,
        max_step: config.max_step,
        restart_file: config.restart_file,
        monitor_particle: config.monitor_particle,
        log_report: None,
    };

    std::thread::spawn(move || {
        if let Err(e) = sph::sph(config) {
            let _ = app.emit("simulation-log", format!("Error: {:?}", e));
        }
    });

    Ok(())
}
