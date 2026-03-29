use std::iter::zip;


/// A struct representing a matrix, which is a 2D array of values.
/// The matrix is stored in a flat vector (`data`) along with its dimensions 
/// (`row_size` and `columm_size`).
/// 
/// 
#[derive(Debug, PartialEq, Eq)]
pub struct HMatrix<T>
where
    T: Copy + Into<f64>,
{
    data: Vec<T>,
    row_size: usize,
    columm_size: usize,
}

impl<T> HMatrix<T>
where
    T: Copy + Into<f64>,
{
    pub fn new() -> Self {
        HMatrix { 
            data: Vec::new(),
            row_size: 0, 
            columm_size: 0 
        }
    }

    /// Creates a new `HMatrix` from a slice of rows, where each row is a vector of values.
    /// All rows must have the same length, and the slice must not be empty. If these conditions are not met, 
    /// the function returns `None`.
    pub fn new_from_rows(rows: &[Vec<T>]) -> Option<Self> {
        if rows.is_empty() {
            return None;
        }
        let mut row_size_prev: Option<usize> = None;
        let columm_size: usize = rows.len();
        let mut data: Vec<T> = Vec::new();

        for row in rows {
            if row_size_prev == None {
                row_size_prev = Some(row.len());
                if row_size_prev == Some(0) {
                    return None;
                }
            }
            if row_size_prev != Some(row.len()) {
                return None;
            }
            for item in row {
                data.push(*item);
            }
        }
        Some(HMatrix {
            data,
            row_size: row_size_prev.unwrap(),
            columm_size,
        })
    }
    /// Creates a new `HMatrix` from a slice of columns, where each column is a vector of values.
    /// All columns must have the same length, and the slice must not be empty. If these conditions are not met, 
    /// the function returns `None`.
    pub fn new_from_cols(cols: &[Vec<T>]) -> Option<Self> {
        if cols.is_empty() {
            return None;
        }
        let mut data: Vec<T> = Vec::new();
        let row_size: usize = cols[0].len();
        let mut columm_size_prev: Option<usize> = None;

        for r_i in 0..cols[0].len() {
            for c_i in 0..cols.len() {
                if columm_size_prev == None {
                    columm_size_prev = Some(cols[c_i].len());
                }
                if Some(cols[c_i].len()) != columm_size_prev {
                    return None;
                }
                if cols[c_i].is_empty() {
                    return None;
                }
                data.push(cols[c_i][r_i]);
            }
        }
        Some(HMatrix {
            data,
            row_size,
            columm_size: cols.len(),
        })
    }

    /// Retrieves the value at the specified row and column indices. If the indices are out of bounds,
    /// the function returns `None`. Otherwise, it returns the value wrapped in `Some`.
    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if row >= self.columm_size || col >= self.row_size {
            return None;
        }
        Some(self.data[row * self.row_size + col])
    }

    /// Retrieves a submatrix defined by the specified row and column ranges. The `row_range` and `col_range`
    /// are tuples containing the minimum (inclusive) and maximum (exclusive) indices for rows and columns, respectively.
    /// If the specified ranges are invalid (e.g., out of bounds, or min >= max), the function returns `None`. 
    /// Otherwise, it constructs and returns the submatrix as an `HMatrix` wrapped in `Some`.
    pub fn get_mult(&self, row_range: (usize, usize), col_range: (usize, usize)) -> Option<HMatrix<T>> {
        let (row_min, row_max) = row_range;
        let (col_min, col_max) = col_range;

        if row_min >= self.columm_size || row_max > self.columm_size || row_min >= row_max {
            return None;
        }
        if col_min >= self.row_size || col_max > self.row_size || col_min >= col_max {
            return None;
        }

        let mut data: Vec<T> = Vec::new();
        for r in row_min..row_max {
            for c in col_min..col_max {
                data.push(self.data[r * self.row_size + c]);
            }
        }

        Some(HMatrix {
            data,
            row_size: col_max - col_min,
            columm_size: row_max - row_min,
        })
    }

    /// Retrieves the specified row as a vector. If the index is out of bounds, the function returns `None`.
    /// Otherwise, it constructs and returns the row as a `Vec<T>` wrapped in `Some`.
    pub fn get_row(&self, index: usize) -> Option<Vec<T>> {
        let mut row: Vec<T> = Vec::new();

        if index >= self.columm_size {
            return None;
        }

        // 1, 2, 3
        // 4, 5, 6
        // 7, 8, 9
        // index = 1
        // 3*1 = 4

        let start_index: usize = self.row_size * index;
        for i in start_index..start_index+self.row_size {
            row.push(self.data[i]);
        }
        return Some(row);
    }

    pub fn get_col(&self, index: usize) -> Option<Vec<T>> {
        let mut columm: Vec<T> = Vec::new();
        if index >= self.row_size {
            return None;
        }
        for i in 0..self.columm_size {
            columm.push(self.data[i * self.row_size + index]);
        }   
        return Some(columm);
    }

    pub fn add_row(&mut self, row: Vec<T>) -> Result<(), String> {
        if self.columm_size == 0 && self.row_size == 0 {
            for i in row {
                self.data.push(i);
                self.row_size += 1;
            }
            self.columm_size += 1;
            return Ok(());
        } else {
            if row.len() != self.row_size {
                return Err(String::from("The row does not have the same size as the other in the Matrix. from: HMatrix, add_row()"));
            }
            else {
                for i in row {
                    self.data.push(i);
                }
                self.columm_size += 1;
                return Ok(());
            }
        }
    }
    pub fn add_col(&mut self, col: Vec<T>) -> Result<(), String> {
        if col.len() != self.columm_size {
            return Err(format!(
                "column length {} does not match matrix row count {}",
                col.len(),
                self.columm_size
            ));
        }
        for r in 0..self.columm_size {
            let insert_index = (r + 1) * self.row_size + r;
            self.data.insert(insert_index, col[r]);
        }
        self.row_size += 1;
        Ok(())
    }
}


