use super::parameters::{DIM, MAX_NEAR_SUM};
use super::sph::Particle;
use anyhow::{bail, Context, Ok, Result};
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct NeighoringList<const N: usize> {
    pair_i: usize,
    pair_j: usize,
    w: f64,
    dwdq: [f64; N],
}
impl<const N: usize> NeighoringList<N> {
    fn new() -> Self {
        Self {
            pair_i: 0,
            pair_j: 0,
            w: 0.0,
            dwdq: [0.0; N],
        }
    }
}

pub fn distance(x1: &[f64], x2: &[f64]) -> f64 {
    x1.par_iter()
        .zip(x2.par_iter())
        .map(|(xi, yi)| (xi - yi).powi(2))
        .sum()
}

pub fn b_spline_kernel(q: f64) -> (f64, f64) {
    match q {
        0.0..=1.0 => {
            let q2 = q.powf(2.0);
            let w = 1.0 - 1.5 * q2 + 0.75 * q2 * q;
            let dwdq = -3.0 * q + 2.25 * q2;
            (w, dwdq)
        }
        1.0..=2.0 => {
            let q3 = (2.0 - q).powf(3.0);
            let w = 0.25 * q3;
            let dwdq = -0.75 * q3;
            (w, dwdq)
        }
        _ => (0.0, 0.0),
    }
}

pub fn search_near_particles(particles: &mut [Particle]) -> Result<()> {
    let mut neigh_list: Vec<NeighoringList<DIM>> = vec![NeighoringList::new(); MAX_NEAR_SUM];

    let smooth_length = 0.1;

    // i -> j loop
    let mut total_pair: usize = 0;

    for i in 0..particles.len() {
        for j in i..particles.len() {
            let d = distance(&particles[i].x, &particles[j].x);

            if total_pair >= MAX_NEAR_SUM {
                bail!("Exceeded the maximum number of pair particles.");
            }

            if 0.0 < d && d < (2.0 * smooth_length) * (2.0 * smooth_length) {
                total_pair += 1;

                let r = d.sqrt();
                let q = r / smooth_length;

                let (w, dwdq) = b_spline_kernel(q);
                let mut dwdr = [dwdq / smooth_length; DIM];
                dwdr[0] *= particles[i].x[0] / r;
                dwdr[1] *= particles[i].x[1] / r;
                dwdr[2] *= particles[i].x[2] / r;

                neigh_list[total_pair].w = w;
                neigh_list[total_pair].dwdq = dwdr;
            }
        }
    }
    // dbg!(total_pair);
    write_kernel_to_csv(&neigh_list[0..total_pair])?;

    Ok(())
}

// Write only the particles created
pub fn write_kernel_to_csv(neibors: &[NeighoringList<DIM>]) -> Result<()> {
    let filename = "./results/kernel.csv";
    let mut csv = String::new();

    // CSV header
    csv.push_str("num,w,dwdq1,dwdq2,dwdq3\n");

    // Output coordinates of the particles created in `make_model`
    for (i, neigh) in neibors.iter().enumerate() {
        csv.push_str(&format!(
            "{},{:.3},{:.3},{:.3},{:.3}\n",
            i + 1,
            neigh.w,
            neigh.dwdq[0],
            neigh.dwdq[1],
            neigh.dwdq[2]
        ));
    }

    std::fs::write(filename, &csv).context("Failed to create CSV file")?;
    Ok(())
}
