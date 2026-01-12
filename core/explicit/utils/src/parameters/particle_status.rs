use crate::parameters::Vector;
use std::fmt::Debug;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ParticleLog {
    RestartInfo {
        step: Option<usize>,
        time: f64,
        message: String,
    },
    LogInfo(String),
    Info3 {
        monitor_particle: usize,
        step: usize,
        time: f64,

        /// location vector [m]
        x: Vector<3>,
        /// velocity [m/s]
        v: Vector<3>,
        /// acceleration [m/s^2]
        dvdt: Vector<3>,
        // TODO: Energy and Temperature
    },
}

// type alias: Reporting particle status
pub type LogReporterFn = Box<dyn Fn(ParticleLog) + Send + Sync>;
/// (step: usize) -> bool
/// - true: then stop simulation
pub type StopJudgeFn = Box<dyn Fn(usize) -> bool + Send + Sync>;
