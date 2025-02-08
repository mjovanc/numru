use crate::shape::Shape;
use crate::Dimension;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Array<T, D: Dimension> {
    data: Vec<T>,
    shape: Shape<D>,
}

impl<T, D: Dimension> Array<T, D> {
    pub fn new(data: Vec<T>, shape: Shape<D>) -> Self {
        assert_eq!(data.len(), shape.size(), "Data size must match shape size");
        Array { data, shape }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn shape(&self) -> &Shape<D> {
        &self.shape
    }
}

impl<D: Dimension> Array<i64, D> {
    pub fn dtype(&self) -> &'static str {
        "int64"
    }
}

impl<D: Dimension> Array<f64, D> {
    pub fn dtype(&self) -> &'static str {
        "float64"
    }
}

impl<T: PartialOrd + Copy, D: Dimension> Array<T, D> {
    pub fn max(&self) -> MaxBuilder<T, D> {
        MaxBuilder::new(self)
    }
}

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
        let array = self.array;
        match self.axis {
            None => vec![*array
                .data
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .expect("Array is empty")],
            Some(axis) => {
                let raw_dim = array.shape.raw_dim();
                let ndim = raw_dim.ndim();

                assert!(
                    axis < ndim,
                    "Axis {} is out of bounds for array with {} dimensions",
                    axis,
                    ndim
                );

                match ndim {
                    1 => vec![*array
                        .data
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .expect("Array is empty")],
                    2 => {
                        let rows = raw_dim.dims()[0];
                        let cols = raw_dim.dims()[1];

                        if axis == 0 {
                            (0..cols)
                                .map(|col| {
                                    (0..rows)
                                        .map(|row| array.data[row * cols + col])
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .unwrap()
                                })
                                .collect()
                        } else {
                            (0..rows)
                                .map(|row| {
                                    array.data[row * cols..(row + 1) * cols]
                                        .iter()
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .unwrap()
                                        .to_owned()
                                })
                                .collect()
                        }
                    }
                    3 => {
                        let depth = raw_dim.dims()[0];
                        let rows = raw_dim.dims()[1];
                        let cols = raw_dim.dims()[2];

                        match axis {
                            0 => (0..rows * cols)
                                .map(|i| {
                                    (0..depth)
                                        .map(|d| array.data[d * rows * cols + i])
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .unwrap()
                                })
                                .collect(),
                            1 => (0..depth)
                                .flat_map(|d| {
                                    (0..cols).map(move |c| {
                                        (0..rows)
                                            .map(|r| array.data[d * rows * cols + r * cols + c])
                                            .max_by(|a, b| a.partial_cmp(b).unwrap())
                                            .unwrap()
                                    })
                                })
                                .collect(),
                            2 => (0..depth)
                                .flat_map(|d| {
                                    (0..rows).map(move |r| {
                                        let row_start = d * rows * cols + r * cols;
                                        array.data[row_start..row_start + cols]
                                            .iter()
                                            .max_by(|a, b| a.partial_cmp(b).unwrap())
                                            .unwrap()
                                            .to_owned()
                                    })
                                })
                                .collect(),
                            _ => unreachable!(),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
        }
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

impl<D: Dimension, T: Display> Array<T, D> {
    pub fn visualize(&self) {
        let dims = self.shape.dims();
        let ndim = dims.len();

        if ndim == 1 {
            let rows = dims[0];
            print!("[");
            for i in 0..rows {
                let value = &self.data[i];
                let value_str = format!("{}", value);
                print!("{}", value_str);
                if i < rows - 1 {
                    print!(", ");
                }
            }
            println!("]");
        } else if ndim == 2 {
            let rows = dims[0];
            let cols = dims[1];

            let mut column_widths = vec![0; cols];
            for i in 0..rows {
                for j in 0..cols {
                    let value = &self.data[i * cols + j];
                    let width = format!("{}", value).len();
                    column_widths[j] = column_widths[j].max(width);
                }
            }

            println!("[");
            for i in 0..rows {
                print!("   [");
                for j in 0..cols {
                    let value = &self.data[i * cols + j];
                    let value_str = format!("{}", value);
                    print!("{:width$}", value_str, width = column_widths[j]);
                    if j < cols - 1 {
                        print!(", ");
                    }
                }
                println!("]");
            }
            println!("]");
        } else if ndim == 3 {
            let depth = dims[0];
            let rows = dims[1];
            let cols = dims[2];

            let mut column_widths = vec![0; cols];
            for i in 0..depth {
                for j in 0..rows {
                    for k in 0..cols {
                        let value = &self.data[(i * rows * cols) + (j * cols) + k];
                        let width = format!("{}", value).len();
                        column_widths[k] = column_widths[k].max(width);
                    }
                }
            }

            println!("[");
            for i in 0..depth {
                println!("   [");
                for j in 0..rows {
                    print!("      [");
                    for k in 0..cols {
                        let value = &self.data[(i * rows * cols) + (j * cols) + k];
                        let value_str = format!("{}", value);
                        print!("{:width$}", value_str, width = column_widths[k]);
                        if k < cols - 1 {
                            print!(", ");
                        }
                    }
                    println!("]");
                }
                println!("   ]");
            }
            println!("]");
        } else {
            // Handle higher dimensions (4D, 5D, etc.) in the future if needed
            println!("Unsupported dimension: {}", ndim);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

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
}
