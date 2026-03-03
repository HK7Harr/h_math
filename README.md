# h_math

A comprehensive math library for Rust, providing a wide range of mathematical functions and utilities. The library is designed to be easy to use with traits and generics.

## Features

- **Naming Convention**: All traits and functions follow the `h_` prefix convention (e.g., `h_average`, `h_median`, `h_factorial`, `h_sphere_volume`)
- **Generic Traits**: Implemented for primitive types, allowing direct method calls on values
- **No Naming Conflicts**: The `h_` prefix ensures compatibility with other libraries
- **Comprehensive Coverage**: Core math, algebra, geometry, statistics, finance, and more

## Getting Started

```rust
use h_math::prelude::*;
```

This imports all traits and functions into scope. Since traits are implemented for primitive types, you can call methods directly:

```rust
let avg = &[1.0, 2.0, 3.0, 4.0, 5.0].h_average();
let factorial = 5u32.h_factorial();
let volume = 5.0.h_sphere_volume();
```

---

## Documentation
### Core Mathematics Functions

#### Factorial Trait
- **Function**: `h_factorial()` 
- **Input**: Any numeric type implementing `Copy + PartialOrd + Into<u32>`
- **Output**: `u64`
- **Purpose**: Calculates the factorial of a number (n!)
- **Formula**: n! = n × (n-1) × (n-2) × ... × 1
- **Panics**: If given a negative number

#### Root Degree Trait
- **Function**: `h_root_degree(degree: u32)`
- **Input**: Self (numeric type), degree (u32)
- **Output**: `f64`
- **Purpose**: Calculates the nth root of a number
- **Formula**: ⁿ√x = x^(1/n)

#### h_sigma Function
- **Signature**: `h_sigma<T>(start: T, repetitions: u32, steps: T) -> f64`
- **Input**: start value, number of repetitions, step size
- **Output**: `f64` (sum)
- **Purpose**: Calculates the sum of an arithmetic sequence
- **Example**: h_sigma(1, 5, 1) sums 1+2+3+4+5 = 15

#### h_arrange_vec Function
- **Signature**: `h_arrange_vec<I>(start: I, stop: I, step: I) -> Vec<f64>`
- **Input**: start value, stop value, step size
- **Output**: `Vec<f64>` containing evenly spaced values
- **Purpose**: Creates a vector of evenly spaced numbers from start to stop
- **Returns empty**: If step is 0, or if direction is invalid (positive step but start > stop)

#### h_pi Function
- **Signature**: `h_pi() -> f64`
- **Input**: None
- **Output**: `f64` (π)
- **Purpose**: Returns the mathematical constant π (3.14159265...)

### Algebra Functions

#### h_quadratic_equation Function
- **Signature**: `h_quadratic_equation<A,B,C>(a: A, b: B, c: C) -> (f64, f64)`
- **Input**: Coefficients a, b, c of ax² + bx + c = 0
- **Output**: `(f64, f64)` tuple with two roots
- **Purpose**: Solves quadratic equations using the quadratic formula
- **Formula**: x = (-b ± √(b² - 4ac)) / 2a
- **Panics**: If discriminant is negative (no real roots)

#### h_simple_eq_checker_x Function
- **Signature**: `h_simple_eq_checker_x<...>(x_value: X, num_left: NL, x_left: XL, num_right: NR, x_right: XR) -> bool`
- **Input**: x value to check, left side numeric coefficient, left side x coefficient, right side numeric coefficient, right side x coefficient
- **Output**: `bool`
- **Purpose**: Checks if a given x value satisfies a linear equation: (x_left × x + num_left) = (x_right × x + num_right)

### Temperature Conversions

#### CelsiusToFahrenheit Trait
- **Function**: `h_celsius_to_fahrenheit()`
- **Input**: Temperature in Celsius
- **Output**: `f64` (Fahrenheit)
- **Formula**: (C × 9/5) + 32

#### FahrenheitToCelsius Trait
- **Function**: `h_fahrenheit_to_celsius()`
- **Input**: Temperature in Fahrenheit
- **Output**: `f64` (Celsius)
- **Formula**: (F - 32) × 5/9

