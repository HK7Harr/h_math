// --------------------------------- Algebra ------------------------------


/// Solves the quadratic equation ax^2 + bx + c = 0 and returns the two roots as a tuple (root1, root2).
/// The coefficients a, b, and c can be of any type that can be converted into
/// f64. If the discriminant is negative, the function will panic with a message indicating that no real roots exist.
/// Example usage:
/// let (root1, root2) = h_quadratic_equation(1.0, -3.0, 2.0);
/// The result will be (2.0, 1.0), because the roots of the equation x^2 - 3x + 2 = 0 are x = 2 and x = 1.
/// let (root1, root2) = h_quadratic_equation(1.0, 2.0, 5.0);
/// The function will panic with the message "No real roots exist" because the discriminant (b^2 - 4ac) is negative.
/// Note: The order of the roots in the returned tuple is not guaranteed, so root1 may be the larger or smaller root depending on the coefficients.
pub fn h_quadratic_equation<A,B,C>(a: A, b: B, c: C) -> (f64, f64)
where
    A: Copy + Into<f64>,
    B: Copy + Into<f64>,
    C: Copy + Into<f64>,
{
    let a_f = a.into();
    let b_f = b.into();
    let c_f = c.into();

    let discriminant = b_f * b_f - 4.0 * a_f * c_f;

    if discriminant < 0.0 {
        panic!("No real roots exist");
    }

    let sqrt_discriminant = discriminant.sqrt();
    let root1: f64 = (-b_f + sqrt_discriminant) / (2.0 * a_f);
    let root2: f64 = (-b_f - sqrt_discriminant) / (2.0 * a_f);

    (root1, root2)
}

/// This function i just for testing purposes, it checks if the value of x satisfies the equation num_left + x*x_left = num_right + x*x_right
/// Example usage:
/// let is_solution = h_simple_eq_checker_x(2.0, 3.0, 4.0, 5.0, 6.0);
/// The result will be true, because 3.0 + 2.0*4.0 = 5.0 + 2.0*5.0, which simplifies to 3.0 + 8.0 = 5.0 + 10.0, and both sides equal 11.0.
/// let is_solution = h_simple_eq_checker_x(1.0, 3.0, 4.0, 5.0, 6.0);
/// The result will be false, because 3.0 + 1.0*4.0 = 5.0 + 1.0*5.0, which simplifies to 3.0 + 4.0 = 5.0 + 5.0, and the left side equals 7.0 while the right side equals 10.0, so they are not equal.
/// Note: This function is not a general equation solver, it only checks if a specific value of x satisfies the given linear equation. It is intended for testing and demonstration purposes only.
pub fn h_simple_eq_checker_x<NL, XL, NR, XR, X>(x_value: X, num_left: NL, x_left: XL, num_right: NR, x_right: XR) -> bool 
where
    NL: Copy + Into<f64>,
    XL: Copy + Into<f64>,
    NR: Copy + Into<f64>,
    XR: Copy + Into<f64>,
    X: Copy + Into<f64>,
{
    if x_left.into()*x_value.into() + num_left.into() == x_right.into()*x_value.into() + num_right.into() {
        return true;
    }
    false
}
