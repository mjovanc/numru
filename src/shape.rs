use std::fmt::{Debug, Formatter, Result};

use crate::Dimension;

/// Represents the shape of an array or matrix, encapsulating the dimensions.
///
/// This structure wraps a type that implements `Dimension`, allowing for
/// flexible handling of array shapes in different dimensional contexts.
#[derive(Clone)]
pub struct Shape<D> {
    dims: D,
}

impl<D> Shape<D>
where
    D: Dimension,
{
    /// Constructs a new `Shape` from a given dimension.
    pub fn new(dims: D) -> Self {
        Shape { dims }
    }

    /// Returns a reference to the underlying dimension structure.
    pub fn raw_dim(&self) -> &D {
        &self.dims
    }

    /// Calculates and returns the total number of elements in the array.
    ///
    /// This is essentially the product of all dimensions.
    pub fn size(&self) -> usize {
        self.dims.size()
    }

    /// Returns a slice of the dimensions.
    pub fn dims(&self) -> &[usize] {
        self.dims.dims()
    }
}

impl<D> Debug for Shape<D>
where
    D: Debug,
{
    /// Formats the `Shape` for debugging purposes.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Shape={:?}", self.dims)
    }
}

impl<D> From<D> for Shape<D>
where
    D: Dimension,
{
    /// Converts a `Dimension` into a `Shape`.
    fn from(dimension: D) -> Shape<D> {
        Shape { dims: dimension }
    }
}
