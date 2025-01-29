/// Trait to define methods common to numeric types like `i64` and `f64`.
pub trait Numeric {
    fn zeros(len: usize) -> Vec<Self>
    where
        Self: Sized;
    fn ones(len: usize) -> Vec<Self>
    where
        Self: Sized;
}

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

impl<T: Numeric> Array<T> {
    /// Sets all elements of the array to zero.
    pub fn zeros(&mut self) {
        let len = self.data.len();
        self.data = T::zeros(len);
    }

    /// Sets all elements of the array to one.
    pub fn ones(&mut self) {
        let len = self.data.len();
        self.data = T::ones(len);
    }
}

impl Numeric for i64 {
    fn zeros(len: usize) -> Vec<Self> {
        vec![0; len]
    }

    fn ones(len: usize) -> Vec<Self> {
        vec![1; len]
    }
}

impl Numeric for f64 {
    fn zeros(len: usize) -> Vec<Self> {
        vec![0.0; len]
    }

    fn ones(len: usize) -> Vec<Self> {
        vec![1.0; len]
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_array_creation() {
        let a = array![1, 2, 3];
        assert_eq!(a.data(), &vec![1, 2, 3]);
        assert_eq!(a.dtype(), "int64");

        let f = array![1.0, 2.0, 3.0];
        assert_eq!(f.data(), &vec![1.0, 2.0, 3.0]);
        assert_eq!(f.dtype(), "float64");
    }

    #[test]
    fn test_array_creation_zeros() {
        let mut a2 = array![1, 2, 3];
        a2.zeros();
        assert_eq!(a2.data(), &vec![0, 0, 0]);
    }

    #[test]
    fn test_array_creation_ones() {
        let mut a2 = array![1, 2, 3];
        a2.ones();
        assert_eq!(a2.data(), &vec![1, 1, 1]);
    }
}
