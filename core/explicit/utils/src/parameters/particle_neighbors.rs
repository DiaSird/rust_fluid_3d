use crate::parameters::Vector;

// SPH Neighboring List
#[derive(Debug, PartialEq)]
pub struct NeighboringList<const DIM: usize> {
    pub i: usize, // pair i
    pub j: usize, // pair j
    pub w: f64,
    pub dwdr: Vector<DIM>,
}

impl<const DIM: usize> Default for NeighboringList<DIM> {
    fn default() -> Self {
        Self {
            i: 0,
            j: 0,
            w: 0.0,
            dwdr: Vector::zeros(),
        }
    }
}

impl<const DIM: usize> NeighboringList<DIM> {
    pub fn kernel_axis3(&self) -> (f64, f64, f64) {
        let dwdr1 = self.dwdr[0];
        let dwdr2 = self.dwdr[1];
        let dwdr3 = self.dwdr[2];
        (dwdr1, dwdr2, dwdr3)
    }
}
