// --------------------------------- Algebra ------------------------------

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
