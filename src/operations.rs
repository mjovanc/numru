use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Div;

use crate::{Array, Dimension};

pub struct MaxBuilder<'a, T, D>
where
    T: PartialOrd + Copy,
    D: Dimension,
{
    array: &'a Array<T, D>,
    axis: Option<usize>,
}

impl<'a, T, D> MaxBuilder<'a, T, D>
where
    T: PartialOrd + Copy,
    D: Dimension,
{
    pub fn new(array: &'a Array<T, D>) -> Self {
        Self { array, axis: None }
    }

    pub fn axis(mut self, axis: usize) -> Self {
        self.axis = Some(axis);
        self
    }

    pub fn compute(self) -> Vec<T> {
        self.array.max_compute(self.axis)
    }
}

pub struct MinBuilder<'a, T, D>
where
    T: PartialOrd + Copy,
    D: Dimension,
{
    array: &'a Array<T, D>,
    axis: Option<usize>,
}

impl<'a, T, D> MinBuilder<'a, T, D>
where
    T: PartialOrd + Copy,
    D: Dimension,
{
    pub fn new(array: &'a Array<T, D>) -> Self {
        Self { array, axis: None }
    }

    pub fn axis(mut self, axis: usize) -> Self {
        self.axis = Some(axis);
        self
    }

    pub fn compute(self) -> Vec<T> {
        self.array.min_compute(self.axis)
    }
}

pub struct MeanBuilder<'a, T, D>
where
    T: Copy + Add<Output = T> + Div<f64, Output = T> + Sum<T>,
    D: Dimension,
{
    array: &'a Array<T, D>,
    axis: Option<usize>,
}

impl<'a, T, D> MeanBuilder<'a, T, D>
where
    T: Copy + Add<Output = T> + Div<f64, Output = T> + Sum<T>,
    D: Dimension,
{
    pub fn new(array: &'a Array<T, D>) -> Self {
        Self { array, axis: None }
    }

    pub fn axis(mut self, axis: usize) -> Self {
        self.axis = Some(axis);
        self
    }

    pub fn compute(self) -> Vec<T> {
        self.array.mean_compute(self.axis)
    }
}

impl<T: PartialOrd + Copy, D: Dimension> Array<T, D> {
    pub fn max(&self) -> MaxBuilder<T, D> {
        MaxBuilder::new(self)
    }

    pub fn min(&self) -> MinBuilder<T, D> {
        MinBuilder::new(self)
    }
}

impl<T, D> Debug for MaxBuilder<'_, T, D>
where
    T: PartialOrd + Copy,
    D: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaxBuilder")
            .field(
                "array",
                &format_args!(
                    "Array<{}, {}>",
                    std::any::type_name::<T>(),
                    std::any::type_name::<D>()
                ),
            )
            .field("axis", &self.axis)
            .finish()
    }
}

impl<T, D> Debug for MinBuilder<'_, T, D>
where
    T: PartialOrd + Copy,
    D: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MinBuilder")
            .field(
                "array",
                &format_args!(
                    "Array<{}, {}>",
                    std::any::type_name::<T>(),
                    std::any::type_name::<D>()
                ),
            )
            .field("axis", &self.axis)
            .finish()
    }
}

impl<T, D> Debug for MeanBuilder<'_, T, D>
where
    T: Copy + Add<Output = T> + Div<f64, Output = T> + Sum<T>,
    D: Dimension,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeanBuilder")
            .field(
                "array",
                &format_args!(
                    "Array<{}, {}>",
                    std::any::type_name::<T>(),
                    std::any::type_name::<D>()
                ),
            )
            .field("axis", &self.axis)
            .finish()
    }
}
