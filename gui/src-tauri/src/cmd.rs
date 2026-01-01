use rust_fluid_3d::explicit::sph::sph;
use tauri::{AppHandle, Emitter};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Config {
    pub length: f64,
    pub width: f64,
    pub height: f64,

    pub n_axis: usize,
    pub smooth_length: f64,
    pub cell_size: f64,
    pub beta: f64,
    pub cs_rate: f64,

    pub dx: f64,
    pub dy: f64,
    pub dz: f64,

    pub dt: f64,
    pub out_step: usize,
    pub max_step: usize,
}

#[tauri::command]
pub(crate) async fn run_simulation(app: AppHandle, config: Config) -> Result<(), String> {
    let _ = app.emit("simulation-log", "Simulation started.");

    let dt = config.dt;
    let out_step = config.out_step;
    let max_step = config.max_step;

    let _ = app.emit("simulation-log", format!("config = {:?}", config));

    let _ = std::fs::create_dir_all("./results");
    sph(dt, out_step, max_step, Some("results/checkpoint.bin")).map_err(|e| e.to_string())
}
