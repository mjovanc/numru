#[macro_export]
macro_rules! array {
    ($($elem:expr),*) => {
        crate::array::Array::new(vec![$($elem),*])
    };

    ([ $($row:expr),* ]) => {{
        let mut rows = Vec::new();
        $(
            rows.push(vec![$($row),*]);
        )*
        crate::array::Array::new(rows)
    }};

    ([[ $($row:expr),* ]]) => {{
        let mut rows = Vec::new();
        $(
            let mut row_vec = Vec::new();
            $(
                row_vec.push(vec![$($row),*]);
            )*
            rows.push(row_vec);
        )*
        crate::array::Array::new(rows)
    }};
}
