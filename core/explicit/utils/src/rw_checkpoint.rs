use crate::error::{FailedReadFileSnafu, FailedWriteFileSnafu, PostcardSnafu, SimError};
use crate::parameters::{CheckpointConfig, DIM, NeighboringList, Particle};
use serde::{Deserialize, Serialize};
use snafu::ResultExt as _;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct State<'a, const D: usize> {
    #[serde(bound(deserialize = "Cow<'a, CheckpointConfig>: serde::Deserialize<'de>"))]
    pub checkpoint_config: Cow<'a, CheckpointConfig>,
    #[serde(bound(deserialize = "Cow<'a, [Particle<D>]>: serde::Deserialize<'de>"))]
    pub particles: Cow<'a, [Particle<D>]>,
    #[serde(bound(deserialize = "Cow<'a, [NeighboringList<D>]>: serde::Deserialize<'de>"))]
    pub neighbors: Cow<'a, [NeighboringList<D>]>,
    pub step: usize,
    pub time: f64,
}

/// # Errors
pub fn read_checkpoint_and_set_buffer<P>(path: P) -> Result<Vec<u8>, SimError>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    std::fs::read(path).with_context(|_| FailedReadFileSnafu {
        path: path.to_path_buf(),
    })
}

/// # Errors
pub fn load_data_from_checkpoint<'a, const D: usize, P>(path: P, buf: &'a [u8]) -> Result<State<'a, D>, SimError>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    let state: State<D> = postcard::from_bytes(buf).with_context(|_| PostcardSnafu {
        path: path.to_path_buf(),
    })?;

    Ok(state)
}

/// # Errors
pub fn write_checkpoint<const D: usize>(path: impl AsRef<std::path::Path>, state: &State<D>) -> Result<(), SimError> {
    let path = path.as_ref();
    let buf = postcard::to_allocvec(state).with_context(|_| PostcardSnafu {
        path: path.to_path_buf(),
    })?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|_| FailedWriteFileSnafu {
            path: parent.to_path_buf(),
        })?;
    }

    std::fs::write(path, buf).with_context(|_| FailedWriteFileSnafu {
        path: path.to_path_buf(),
    })?;
    Ok(())
}

/// # Errors
pub fn write_sim_checkpoint(
    out_file: impl AsRef<std::path::Path>,
    config: &CheckpointConfig,
    particles: &[Particle<DIM>],
    neighbors: &[NeighboringList<DIM>],
    step: usize,
    time: f64,
) -> Result<(), SimError> {
    let state = State {
        checkpoint_config: Cow::Borrowed(config),
        particles: Cow::Borrowed(particles),
        neighbors: Cow::Borrowed(neighbors),
        step,
        time,
    };

    write_checkpoint(out_file, &state)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::{Config, DIM, Fluid, Particle};

    #[test]
    fn test_checkpoint_roundtrip() {
        // Write and read bin
        let test_file = "test_checkpoint.bin";

        let config = Config::default();
        let n = 3;
        let water = Fluid::Water;
        let particles: Vec<Particle<DIM>> = (0..n).map(|_| Particle::new(water)).collect();
        let neighbors: Vec<NeighboringList<DIM>> = (0..n).map(|_| NeighboringList::default()).collect();

        let step = 10;
        let time = config.checkpoint_config.dt * step as f64;
        let state = State {
            checkpoint_config: Cow::Borrowed(&config.checkpoint_config),
            particles: Cow::Borrowed(&particles[0..n]),
            neighbors: Cow::Borrowed(&neighbors[0..n]),
            step,
            time,
        };

        write_checkpoint(test_file, &state).expect("write failed");

        let buf = read_checkpoint_and_set_buffer(test_file).expect("load failed");
        let state = load_data_from_checkpoint::<DIM, _>(test_file, &buf).expect("load failed");
        let loaded_step = state.step;
        let loaded_time = state.time;
        let loaded_dt = state.checkpoint_config.dt;
        let loaded_n = state.particles.len();
        let loaded_particles = state.particles;
        let loaded_neighbors = state.neighbors;

        assert_eq!(loaded_step, step);
        assert!((loaded_time - time).abs() < 1e-10);
        assert!((loaded_dt - config.checkpoint_config.dt).abs() < 1e-10);
        assert_eq!(loaded_n, n);
        assert_eq!(loaded_particles.len(), n);
        assert_eq!(loaded_particles, particles);
        assert_eq!(loaded_neighbors, neighbors);
        println!("Load and assert checkpoint file.");

        std::fs::remove_file(test_file).unwrap();
        println!("Remove checkpoint file.");
    }
}
