# h_math

A comprehensive math library for Rust, providing a wide range of mathematical functions and utilities. The library is designed to be easy to use with traits and generics.

## Features

- **Naming Convention**: All traits and functions follow the `h_` prefix convention (e.g., `h_average`, `h_median`, `h_factorial`, `h_sphere_volume`)
- **Generic Traits**: Implemented for primitive types, allowing direct method calls on values
- **No Naming Conflicts**: The `h_` prefix ensures compatibility with other libraries
- **Comprehensive Coverage**: Core math, algebra, geometry, statistics, finance, and more
- **Lightweight, but capable**: This library has many simple and some more complicated function, but they are more lightweight and broad compared to other math libraries like nalgebra and ndarray, which are sturdier but less casual to use
- **One for all**: Though this library is relatively small for now, it does have great variety and i plan on on developing it for better coverage and more features
## Getting Started

```rust
use h_math::prelude::*;
```

This imports all traits and functions into scope. Since traits are implemented for primitive types, you can call methods directly:

```rust
let avg = [1.0, 2.0, 3.0, 4.0, 5.0].h_average();
let factorial = 5u32.h_factorial();
let volume = 5.h_sphere_volume();
let golden_crosses = [1, 2, 3, 4, 5, 6].h_golden_crosses();

let hypot = h_pythagorean_theorem(3, 4.0);
let added_vector = h_vector_add([3, 6], vec![1.0, 9.0]);
```


