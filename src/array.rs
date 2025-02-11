use crate::ArrayError;
use crate::{Dimension, Shape};
use std::fmt::Debug;

/// Represents a multi-dimensional array with elements of type `T` and dimension `D`.
#[derive(Debug)]
pub struct Array<T, D: Dimension> {
    data: Vec<T>,
    shape: Shape<D>,
}

impl<T, D: Dimension> Array<T, D> {
    /// Constructs a new `Array` from a vector of data and a shape.
    ///
    /// # Errors
    ///
    /// Returns `ArrayError::DimensionMismatch` if the length of `data` does not match the size of `shape`,
    /// providing details on the expected and actual number of elements.
    pub fn new(data: Vec<T>, shape: Shape<D>) -> Result<Self, ArrayError> {
        let expected_size = shape.size();
        if data.len() != expected_size {
            return Err(ArrayError::DimensionMismatch {
                expected: expected_size,
                actual: data.len(),
            });
        }
        Ok(Array { data, shape })
    }

    /// Returns a reference to the underlying data vector.
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a reference to the shape of the array.
    pub fn shape(&self) -> &Shape<D> {
        &self.shape
    }
}

impl<D: Dimension> Array<i64, D> {
    /// Returns the data type string for an array of `i64`.
    pub fn dtype(&self) -> &'static str {
        "int64"
    }
}

impl<D: Dimension> Array<f64, D> {
    /// Returns the data type string for an array of `f64`.
    pub fn dtype(&self) -> &'static str {
        "float64"
    }
}

