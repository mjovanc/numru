/// Trait for types that can describe the dimensions of an array.
///
/// This trait allows for different representations of dimensions while providing
/// a common interface for querying array properties.
pub trait Dimension {
    /// Returns the number of dimensions.
    fn ndim(&self) -> usize;

    /// Returns the total size (number of elements) of the array.
    fn size(&self) -> usize;

    /// Returns a slice of the dimensions.
    fn dims(&self) -> &[usize];
}

// Enum to specify the dimensionality of an array when creating it from a vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionType {
    D1,
    D2,
    D3,
}
