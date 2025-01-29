#[macro_export]
macro_rules! array {
    () => {
        crate::nr::Array::new(Vec::new())
    };
    ($($elem:expr),*) => {
        crate::nr::Array::new(vec![$($elem),*])
    };
}
