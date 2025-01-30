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
}
