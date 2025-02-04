#[macro_export]
macro_rules! arr {
    // 1D array
    ($($elem:expr),* $(,)?) => {{
        let data = vec![$($elem),*];
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![data.len()]));
        $crate::array::Array::new(data, shape)
    }};

    // 2D array
    ([ $([$($elem:expr),* $(,)?]),* $(,)? ]) => {{
        let nested = vec![$(vec![$($elem),*]),*];
        let rows = nested.len();
        let cols = nested[0].len();
        let data: Vec<_> = nested.into_iter().flat_map(|v| v.into_iter()).collect();
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![rows, cols]));
        $crate::array::Array::new(data, shape)
    }};

    // 3D array
    ([[ $([$($elem:expr),* $(,)?]),* $(,)? ], * $(,)?]) => {{
        let nested = vec![$(vec![$(vec![$($elem),*]),*]),*];
        let depth = nested.len();
        let rows = nested[0].len();
        let cols = nested[0][0].len();
        let data: Vec<_> = nested.into_iter()
            .flat_map(|m| m.into_iter())
            .flat_map(|v| v.into_iter())
            .collect();
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![depth, rows, cols]));
        $crate::array::Array::new(data, shape)
    }};
}
