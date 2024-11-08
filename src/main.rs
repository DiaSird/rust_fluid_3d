use anyhow::Result;
use rust_fluid_3d::explicit::sph::sph;

fn main() -> Result<()> {
    let dt = 0.001; // time step [s]
    let max_step = 100;

    sph(dt, max_step)
}
