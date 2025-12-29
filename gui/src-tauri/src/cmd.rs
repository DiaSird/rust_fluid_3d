use rust_fluid_3d::explicit::sph::sph;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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
}

#[tauri::command]
pub(crate) async fn set_model_config(config: Config) -> Result<(), String> {
    let dt = 1e-5; // time step [s]
    let out_step = 10; // output step
    let max_step = 100;

    let _ = std::fs::create_dir_all("./results");
    log::info!("config = {:?}", config);

    sph(dt, out_step, max_step, Some("results/checkpoint.bin")).map_err(|e| e.to_string())
}
