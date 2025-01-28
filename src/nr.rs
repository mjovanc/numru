/// Represents an array with elements of type `T`.
#[derive(Debug)]
pub struct Array<T> {
    data: Vec<T>,
}

impl<T> Array<T> {
    /// Creates a new array from a vector of elements.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of elements to be wrapped into an array.
    pub fn new(data: Vec<T>) -> Self {
        Array { data }
    }

    /// Returns a reference to the array's data.
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

impl Array<i64> {
    /// Returns the dtype of the array (for `i64` type).
    pub fn dtype(&self) -> &'static str {
        "int64"
    }
}

impl Array<f64> {
    /// Returns the dtype of the array (for `f64` type).
    pub fn dtype(&self) -> &'static str {
        "float64"
    }
}
