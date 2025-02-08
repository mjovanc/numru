use crate::Dimension;

/// Fixed-size index type for multi-dimensional arrays.
///
/// This structure represents the dimensionality of an array, where `N` is the number of dimensions.
/// For example, `Ix<2>` would represent a 2D array, while `Ix<3>` would represent a 3D array.
#[derive(Debug, Clone, Copy)]
pub struct Ix<const N: usize> {
    dims: [usize; N],
}

impl<const N: usize> Ix<N> {
    /// Creates a new `Ix` from a fixed-size array of dimensions.
    pub fn new(dims: [usize; N]) -> Self {
        Ix { dims }
    }
}

impl<const N: usize> Dimension for Ix<N> {
    /// Returns the number of dimensions represented by this `Ix`.
    fn ndim(&self) -> usize {
        N
    }

    /// Calculates the total number of elements in the array described by this `Ix`.
    ///
    /// This is the product of all dimensions.
    fn size(&self) -> usize {
        self.dims.iter().product()
    }

    /// Returns a slice of the dimensions stored in this `Ix`.
    fn dims(&self) -> &[usize] {
        &self.dims
    }
}
