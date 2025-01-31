#[macro_export]
macro_rules! array {
    ($($elem:expr),* $(,)?) => {{
        let data = vec![$($elem),*];
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![data.len()]));
        crate::array::Array::new(data).with_shape(shape)
    }};

    ([ $([$($elem:expr),* $(,)?]),* $(,)? ]) => {{
        let data = vec![$(vec![$($elem),*]),*];
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![data.len(), data[0].len()]));
        crate::array::Array::new(data).with_shape(shape)
    }};

    ([ [ $([$($elem:expr),* $(,)?]),* $(,)? ] $(,)? ]) => {{
        let data = vec![$(vec![$(vec![$($elem),*]),*]),*];
        let shape = $crate::Shape::new($crate::shape::IxDyn::new(vec![data.len(), data[0].len(), data[0][0].len()]));
        crate::array::Array::new(data).with_shape(shape)
    }};
}