use anyhow::Result;
use rust_fluid_3d::explicit::sph::sph;

fn main() -> Result<()> {
    // Initialize
    let dt = 0.001; // step time

    // Simulation Tests
    let max_step = 100;

    sph(dt, max_step)
}
