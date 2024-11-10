use anyhow::Result;
use rust_fluid_3d::explicit::sph::sph;

fn main() -> Result<()> {
    let dt = 0.001; // time step [s]
    let out_step = 10;
    let max_step = 100;

    sph(dt, out_step, max_step)
}
