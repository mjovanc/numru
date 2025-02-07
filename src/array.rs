use crate::shape::{Dimension, Shape};
use std::fmt::{self, Display};
use std::fmt::{Debug, Error, Formatter};

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

    pub fn mean(&self, axis: Option<usize>) -> Vec<i64> {
        !unimplemented!()
    }

    pub fn min(&self, axis: Option<usize>) -> i64 {
        !unimplemented!()
    }

    pub fn max(&self, axis: Option<usize>) -> i64 {
        !unimplemented!()
    }
}

impl<D: Dimension> Array<f64, D> {
    pub fn dtype(&self) -> &'static str {
        "float64"
    }

    pub fn mean(&self, axis: Option<usize>) -> Vec<f64> {
        !unimplemented!()
    }

    pub fn min(&self, axis: Option<usize>) -> f64 {
        !unimplemented!()
    }

    pub fn max(&self, axis: Option<usize>) -> f64 {
        !unimplemented!()
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
    use crate::shape::{Dimension, Ix, Shape};

    #[test]
    fn test_array_creation_i64_1d() {
        let data = arr![1, 2, 3, 4];
        let ix = Ix::<1>::new([4]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 4);
        assert_eq!(data.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn test_array_creation_i64_2d() {
        let data = arr![[1, 2], [3, 4], [5, 6]];
        let ix = Ix::<2>::new([3, 2]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 6);
        assert_eq!(data.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn test_array_creation_i64_3d() {
        let data = arr![[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]];
        let ix = Ix::<3>::new([2, 2, 3]);
        let shape = Shape::new(ix);

        assert_eq!(data.shape().raw_dim().size(), 12);
        assert_eq!(data.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }
}
