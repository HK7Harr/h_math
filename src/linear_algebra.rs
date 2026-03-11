use std::iter::zip;

/// Hadamard product of two vectors. The vectors must have the same length.
/// Formula: C = A ⊙ B, where C[i] = A[i] * B[i]
/// Example:
/// let vec1 = vec![1.0, 2.0, 3.0];
/// let vec2 = vec![4.0, 5.0, 6.0];
/// let result = h_hadamard_product(&vec1, &vec2);
/// The result will be vec![4.0, 10.0, 18.0], because C[0] = 1*4 = 4, C[1] = 2*5 = 10, C[2] = 3*6 = 18.
pub fn h_hadamard_product<I, T>(vec1: &[I], vec2: &[T]) -> Vec<f64> 
where
    T: Copy + Into<f64>,
    I: Copy + Into<f64>,
{
    let mut new_vec: Vec<f64> = Vec::new();
    if vec1.len() != vec2.len() {
        panic!("from: h_hadamard_product, Vectors must have the same length");
    }
    for (a, b) in zip(vec1, vec2) {
        new_vec.push((*a).into() * (*b).into())
    }
    return new_vec;
}


/// Calculates the dot product of two vectors. The vectors must have the same length.
/// Formula: C = A · B, where C = Σ(A[i] * B[i])
/// Example:
/// let vec1 = vec![1.0, 2.0, 3.0];
/// let vec2 = vec![4.0, 5.0, 6.0];
/// let result = h_2d_dot_product(&vec1, &vec2);
/// The result will be 32.0, because 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32.
pub fn h_dot<A, B>(vec1: &[A], vec2: &[B]) -> f64 
where 
    A: Copy + Into<f64>,
    B: Copy + Into<f64>,
{
    if vec1.len() != vec2.len() {
        panic!("h_2d_dot_product, the two vectors does not have the same length");
    }
    let mut sum: f64 = 0.0;
    for (a, b) in zip(vec1, vec2) {
        sum += (*a).into()*(*b).into();
    }
    return sum;
}


/// Calculates the magnitude of a vector. The magnitude is the square root of the sum of the squares of the components.
/// Formula: ||A|| = sqrt(A[0]^2 + A[1]^2 + ... + A[n]^2)
/// Example:
/// let vec = vec![3.0, 4.0];
/// let result = vec.h_vector_magnitude();
/// The result will be 5.0, because sqrt(3^2 + 4^2) = sqrt(9 + 16) = sqrt(25) = 5.
/// If the vector were vec![1.0, 2.0, 2.0], the result would be 3.0, because sqrt(1^2 + 2^2 + 2^2) = sqrt(1 + 4 + 4) = sqrt(9) = 3.
/// the reason for the formula is that that is the pythagorean theorem for n dimensions, 
/// and since you can always make a right triangle with the vector as the hypotenuse, the magnitude is the length of the hypotenuse, which is the square root of the sum of the squares of the legs, which are the components of the vector.
/// If the vector were vec![0.0, 0.0, 0.0], the result would be 0.0, because sqrt(0^2 + 0^2 + 0^2) = sqrt(0) = 0.
/// it is implemented for n-dimensional space but i dont know why anyone would want to use it for more than 3 dimensions, but it is there if you need it.
/// nut the feature is there if you need it.
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

pub fn h_angle_between_vectors<T, I>(vec1: &[T], vec2: &[I]) -> f64 
where   
    T: Copy + Into<f64>,
    I: Copy + Into<f64>,
{
    let vec1_magnitude: f64 = vec1.h_magnitude();
    let vec2_magnitude: f64 = vec2.h_magnitude();

    let angle_between: f64 = ((h_dot(vec1, vec2))/(vec1_magnitude*vec2_magnitude)).acos();
    return angle_between;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_hadamard_product() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![4.0, 5.0, 6.0];
        let result = h_hadamard_product(&vec1, &vec2);
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
}