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

#[derive(Debug, Clone)]
pub struct StrideShape<D> {
    _dims: D,
    strides: D,
}

impl<D> StrideShape<D>
where
    D: Dimension,
{
    pub fn new(_dims: D, strides: D) -> Self {
        StrideShape { _dims, strides }
    }

    pub fn strides(&self) -> &D {
        &self.strides
    }

    pub fn set_strides(mut self, strides: D) -> Self {
        self.strides = strides;
        self
    }
}

pub trait Dimension {
    fn ndim(&self) -> usize;
    fn size(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct IxDyn {
    dims: Vec<usize>,
}

impl IxDyn {
    pub fn new(dims: Vec<usize>) -> Self {
        IxDyn { dims }
    }
}

impl Dimension for IxDyn {
    fn ndim(&self) -> usize {
        self.dims.len()
    }
    fn size(&self) -> usize {
        self.dims.iter().product()
    }
}
