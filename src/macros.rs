/// Purpose:
///
/// The `arr!` macro is designed to accept arrays of any depth (1D, 2D, 3D, etc.) and flatten them into a
/// single-dimensional vector. It also tracks and stores the shape (dimensions) of the array, which includes
/// the number of rows, columns, and further dimensions as needed.
///
/// Flattening Process:
///
/// - The macro first initializes two vectors: one for storing the flattened data (`data`) and another for
///   storing the shape (the dimensions) of the array (`shape`).
/// - It iterates over the elements passed to the macro, checking each element to determine if it’s a scalar
///   or a nested array.
/// - If the element is a nested array (i.e., a `Vec`), the macro recursively flattens it into the `data`
///   vector and records the size of each nested array in the `shape` vector.
/// - Scalar values (non-array values) are directly added to the `data` vector.
/// - This recursive flattening allows the macro to handle arrays of any depth.
///
/// Shape Representation:
///
/// - The shape of the array is tracked by storing the length of each dimension in the `shape` vector. The
///   first value in `shape` represents the top-level dimension (number of elements in the outermost array),
///   and subsequent values represent the sizes of nested dimensions.
/// - For example, for a 2D array with 3 rows and 2 columns, the shape would be represented as `[3, 2]`.
///
/// Usage and Output:
///
/// - The macro returns the flattened data and the shape of the array, allowing you to visualize both the
///   values and the structure of the array.
/// - For debugging purposes, both the flattened array (`data`) and its shape (`shape`) are printed, so you
///   can see how the input is processed.
///
/// Example (1D):
///
/// For an input like:
/// ```txt
/// let b = arr![1, 2, 3, 4];
/// ```
/// The macro will:
/// - Flatten the data into a single vector: `[1, 2, 3, 4]`
/// - Track the shape as `[4]` (1D array with 4 elements).
///
/// The output would look like:
/// ```txt
/// Flattened data: [1, 2, 3, 4]
/// Shape: [4]
/// ```
///
/// Example (2D):
///
/// For an input like:
/// ```txt
/// let b = arr![[1, 2], [3, 4], [5, 6]];
/// ```
/// The macro will:
/// - Flatten the data into a single vector: `[1, 2, 3, 4, 5, 6]`
/// - Track the shape as `[3, 2]` (3 rows, 2 columns).
///
/// The output would look like:
/// ```txt
/// Flattened data: [1, 2, 3, 4, 5, 6]
/// Shape: [3, 2]
/// ```
///
/// Example (3D):
///
/// For an input like:
/// ```txt
/// let b = arr![[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
/// ```
/// The macro will:
/// - Flatten the data into a single vector: `[1, 2, 3, 4, 5, 6, 7, 8]`
/// - Track the shape as `[2, 2, 2]` (2 blocks, 2 rows per block, 2 elements per row).
///
/// The output would look like:
/// ```txt
/// Flattened data: [1, 2, 3, 4, 5, 6, 7, 8]
/// Shape: [2, 2, 2]
/// ```
///
/// Future Extensions:
///
/// In the future, you could replace the placeholder part of the macro with logic to return a structured array
/// (such as a custom `Shape` or `Array` type), but for now, this approach allows you to debug and visualize
/// the array’s contents and shape effectively.
#[macro_export]
macro_rules! arr {
    // Recursive case: Nested arrays
    // TODO: add support for 3D and higher-dimensional arrays
    ($([$($elems:tt),+]),+ $(,)?) => {{
        fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
            nested.iter().flat_map(|inner| inner.clone()).collect()
        }

        fn get_shape<T>(nested: &[Vec<T>]) -> Vec<usize> {
            let mut shape = vec![nested.len()];
            if let Some(first) = nested.first() {
                shape.push(first.len());
            }
            shape
        }

        let temp = vec![$(vec![$($elems),+]),+];
        let data = flatten(&temp);
        print!("Flattened data:");
        let shape = get_shape(&temp);

        $crate::Array::new(data, $crate::Shape::new($crate::shape::Ix::<2>::new(shape.try_into().unwrap())))
    }};

    // Base case: Single scalar (1D array)
    ($($elem:expr),+ $(,)?) => {{
        let data = vec![$($elem),+];
        let shape = vec![data.len()];
        $crate::Array::new(data, $crate::Shape::new($crate::shape::Ix::<1>::new(shape.try_into().unwrap())))
    }};
}
