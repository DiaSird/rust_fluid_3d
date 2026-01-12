use crate::parameters::{BC, LogReporterFn, particle_status::StopJudgeFn};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelScale {
    // Model size
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Resolution {
    // Resolution
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub checkpoint_config: CheckpointConfig,
    #[serde(skip)]
    pub log_report: Option<LogReporterFn>,
    #[serde(skip)]
    pub stop_step: Option<StopJudgeFn>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckpointConfig {
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
    pub out_file: std::path::PathBuf,

    // Monitoring and log report
    pub monitor_particle: usize,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            // particle config
            max_n: 60000,
            max_near_n: 100,

            // model scale
            model_scale: ModelScale {
                length: 0.5,
                width: 0.5,
                height: 0.5,
            },

            // boundary condition
            bc_pattern: BC::CavityFlow,
            u_lid: 5.0,

            // SPH parameters
            smooth_length: 0.0324,
            cell_scale: 2.0,
            beta: 0.3,
            cs_rate: 0.05,

            // resolution
            dx: Resolution {
                dx: 0.027,
                dy: 0.027,
                dz: 0.027,
            },

            // time control
            dt: 0.001,
            out_step: 10,
            max_step: 10_000,

            // misc
            restart_file: None,
            out_file: std::path::PathBuf::from("./sim_checkpoint.bin"),
            monitor_particle: 0,
        }
    }
}
