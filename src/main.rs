use anyhow::Result;
use rust_fluid_3d::explicit::sph::sph;

fn main() -> Result<()> {
    let dt = 1e-5; // time step [s]
    let out_step = 10; // output step
    let max_step = 300000;

    // sph(dt, out_step, max_step, None)?;
    sph(dt, out_step, max_step, Some("results/checkpoint.bin"))?;
    Ok(())
}
