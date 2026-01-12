use nalgebra as na;
use rayon::prelude::*;
use utils::parameters::{DIM, NeighboringList as Neighbor, Particle};

pub(crate) fn update_artificial_viscosity(
    particles: &mut [Particle<DIM>],
    neighbors: &[Neighbor<DIM>],
    smooth_length: f64,
    beta: f64,
) {
    let n = particles.len();

    // Parallel computation using per-thread buffers (fold + reduce)
    let stress_buf = neighbors
        .par_iter()
        .fold(
            || vec![na::Matrix3::zeros(); n], // thread-local buffer
            |mut local_buf, neigh| {
                // Average
                let rho_ij = 0.5 * (particles[neigh.i].rho + particles[neigh.j].rho);
                let cij = 0.5 * (particles[neigh.i].sound_v + particles[neigh.j].sound_v);

                // Relative distance
                let vij = na::Vector3::from(particles[neigh.i].v) - na::Vector3::from(particles[neigh.j].v);
                let xij = na::Vector3::from(particles[neigh.i].x) - na::Vector3::from(particles[neigh.j].x);

                let v_dot_x = vij.dot(&xij);
                if v_dot_x < 0.0 {
                    let coef = v_dot_x / (0.1 * smooth_length).mul_add(0.1 * smooth_length, xij.dot(&xij));
                    let identity = na::Matrix3::identity();
                    let coef_val = (-beta * cij).mul_add(coef, beta * coef.powi(2)) / rho_ij;

                    // add stress contributions to local buffer
                    local_buf[neigh.i] += coef_val * identity;
                    local_buf[neigh.j] += coef_val * identity;
                }
                local_buf
            },
        )
        .reduce(
            || vec![na::Matrix3::zeros(); n],
            |mut a, b| {
                // merge two buffers
                for i in 0..n {
                    a[i] += b[i];
                }
                a
            },
        );

    // Add the accumulated stress to particles
    for (i, s) in stress_buf.iter().enumerate() {
        particles[i].stress += *s;
    }
}
