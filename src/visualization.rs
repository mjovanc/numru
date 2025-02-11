use crate::{Array, Dimension};
use std::fmt::Display;

/// Trait for formatting values based on their type.
pub trait FormatValue {
    /// Formats the value as a string, taking into account the type's specific formatting rules.
    /// The `precision` parameter specifies the number of decimal points for floating-point values.
    fn format_value(&self, precision: usize) -> String;
}

impl FormatValue for i64 {
    fn format_value(&self, _precision: usize) -> String {
        format!("{}", self) // Ignore precision for i64
    }
}

impl FormatValue for f64 {
    fn format_value(&self, precision: usize) -> String {
        // Ensure the specified number of decimal places for f64
        format!("{:.precision$}", self, precision = precision)
    }
}

/// Builder for configuring visualization options.
pub struct VisualizeBuilder<'a, T, D: Dimension> {
    array: &'a Array<T, D>,
    decimal_points: usize,
}

impl<T: Display + FormatValue, D: Dimension> Array<T, D> {
    /// Starts the visualization process with default settings.
    pub fn visualize(&self) -> VisualizeBuilder<T, D> {
        VisualizeBuilder {
            array: self,
            decimal_points: 1,
        }
    }
}

impl<'a, T: Display + FormatValue, D: Dimension> VisualizeBuilder<'a, T, D> {
    /// Sets the number of decimal points for floating-point values.
    pub fn decimal_points(mut self, points: usize) -> Self {
        self.decimal_points = points;
        self
    }

    /// Executes the visualization with the configured settings.
    pub fn execute(&self) {
        let dims = self.array.shape().dims();
        let ndim = dims.len();

        if ndim == 1 {
            let rows = dims[0];
            print!("[");
            for i in 0..rows {
                let value = &self.array.data()[i];
                let value_str = value.format_value(self.decimal_points);
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
                    let value = &self.array.data()[i * cols + j];
                    let width = value.format_value(self.decimal_points).len();
                    column_widths[j] = column_widths[j].max(width);
                }
            }

            println!("[");
            for i in 0..rows {
                print!("   [");
                for j in 0..cols {
                    let value = &self.array.data()[i * cols + j];
                    let value_str = value.format_value(self.decimal_points);
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
                        let value = &self.array.data()[(i * rows * cols) + (j * cols) + k];
                        let width = value.format_value(self.decimal_points).len();
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
                        let value = &self.array.data()[(i * rows * cols) + (j * cols) + k];
                        let value_str = value.format_value(self.decimal_points);
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
