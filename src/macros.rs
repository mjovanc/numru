#[macro_export]
macro_rules! array {
    ($($elem:expr),*) => {
        nr::Array::new(vec![$($elem),*])
    };
}