impl<T, D: Dimension> Array<T, D>
where
    T: PartialOrd + Copy,
{
    /// Computes the maximum value(s) of the array along a specified axis or for the whole array.
    ///
    /// # Success
    ///
    /// - If `axis` is `None`, returns a single-element `Vec<T>` with the maximum value of the entire array.
    /// - If `axis` is provided, returns a `Vec<T>` where each element is the maximum along that axis.
    ///
    /// # Errors
    ///
    /// - Returns `ArrayError::EmptyArray` if the array is empty.
    /// - Returns `ArrayError::InvalidAxis` if the specified axis is out of bounds for the array's dimensions.
    /// - Returns `ArrayError::UnimplementedDimension` for unsupported dimensions.
    pub fn max_compute(&self, axis: Option<usize>) -> Result<Vec<T>, ArrayError> {
        if self.data.is_empty() {
            return Err(ArrayError::EmptyArray);
        }

        let raw_dim = self.shape.raw_dim();
        let ndim = raw_dim.ndim();

        if let Some(axis) = axis {
            if axis >= ndim {
                return Err(ArrayError::InvalidAxis(format!(
                    "Axis {} is out of bounds for array with {} dimensions",
                    axis, ndim
                )));
            }
        }

        match ndim {
            1 => Ok(vec![*self
                .data
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .ok_or(ArrayError::EmptyArray)?]),
            2 => {
                let rows = raw_dim.dims()[0];
                let cols = raw_dim.dims()[1];

                if let Some(axis) = axis {
                    if axis == 0 {
                        (0..cols)
                            .map(|col| {
                                (0..rows)
                                    .map(|row| self.data[row * cols + col])
                                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                                    .ok_or(ArrayError::EmptyArray)
                            })
                            .collect::<Result<Vec<T>, _>>()
                    } else {
                        (0..rows)
                            .map(|row| {
                                self.data[row * cols..(row + 1) * cols]
                                    .iter()
                                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                                    .map(|&v| v)
                                    .ok_or(ArrayError::EmptyArray)
                            })
                            .collect::<Result<Vec<T>, _>>()
                    }
                } else {
                    Ok(vec![*self
                        .data
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .ok_or(ArrayError::EmptyArray)?])
                }
            }
            3 => {
                let depth = raw_dim.dims()[0];
                let rows = raw_dim.dims()[1];
                let cols = raw_dim.dims()[2];

                if let Some(axis) = axis {
                    match axis {
                        0 => (0..rows * cols)
                            .map(|i| {
                                (0..depth)
                                    .map(|d| self.data[d * rows * cols + i])
                                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                                    .ok_or(ArrayError::EmptyArray)
                            })
                            .collect::<Result<Vec<T>, _>>(),
                        1 => (0..depth)
                            .flat_map(|d| {
                                (0..cols).map(move |c| {
                                    (0..rows)
                                        .map(|r| self.data[d * rows * cols + r * cols + c])
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .ok_or(ArrayError::EmptyArray)
                                })
                            })
                            .collect::<Result<Vec<T>, _>>(),
                        2 => (0..depth)
                            .flat_map(|d| {
                                (0..rows).map(move |r| {
                                    let row_start = d * rows * cols + r * cols;
                                    self.data[row_start..row_start + cols]
                                        .iter()
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .map(|&v| v)
                                        .ok_or(ArrayError::EmptyArray)
                                })
                            })
                            .collect::<Result<Vec<T>, _>>(),
                        _ => unreachable!(),
                    }
                } else {
                    Ok(vec![*self
                        .data
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .ok_or(ArrayError::EmptyArray)?])
                }
            }
            _ => Err(ArrayError::UnimplementedDimension(format!(
                "Dimension {} for max computation not implemented",
                ndim
            ))),
        }
    }

    /// Computes the minimum value(s) of the array along a specified axis or for the whole array.
    ///
    /// # Success
    ///
    /// - If `axis` is `None`, returns a single-element `Vec<T>` with the minimum value of the entire array.
    /// - If `axis` is provided, returns a `Vec<T>` where each element is the minimum along that axis.
    ///
    /// # Errors
    ///
    /// - Returns `ArrayError::EmptyArray` if the array is empty.
    /// - Returns `ArrayError::InvalidAxis` if the specified axis is out of bounds for the array's dimensions.
    /// - Returns `ArrayError::UnimplementedDimension` for unsupported dimensions.
    pub fn min_compute(&self, axis: Option<usize>) -> Result<Vec<T>, ArrayError> {
        if self.data.is_empty() {
            return Err(ArrayError::EmptyArray);
        }

        let raw_dim = self.shape.raw_dim();
        let ndim = raw_dim.ndim();

        if let Some(axis) = axis {
            if axis >= ndim {
                return Err(ArrayError::InvalidAxis(format!(
                    "Axis {} is out of bounds for array with {} dimensions",
                    axis, ndim
                )));
            }
        }

        match ndim {
            1 => Ok(vec![*self
                .data
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .ok_or(ArrayError::EmptyArray)?]),
            2 => {
                let rows = raw_dim.dims()[0];
                let cols = raw_dim.dims()[1];

                if let Some(axis) = axis {
                    if axis == 0 {
                        (0..cols)
                            .map(|col| {
                                (0..rows)
                                    .map(|row| self.data[row * cols + col])
                                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                                    .ok_or(ArrayError::EmptyArray)
                            })
                            .collect::<Result<Vec<T>, _>>()
                    } else {
                        (0..rows)
                            .map(|row| {
                                self.data[row * cols..(row + 1) * cols]
                                    .iter()
                                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                                    .map(|&v| v)
                                    .ok_or(ArrayError::EmptyArray)
                            })
                            .collect::<Result<Vec<T>, _>>()
                    }
                } else {
                    Ok(vec![*self
                        .data
                        .iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .ok_or(ArrayError::EmptyArray)?])
                }
            }
            3 => {
                let depth = raw_dim.dims()[0];
                let rows = raw_dim.dims()[1];
                let cols = raw_dim.dims()[2];

                if let Some(axis) = axis {
                    match axis {
                        0 => (0..rows * cols)
                            .map(|i| {
                                (0..depth)
                                    .map(|d| self.data[d * rows * cols + i])
                                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                                    .ok_or(ArrayError::EmptyArray)
                            })
                            .collect::<Result<Vec<T>, _>>(),
                        1 => (0..depth)
                            .flat_map(|d| {
                                (0..cols).map(move |c| {
                                    (0..rows)
                                        .map(|r| self.data[d * rows * cols + r * cols + c])
                                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                                        .ok_or(ArrayError::EmptyArray)
                                })
                            })
                            .collect::<Result<Vec<T>, _>>(),
                        2 => (0..depth)
                            .flat_map(|d| {
                                (0..rows).map(move |r| {
                                    let row_start = d * rows * cols + r * cols;
                                    self.data[row_start..row_start + cols]
                                        .iter()
                                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                                        .map(|&v| v)
                                        .ok_or(ArrayError::EmptyArray)
                                })
                            })
                            .collect::<Result<Vec<T>, _>>(),
                        _ => unreachable!(),
                    }
                } else {
                    Ok(vec![*self
                        .data
                        .iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .ok_or(ArrayError::EmptyArray)?])
                }
            }
            _ => Err(ArrayError::UnimplementedDimension(format!(
                "Dimension {} for min computation not implemented",
                ndim
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI, TAU};

    use crate::{Dimension, Ix, Shape};

    #[test]
    fn array_creation_i64_1d() {
        let data = arr![1, 2, 3, 4];
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 4);
        assert_eq!(data.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_i64_2d() {
        let data = arr![[1, 2], [3, 4], [5, 6]];
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 6);
        assert_eq!(data.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_i64_3d() {
        let data = arr![[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]];
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 12);
        assert_eq!(data.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_f64_1d() {
        let data = arr![1.1, 2.2, 3.3, 4.4];
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 4);
        assert_eq!(data.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_f64_2d() {
        let data = arr![[1.1, 2.2], [3.3, 4.4], [5.5, 6.6]];
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 6);
        assert_eq!(data.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_f64_3d() {
        let data = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 12);
        assert_eq!(data.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn max_i64_1d() {
        let data = arr![42, -17, 256, 3, 99, -8];
        assert_eq!(data.max().compute(), vec![256]);
    }

    #[test]
    fn max_f64_1d() {
        let data = arr![PI, 2.71, -1.0, 42.0, 0.98];
        assert_eq!(data.max().compute(), vec![42.0]);
    }

    #[test]
    fn max_i64_2d() {
        let data = arr![[1, 5, 3], [4, 2, 6], [0, 9, 8]];
        assert_eq!(data.max().compute(), vec![9]);
        assert_eq!(data.max().axis(0).compute(), vec![4, 9, 8]);
        assert_eq!(data.max().axis(1).compute(), vec![5, 6, 9]);
    }

    #[test]
    fn max_f64_2d() {
        let data = arr![[PI, -2.71, 1.61], [2.72, 0.98, -7.42], [4.67, -0.45, 8.88]];
        assert_eq!(data.max().compute(), vec![8.88]);
        assert_eq!(data.max().axis(0).compute(), vec![4.67, 0.98, 8.88]);
        assert_eq!(data.max().axis(1).compute(), vec![PI, 2.72, 8.88]);
    }

    #[test]
    fn max_i64_3d() {
        let data = arr![
            [[101, 202, 303], [404, 505, 606]],
            [[-707, -808, -909], [111, 222, 333]]
        ];
        assert_eq!(data.max().compute(), vec![606]);
        assert_eq!(
            data.max().axis(0).compute(),
            vec![101, 202, 303, 404, 505, 606]
        );
        assert_eq!(
            data.max().axis(1).compute(),
            vec![404, 505, 606, 111, 222, 333]
        );
        assert_eq!(data.max().axis(2).compute(), vec![303, 606, -707, 333]);
    }

    #[test]
    fn max_f64_3d() {
        let data = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        assert_eq!(data.max().compute(), vec![12.2]);
        assert_eq!(
            data.max().axis(0).compute(),
            vec![7.7, 8.8, 9.9, 10.0, 11.1, 12.2]
        );
        assert_eq!(
            data.max().axis(1).compute(),
            vec![4.4, 5.5, 6.6, 10.0, 11.1, 12.2]
        );
        assert_eq!(data.max().axis(2).compute(), vec![3.3, 6.6, 9.9, 12.2]);
    }

    #[test]
    fn min_i64_1d() {
        let a = arr![42, -17, 256, 3, 99, -8];
        assert_eq!(a.min().compute(), vec![-17]);
        assert_eq!(a.min().axis(0).compute(), vec![-17]);
    }

    #[test]
    fn min_f64_1d() {
        let a = arr![PI, 2.71, -1.0, 42.0, 0.98];
        assert_eq!(a.min().compute(), vec![-1.0]);
        assert_eq!(a.min().axis(0).compute(), vec![-1.0]);
    }

    #[test]
    fn min_i64_2d() {
        let b = arr![[1, 5, 3], [4, 2, 6], [0, 9, 8]];
        assert_eq!(b.min().compute(), vec![0]);
        assert_eq!(b.min().axis(0).compute(), vec![0, 2, 3]);
        assert_eq!(b.min().axis(1).compute(), vec![1, 2, 0]);
    }

    #[test]
    fn min_f64_2d() {
        let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
        assert_eq!(b.min().compute(), vec![-7.42]);
        assert_eq!(b.min().axis(0).compute(), vec![E, -PI, -7.42]);
        assert_eq!(b.min().axis(1).compute(), vec![-PI, -7.42, -0.45]);
    }

    #[test]
    fn min_i64_3d() {
        let c = arr![
            [[101, 202, 303], [404, 505, 606]],
            [[-707, -808, -909], [111, 222, 333]]
        ];
        assert_eq!(c.min().compute(), vec![-909]);
        assert_eq!(
            c.min().axis(0).compute(),
            vec![-707, -808, -909, 111, 222, 333]
        );
        assert_eq!(
            c.min().axis(1).compute(),
            vec![101, 202, 303, -707, -808, -909]
        );
        assert_eq!(c.min().axis(2).compute(), vec![101, 404, -909, 111]);
    }

    #[test]
    fn min_f64_3d() {
        let c = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        assert_eq!(c.min().compute(), vec![1.1]);
        assert_eq!(
            c.min().axis(0).compute(),
            vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]
        );
        assert_eq!(
            c.min().axis(1).compute(),
            vec![1.1, 2.2, 3.3, 7.7, 8.8, 9.9]
        );
        assert_eq!(c.min().axis(2).compute(), vec![1.1, 4.4, 7.7, 10.0]);
    }
}