/// Hadamard product (element-wise multiplication) of two vectors.
/// Both vectors must have the same length.
/// Formula: C = A ⊙ B, where C[i] = A[i] * B[i]
///
/// Example:
/// 
/// let vec1 = vec![1.0, 2.0, 3.0];
/// let vec2 = vec![4.0, 5.0, 6.0];
/// let result = h_hadamard(&vec1, &vec2);
/// assert_eq!(result, vec![4.0, 10.0, 18.0]);
/// The Hadamard product is calculated as (1*4, 2*5, 3*6) = (4.0, 10.0, 18.0).

pub fn h_hadamard<I, T>(vec1: &[I], vec2: &[T]) -> Vec<f64> 
where
    T: Copy + Into<f64>,
    I: Copy + Into<f64>,
{
    let mut new_vec: Vec<f64> = Vec::new();
    if vec1.len() != vec2.len() {
        panic!("from: h_hadamard, vectors must have the same length");
    }
    for (a, b) in zip(vec1, vec2) {
        new_vec.push((*a).into() * (*b).into())
    }
    return new_vec;
}


/// Calculates the dot product of two vectors. The vectors must have the same length.
/// Formula: C = A · B, where C = Σ(A[i] * B[i])
///
/// Example:
/// 
/// let vec1 = vec![1.0, 2.0, 3.0];
/// let vec2 = vec![4.0, 5.0, 6.0];
/// let result = h_dot(&vec1, &vec2);
/// assert_eq!(result, 32.0);
/// The dot product is calculated as (1*4) + (2*5) + (3*6) = 4 + 10 + 18 = 32.0.

pub fn h_dot<A, B>(vec1: &[A], vec2: &[B]) -> f64 
where 
    A: Copy + Into<f64>,
    B: Copy + Into<f64>,
{
    if vec1.len() != vec2.len() {
        panic!("h_dot: the two vectors do not have the same length");
    }
    let mut sum: f64 = 0.0;
    for (a, b) in zip(vec1, vec2) {
        sum += (*a).into()*(*b).into();
    }
    return sum;
}


