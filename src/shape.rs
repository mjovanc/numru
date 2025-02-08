use std::fmt::{Debug, Formatter, Result};

/// Represents the shape of an array or matrix, encapsulating the dimensions.
///
/// This structure wraps a type that implements `Dimension`, allowing for
/// flexible handling of array shapes in different dimensional contexts.
#[derive(Clone)]
pub struct Shape<D> {
    dims: D,
}

impl<D> Shape<D>
where
    D: Dimension,
{
    /// Constructs a new `Shape` from a given dimension.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::{Dimension, Shape};
    /// use numru::ix::Ix;
    ///
    /// let shape = Shape::new(Ix::<2>::new([3, 4]));
    /// ```
    pub fn new(dims: D) -> Self {
        Shape { dims }
    }

    /// Returns a reference to the underlying dimension structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::{Dimension, Shape};
    /// use numru::ix::Ix;
    ///
    /// let shape = Shape::new(Ix::<2>::new([3, 4]));
    /// let raw_dim = shape.raw_dim();
    /// assert_eq!(raw_dim.dims(), &[3, 4]);
    /// ```
    pub fn raw_dim(&self) -> &D {
        &self.dims
    }

    /// Calculates and returns the total number of elements in the array.
    ///
    /// This is essentially the product of all dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::{Dimension, Shape};
    /// use numru::ix::Ix;
    ///
    /// let shape = Shape::new(Ix::<3>::new([2, 3, 4]));
    /// assert_eq!(shape.size(), 24);
    /// ```
    pub fn size(&self) -> usize {
        self.dims.size()
    }

    /// Returns a slice of the dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::{Dimension, Shape};
    /// use numru::ix::Ix;
    ///
    /// let shape = Shape::new(Ix::<2>::new([5, 5]));
    /// assert_eq!(shape.dims(), &[5, 5]);
    /// ```
    pub fn dims(&self) -> &[usize] {
        self.dims.dims()
    }
}

impl<D> Debug for Shape<D>
where
    D: Debug,
{
    /// Formats the `Shape` for debugging purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::{Dimension, Shape};
    /// use numru::ix::Ix;
    ///
    /// let shape = Shape::new(Ix::<2>::new([3, 4]));
    /// assert_eq!(format!("{:?}", shape), "Shape=Ix { dims: [3, 4] }");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Shape={:?}", self.dims)
    }
}

impl<D> From<D> for Shape<D>
where
    D: Dimension,
{
    /// Converts a `Dimension` into a `Shape`.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::{Dimension, Shape};
    /// use numru::ix::Ix;
    ///
    /// let ix = Ix::<2>::new([3, 4]);
    /// let shape: Shape<Ix<2>> = Shape::from(ix);
    /// assert_eq!(shape.dims(), &[3, 4]);
    /// ```
    fn from(dimension: D) -> Shape<D> {
        Shape { dims: dimension }
    }
}

/// Trait for types that can describe the dimensions of an array.
///
/// This trait allows for different representations of dimensions while providing
/// a common interface for querying array properties.
pub trait Dimension {
    /// Returns the number of dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::Dimension;
    /// use numru::ix::Ix;
    ///
    /// let ix = Ix::<2>::new([3, 4]);
    /// assert_eq!(ix.ndim(), 2);
    /// ```
    fn ndim(&self) -> usize;

    /// Returns the total size (number of elements) of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::Dimension;
    /// use numru::ix::Ix;
    ///
    /// let ix = Ix::<3>::new([2, 3, 4]);
    /// assert_eq!(ix.size(), 24);
    /// ```
    fn size(&self) -> usize;

    /// Returns a slice of the dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::Dimension;
    /// use numru::ix::Ix;
    ///
    /// let ix = Ix::<2>::new([5, 5]);
    /// assert_eq!(ix.dims(), &[5, 5]);
    /// ```
    fn dims(&self) -> &[usize];
}
