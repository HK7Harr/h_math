use std::iter::zip;
use crate::prelude;

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
pub fn h_2d_dot_product<A, B>(vec1: &Vec<A>, vec2: &Vec<B>) -> f64 
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


pub trait Magnitude {
    fn h_vector_magnitude(&self) -> f64;
}

impl<T> Magnitude for [T] 
where 
    T: Copy + Into<f64>,
{
    fn h_vector_magnitude(&self) -> f64 {
        let mut hypot_or_length: f64 = 0.0;
        let mut hyp_counter: usize = 1;
        for _ in 2..=self.len() {
            let mut short_sides: Vec<f64> = Vec::new();

            for i in hyp_counter-1..=hyp_counter {
                short_sides.push((self[i]).into())
            }

            hyp_counter += 1;
        }
        return 0.0;
    }
}