/// Trait that provides the Euclidean magnitude (length) of a vector.
///
/// The magnitude of a vector `A` is defined as:
/// ```text
/// ||A|| = sqrt(A[0]^2 + A[1]^2 + ... + A[n]^2)
/// ```
///
/// This is simply the Euclidean norm and corresponds to the length of the
/// vector in n‑dimensional space.
///
/// # Examples
///
/// 
/// let v = vec![3.0, 4.0];
/// assert_eq!(v.h_magnitude(), 5.0);
/// 
///
/// The trait is implemented for slices whose element type can be converted into
/// `f64`.
pub trait Magnitude {
    fn h_magnitude(&self) -> f64;
}

impl<T> Magnitude for [T] 
where 
    T: Copy + Into<f64>,
{
    fn h_magnitude(&self) -> f64 {
        let sum_of_squares: f64 = self.iter().map(|x| {
            let v: f64 = (*x).into();
            v * v
        }).sum();
        sum_of_squares.sqrt()
    }
}



/// Adds two vectors element-wise. The vectors must have the same length.
/// Formula: C = A + B, where C[i] = A[i] + B[i]
///
/// Example:
/// 
/// let vec1 = vec![1.0, 2.0, 3.0];
/// let vec2 = vec![4.0, 5.0, 6.0];
/// let result = h_vector_add(&vec1, &vec2);
/// assert_eq!(result, vec![5.0, 7.0, 9.0]);
/// The vector addition is calculated as (1+4, 2+5, 3+6) = (5.0, 7.0, 9.0).

pub fn h_vector_add<T, I>(vec1: &[T], vec2: &[I]) -> Vec<f64>
where
    T: Copy + Into<f64>,
    I: Copy + Into<f64>,
{
    if vec1.len() != vec2.len() {
        panic!("from: h_vector_add, the vectors do not have the same length");
    }

    zip(vec1, vec2)
        .map(|(a, b)| (*a).into() + (*b).into())
        .collect()
}

/// Subtracts the second vector from the first, element‑wise. Both vectors must have
/// the same length.
///
/// Example:
/// 
/// let vec1 = vec![5.0, 7.0, 9.0];
/// let vec2 = vec![1.0, 2.0, 3.0];
/// let result = h_vector_sub(&vec1, &vec2);
/// assert_eq!(result, vec![4.0, 5.0, 6.0]);
/// 
pub fn h_vector_sub<T, I>(vec1: &[T], vec2: &[I]) -> Vec<f64>
where
    T: Copy + Into<f64>,
    I: Copy + Into<f64>,
{
    if vec1.len() != vec2.len() {
        panic!("from: h_vector_sub, the vectors do not have the same length");
    }

    zip(vec1, vec2)
        .map(|(a, b)| (*a).into() - (*b).into())
        .collect()
}

/// Multiply every element of a slice by a scalar value.
///
/// This trait is implemented for slice types so that you can write
/// `my_slice.h_vector_scalar_mult(k)` and receive a new `Vec<f64>` containing
/// each element of `my_slice` multiplied by `k`.
///
/// # Example
/// let v = vec![1.0, 2.0, 3.0];
/// assert_eq!(v.h_vector_scalar_mult(2.0), vec![2.0, 4.0, 6.0]);

pub trait VectorScalarMultiply<S>
where
    S: Copy + Into<f64>,
{
    fn h_vector_scalar_mult(&self, scalar: S) -> Vec<f64>;
}

impl<S, T> VectorScalarMultiply<S> for [T]
where 
    S: Copy + Into<f64>,
    T: Copy + Into<f64>,
{
    fn h_vector_scalar_mult(&self, scalar: S) -> Vec<f64> {
        self.iter().map(|x| (*x).into() * scalar.into()).collect()
    }
}



