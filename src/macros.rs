#[macro_export]
macro_rules! arr {
    // 1D array
    ($($elem:expr),* $(,)?) => {{
        println!("Creating 1D array");
        let data = vec![$($elem),*];
        println!("Data: {:?}", data);
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![data.len()]));
        println!("Shape: {:?}", shape);
        $crate::array::Array::new(data).with_shape(shape)
    }};

    // 2D array
    ([ $([$($elem:expr),* $(,)?]),+ $(,)? ]) => {{
        println!("Creating 2D array");
        let nested: Vec<Vec<_>> = vec![$(vec![$($elem),*]),+];
        let rows = nested.len();
        let cols = nested[0].len();
        let data: Vec<_> = nested.into_iter()
            .flat_map(|row| row.into_iter())
            .collect();
        println!("Data: {:?}", data);
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![rows, cols]));
        println!("Shape: {:?}", shape);
        $crate::array::Array::new(data).with_shape(shape)
    }};

    // 3D array
    ([[ $([$($elem:expr),+ $(,)?]),+ $(,)? ],+ $(,)?]) => {{
        println!("Creating 3D array");
        let nested: Vec<Vec<Vec<_>>> = vec![$(vec![$(vec![$($elem),+]),+]),+];
        let depth = nested.len();
        let rows = nested[0].len();
        let cols = nested[0][0].len();
        let data: Vec<_> = nested.into_iter()
            .flat_map(|matrix| matrix.into_iter())
            .flat_map(|row| row.into_iter())
            .collect();
        println!("Data: {:?}", data);
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![depth, rows, cols]));
        println!("Shape: {:?}", shape);
        $crate::array::Array::new(data).with_shape(shape)
    }};
}
