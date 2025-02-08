use thiserror::Error;

/// Custom error types for array operations.
///
/// This enum encapsulates various error conditions that can occur when working with arrays,
/// providing specific error types for better error handling and debugging.
#[derive(Debug, Error)]
pub enum ArrayError {
    /// Occurs when the dimensions of data do not match the expected shape.
    /// The error message includes the expected number of elements based on the shape
    /// and the actual number of elements provided in the data vector.
    #[error("Dimension mismatch: Expected {expected} elements based on the shape, but the data vector contains {actual} elements")]
    DimensionMismatch { expected: usize, actual: usize },

    /// Raised when an index used to access the array is outside its bounds.
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(String),

    /// Error signaling that the array contains no elements when an operation requires at least one.
    #[error("Array is empty")]
    EmptyArray,

    /// Indicates that there was a mismatch between expected and actual data types within the array.
    #[error("Data type mismatch: {0}")]
    DataTypeMismatch(String),

    /// Used when an invalid axis is specified for operations like `max`, `min`, etc.
    #[error("Invalid axis specified: {0}")]
    InvalidAxis(String),

    /// Signals that the operation requested for an array of a certain dimension is not implemented.
    #[error("Unimplemented dimension: {0}")]
    UnimplementedDimension(String),
}
