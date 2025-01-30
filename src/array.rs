pub trait Numeric {
    fn zeros(len: usize) -> Vec<Self>
    where
        Self: Sized;
    fn ones(len: usize) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Array<T> {
    data: Vec<T>,
}

impl<T> Array<T> {
    pub fn new(data: Vec<T>) -> Self {
        Array { data }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: Numeric> Array<T> {
    pub fn zeros(&mut self) {
        let len = self.data.len();
        self.data = T::zeros(len);
    }

    pub fn zeros_with_len(&mut self, len: usize) {
        self.data = T::zeros(len);
    }

    pub fn ones(&mut self) {
        let len = self.data.len();
        self.data = T::ones(len);
    }

    pub fn ones_with_len(&mut self, len: usize) {
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
    pub fn dtype(&self) -> &'static str {
        "int64"
    }
}

impl Array<f64> {
    pub fn dtype(&self) -> &'static str {
        "float64"
    }
}

#[cfg(test)]
mod tests {
    use super::Array;

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
    fn test_dtype_correctness() {
        let a = array![1, 2, 3];
        assert_eq!(a.dtype(), "int64");

        let b = array![1.0, 2.0, 3.0];
        assert_eq!(b.dtype(), "float64");
    }

    #[test]
    fn test_array_creation_empty() {
        let a: Array<i64> = array![];
        assert!(a.data().is_empty());
    }

    #[test]
    fn test_array_i64_zeros() {
        let mut a2 = array![1, 2, 3];
        a2.zeros();
        assert_eq!(a2.data(), &vec![0, 0, 0]);
    }

    #[test]
    fn test_array_f64_zeros() {
        let mut a = array![1.0, 2.0, 3.0];
        a.zeros();
        assert_eq!(a.data(), &vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_array_i64_zeros_with_len() {
        let mut a2 = array![1, 2, 3];
        a2.zeros_with_len(5);
        assert_eq!(a2.data(), &vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_array_f64_zeros_with_len() {
        let mut a2 = array![1.0, 2.0, 3.0];
        a2.zeros_with_len(5);
        assert_eq!(a2.data(), &vec![0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_array_i64_ones() {
        let mut a2 = array![1, 2, 3];
        a2.ones();
        assert_eq!(a2.data(), &vec![1, 1, 1]);
    }

    #[test]
    fn test_array_f64_ones() {
        let mut a = array![1.0, 2.0, 3.0];
        a.ones();
        assert_eq!(a.data(), &vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_array_i64_ones_with_len() {
        let mut a2 = array![1, 2, 3];
        a2.ones_with_len(2);
        assert_eq!(a2.data(), &vec![1, 1]);
    }

    #[test]
    fn test_array_f64_ones_with_len() {
        let mut a2 = array![1.0, 2.0, 3.0];
        a2.ones_with_len(2);
        assert_eq!(a2.data(), &vec![1.0, 1.0]);
    }
}
