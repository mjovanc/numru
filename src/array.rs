use crate::shape::IxDyn;
use crate::Shape;

#[derive(Debug)]
pub struct Array<T> {
    data: Vec<T>,
    shape: Shape<IxDyn>, // TODO: we should have a generic Ix type here to allow for different index types
}

impl<T> Array<T> {
    pub fn new(data: Vec<T>, shape: Shape<IxDyn>) -> Self {
        Array { data, shape }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn shape(&self) -> &Shape<IxDyn> {
        &self.shape
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
