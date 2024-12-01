use anyhow::Result;
use rust_fluid_3d::explicit::sph::sph;

fn main() -> Result<()> {
    let dt = 1e-6; // time step [s]
    let out_step = 50;
    let max_step = 1000;

    sph(dt, out_step, max_step)
}
