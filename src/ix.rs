use crate::shape::Dimension;

/// Fixed-size index type for multi-dimensional arrays.
///
/// This structure represents the dimensionality of an array, where `N` is the number of dimensions.
/// For example, `Ix<2>` would represent a 2D array, while `Ix<3>` would represent a 3D array.
///
/// # Examples
///
/// ```
/// use numru::shape::Dimension;
/// use numru::ix::Ix;
///
/// // Create a 2D array index
/// let ix2d = Ix::<2>::new([3, 4]);
/// assert_eq!(ix2d.ndim(), 2);
/// assert_eq!(ix2d.size(), 12);
/// assert_eq!(ix2d.dims(), &[3, 4]);
///
/// // Create a 3D array index
/// let ix3d = Ix::<3>::new([2, 3, 4]);
/// assert_eq!(ix3d.ndim(), 3);
/// assert_eq!(ix3d.size(), 24);
/// assert_eq!(ix3d.dims(), &[2, 3, 4]);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Ix<const N: usize> {
    dims: [usize; N],
}

impl<const N: usize> Ix<N> {
    /// Creates a new `Ix` from a fixed-size array of dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::ix::Ix;
    ///
    /// let ix = Ix::<2>::new([5, 5]); // 5x5 2D array
    /// ```
    pub fn new(dims: [usize; N]) -> Self {
        Ix { dims }
    }
}

impl<const N: usize> Dimension for Ix<N> {
    /// Returns the number of dimensions represented by this `Ix`.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::Dimension;
    /// use numru::ix::Ix;
    ///
    /// let ix = Ix::<3>::new([2, 2, 2]);
    /// assert_eq!(ix.ndim(), 3);
    /// ```
    fn ndim(&self) -> usize {
        N
    }

    /// Calculates the total number of elements in the array described by this `Ix`.
    ///
    /// This is the product of all dimensions.
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
    fn size(&self) -> usize {
        self.dims.iter().product()
    }

    /// Returns a slice of the dimensions stored in this `Ix`.
    ///
    /// # Examples
    ///
    /// ```
    /// use numru::shape::Dimension;
    /// use numru::ix::Ix;
    ///
    /// let ix = Ix::<2>::new([10, 20]);
    /// assert_eq!(ix.dims(), &[10, 20]);
    /// ```
    fn dims(&self) -> &[usize] {
        &self.dims
    }
}