/// Divide every element of a slice by a scalar value.
///
/// Similar to [`VectorScalarMultiply`], this trait is implemented for slice types
/// and returns a `Vec<f64>` containing each element divided by the provided
/// scalar.
///
/// # Example
///
/// 
/// let v = vec![4.0, 6.0, 8.0];
/// assert_eq!(v.h_vector_scalar_div(2.0), vec![2.0, 3.0, 4.0]);
/// The vector scalar division is calculated as (4/2, 6/2, 8/2) = (2.0, 3.0, 4.0).
pub trait VectorScalarDivision<S>
where
    S: Copy + Into<f64>,
{
    fn h_vector_scalar_div(&self, scalar: S) -> Vec<f64>;
}

impl<S, T> VectorScalarDivision<S> for [T]
where 
    S: Copy + Into<f64>,
    T: Copy + Into<f64>,
{
    fn h_vector_scalar_div(&self, scalar: S) -> Vec<f64> {
        self.iter().map(|x| (*x).into() / scalar.into()).collect()
    }
}

/// Specifies the unit in which an angle is returned.
#[derive(Debug, Eq, PartialEq)]
pub enum Measurement {
    /// Return the angle in radians.
    Radians,
    /// Return the angle in degrees.
    Degrees,
}

/// Calculates the angle between two 2‑dimensional vectors.
///
/// Both `vec1` and `vec2` must have exactly two components; otherwise,
/// the function returns `None`. If either vector has zero magnitude the
/// angle is undefined and `None` is returned as well.
///
/// The result is returned in radians or degrees depending on the
/// `return_measurement` parameter.
///
/// # Examples
///
/// 
/// use h_math::linear_algebra::{h_vectors_angle, Measurement};
/// let a = vec![0.0, 1.0];
/// let b = vec![1.0, 0.0];
/// assert_eq!(h_vectors_angle(&a, &b, Measurement::Degrees).unwrap(), 90.0);
/// The angle between the vectors (0, 1) and (1, 0) is 90 degrees because they are perpendicular to each other.
pub fn h_vectors_angle<T, I>(vec1: &[T], vec2: &[I], return_measurement: Measurement) -> Option<f64> 
where   
    T: Copy + Into<f64>,
    I: Copy + Into<f64>,
{
    if vec1.len() != 2 || vec2.len() != 2 {
        return None;
    }
    let vec1_magnitude: f64 = vec1.h_magnitude();
    let vec2_magnitude: f64 = vec2.h_magnitude();

    if vec1_magnitude == 0.0 || vec2_magnitude == 0.0 {
        return None;
    }

    let angle_between: f64 = ((h_dot(vec1, vec2))/(vec1_magnitude*vec2_magnitude)).acos();
    match return_measurement {
        Measurement::Radians => return Some(angle_between),
        Measurement::Degrees => return Some(angle_between.to_degrees()),
    }
}



/// Trait for performing linear transformations on vectors using a matrix.
/// The trait is implemented for slices of any type that can be converted into `f64`.
/// The transformation is performed by multiplying the vector by the provided matrix.
/// The matrix is represented by the `HMatrix` struct, which contains the data and dimensions of the matrix.
/// The `h_linear_transform` method takes a reference to an `HMatrix` and returns an `Option<Vec<f64>>` 
/// the formula for a 2d transformation is::
/// vec = [x, y]
/// vec = [3, 6]
/// matrix = [ix, jx]
///          [iy, jy]
/// matrix = [2, 1]
///          [3, 4]
/// result:
/// x* [ix, jx] + y*[iy, jy]
pub trait LinearTransform<T>
where
    T: Copy + Into<f64>,
{
    fn h_linear_transform(&self, matrix: &HMatrix<T>) -> Option<Vec<f64>>;
}

impl<T, S> LinearTransform<T> for [S]
where
    T: Copy + Into<f64>,
    S: Copy + Into<f64>,
{
    fn h_linear_transform(&self, matrix: &HMatrix<T>) -> Option<Vec<f64>> {
        if self.len() != matrix.row_size {
            return None;
        }
        let result: Vec<f64> = (0..matrix.columm_size)
            .map(|r| {
                let row = matrix.get_row(r).unwrap();
                h_dot(&row, self)
            })
            .collect();
        Some(result)
    }
}


