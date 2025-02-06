use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub struct Shape<D> {
    dims: D,
}

impl<D> Shape<D>
where
    D: Dimension,
{
    pub fn new(dims: D) -> Self {
        Shape { dims }
    }

    pub fn raw_dim(&self) -> &D {
        &self.dims
    }

    pub fn size(&self) -> usize {
        self.dims.size()
    }
}

impl<D> Debug for Shape<D>
where
    D: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Shape={:?}", self.dims)
    }
}

impl<D> From<D> for Shape<D>
where
    D: Dimension,
{
    fn from(dimension: D) -> Shape<D> {
        Shape { dims: dimension }
    }
}

pub trait Dimension {
    fn ndim(&self) -> usize;
    fn size(&self) -> usize;
}

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
}