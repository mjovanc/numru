/// The `arr!` macro is designed to accept arrays of depth 1D, 2D and 3D and flatten them into a
/// single-dimensional vector. It also tracks and stores the shape (dimensions) of the array, which includes
/// the number of rows, columns, and further dimensions as needed.
#[macro_export]
macro_rules! arr {
    ($([$([$($elems:expr),+]),+]),+ $(,)?) => {{
        fn flatten_3d<T: Clone>(nested: &[Vec<Vec<T>>]) -> Vec<T> {
            nested.iter().flat_map(|inner| inner.iter().flat_map(|v| v.clone())).collect()
        }

        fn get_shape_3d<T>(nested: &[Vec<Vec<T>>]) -> Vec<usize> {
            let mut shape = vec![nested.len()];
            if let Some(first) = nested.first() {
                shape.push(first.len());
                if let Some(second) = first.first() {
                    shape.push(second.len());
                }
            }
            shape
        }

        let temp_3d = vec![$(vec![$(vec![$($elems),+]),+]),+];
        let data_3d = flatten_3d(&temp_3d);
        let shape_3d = get_shape_3d(&temp_3d);

        $crate::Array::new(data_3d, $crate::Shape::new($crate::ix::Ix::<3>::new(shape_3d.try_into().unwrap()))).unwrap()
    }};

    ($([$($elems:expr),+]),+ $(,)?) => {{
        fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
            nested.iter().flat_map(|inner| inner.clone()).collect()
        }

        fn get_shape<T>(nested: &[Vec<T>]) -> Vec<usize> {
            let mut shape = vec![nested.len()];
            if let Some(first) = nested.first() {
                shape.push(first.len());
            }
            shape
        }

        let temp = vec![$(vec![$($elems),+]),+];
        let data = flatten(&temp);
        let shape = get_shape(&temp);

        $crate::Array::new(data, $crate::Shape::new($crate::ix::Ix::<2>::new(shape.try_into().unwrap()))).unwrap()
    }};

    ($($elem:expr),+ $(,)?) => {{
        let data = vec![$($elem),+];
        let shape = vec![data.len()];
        $crate::Array::new(data, $crate::Shape::new($crate::ix::Ix::<1>::new(shape.try_into().unwrap()))).unwrap()
    }};
}

/// The `zeros!` macro creates a multi-dimensional array filled with zeros of the specified data type,
/// supporting 1D, 2D, and 3D arrays. It generates a flattened vector of zeros and tracks the shape
/// (dimensions) of the array, which includes the number of rows, columns, and further dimensions as needed.
#[macro_export]
macro_rules! zeros {
    ($ty:ty, $dim:expr) => {{
        let shape = vec![$dim];
        let size = shape.iter().product::<usize>();

        let zero_value: $ty = <$ty as Default>::default();
        let data: Vec<$ty> = vec![zero_value; size];

        let shape = $crate::Shape::new($crate::ix::Ix::<1>::new(shape.try_into().unwrap()));
        $crate::Array::new(data, shape).unwrap()
    }};

    ($ty:ty, $dim1:expr, $dim2:expr) => {{
        let shape = vec![$dim1, $dim2];
        let size = shape.iter().product::<usize>();

        let zero_value: $ty = <$ty as Default>::default();
        let data: Vec<$ty> = vec![zero_value; size];

        let shape = $crate::Shape::new($crate::ix::Ix::<2>::new(shape.try_into().unwrap()));
        $crate::Array::new(data, shape).unwrap()
    }};

    ($ty:ty, $dim1:expr, $dim2:expr, $dim3:expr) => {{
        let shape = vec![$dim1, $dim2, $dim3];
        let size = shape.iter().product::<usize>();

        let zero_value: $ty = <$ty as Default>::default();
        let data: Vec<$ty> = vec![zero_value; size];

        let shape = $crate::Shape::new($crate::ix::Ix::<3>::new(shape.try_into().unwrap()));
        $crate::Array::new(data, shape).unwrap()
    }};

    ($ty:ty, $($dim:expr),+) => {{
        let shape = vec![$($dim),+];
        let dimension = shape.len();
        panic!("Unsupported number of dimensions (only 1D, 2D, and 3D are supported): {}", dimension);
    }};
}

/// The `ones!` macro creates a multi-dimensional array filled with ones of the specified data type,
/// supporting 1D, 2D, and 3D arrays. It generates a flattened vector of zeros and tracks the shape
/// (dimensions) of the array, which includes the number of rows, columns, and further dimensions as needed.
#[macro_export]
macro_rules! ones {
    ($ty:ty, $dim:expr) => {{
        use ::num_traits::One;
        let shape = vec![$dim];
        let size = shape.iter().product::<usize>();

        let one_value: $ty = <$ty as One>::one();
        let data: Vec<$ty> = vec![one_value; size];

        let shape = $crate::Shape::new($crate::ix::Ix::<1>::new(shape.try_into().unwrap()));
        $crate::Array::new(data, shape).unwrap()
    }};

    ($ty:ty, $dim1:expr, $dim2:expr) => {{
        use ::num_traits::One;
        let shape = vec![$dim1, $dim2];
        let size = shape.iter().product::<usize>();

        let one_value: $ty = <$ty as One>::one();
        let data: Vec<$ty> = vec![one_value; size];

        let shape = $crate::Shape::new($crate::ix::Ix::<2>::new(shape.try_into().unwrap()));
        $crate::Array::new(data, shape).unwrap()
    }};

    ($ty:ty, $dim1:expr, $dim2:expr, $dim3:expr) => {{
        use ::num_traits::One;
        let shape = vec![$dim1, $dim2, $dim3];
        let size = shape.iter().product::<usize>();

        let one_value: $ty = <$ty as One>::one();
        let data: Vec<$ty> = vec![one_value; size];

        let shape = $crate::Shape::new($crate::ix::Ix::<3>::new(shape.try_into().unwrap()));
        $crate::Array::new(data, shape).unwrap()
    }};

    ($ty:ty, $($dim:expr),+) => {{
        let shape = vec![$($dim),+];
        let dimension = shape.len();
        panic!("Unsupported number of dimensions (only 1D, 2D, and 3D are supported): {}", dimension);
    }};
}