/// Computes the linear composition (matrix multiplication) of two matrices.
///
/// Multiplies `matrix2 * matrix1`, applying `matrix2` as a linear transform
/// to each column of `matrix1`. The number of columns in `matrix2` must equal
/// the number of rows in `matrix1`.
///
/// # Parameters
/// - `matrix2`: the left-hand matrix, shape (m × n)
/// - `matrix1`: the right-hand matrix, shape (n × p)
///
/// # Returns
/// - `None` if `matrix2.row_size != matrix1.columm_size`
/// - `Some(HMatrix<f64>)` of shape (m × p) on success
///
/// # Note
/// The result is always `HMatrix<f64>` regardless of input type, because all
/// values are converted to `f64` during the dot product calculations.
///
/// # Example
/// 
/// let m1 = HMatrix::new_from_rows(&[vec![1, 2], vec![3, 4]]).unwrap();
/// let m2 = HMatrix::new_from_rows(&[vec![5, 6], vec![7, 8]]).unwrap();
/// let result = h_linear_composition(&m2, &m1).unwrap();
/// result is a 2x2 matrix: [[19, 22], [43, 50]]
pub fn h_linear_composition<T>(matrix2: &HMatrix<T>, matrix1: &HMatrix<T>) -> Option<HMatrix<f64>>
where 
    T: Copy + Into<f64>,
{
    if matrix2.row_size != matrix1.columm_size {
        return None;
    }
    let mut new_matrix: HMatrix<f64> = HMatrix::new();
    for i in 0..matrix1.row_size {
        // this i guaranteed to be safe because of the rules already in place
        new_matrix.add_col(matrix1.get_col(i).unwrap().h_linear_transform(&matrix2).unwrap());
    }
    return Some(new_matrix);
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_hadamard() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];
        let result = h_hadamard(&vec1, &vec2);
        assert_eq!(result, vec![4.0, 10.0, 18.0]);
    }

    #[test]
    fn test_h_dot() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];
        let result = h_dot(&vec1, &vec2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_magnitude() {
        let vec = vec![3.0, 4.0];
        assert_eq!(vec.h_magnitude(), 5.0);
    }

    #[test]
    fn test_h_vector_add() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];
        let result = h_vector_add(&vec1, &vec2);
        assert_eq!(result, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_h_vector_sub() {
        let vec1 = vec![5.0, 7.0, 9.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        let result = h_vector_sub(&vec1, &vec2);
        assert_eq!(result, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_vector_scalar_mult() {
        let vec = vec![1.0, 2.0, 3.0];
        let result = vec.h_vector_scalar_mult(2.0);
        assert_eq!(result, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_vector_scalar_div() {
        let vec = vec![4.0, 6.0, 8.0];
        let result = vec.h_vector_scalar_div(2.0);
        assert_eq!(result, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_vectors_angle() {
        let vec1: Vec<i32> = vec![0, 1];
        let vec2: Vec<i32> = vec![1, 0];
        assert_eq!(h_vectors_angle(&vec1, &vec2, Measurement::Degrees).unwrap_or(0.0), 90.0);
    }

    #[test]
    fn test_h_linear_transform() {
        let vec = vec![1.0, 2.0];
        let matrix = HMatrix::new_from_rows(&vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
        let result = vec.h_linear_transform(&matrix);
        assert_eq!(result.unwrap_or_else(|| vec![]), vec![1.0, 2.0]);
    }

    #[test]
    fn test_h_linear_composition() {
        let m1 = HMatrix::new_from_rows(&vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let m2 = HMatrix::new_from_cols(&vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let m3 = HMatrix::new_from_cols(&vec![vec![14.0, 32.0], vec![32.0, 77.0]]).unwrap();
        assert_eq!(m3, h_linear_composition(&m2, &m1).unwrap());
    }
}

