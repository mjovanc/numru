#[macro_export]
macro_rules! array {
    () => {
        crate::array::Array::new(Vec::new())
    };
    ($($elem:expr),*) => {
        crate::array::Array::new(vec![$($elem),*])
    };
}
