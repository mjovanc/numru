use crate::shape::IxDyn;
use crate::Shape;

#[derive(Debug)]
pub struct Array<T> {
    data: Vec<T>,
    shape: Option<Shape<IxDyn>>,
}

impl<T> Array<T> {
    pub fn new(data: Vec<T>) -> Self {
        Array { data, shape: None }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn with_shape(mut self, shape: Shape<IxDyn>) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn shape(&self) -> Option<&Shape<IxDyn>> {
        self.shape.as_ref()
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
        let a = arr![1, 2, 3];
        assert_eq!(a.data(), &vec![1, 2, 3]);
        assert_eq!(a.dtype(), "int64");

        let f = arr![1.0, 2.0, 3.0];
        assert_eq!(f.data(), &vec![1.0, 2.0, 3.0]);
        assert_eq!(f.dtype(), "float64");
    }

    #[test]
    fn test_dtype_correctness() {
        let a = arr![1, 2, 3];
        assert_eq!(a.dtype(), "int64");

        let b = arr![1.0, 2.0, 3.0];
        assert_eq!(b.dtype(), "float64");
    }

    #[test]
    fn test_array_creation_empty() {
        let a: Array<i64> = arr![];
        assert!(a.data().is_empty());

        let a: Array<f64> = arr![];
        assert!(a.data().is_empty());
    }
}
