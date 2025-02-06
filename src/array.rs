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
}

impl<D: Dimension> Array<f64, D> {
    pub fn dtype(&self) -> &'static str {
        "float64"
    }
}

#[cfg(test)]
mod tests {
    use crate::shape::{Ix, Shape, Dimension};
    use crate::array::Array;

    #[test]
    fn test_array_creation_1d() {
        let data = vec![1, 2, 3, 4];
        let shape = Shape::new(Ix::<1>::new([4]));

        let array = Array::new(data.clone(), shape.clone());

        assert_eq!(array.data(), &data);
        assert_eq!(array.shape().raw_dim().size(), 4);
        assert_eq!(array.shape().raw_dim().ndim(), 1);
        assert_eq!(format!("{:?}", array.shape()), format!("{:?}", shape));
    }

    #[test]
    fn test_array_creation_2d() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let shape = Shape::new(Ix::<2>::new([3, 2])); // 3 rows, 2 columns

        let array = Array::new(data.clone(), shape.clone());

        assert_eq!(array.data(), &data);
        assert_eq!(array.shape().raw_dim().size(), 6);
        assert_eq!(array.shape().raw_dim().ndim(), 2);
        assert_eq!(format!("{:?}", array.shape()), format!("{:?}", shape));
    }

    #[test]
    fn test_array_creation_3d() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let shape = Shape::new(Ix::<3>::new([2, 2, 2])); // 2 blocks, 2 rows, 2 columns

        let array = Array::new(data.clone(), shape.clone());

        assert_eq!(array.data(), &data);
        assert_eq!(array.shape().raw_dim().size(), 8);
        assert_eq!(array.shape().raw_dim().ndim(), 3);
        assert_eq!(format!("{:?}", array.shape()), format!("{:?}", shape));
    }
}