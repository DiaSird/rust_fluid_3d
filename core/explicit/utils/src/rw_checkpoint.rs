use crate::error::{FailedReadFileSnafu, FailedWriteFileSnafu, PostcardSnafu, SimError};
use crate::parameters::{DIM, Particle};
use serde::{Deserialize, Serialize};
use snafu::ResultExt as _;

#[derive(Serialize, Deserialize)]
pub struct State<const D: usize> {
    pub step: usize,
    pub time: f64,
    pub dt: f64,
    pub n: usize,
    pub particles: Vec<Particle<D>>,
}

/// # Errors
pub fn load_checkpoint<const D: usize, P>(path: P) -> Result<State<D>, SimError>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    let buf = std::fs::read(path).with_context(|_| FailedReadFileSnafu {
        path: path.to_path_buf(),
    })?;
    let state: State<D> = postcard::from_bytes(&buf).with_context(|_| PostcardSnafu {
        path: path.to_path_buf(),
    })?;

    Ok(state)
}

/// # Errors
pub fn write_checkpoint<const D: usize>(
    path: impl AsRef<std::path::Path>,
    state: &State<D>,
    buffer_size: usize,
) -> Result<(), SimError> {
    let path = path.as_ref();
    let mut buf: Vec<u8> = vec![0_u8; buffer_size];
    let used = postcard::to_slice(state, &mut buf)
        .with_context(|_| PostcardSnafu {
            path: path.to_path_buf(),
        })?
        .len();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|_| FailedWriteFileSnafu {
            path: parent.to_path_buf(),
        })?;
    }

    std::fs::write(path, buf[..used].iter().as_slice()).with_context(|_| FailedWriteFileSnafu {
        path: path.to_path_buf(),
    })?;
    Ok(())
}

/// # Errors
pub fn write_sim_checkpoint(
    out_file: impl AsRef<std::path::Path>,
    step: usize,
    time: f64,
    dt: f64,
    n: usize,
    particles: &[Particle<DIM>],
) -> Result<(), SimError> {
    let state = State {
        step,
        time,
        dt,
        n,
        particles: particles[0..n].to_vec(),
    };
    write_checkpoint(out_file, &state, 1024 * 10000)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::{DIM, Fluid, Particle};
    use std::fs;

    #[test]
    fn test_checkpoint_roundtrip() {
        // Write and read bin
        let test_file = "test_checkpoint.bin";

        let n = 3;
        let water = Fluid::Water;
        let particles: Vec<Particle<DIM>> = (0..n).map(|_| Particle::new(water)).collect();

        let step = 42;
        let time = 1.23;
        let dt = 0.01;

        let state = State {
            step,
            time,
            dt,
            n,
            particles: particles.clone(),
        };

        write_checkpoint(test_file, &state, 1024 * 120).expect("write failed");

        let state = load_checkpoint::<DIM, _>(test_file).expect("load failed");
        let loaded_step = state.step;
        let loaded_time = state.time;
        let loaded_dt = state.dt;
        let loaded_n = state.n;
        let loaded_particles = state.particles;

        assert_eq!(loaded_step, step);
        assert!((loaded_time - time).abs() < 1e-10);
        assert!((loaded_dt - dt).abs() < 1e-10);
        assert_eq!(loaded_n, n);
        assert_eq!(loaded_particles.len(), n);
        assert_eq!(loaded_particles, particles);

        fs::remove_file(test_file).unwrap();
    }
}
