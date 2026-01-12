use tauri::{Emitter, Listener, Window};
use utils::parameters::{BC, CheckpointConfig, Config, ModelScale, Resolution, StopJudgeFn};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct GuiConfig {
    /// Max particles
    pub max_n: usize,
    pub max_near_n: usize,

    // Model size
    pub model_scale: ModelScale,

    // Boundary Condition
    pub bc_pattern: BC,
    pub u_lid: f64,

    // SPH parameters
    pub smooth_length: f64,
    pub cell_scale: f64,
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

impl From<GuiConfig> for Config {
    fn from(gui_config: GuiConfig) -> Self {
        let restart_file = match gui_config.restart_file {
            Some(restart_file) if restart_file.exists() => Some(restart_file),
            _ => None,
        };
        Self {
            checkpoint_config: CheckpointConfig {
                max_n: gui_config.max_n,
                max_near_n: gui_config.max_near_n,
                model_scale: gui_config.model_scale,
                bc_pattern: gui_config.bc_pattern,
                u_lid: gui_config.u_lid,
                smooth_length: gui_config.smooth_length,
                cell_scale: gui_config.cell_scale,
                beta: gui_config.beta,
                cs_rate: gui_config.cs_rate,
                dx: gui_config.dx,
                dt: gui_config.dt,
                out_step: gui_config.out_step,
                max_step: gui_config.max_step,
                restart_file,
                monitor_particle: gui_config.monitor_particle,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

/// Create closure that reports.
fn sender<S>(window: Window, event: &'static str) -> impl Fn(S) + Clone
where
    S: serde::Serialize + Clone,
{
    move |payload: S| {
        if let Err(err) = window.emit(event, payload) {
            println!("{err}");
            // tracing::error!("{}", err);
        };
    }
}

fn new_stop_listener(window: Window) -> StopJudgeFn {
    let stop_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    {
        let stop_flag = std::sync::Arc::clone(&stop_flag);
        window.listen("terra://simulation-stop-event", move |_event| {
            stop_flag.store(true, std::sync::atomic::Ordering::Release);
        });
    }

    let stop_flag = std::sync::Arc::clone(&stop_flag);
    Box::new(move |_step| stop_flag.load(std::sync::atomic::Ordering::Acquire))
}

#[tauri::command]
pub(crate) async fn run_simulation(window: Window, config: GuiConfig) -> Result<(), String> {
    let mut config: Config = config.into();
    config.log_report = Some(Box::new(sender(window.clone(), "terra://simulation-log")));
    config.stop_step = Some(new_stop_listener(window));

    sph::sph::sph(config).map_err(|e| e.to_string())
}
