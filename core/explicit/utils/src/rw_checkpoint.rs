use crate::parameters::Particle;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct State<const D: usize> {
    pub step: usize,
    pub time: f64,
    pub dt: f64,
    pub n: usize,
    pub particles: Vec<Particle<D>>,
}

/// # Errors
pub fn load_checkpoint<const D: usize, P>(path: P) -> Result<State<D>>
where
    P: AsRef<std::path::Path>,
{
    let buf = std::fs::read(path)?;
    let state: State<D> = postcard::from_bytes(&buf)?;
    Ok(state)
}

/// # Errors
pub fn write_checkpoint<const D: usize>(
    path: impl AsRef<std::path::Path>,
    state: &State<D>,
    buffer_size: usize,
) -> anyhow::Result<()> {
    let mut buf: Vec<u8> = vec![0_u8; buffer_size];
    let used = postcard::to_slice(state, &mut buf)?.len();
    let mut file = File::create(path)?;

    file.write_all(&buf[..used])?;
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
