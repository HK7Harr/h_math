mame: h_math
description: This is a general math library/crate for Rust. When i consider this to be done i will publish it commercially on crate.io.

I strongly reccomend using h_math::prelude::*

evrything you will be calling if you use prelude will start with h_

i will only be implementing .h_average() syntax

Documentation:

h_average
{
    // the return type will be f64
    let list: Vec<i32> = vec![4,8,12];
    let average: f64 = list.h_average();
    println!("Average: {}", average); // output: 8.0, 4+8+12 = 24/3 = 8
}

h_median
{
    // the return type will be f64
    let list: Vec<i32> = vec![4,8,12];
    let median: f64 = list.h_median();
    println!("Median: {}", median); // output: 8.0, len = odd, list[mid] = 8
}

h_sum
{
    // the return type will be the type of the array/list
    let list: Vec<i32> = vec![4,8,12];
    let sum: i32 = list.h_sum();
    println!("Sum: {}", sum); // output: 24
    // i don`t belive the dynamic type output will be a problem since you already know the type of the list
}

h_variance
{
    // the return type will be the type of the array/list
    let list: Vec<i32> = vec![4,8,12];
    let variance: i32 = list.h_variance();
    println!("Variance: {}", variance); // output 8, 12 - 4 = 8
}

h_modus_mult 
{
    // the return type will be a vector of type of the array/vec
    let list: Vec<i32> = vec![10, 10, 20, 20, 20, 25, 25, 25];
    let modus_mult_list: Vec<i32> = list.h_modus_mult();
    println!("Modus Mult, {:?}", modus_mult_list); // output: 20, 25
    // modus mult returns the element(s) that shows up the most
    // if there are no likewise elements or no elements at all the return element will be an empty list
}

