h_math

Description:
`h_math` is a general-purpose math library (crate) for Rust.

It provides statistical, geometric, core math, and finance-related calculations
through simple and intuitive traits.

Usage:
use h_math::prelude::*;

Example:
let list = vec![4, 8, 12];
let avg = list.h_average();

Documentation

h_average:
    let list = vec![4,8,12];
    let avg = list.h_average(); // 8.0

h_median:
    let list = vec![4,8,12];
    let median = list.h_median(); // 8.0

h_sum:
    let list = vec![4,8,12];
    let sum = list.h_sum(); // 24

h_variance:
    let list = vec![4,8,12];
    let var = list.h_variance(); // 8

h_modus_mult:
    let list = vec![10,10,20,20,20,25,25,25];
    let mode = list.h_modus_mult(); // [20.0,25.0]

h_search:
    let list = vec![1,2,3];
    list.h_search(2.0); // true

h_circle_circumference:
    let r = 10.0;
    r.h_circle_circumference(); // 62.8318

h_circle_area:
    let r = 10.0;
    r.h_circle_area(); // 314.159

h_sphere_volume:
    let r = 10.0;
    r.h_sphere_volume(); // 4188.79

h_sphere_surface_area:
    let r = 10.0;
    r.h_sphere_surface_area(); // 1256.637

h_factorial:
    let n = 5;
    n.h_factorial(); // 120

h_sqrt_degree:
    let num = 27.0;
    num.h_sqrt_degree(3); // 3.0

h_return_on_investment:
    let start = 100.0;
    start.h_return_on_investment(150.0); // 50%








