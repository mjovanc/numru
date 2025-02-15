use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::{Array, Dimension};

/// A builder for computing the maximum values of an array.
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
    /// Creates a new `MaxBuilder` with the given array.
    pub fn new(array: &'a Array<T, D>) -> Self {
        Self { array, axis: None }
    }

    /// Sets the axis along which to compute the maximum.
    pub fn axis(mut self, axis: usize) -> Self {
        self.axis = Some(axis);
        self
    }

    /// Computes the maximum values based on the current configuration.
    pub fn compute(self) -> Vec<T> {
        self.array.max_compute(self.axis).unwrap()
    }
}

/// A builder for computing the minimum values of an array.
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
    /// Creates a new `MinBuilder` with the given array.
    pub fn new(array: &'a Array<T, D>) -> Self {
        Self { array, axis: None }
    }

    /// Sets the axis along which to compute the minimum.
    pub fn axis(mut self, axis: usize) -> Self {
        self.axis = Some(axis);
        self
    }

    /// Computes the minimum values based on the current configuration.
    pub fn compute(self) -> Vec<T> {
        self.array.min_compute(self.axis).unwrap()
    }
}

/// A builder for computing the mean values of an array.
pub struct MeanBuilder<'a, T, D> 
where
    T: PartialOrd + Copy + Into<f64>,
    D: Dimension
{
    array: &'a Array<T,D>,
    axis: Option<usize>    
}

impl<'a, T, D> MeanBuilder<'a, T, D> 
where
    T: PartialOrd + Copy + Into<f64>,
    D: Dimension
{
    /// Creates a new `MeanBuilder` with the given array.
    pub fn new(array: &'a Array<T, D>) -> Self {
        Self { array, axis: None }
    }

    /// Sets the axis along which to compute the minimum.
    pub fn axis(mut self, axis: usize) -> Self {
        self.axis = Some(axis);
        self
    }

    /// Computes the mean values based on the current configuration.
    pub fn compute(self) -> Vec<f64> {
        self.array.mean_compute(self.axis).unwrap()
    }
}

impl<T: PartialOrd + Copy, D: Dimension> Array<T, D> {
    /// Starts building a computation for the maximum values of this array.
    pub fn max(&self) -> MaxBuilder<T, D> {
        MaxBuilder::new(self)
    }

    /// Starts building a computation for the minimum values of this array.
    pub fn min(&self) -> MinBuilder<T, D> {
        MinBuilder::new(self)
    }

    /// Starts building a computation for the mean values of this array.
    pub fn mean(&self) -> MeanBuilder<T, D> 
    where 
        T: Into<f64>
    {
        MeanBuilder::new(self)
    }

}

impl<T, D> Debug for MaxBuilder<'_, T, D>
where
    T: PartialOrd + Copy,
    D: Dimension,
{
    /// Formats the `MaxBuilder` for debugging.
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
    /// Formats the `MinBuilder` for debugging.
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
    T: PartialOrd + Copy + Into<f64>,
    D: Dimension,
{
    /// Formats the `MeanBuilder` for debugging.
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