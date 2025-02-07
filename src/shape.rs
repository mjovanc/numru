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

    pub fn dims(&self) -> &[usize] {
        self.dims.dims()
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
    fn dims(&self) -> &[usize];
}
