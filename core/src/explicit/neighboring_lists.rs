use super::parameters::{
    NeighboringList as Neighbor, Particle, CELL_SIZE, DIM, MAX_NEAR_SUM, SMOOTH_LENGTH,
};
use anyhow::{bail, Context as _, Result};
use nalgebra as na;
use nalgebra::SimdComplexField;
use std::collections::HashMap;

// Kernel function
pub fn b_spline_kernel(q: f64) -> (f64, f64) {
    match q {
        0.0..=1.0 => {
            let q2 = q.simd_powf(2.0);
            let w = (0.75 * q2).mul_add(q, 1.5_f64.mul_add(-q2, 1.0));
            let dwdq = (-3.0_f64).mul_add(q, 2.25 * q2);
            (w, dwdq)
        }
        1.0..=2.0 => {
            let q3 = (2.0 - q).simd_powf(3.0);
            let w = 0.25 * q3;
            let dwdq = -0.75 * q3;
            (w, dwdq)
        }
        _ => (0.0, 0.0),
    }
}

type Grid = HashMap<(usize, usize, usize), Vec<usize>>;

pub fn cll_property(particles: &[Particle<DIM>]) -> (f64, f64, f64, Grid) {
    // HashMap for cell layout
    let mut grid: Grid = HashMap::new();

    fn min_location(particles: &[Particle<DIM>], index: usize) -> f64 {
        particles
            .iter()
            .map(|p| p.x[index])
            .fold(f64::INFINITY, f64::min)
    }

    // Calculate the minimum coordinates
    let min_x = min_location(particles, 0);
    let min_y = min_location(particles, 1);
    let min_z = min_location(particles, 2);

    fn cell_location(x: f64, min: f64) -> usize {
        ((x - min) / CELL_SIZE).floor() as usize
    }

    // Place particles into cells
    for (i, particle) in particles.iter().enumerate() {
        // Subtract the minimum coordinates and divide by cell size to calculate cell index
        let cell_x = cell_location(particle.x[0], min_x);
        let cell_y = cell_location(particle.x[1], min_y);
        let cell_z = cell_location(particle.x[2], min_z);

        grid.entry((cell_x, cell_y, cell_z)).or_default().push(i);
    }

    (min_x, min_y, min_z, grid)
}

// Searching
pub fn search_near_particles(
    particles: &mut [Particle<DIM>],
    neigh_lists: &mut [Neighbor<DIM>],
) -> Result<usize> {
    let smooth_length_squared = (2.0 * SMOOTH_LENGTH).simd_powf(2.0);
    let (min_x, min_y, min_z, grid) = cll_property(particles);

    // i -> j loop
    let mut total_pair: usize = 0;
    for (i, particle) in particles.iter().enumerate() {
        let cell_x = ((particle.x[0] - min_x) / CELL_SIZE).floor() as isize;
        let cell_y = ((particle.x[1] - min_y) / CELL_SIZE).floor() as isize;
        let cell_z = ((particle.x[2] - min_z) / CELL_SIZE).floor() as isize;

        // Check the 8 surrounding cells (self cell + neighboring cells)
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let neighbor_cell = (cell_x + dx, cell_y + dy, cell_z + dz);

                    if let Some(neighbors) = grid.get(&(
                        neighbor_cell.0 as usize,
                        neighbor_cell.1 as usize,
                        neighbor_cell.2 as usize,
                    )) {
                        for &j in neighbors {
                            if i != j {
                                let d = particles[i].x.metric_distance(&particles[j].x);

                                if total_pair >= MAX_NEAR_SUM {
                                    bail!("Exceeded the maximum number of pair particles.");
                                }

                                // If the distance is valid, add as a neighboring pair
                                if d < smooth_length_squared.simd_sqrt() {
                                    total_pair += 1;

                                    // Store pair particles
                                    neigh_lists[total_pair].i = i;
                                    neigh_lists[total_pair].j = j;

                                    let q = d / SMOOTH_LENGTH;

                                    let (w, dwdq) = b_spline_kernel(q);
                                    let mut dwdr = na::Vector::from([dwdq / SMOOTH_LENGTH; DIM]);

                                    // multiply dwdr by base vector: ei = x/r
                                    dwdr[0] *= particles[i].x[0] / d;
                                    dwdr[1] *= particles[i].x[1] / d;
                                    dwdr[2] *= particles[i].x[2] / d;

                                    neigh_lists[total_pair].w = w;
                                    neigh_lists[total_pair].dwdr = dwdr;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if total_pair == 0 {
        bail!("Total pair particle is zero.");
    }

    make_neighboring_list(particles, &neigh_lists[0..total_pair]);
    write_kernel_to_csv(particles, &neigh_lists[0..total_pair])?;

    Ok(total_pair)
}

pub fn make_neighboring_list(particles: &mut [Particle<DIM>], neighbors: &[Neighbor<DIM>]) {
    for (pair, neigh) in neighbors.iter().enumerate() {
        // calculate pair numbers per one particle
        particles[neigh.i].pair = pair;
    }
}

// Write only the particles created
pub fn write_kernel_to_csv(particles: &[Particle<DIM>], neighbors: &[Neighbor<DIM>]) -> Result<()> {
    let filename = "./results/kernel.csv";
    let mut csv = String::new();

    // CSV header
    csv.push_str("pair,x,y,z,w,dwdq1,dwdq2,dwdq3\n");

    // Output coordinates of the particles created in `make_model`
    for (pair, neigh) in neighbors.iter().enumerate() {
        let Neighbor { i, j, w, .. } = neigh;
        let (x, y, z) = particles[*i].axis();
        let (dwdr1, dwdr2, dwdr3) = neigh.kernel_axis3();

        csv.push_str(&format!(
            "{},{i},{j},{x:.3},{y:.3},{z:.3},{w:.3},{dwdr1:.3},{dwdr2:.3},{dwdr3:.3}\n",
            pair,
        ));
    }

    std::fs::write(filename, &csv).context("Failed to create CSV file")?;
    Ok(())
}
