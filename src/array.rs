use crate::shape::{Shape, Dimension};

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

#[cfg(test)]
mod tests {
    use crate::shape::{Ix, Shape, Dimension};

    #[test]
    fn test_array_creation_1d() {
        let data = arr![1, 2, 3, 4];
        let shape = Shape::new(Ix::<1>::new([4]));

        assert_eq!(data.shape().raw_dim().size(), 4);
        assert_eq!(data.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

    #[test]
    fn test_array_creation_2d() {
        let data = arr![[1, 2], [3, 4], [5, 6]];
        let shape = Shape::new(Ix::<2>::new([3, 2]));

        assert_eq!(data.shape().raw_dim().size(), 6);
        assert_eq!(data.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", data.shape()), format!("{:?}", shape));
    }

}