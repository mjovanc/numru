use crate::shape::Dimension;

/// Fixed-size index type (e.g., `Ix<2>` for 2D, `Ix<3>` for 3D)
#[derive(Debug, Clone, Copy)]
pub struct Ix<const N: usize> {
    dims: [usize; N],
}

impl<const N: usize> Ix<N> {
    pub fn new(dims: [usize; N]) -> Self {
        Ix { dims }
    }
}

impl<const N: usize> Dimension for Ix<N> {
    fn ndim(&self) -> usize {
        N
    }

    fn size(&self) -> usize {
        self.dims.iter().product()
    }

    fn dims(&self) -> &[usize] {
        &self.dims
    }
}
