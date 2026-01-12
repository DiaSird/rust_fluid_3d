mod gui_config;
mod gui_state;
mod send_listen;

use tauri::Window;
use utils::parameters::{Config, DIM};

use gui_config::GuiConfig;
use send_listen::{new_stop_listener, sender};

use crate::run_sim::gui_state::GuiState;

#[tauri::command]
pub(crate) async fn run_simulation(window: Window, config: GuiConfig) -> Result<(), String> {
    let mut config: Config = config.into();
    config.log_report = Some(Box::new(sender(window.clone(), "terra://simulation-log")));
    config.stop_step = Some(new_stop_listener(window));

    sph::sph::sph(config).map_err(|e| e.to_string())
}

#[tauri::command]
pub(crate) async fn load_particle_state(path: std::path::PathBuf) -> Result<GuiState<DIM>, String> {
    GuiState::new(&path).map_err(|e| e.to_string())
}