#### CelsiusToKelvin Trait
- **Function**: `h_celsius_to_kelvin()`
- **Input**: Temperature in Celsius
- **Output**: `f64` (Kelvin)
- **Formula**: C + 273.15

#### KelvinToCelsius Trait
- **Function**: `h_kelvin_to_celsius()`
- **Input**: Temperature in Kelvin
- **Output**: `f64` (Celsius)
- **Formula**: K - 273.15

#### FahrenheitToKelvin Trait
- **Function**: `h_fahrenheit_to_kelvin()`
- **Input**: Temperature in Fahrenheit
- **Output**: `f64` (Kelvin)
- **Formula**: ((F - 32) × 5/9) + 273.15

#### KelvinToFahrenheit Trait
- **Function**: `h_kelvin_to_fahrenheit()`
- **Input**: Temperature in Kelvin
- **Output**: `f64` (Fahrenheit)
- **Formula**: ((K - 273.15) × 9/5) + 32

### Length Conversions

#### MetersToKilometers Trait
- **Function**: `h_meters_to_kilometers()`
- **Input**: Distance in meters
- **Output**: `f64` (kilometers)
- **Formula**: m ÷ 1000

#### KilometersToMeters Trait
- **Function**: `h_kilometers_to_meters()`
- **Input**: Distance in kilometers
- **Output**: `f64` (meters)
- **Formula**: km × 1000

#### CentimetersToMeters Trait
- **Function**: `h_centimeters_to_meters()`
- **Input**: Distance in centimeters
- **Output**: `f64` (meters)
- **Formula**: cm ÷ 100

#### MetersToCentimeters Trait
- **Function**: `h_meters_to_centimeters()`
- **Input**: Distance in meters
- **Output**: `f64` (centimeters)
- **Formula**: m × 100

#### CentimetersToMillimeters Trait
- **Function**: `h_centimeters_to_millimeters()`
- **Input**: Distance in centimeters
- **Output**: `f64` (millimeters)
- **Formula**: cm × 10

#### MillimetersToCentimeters Trait
- **Function**: `h_millimeters_to_centimeters()`
- **Input**: Distance in millimeters
- **Output**: `f64` (centimeters)
- **Formula**: mm ÷ 10

#### KilometersToMiles Trait
- **Function**: `h_kilometers_to_miles()`
- **Input**: Distance in kilometers
- **Output**: `f64` (miles)
- **Formula**: km × 0.621371

#### MilesToKilometers Trait
- **Function**: `h_miles_to_kilometers()`
- **Input**: Distance in miles
- **Output**: `f64` (kilometers)
- **Formula**: miles ÷ 0.621371

#### InchesToCentimeters Trait
- **Function**: `h_inches_to_centimeters()`
- **Input**: Distance in inches
- **Output**: `f64` (centimeters)
- **Formula**: inches × 2.54

#### CentimetersToInches Trait
- **Function**: `h_centimeters_to_inches()`
- **Input**: Distance in centimeters
- **Output**: `f64` (inches)
- **Formula**: cm ÷ 2.54

#### CentimetersToDecimeters Trait
- **Function**: `h_centimeters_to_decimeters()`
- **Input**: Distance in centimeters
- **Output**: `f64` (decimeters)
- **Formula**: cm ÷ 10

#### DecimetersToCentimeters Trait
- **Function**: `h_decimeters_to_centimeters()`
- **Input**: Distance in decimeters
- **Output**: `f64` (centimeters)
- **Formula**: dm × 10

### Geometry Functions

#### CircleCircumference Trait
- **Function**: `h_circle_circumference()`
- **Input**: Radius of circle
- **Output**: `f64`
- **Purpose**: Calculates the circumference of a circle
- **Formula**: 2πr

#### CircleArea Trait
- **Function**: `h_circle_area()`
- **Input**: Radius of circle
- **Output**: `f64`
- **Purpose**: Calculates the area of a circle
- **Formula**: πr²

#### h_pythagorean_theorem Function
- **Signature**: `h_pythagorean_theorem<A, B>(a: A, b: B) -> f64`
- **Input**: Two sides of a right triangle (a, b)
- **Output**: `f64` (hypotenuse)
- **Purpose**: Calculates the hypotenuse of a right triangle
- **Formula**: c = √(a² + b²)

