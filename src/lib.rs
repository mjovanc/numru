#[macro_use]
pub mod macros;
pub mod array;
pub mod dimension;
pub mod errors;
pub mod ix;
pub mod shape;

pub use array::Array;
pub use dimension::Dimension;
pub use errors::ArrayError;
pub use ix::Ix;
pub use shape::Shape;
