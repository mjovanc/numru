use num_traits::{One, Zero};

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

impl<T: Zero + One + Copy, D: Dimension> Array<T, D> {
    /// Replaces all elements in the array with zeros using num_traits::Zero.
    /// The shape and dimension of the array are preserved.
    pub fn zeros(&mut self) {
        let zero = T::zero(); // Use Zero::zero() instead of T::default()
        self.data.iter_mut().for_each(|x| *x = zero);
    }

    /// Replaces all elements in the array with ones using num_traits::One.
    /// The shape and dimension of the array are preserved.
    pub fn ones(&mut self) {
        let one = T::one(); // Use One::one() to get the one value
        self.data.iter_mut().for_each(|x| *x = one);
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

    /// Computes the mean value(s) of the array along a specified axis or for the whole array.
    pub fn mean_compute(&self, axis: Option<usize>) -> Result<Vec<f64>, ArrayError>
    where
        T: Into<f64>
    {
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
            1 => {
                let sum: f64 = self.data.iter().map(|&x| Into::<f64>::into(x)).sum();
                Ok(vec![sum / self.data.len() as f64])
            }
            2 => {
                let rows = raw_dim.dims()[0];
                let cols = raw_dim.dims()[1];

                if let Some(axis) = axis {
                    if axis == 0 {
                        (0..cols)
                            .map(|col| {
                                let sum: f64 = (0..rows)
                                    .map(|row| Into::<f64>::into(self.data[row * cols + col]))
                                    .sum();
                                Ok(sum / rows as f64)
                            })
                            .collect()
                    } else {
                        (0..rows)
                            .map(|row| {
                                let sum: f64 = self.data[row * cols..(row + 1) * cols]
                                    .iter()
                                    .map(|&x| Into::<f64>::into(x))
                                    .sum();
                                Ok(sum / cols as f64)
                            })
                            .collect()
                    }
                } else {
                    let sum: f64 = self.data.iter().map(|&x| Into::<f64>::into(x)).sum();
                    Ok(vec![sum / (rows * cols) as f64])
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
                                let sum: f64 = (0..depth)
                                    .map(|d| Into::<f64>::into(self.data[d * rows * cols + i]))
                                    .sum();
                                Ok(sum / depth as f64)
                            })
                            .collect(),
                        1 => (0..depth)
                            .flat_map(|d| {
                                (0..cols).map(move |c| {
                                    let sum: f64 = (0..rows)
                                        .map(|r| Into::<f64>::into(self.data[d * rows * cols + r * cols + c]))
                                        .sum();
                                    Ok(sum / rows as f64)
                                })
                            })
                            .collect(),
                        2 => (0..depth)
                            .flat_map(|d| {
                                (0..rows).map(move |r| {
                                    let row_start = d * rows * cols + r * cols;
                                    let sum: f64 = self.data[row_start..row_start + cols]
                                        .iter()
                                        .map(|&x| Into::<f64>::into(x))
                                        .sum();
                                    Ok(sum / cols as f64)
                                })
                            })
                            .collect(),
                        _ => unreachable!(),
                    }
                } else {
                    let sum: f64 = self.data.iter().map(|&x| Into::<f64>::into(x)).sum();
                    Ok(vec![sum / (depth * rows * cols) as f64])
                }
            }
            _ => Err(ArrayError::UnimplementedDimension(format!(
                "Dimension {} for mean computation not implemented",
                ndim
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI, TAU};

    use crate::{Dimension, Ix, Shape};

    fn round_to_3dp(value: f64) -> f64 {
        (value * 1000.0).round() / 1000.0
    }

    fn assert_vec_approx_eq(actual: Vec<f64>, expected: Vec<f64>) {
        assert_eq!(actual.len(), expected.len(), "Vectors have different lengths");
        for (a, e) in actual.iter().zip(expected.iter()) {
            assert_eq!(round_to_3dp(*a), round_to_3dp(*e), "Values differ: {} != {}", a, e);
        }
    }

    #[test]
    fn array_creation_i64_1d() {
        let arr = arr![1, 2, 3, 4];
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_i64_2d() {
        let arr = arr![[1, 2], [3, 4], [5, 6]];
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 6);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_i64_3d() {
        let arr = arr![[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]];
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_f64_1d() {
        let arr = arr![1.1, 2.2, 3.3, 4.4];
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_f64_2d() {
        let arr = arr![[1.1, 2.2], [3.3, 4.4], [5.5, 6.6]];
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 6);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
    }

    #[test]
    fn array_creation_f64_3d() {
        let arr = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
    }

    #[test]
    fn max_i64_1d() {
        let arr = arr![42, -17, 256, 3, 99, -8];
        assert_eq!(arr.max().compute(), vec![256]);
    }

    #[test]
    fn max_f64_1d() {
        let arr = arr![PI, 2.71, -1.0, 42.0, 0.98];
        assert_eq!(arr.max().compute(), vec![42.0]);
    }

    #[test]
    fn max_i64_2d() {
        let arr = arr![[1, 5, 3], [4, 2, 6], [0, 9, 8]];
        assert_eq!(arr.max().compute(), vec![9]);
        assert_eq!(arr.max().axis(0).compute(), vec![4, 9, 8]);
        assert_eq!(arr.max().axis(1).compute(), vec![5, 6, 9]);
    }

    #[test]
    fn max_f64_2d() {
        let arr = arr![[PI, -2.71, 1.61], [2.72, 0.98, -7.42], [4.67, -0.45, 8.88]];
        assert_eq!(arr.max().compute(), vec![8.88]);
        assert_eq!(arr.max().axis(0).compute(), vec![4.67, 0.98, 8.88]);
        assert_eq!(arr.max().axis(1).compute(), vec![PI, 2.72, 8.88]);
    }

    #[test]
    fn max_i64_3d() {
        let arr = arr![
            [[101, 202, 303], [404, 505, 606]],
            [[-707, -808, -909], [111, 222, 333]]
        ];
        assert_eq!(arr.max().compute(), vec![606]);
        assert_eq!(
            arr.max().axis(0).compute(),
            vec![101, 202, 303, 404, 505, 606]
        );
        assert_eq!(
            arr.max().axis(1).compute(),
            vec![404, 505, 606, 111, 222, 333]
        );
        assert_eq!(arr.max().axis(2).compute(), vec![303, 606, -707, 333]);
    }

    #[test]
    fn max_f64_3d() {
        let arr = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        assert_eq!(arr.max().compute(), vec![12.2]);
        assert_eq!(
            arr.max().axis(0).compute(),
            vec![7.7, 8.8, 9.9, 10.0, 11.1, 12.2]
        );
        assert_eq!(
            arr.max().axis(1).compute(),
            vec![4.4, 5.5, 6.6, 10.0, 11.1, 12.2]
        );
        assert_eq!(arr.max().axis(2).compute(), vec![3.3, 6.6, 9.9, 12.2]);
    }

    #[test]
    fn min_i64_1d() {
        let arr = arr![42, -17, 256, 3, 99, -8];
        assert_eq!(arr.min().compute(), vec![-17]);
        assert_eq!(arr.min().axis(0).compute(), vec![-17]);
    }

    #[test]
    fn min_f64_1d() {
        let arr = arr![PI, 2.71, -1.0, 42.0, 0.98];
        assert_eq!(arr.min().compute(), vec![-1.0]);
        assert_eq!(arr.min().axis(0).compute(), vec![-1.0]);
    }

    #[test]
    fn min_i64_2d() {
        let arr = arr![[1, 5, 3], [4, 2, 6], [0, 9, 8]];
        assert_eq!(arr.min().compute(), vec![0]);
        assert_eq!(arr.min().axis(0).compute(), vec![0, 2, 3]);
        assert_eq!(arr.min().axis(1).compute(), vec![1, 2, 0]);
    }

    #[test]
    fn min_f64_2d() {
        let arr = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
        assert_eq!(arr.min().compute(), vec![-7.42]);
        assert_eq!(arr.min().axis(0).compute(), vec![E, -PI, -7.42]);
        assert_eq!(arr.min().axis(1).compute(), vec![-PI, -7.42, -0.45]);
    }

    #[test]
    fn min_i64_3d() {
        let arr = arr![
            [[101, 202, 303], [404, 505, 606]],
            [[-707, -808, -909], [111, 222, 333]]
        ];
        assert_eq!(arr.min().compute(), vec![-909]);
        assert_eq!(
            arr.min().axis(0).compute(),
            vec![-707, -808, -909, 111, 222, 333]
        );
        assert_eq!(
            arr.min().axis(1).compute(),
            vec![101, 202, 303, -707, -808, -909]
        );
        assert_eq!(arr.min().axis(2).compute(), vec![101, 404, -909, 111]);
    }

    #[test]
    fn min_f64_3d() {
        let arr = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        assert_eq!(arr.min().compute(), vec![1.1]);
        assert_eq!(
            arr.min().axis(0).compute(),
            vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6]
        );
        assert_eq!(
            arr.min().axis(1).compute(),
            vec![1.1, 2.2, 3.3, 7.7, 8.8, 9.9]
        );
        assert_eq!(arr.min().axis(2).compute(), vec![1.1, 4.4, 7.7, 10.0]);
    }

    #[test]
    fn zeros_macro_i64_1d() {
        let arr = zeros!(i64, 4);
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![0i64; 4]);
    }

    #[test]
    fn zeros_macro_i64_2d() {
        let arr = zeros!(i64, 3, 2);
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 6);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![0i64; 6]);
    }

    #[test]
    fn zeros_macro_i64_3d() {
        let arr = zeros!(i64, 2, 2, 3);
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![0i64; 12]);
    }

    #[test]
    fn zeros_macro_f64_1d() {
        let arr = zeros!(f64, 4);
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![0.0f64; 4]);
    }

    #[test]
    fn zeros_macro_f64_2d() {
        let arr = zeros!(f64, 3, 2);
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 6);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![0.0f64; 6]);
    }

    #[test]
    fn zeros_macro_f64_3d() {
        let arr = zeros!(f64, 2, 2, 3);
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![0.0f64; 12]);
    }

    #[test]
    fn zeros_method_i64_1d() {
        let mut arr = arr![1, 2, 3, 4];
        let original_shape = format!("{:?}", arr.shape());

        arr.zeros();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![0i64; 4]);
    }

    #[test]
    fn zeros_method_i64_2d() {
        let mut arr = arr![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let original_shape = format!("{:?}", arr.shape());

        arr.zeros();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 9);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![0i64; 9]);
    }

    #[test]
    fn zeros_method_i64_3d() {
        let mut arr = arr![[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]];
        let original_shape = format!("{:?}", arr.shape());

        arr.zeros();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![0i64; 12]);
    }

    #[test]
    fn zeros_method_f64_1d() {
        let mut arr = arr![1.1, 2.2, 3.3, 4.4];
        let original_shape = format!("{:?}", arr.shape());

        arr.zeros();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![0.0f64; 4]);
    }

    #[test]
    fn zeros_method_f64_2d() {
        let mut arr = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
        let original_shape = format!("{:?}", arr.shape());

        arr.zeros();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 9);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![0.0f64; 9]);
    }

    #[test]
    fn zeros_method_f64_3d() {
        let mut arr = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        let original_shape = format!("{:?}", arr.shape());

        arr.zeros();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![0.0f64; 12]);
    }

    #[test]
    fn ones_macro_i64_1d() {
        let arr = ones!(i64, 4);
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![1i64; 4]);
    }

    #[test]
    fn ones_macro_i64_2d() {
        let arr = ones!(i64, 3, 2);
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 6);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![1i64; 6]);
    }

    #[test]
    fn ones_macro_i64_3d() {
        let arr = ones!(i64, 2, 2, 3);
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![1i64; 12]);
    }

    #[test]
    fn ones_macro_f64_1d() {
        let arr = ones!(f64, 4);
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![1.0f64; 4]);
    }

    #[test]
    fn ones_macro_f64_2d() {
        let arr = ones!(f64, 3, 2);
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 6);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![1.0f64; 6]);
    }

    #[test]
    fn ones_macro_f64_3d() {
        let arr = ones!(f64, 2, 2, 3);
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", arr.shape()), format!("{:?}", shape));
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![1.0f64; 12]);
    }

    #[test]
    fn ones_method_i64_1d() {
        let mut arr = arr![1, 2, 3, 4];
        let original_shape = format!("{:?}", arr.shape());

        arr.ones();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![1i64; 4]);
    }

    #[test]
    fn ones_method_i64_2d() {
        let mut arr = arr![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let original_shape = format!("{:?}", arr.shape());

        arr.ones();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 9);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![1i64; 9]);
    }

    #[test]
    fn ones_method_i64_3d() {
        let mut arr = arr![[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]];
        let original_shape = format!("{:?}", arr.shape());

        arr.ones();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(arr.dtype(), "int64");
        assert_eq!(arr.data(), &vec![1i64; 12]);
    }

    #[test]
    fn ones_method_f64_1d() {
        let mut arr = arr![1.1, 2.2, 3.3, 4.4];
        let original_shape = format!("{:?}", arr.shape());

        arr.ones();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 4);
        assert_eq!(arr.shape().raw_dim().ndim(), 1);
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![1.0f64; 4]);
    }

    #[test]
    fn ones_method_f64_2d() {
        let mut arr = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
        let original_shape = format!("{:?}", arr.shape());

        arr.ones();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 9);
        assert_eq!(arr.shape().raw_dim().ndim(), 2);
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![1.0f64; 9]);
    }

    #[test]
    fn ones_method_f64_3d() {
        let mut arr = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        let original_shape = format!("{:?}", arr.shape());

        arr.ones();

        assert_eq!(format!("{:?}", arr.shape()), original_shape);
        assert_eq!(arr.shape().raw_dim().size(), 12);
        assert_eq!(arr.shape().raw_dim().ndim(), 3);
        assert_eq!(arr.dtype(), "float64");
        assert_eq!(arr.data(), &vec![1.0f64; 12]);
    }

    #[test]
    fn mean_i64_1d() {
        let arr = arr![42, -17, 256, 3, 99, -8];
        let expected_mean = vec![62.5];
        assert_vec_approx_eq(arr.mean().compute(), expected_mean);
    }

    #[test]
    fn mean_f64_1d() {
        let arr = arr![PI, 2.71, -1.0, 42.0, 0.98];
        let expected_mean = vec![9.566];
        assert_vec_approx_eq(arr.mean().compute(), expected_mean);
    }

    #[test]
    fn mean_i64_2d() {
        let arr = arr![[1, 5, 3], [4, 2, 6], [0, 9, 8]];
        let expected_mean = vec![4.222];
        let expected_mean_axis_0 = vec![1.667, 5.333, 5.667];
        let expected_mean_axis_1 = vec![3.0, 4.0, 5.667];
        assert_vec_approx_eq(arr.mean().compute(), expected_mean);
        assert_vec_approx_eq(arr.mean().axis(0).compute(), expected_mean_axis_0);
        assert_vec_approx_eq(arr.mean().axis(1).compute(), expected_mean_axis_1);
    }

    #[test]
    fn mean_f64_2d() {
        let arr = arr![[PI, -2.71, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
        let expected_mean = vec![1.269];
        let expected_mean_axis_0 = vec![3.51, -0.727, 1.023];
        let expected_mean_axis_1 = vec![0.681, -1.241, 4.367];
        assert_vec_approx_eq(arr.mean().compute(), expected_mean);
        assert_vec_approx_eq(arr.mean().axis(0).compute(), expected_mean_axis_0);
        assert_vec_approx_eq(arr.mean().axis(1).compute(), expected_mean_axis_1);
    }

    #[test]
    fn mean_i64_3d() {
        let arr = arr![
            [[101, 202, 303], [404, 505, 606]],
            [[-707, -808, -909], [111, 222, 333]]
        ];
        let expected_mean = vec![30.25];
        let expected_mean_axis_0 = vec![-303.0, -303.0, -303.0, 257.5, 363.5, 469.5];
        let expected_mean_axis_1 = vec![252.5, 353.5, 454.5, -298.0, -293.0, -288.0];
        let expected_mean_axis_2 = vec![202.0, 505.0, -808.0, 222.0];
        assert_vec_approx_eq(arr.mean().compute(), expected_mean);
        assert_vec_approx_eq(arr.mean().axis(0).compute(), expected_mean_axis_0);
        assert_vec_approx_eq(arr.mean().axis(1).compute(), expected_mean_axis_1);
        assert_vec_approx_eq(arr.mean().axis(2).compute(), expected_mean_axis_2);
    }

    #[test]
    fn mean_f64_3d() {
        let arr = arr![
            [[1.1, 2.2, 3.3], [4.4, 5.5, 6.6]],
            [[7.7, 8.8, 9.9], [10.0, 11.1, 12.2]]
        ];
        let expected_mean = vec![6.9];
        let expected_mean_axis_0 = vec![4.4, 5.5, 6.6, 7.2, 8.3, 9.4];
        let expected_mean_axis_1 = vec![2.75, 3.85, 4.95, 8.85, 9.95, 11.05];
        let expected_mean_axis_2 = vec![2.2, 5.5, 8.8, 11.1];
        assert_vec_approx_eq(arr.mean().compute(), expected_mean);
        assert_vec_approx_eq(arr.mean().axis(0).compute(), expected_mean_axis_0);
        assert_vec_approx_eq(arr.mean().axis(1).compute(), expected_mean_axis_1);
        assert_vec_approx_eq(arr.mean().axis(2).compute(), expected_mean_axis_2);
    }
}
