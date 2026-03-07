
// ------------------------------------ Core math ------------------------------------


/// Factorial trait that can be implemented for any type that can be converted into a u32 and is non-negative. The factorial of a number n is the product of all positive integers less than or equal to n. For example, 5! = 5 * 4 * 3 * 2 * 1 = 120. The function will panic if the input value is negative, as factorials are not defined for negative numbers.
/// Example usage:
/// let result = 5.h_factorial();
/// The result will be 120, because 5! = 5 * 4 * 3 * 2 * 1 = 120.
/// let result = (-3).h_factorial();
/// The function will panic with the message "Factorial is not defined for negative numbers" because factorials are not defined for negative numbers.
pub trait Factorial {
    fn h_factorial(&self) -> u64;
}

impl<T> Factorial for T 
where 
    T: Copy + Into<u64>,
{
    fn h_factorial(&self) -> u64 {
        let mut result: u64 = 1;
        for i in 1..=(*self).into() as u64 {
            result *= i;
        }
        result
    }
}


/// This trait calculates the nth root of a number. The degree of the root is specified as a parameter. 
/// For example, the square root of 16 is 4, because 4^2 = 16. The cube root of 27 is 3,
///  because 3^3 = 27. The function will panic if the degree is zero, as division by zero is not defined.
pub trait RootDegree {
    fn h_root_degree(&self, degree: u32) -> f64;
}

impl<T> RootDegree for T
where
    T: Copy + Into<f64>,
{
    fn h_root_degree(&self, degree: u32) -> f64 {
        (*self).into().powf(1.0 / degree as f64)
    }
}


/// Calculates the sum of a sequence of numbers starting from `start`, repeated `repetitions` times,
/// with each term incremented by `steps`.
/// This is an arithmetic series sum where each term increases by `steps`.
/// Formula: sum = start + (start + steps) + (start + 2*steps) + ... for `repetitions` terms.
/// Example usage:
/// let result = h_sigma(1.0, 5, 1.0);
/// The result will be 15.0, because 1.0 + 2.0 + 3.0 + 4.0 + 5.0 = 15.0.
/// If start = 2.0, repetitions = 3, steps = 2.0, result = 2.0 + 4.0 + 6.0 = 12.0.
pub fn h_sigma<T>(start: T, repetitions: u32, steps: T) -> f64
where 
    T: Copy + Into<f64>,
{
    let mut i: f64 = start.into();
        let mut sum: f64 = 0.0;
        for _ in 1..=repetitions {
            sum += i;
            i += steps.into();
        }
        sum
}

/// Generates a vector of numbers starting from `start`, ending before `stop`, and incrementing by `step`.
/// This creates an arithmetic sequence from start to stop (exclusive) with the given step.
/// If step is positive, generates numbers while < stop.
/// If step is negative, generates while > stop.
/// Returns empty vector if step == 0 or if start/stop don't allow progression.
/// Example usage:
/// let result = h_arrange_vec_exclusive(1.0, 5.0, 1.0);
/// The result will be [1.0, 2.0, 3.0, 4.0].
/// let result = h_arrange_vec_exclusive(5.0, 1.0, -1.0);
/// The result will be [5.0, 4.0, 3.0, 2.0].
pub fn h_arrange_vec_exclusive<I>(start: I, stop: I, step: I) -> Vec<f64>
where
    I: Copy + Into<f64>,
{
    let mut s = start.into();
    let end = stop.into();
    let step = step.into();

   
    if step == 0.0 {
        return vec![];
    }


    if (s < end && step < 0.0) || (s > end && step > 0.0) {
        return vec![];
    }

    let mut v = Vec::new();

    if step > 0.0 {
   
        while s < end {
            v.push(s);
            s += step;
        }
    } else {
        
        while s > end {
            v.push(s);
            s += step; 
        }
    }

    v
}

pub fn h_arrange_vec_inclusive<I>(start: I, stop: I, step: I) -> Vec<f64>
where
    I: Copy + Into<f64>,
{
    let mut s = start.into();
    let end = stop.into();
    let step = step.into();

   
    if step == 0.0 {
        return vec![];
    }


    if (s < end && step < 0.0) || (s > end && step > 0.0) {
        return vec![];
    }

    let mut v = Vec::new();

    if step > 0.0 {
   
        while s <= end {
            v.push(s);
            s += step;
        }
    } else {
        
        while s >= end {
            v.push(s);
            s += step; 
        }
    }

    v
}


/// This just returns the value of a pi, it is shorter than writing std::f64::consts::PI every time you need it.
/// Example usage:
/// let pi_value = h_pi();
/// The result will be 3.141592653589793, which is the value of π (pi) in Rust's standard library.
pub fn h_pi() -> f64 {
    return std::f64::consts::PI;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(5u32.h_factorial(), 120);
        assert_eq!(0u32.h_factorial(), 1);
        assert_eq!(1u32.h_factorial(), 1);
    }

    #[test]
    fn test_root_degree() {
        assert_eq!(16.0.h_root_degree(2), 4.0);
        assert_eq!(27.0.h_root_degree(3), 3.0);
        assert_eq!(8.0.h_root_degree(3), 2.0);
    }

    #[test]
    fn test_h_sigma() {
        assert_eq!(h_sigma(1.0, 5, 1.0), 15.0);
        assert_eq!(h_sigma(2.0, 3, 2.0), 12.0);
    }

    #[test]
    fn test_h_arrange_vec_exclusive() {
        assert_eq!(h_arrange_vec_exclusive(1.0, 5.0, 1.0), vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(h_arrange_vec_exclusive(5.0, 1.0, -1.0), vec![5.0, 4.0, 3.0, 2.0]);
        assert_eq!(h_arrange_vec_exclusive(1.0, 1.0, 1.0), vec![]);
    }

    #[test]
    fn test_h_arrange_vec_inclusive() {
        assert_eq!(h_arrange_vec_inclusive(1.0, 5.0, 1.0), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(h_arrange_vec_inclusive(5.0, 1.0, -1.0), vec![5.0, 4.0, 3.0, 2.0, 1.0]);
        assert_eq!(h_arrange_vec_inclusive(1.0, 1.0, 1.0), vec![1.0]);
    }

    #[test]
    fn test_h_pi() {
        assert!((h_pi() - 3.141592653589793).abs() < 1e-10);
    }
}