use serde::{Deserialize, Serialize};
use utils::{
    error::SimError,
    parameters::{DIM, Particle},
    rw_checkpoint::{State, load_data_from_checkpoint, read_checkpoint_and_set_buffer},
};

#[derive(Serialize, Deserialize)]
pub struct GuiState<const D: usize> {
    pub particles: Vec<Particle<D>>,
    pub step: usize,
    pub time: f64,
}

impl<'a, const D: usize> From<State<'a, D>> for GuiState<D> {
    fn from(value: State<'a, D>) -> Self {
        Self {
            particles: value.particles.into_owned(),
            step: value.step,
            time: value.time,
        }
    }
}

impl GuiState<DIM> {
    pub fn new(path: &std::path::Path) -> Result<Self, SimError> {
        let buf = read_checkpoint_and_set_buffer(path)?;
        let state = load_data_from_checkpoint::<DIM, _>(path, &buf)?;
        Ok(state.into())
    }
}