#### h_reverse_pythagorean_theorem Function
- **Signature**: `h_reverse_pythagorean_theorem<K, H>(x: K, h: H) -> f64`
- **Input**: One known side (x) and hypotenuse (h)
- **Output**: `f64` (other side)
- **Purpose**: Finds the missing side of a right triangle
- **Formula**: a = √(h² - x²)

#### h_find_equal_legs_from_hypotenuse Function
- **Signature**: `h_find_equal_legs_from_hypotenuse<H>(h: H) -> f64`
- **Input**: Hypotenuse of an isosceles right triangle
- **Output**: `f64` (length of equal legs)
- **Purpose**: Finds the length of equal legs in a 45-45-90 triangle
- **Formula**: leg = √(h²/2)

#### ShortFromLongLeg30_60_90 Trait
- **Function**: `h_short_from_long_leg_30_60_90()`
- **Input**: Long leg of a 30-60-90 triangle
- **Output**: `f64` (short leg)
- **Formula**: short_leg = long_leg ÷ √3

### 3D Geometry Functions

#### SphereVolume Trait
- **Function**: `h_sphere_volume()`
- **Input**: Radius of sphere
- **Output**: `f64`
- **Purpose**: Calculates the volume of a sphere
- **Formula**: V = (4/3)πr³

#### SphereSurfaceArea Trait
- **Function**: `h_sphere_surface_area()`
- **Input**: Radius of sphere
- **Output**: `f64`
- **Purpose**: Calculates the surface area of a sphere
- **Formula**: A = 4πr²

#### CylinderVolume Trait
- **Function**: `h_cylinder_volume(height: H)`
- **Input**: Radius and height of cylinder
- **Output**: `f64`
- **Purpose**: Calculates the volume of a cylinder
- **Formula**: V = πr²h

#### CylinderSurfaceArea Trait
- **Function**: `h_cylinder_surface_area(height: H)`
- **Input**: Radius and height of cylinder
- **Output**: `f64`
- **Purpose**: Calculates the surface area of a cylinder
- **Formula**: A = 2πr² + 2πrh

#### CubeVolume Trait
- **Function**: `h_cube_volume()`
- **Input**: Side length of cube
- **Output**: `f64`
- **Purpose**: Calculates the volume of a cube
- **Formula**: V = s³

#### CubeSurfaceArea Trait
- **Function**: `h_cube_surface_area()`
- **Input**: Side length of cube
- **Output**: `f64`
- **Purpose**: Calculates the surface area of a cube
- **Formula**: A = 6s²

#### ConeVolume Trait
- **Function**: `h_cone_volume(height: H)`
- **Input**: Radius and height of cone
- **Output**: `f64`
- **Purpose**: Calculates the volume of a cone
- **Formula**: V = (1/3)πr²h

#### ConeSurfaceArea Trait
- **Function**: `h_cone_surface_area(height: H)`
- **Input**: Radius and height of cone
- **Output**: `f64`
- **Purpose**: Calculates the surface area of a cone
- **Formula**: A = πr(r + √(r² + h²)) where √(r² + h²) is the slant height

#### RectangularPrismVolume Trait
- **Function**: `h_rectangular_prism_volume(height: H)`
- **Input**: Length/width and height of rectangular prism
- **Output**: `f64`
- **Purpose**: Calculates the volume of a rectangular prism
- **Formula**: V = l × w × h

#### RectangularPrismSurfaceArea Trait
- **Function**: `h_rectangular_prism_surface_area(height: H)`
- **Input**: Length/width and height of rectangular prism
- **Output**: `f64`
- **Purpose**: Calculates the surface area of a rectangular prism
- **Formula**: A = 2(lw + lh + wh)

#### PyramidVolume Trait
- **Function**: `h_pyramid_volume(height: H)`
- **Input**: Base area and height of pyramid
- **Output**: `f64`
- **Purpose**: Calculates the volume of a pyramid
- **Formula**: V = (1/3) × base_area × height

#### SquarePyramidSurfaceArea Trait
- **Function**: `h_square_pyramid_surface_area(height: H)`
- **Input**: Base area and height of square pyramid
- **Output**: `f64`
- **Purpose**: Calculates the surface area of a square pyramid
- **Formula**: A = base_area + 2 × √(base_area/4) × slant_height

### Finance Functions

#### ROI Trait (Return on Investment)
- **Function**: `h_return_on_investment(new_value: f64)`
- **Input**: Initial investment value, new value
- **Output**: `f64` (percentage)
- **Purpose**: Calculates return on investment as a percentage
- **Formula**: ROI = ((new_value - initial) / initial) × 100

#### DiscountedPrice Trait
- **Function**: `h_decreased_price(decrease_percent: f64)`
- **Input**: Original price, discount percentage
- **Output**: `f64` (discounted price)
- **Purpose**: Calculates the price after a percentage discount
- **Formula**: discounted_price = original × (1 - percentage/100)

#### IncreasedPrice Trait
- **Function**: `h_increased_price(increase_percent: f64)`
- **Input**: Original price, increase percentage
- **Output**: `f64` (increased price)
- **Purpose**: Calculates the price after a percentage increase
- **Formula**: new_price = original × (1 + percentage/100)

### Probability Functions

#### h_permutations Function
- **Signature**: `h_permutations<T, S>(total: &T, select: &S) -> u64`
- **Input**: Total items (n), items to select (r)
- **Output**: `u64` (number of permutations)
- **Purpose**: Calculates the number of ways to arrange r items from n items
- **Formula**: P(n,r) = n! / (n-r)!

#### h_combinations Function
- **Signature**: `h_combinations<T, S>(total: &T, select: &S) -> u64`
- **Input**: Total items (n), items to select (r)
- **Output**: `u64` (number of combinations)
- **Purpose**: Calculates the number of ways to choose r items from n items (order doesn't matter)
- **Formula**: C(n,r) = n! / (r! × (n-r)!)

### Statistics Functions

#### Average Trait
- **Function**: `h_average()`
- **Input**: Slice of numeric values `&[T]`
- **Output**: `f64`
- **Purpose**: Calculates the arithmetic mean of a set of numbers
- **Returns 0.0**: If slice is empty

#### Median Trait
- **Function**: `h_median()`
- **Input**: Slice of numeric values `&[T]`
- **Output**: `f64`
- **Purpose**: Calculates the median (middle value) of a set of numbers
- **Behavior**: For even-length sets, returns the average of two middle values
- **Returns 0.0**: If slice is empty

#### Sum Trait
- **Function**: `h_sum()`
- **Input**: Slice or Vec of numeric values
- **Output**: `f64`
- **Purpose**: Calculates the sum of all elements

#### Variance Trait
- **Function**: `h_variance()`
- **Input**: Slice of numeric values `&[T]`
- **Output**: `f64`
- **Purpose**: Calculates the variance (range) between max and min values
- **Formula**: max - min
- **Returns 0.0**: If less than 2 elements

#### ModusMult Trait
- **Function**: `h_modus_mult()`
- **Input**: Slice of numeric values `&[T]`
- **Output**: `Vec<f64>` (all modes)
- **Purpose**: Finds all values that appear most frequently
- **Returns empty**: If no value appears more than once

### Linear Algebra Functions

#### h_hadamard_product Function (private)
- **Signature**: `h_hadamard_product<I, T>(vec1: &[I], vec2: &[T]) -> Vec<f64>`
- **Input**: Two vectors of equal length
- **Output**: `Vec<f64>` (element-wise product)
- **Purpose**: Computes element-wise multiplication of two vectors
- **Panics**: If vectors have different lengths

#### h_2d_dot_product Function
- **Signature**: `h_2d_dot_product<A, B>(vec1: &Vec<A>, vec2: &Vec<B>) -> f64`
- **Input**: Two vectors of equal length
- **Output**: `f64` (scalar)
- **Purpose**: Calculates the dot product of two vectors
- **Formula**: vec1 · vec2 = Σ(a_i × b_i)
- **Panics**: If vectors have different lengths

### Collection/Functionality Traits

#### HashMapValuesToHashSet Trait
- **Function**: `h_hashmap_values_to_hashset()`
- **Input**: HashMap with cloneable values
- **Output**: `HashSet<V>` containing all unique values
- **Purpose**: Extracts all values from a HashMap into a HashSet

#### HashMapKeysToHashSet Trait
- **Function**: `h_hashmap_keys_to_hashset()`
- **Input**: HashMap with cloneable keys
- **Output**: `HashSet<K>` containing all unique keys
- **Purpose**: Extracts all keys from a HashMap into a HashSet

#### ListToHashMap Trait
- **Function**: `h_list_to_hashmap(&self, values: &[V])`
- **Input**: Keys (Vec or slice), values slice
- **Output**: `HashMap<K, V>`
- **Purpose**: Creates a HashMap from a list of keys and a list of values
- **Panics**: If keys and values have different lengths, or if keys contain duplicates

#### ListToHashSet Trait
- **Function**: `h_list_to_hashset()`
- **Input**: Slice or Vec of values
- **Output**: `HashSet<T>` with unique elements
- **Purpose**: Converts a list to a HashSet, removing duplicates

#### Tof64 Trait
- **Function**: `h_f64()`
- **Input**: Any numeric type implementing `Copy + Into<f64>`
- **Output**: `f64`
- **Purpose**: Generic conversion to f64 for any numeric type

#### Toi32 Trait
- **Function**: `h_i32()`
- **Input**: Any numeric type implementing `Copy + Into<i32>`
- **Output**: `i32`
- **Purpose**: Generic conversion to i32 for any numeric type

#### ToVecf64 Trait
- **Function**: `h_to_vec_f64()`
- **Input**: Vec<T> where T implements `Copy + Into<f64>`
- **Output**: `Vec<f64>`
- **Purpose**: Converts a vector of any numeric type to a vector of f64

#### ToVeci32 Trait
- **Function**: `h_to_vec_i32()`
- **Input**: Vec<T> where T implements `Copy + Into<i32>`
- **Output**: `Vec<i32>`
- **Purpose**: Converts a vector of any numeric type to a vector of i32

### Terminal Input Functions

#### h_input_data_single_f64 Function
- **Signature**: `h_input_data_single_f64(length: i32) -> Vec<f64>`
- **Input**: `length` - Number of data points to collect (if ≤ 0, collects until user enters "<<")
- **Output**: `Vec<f64>` containing the input numbers
- **Purpose**: Prompts user to enter f64 numbers one at a time
- **Behavior**:
  - Skips zero values automatically
  - If length > 0, collects exactly that many non-zero numbers
  - If length ≤ 0, continues until user enters "<<"

#### h_input_data_single_i32 Function
- **Signature**: `h_input_data_single_i32(length: i32) -> Vec<i32>`
- **Input**: `length` - Number of data points to collect (if ≤ 0, collects until user enters "<<")
- **Output**: `Vec<i32>` containing the input integers
- **Purpose**: Prompts user to enter i32 integers one at a time
- **Behavior**:
  - Skips zero values automatically
  - If length > 0, collects exactly that many non-zero numbers
  - If length ≤ 0, continues until user enters "<<"

#### InputType Enum
- **Variants**: `Lowercase`, `Uppercase`, `Letters`, `Integer`
- **Purpose**: Specifies validation requirements for input strings
- **Usage**: Used with the ValidateInput trait to validate string content

#### ValidateInput Trait
- **Function**: `h_validate_input(input_requirements: InputType) -> Result<(), String>`
- **Input**: 
  - Self: String to validate
  - `input_requirements`: InputType enum specifying validation rules
- **Output**: `Result<(), String>` 
  - Ok(()) if validation passes
  - Err(String) with error message listing unexpected characters if validation fails
- **Purpose**: Validates string input against specific requirements
- **Validation Options**:
  - `Lowercase`: Only a-z allowed
  - `Uppercase`: Only A-Z allowed
  - `Letters`: Both uppercase and lowercase letters allowed (A-Z, a-z)
  - `Integer`: Only digits 0-9 allowed (must be parseable as i32)
- **Returns Error**: If input is empty or contains invalid characters

