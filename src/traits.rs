use core::panic;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io;

// -------------------------------- Statistics ---------------------------------

pub trait Average {
    fn h_average(&self) -> f64;
}

impl<T> Average for [T]
where
    T: Copy + Into<f64>,
{
    fn h_average(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        let mut sum = 0.0;
        for &x in self {
            sum += x.into();
        }
        sum / self.len() as f64
    }
}

impl<T> Average for Vec<T>
where
    T: Copy + Into<f64>,
{
    fn h_average(&self) -> f64 {
        self.as_slice().h_average()
    }
}



pub trait Median {
    fn h_median(&self) -> f64;
}

impl<T> Median for [T]
where
    T: Copy + Into<f64> + PartialOrd,
{
    fn h_median(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        let mut sorted: Vec<f64> = self.iter().map(|&x| x.into()).collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 1 {
            sorted[mid]
        } else {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        }
    }
}

impl<T> Median for Vec<T>
where
    T: Copy + Into<f64> + PartialOrd,
{
    fn h_median(&self) -> f64 {
        self.as_slice().h_median()
    }
}

// ---------------------------

pub trait Sum {
    fn h_sum(&self) -> f64;
}

impl<T> Sum for [T]
where
    T: Copy + Into<f64>,
{
    fn h_sum(&self) -> f64 {
        let mut sum = 0.0;
        for &x in self {
            sum += x.into();
        }
        sum
    }
}

impl<T> Sum for Vec<T>
where
    T: Copy + Into<f64>,
{
    fn h_sum(&self) -> f64 {
        self.as_slice().h_sum()
    }
}


pub trait Variance {
    fn h_variance(&self) -> f64;
}

impl<T> Variance for [T]
where
    T: Copy + Into<f64> + PartialOrd,
{
    fn h_variance(&self) -> f64 {
        if self.len() < 2 {
            return 0.0;
        }
        let mut sorted: Vec<f64> = self.iter().map(|&x| x.into()).collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[sorted.len() - 1] - sorted[0]
    }
}

impl<T> Variance for Vec<T>
where
    T: Copy + Into<f64> + PartialOrd,
{
    fn h_variance(&self) -> f64 {
        self.as_slice().h_variance()
    }
}

// ---------------------------

pub trait ModusMult {
    fn h_modus_mult(&self) -> Vec<f64>;
}

impl<T> ModusMult for [T]
where
    T: Copy + Into<f64> + PartialEq,
{
    fn h_modus_mult(&self) -> Vec<f64> {
        let mut list_modus: Vec<(f64, i32)> = vec![];
        let mut found_list: Vec<f64> = vec![];

        for &x in self {
            let val = x.into();
            if !found_list.contains(&val) {
                found_list.push(val);
                list_modus.push((val, 0));
            }
        }

        for &x in self {
            let val = x.into();
            for j in &mut list_modus {
                if j.0 == val {
                    j.1 += 1;
                }
            }
        }

        let mut max_count = 0;
        for &(_, count) in &list_modus {
            if count > max_count {
                max_count = count;
            }
        }

        if max_count <= 1 {
            return vec![];
        }

        list_modus
            .into_iter()
            .filter(|(_, count)| *count == max_count)
            .map(|(val, _)| val)
            .collect()
    }
}

impl<T> ModusMult for Vec<T>
where
    T: Copy + Into<f64> + PartialEq,
{
    fn h_modus_mult(&self) -> Vec<f64> {
        self.as_slice().h_modus_mult()
    }
}

// -------------------------------- General uses ---------------------------------

pub trait Search {
    fn h_search(&self, value: f64) -> bool;
}

impl<T> Search for [T]
where
    T: Copy + Into<f64> + PartialEq,
{
    fn h_search(&self, value: f64) -> bool {
        for &x in self {
            if x.into() == value {
                return true;
            }
        }
        false
    }
}

impl<T> Search for Vec<T>
where
    T: Copy + Into<f64> + PartialEq,
{
    fn h_search(&self, value: f64) -> bool {
        self.as_slice().h_search(value)
    }
}

pub fn h_arrange_vec<I>(start: I, stop: I, step: I) -> Vec<f64> 
where 
    I: Copy + Into<f64>,
{
    let mut vector: Vec<f64> = Vec::new();
    let mut start_f: f64 = start.into();
    if start_f > stop.into() && step.into() >= 0.0  {
        return vec![];
    }
    if
    vector.push(start_f);
    while start_f >= stop.into() {
        start_f += step.into();
        if start_f > stop.into() {
            return vector;
        }
        vector.push(start_f);
    }
    return vector;
}

// ------------------------------------ Geometry ------------------------------------

pub trait CircleCircumference {
    fn h_circle_circumference(&self) -> f64;
}

impl<T> CircleCircumference for T
where
    T: Copy + Into<f64>,
{
    fn h_circle_circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * (*self).into()
    }
}

pub trait CircleArea {
    fn h_circle_area(&self) -> f64;
}

impl<T> CircleArea for T
where
    T: Copy + Into<f64>,
{
    fn h_circle_area(&self) -> f64 {
        let r = (*self).into();
        std::f64::consts::PI * r * r
    }
}

pub trait SphereVolume {
    fn h_sphere_volume(&self) -> f64;
}

impl<T> SphereVolume for T
where
    T: Copy + Into<f64>,
{
    fn h_sphere_volume(&self) -> f64 {
        let r = (*self).into();
        (4.0 / 3.0) * std::f64::consts::PI * r * r * r
    }
}

pub trait SphereSurfaceArea {
    fn h_sphere_surface_area(&self) -> f64;
}

impl<T> SphereSurfaceArea for T
where
    T: Copy + Into<f64>,
{
    fn h_sphere_surface_area(&self) -> f64 {
        let r = (*self).into();
        4.0 * std::f64::consts::PI * r * r
    }
}

pub fn h_pythagorean_theorem<A, B>(a: A, b: B) -> f64
where
    A: Copy + Into<f64>,
    B: Copy + Into<f64>,
{
    let a_f = a.into();
    let b_f = b.into();
    (a_f.powf(2.0) + b_f.powf(2.0)).sqrt()
}

pub fn h_reverse_pythagorean_theorem<K, H>(x: K, h: H) -> f64
where 
    K: Copy + Into<f64>,
    H: Copy + Into<f64>,
{
    let xfc: f64 = x.into().powf(2.0);
    let hfc: f64 = h.into().powf(2.0);
    (hfc - xfc).powf(0.5)
}

pub fn h_find_equal_legs_from_hypotenuse<H>(h: H) -> f64 
where 
    H: Copy + Into<f64>,
{
    let hfc: f64 = h.into().powf(2.0);
    (hfc / 2.0).sqrt()
}

pub trait ShortFromLongLeg30_60_90{
    fn h_short_from_long_leg_30_60_90(&self) -> f64;
}

impl<T> ShortFromLongLeg30_60_90 for T
where
    T: Copy + Into<f64>,
{
    fn h_short_from_long_leg_30_60_90(&self) -> f64 {
        (*self).into() / (3.0f64).sqrt()
    }
}

// ------------------------------------ Core math ------------------------------------

pub trait Factorial {
    fn h_factorial(&self) -> u64;
}

impl Factorial for i32 {
    fn h_factorial(&self) -> u64 {
        if *self < 0 {
            panic!("Factorial is not defined for negative numbers");
        }
        let mut result: u64 = 1;
        for i in 1..=*self as u64 {
            result *= i;
        }
        result
    }
}

impl Factorial for u32 {
    fn h_factorial(&self) -> u64 {
        let mut result: u64 = 1;
        for i in 1..=*self as u64 {
            result *= i;
        }
        result
    }
}

pub trait SQRTDegree {
    fn h_sqrt_degree(&self, degree: u32) -> f64;
}

impl<T> SQRTDegree for T
where
    T: Copy + Into<f64>,
{
    fn h_sqrt_degree(&self, degree: u32) -> f64 {
        (*self).into().powf(1.0 / degree as f64)
    }
}



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




// ------------------------------------ Finance ------------------------------------

pub trait ROI {
    fn h_return_on_investment(&self, new_value: f64) -> f64;
}

impl<T> ROI for T
where
    T: Copy + Into<f64>,
{
    fn h_return_on_investment(&self, new_value: f64) -> f64 {
        let start = (*self).into();
        (new_value - start) / start * 100.0
    }
}

pub trait DiscountedPrice {
    fn h_decreased_price(&self, decrease_percent: f64) -> f64;
}

impl<T> DiscountedPrice for T
where
    T: Copy + Into<f64>,
{
    fn h_decreased_price(&self, decrease_percent: f64) -> f64 {
        let percent_discount_opposite: f64 = 1.0 - decrease_percent / 100.0;
        percent_discount_opposite * (*self).into()
    }
}

pub trait IncreasedPrice {
    fn h_increased_price(&self, increase_percent: f64) -> f64;
}

impl<T> IncreasedPrice for T
where
    T: Copy + Into<f64>,
{
    fn h_increased_price(&self, increase_percent: f64) -> f64 {
        let percent_increas_plus_one: f64 = 1.0 + increase_percent / 100.0;
        percent_increas_plus_one * (*self).into()
    }
}

// ------------------------------------ Temperature ------------------------------------
pub trait CelsiusToFahrenheit {
    fn h_celsius_to_fahrenheit(&self) -> f64;
}
impl<T> CelsiusToFahrenheit for T
where
    T: Copy + Into<f64>,
{
    fn h_celsius_to_fahrenheit(&self) -> f64 {
        (*self).into() * 9.0 / 5.0 + 32.0
    }
}
pub trait FahrenheitToCelsius {
    fn h_fahrenheit_to_celsius(&self) -> f64;
}
impl<T> FahrenheitToCelsius for T
where
    T: Copy + Into<f64>,
{
    fn h_fahrenheit_to_celsius(&self) -> f64 {
        ((*self).into() - 32.0) * 5.0 / 9.0
    }
}   

pub trait CelsiusToKelvin {
    fn h_celsius_to_kelvin(&self) -> f64;
}
impl<T> CelsiusToKelvin for T
where
    T: Copy + Into<f64>,
{
    fn h_celsius_to_kelvin(&self) -> f64 {
        (*self).into() + 273.15
    }
}
pub trait KelvinToCelsius {
    fn h_kelvin_to_celsius(&self) -> f64;
}
impl<T> KelvinToCelsius for T
where
    T: Copy + Into<f64>,
{
    fn h_kelvin_to_celsius(&self) -> f64 {
        (*self).into() - 273.15
    }
}   

pub trait FahrenheitToKelvin {
    fn h_fahrenheit_to_kelvin(&self) -> f64;
}
impl<T> FahrenheitToKelvin for T
where
    T: Copy + Into<f64>,
{
    fn h_fahrenheit_to_kelvin(&self) -> f64 {   
        ((*self).into() - 32.0) * 5.0 / 9.0 + 273.15
    }
}
pub trait KelvinToFahrenheit {
    fn h_kelvin_to_fahrenheit(&self) -> f64;
}
impl<T> KelvinToFahrenheit for T
where
    T: Copy + Into<f64>,
{
    fn h_kelvin_to_fahrenheit(&self) -> f64 {
        ((*self).into() - 273.15) * 9.0 / 5.0 + 32.0
    }
}       


// ------------------------------------ Length conversions ------------------------------------
pub trait MetersToKilometers {
    fn h_meters_to_kilometers(&self) -> f64;
}
impl<T> MetersToKilometers for T
where
    T: Copy + Into<f64>,
{
    fn h_meters_to_kilometers(&self) -> f64 {
        (*self).into() / 1000.0
    }
}
pub trait KilometersToMeters {
    fn h_kilometers_to_meters(&self) -> f64;
}
impl<T> KilometersToMeters for T
where
    T: Copy + Into<f64>,
{
    fn h_kilometers_to_meters(&self) -> f64 {
        (*self).into() * 1000.0
    }
}       

pub trait CentimetersToMeters {
    fn h_centimeters_to_meters(&self) -> f64;
}
impl<T> CentimetersToMeters for T
where
    T: Copy + Into<f64>,
{
    fn h_centimeters_to_meters(&self) -> f64 {
        (*self).into() / 100.0
    }
}
pub trait MetersToCentimeters {
    fn h_meters_to_centimeters(&self) -> f64;
}
impl<T> MetersToCentimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_meters_to_centimeters(&self) -> f64 {
        (*self).into() * 100.0
    }
}

pub trait CentimetersToMillimeters {
    fn h_centimeters_to_millimeters(&self) -> f64;
}
impl<T> CentimetersToMillimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_centimeters_to_millimeters(&self) -> f64 {
        (*self).into() * 10.0
    }
}
pub trait MillimetersToCentimeters {
    fn h_millimeters_to_centimeters(&self) -> f64;
}
impl<T> MillimetersToCentimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_millimeters_to_centimeters(&self) -> f64 {
        (*self).into() / 10.0
    }
}

pub trait KilometersToMiles {
    fn h_kilometers_to_miles(&self) -> f64;
}
impl<T> KilometersToMiles for T
where
    T: Copy + Into<f64>,
{
    fn h_kilometers_to_miles(&self) -> f64 {
        (*self).into() * 0.621371
    }
}
pub trait MilesToKilometers {
    fn h_miles_to_kilometers(&self) -> f64;
}
impl<T> MilesToKilometers for T
where
    T: Copy + Into<f64>,
{
    fn h_miles_to_kilometers(&self) -> f64 {
        (*self).into() / 0.621371
    }
}

pub trait InchesToCentimeters {
    fn h_inches_to_centimeters(&self) -> f64;
}
impl<T> InchesToCentimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_inches_to_centimeters(&self) -> f64 {
        (*self).into() * 2.54
    }
}

pub trait CentimetersToInches {
    fn h_centimeters_to_inches(&self) -> f64;
}
impl<T> CentimetersToInches for T
where
    T: Copy + Into<f64>,
{
    fn h_centimeters_to_inches(&self) -> f64 {
        (*self).into() / 2.54
    }
}

pub trait CentimetersToDecimeters {
    fn h_centimeters_to_decimeters(&self) -> f64;
}
impl<T> CentimetersToDecimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_centimeters_to_decimeters(&self) -> f64 {
        (*self).into() / 10.0
    }
}
pub trait DecimetersToCentimeters {
    fn h_decimeters_to_centimeters(&self) -> f64;
}
impl<T> DecimetersToCentimeters for T
where
    T: Copy + Into<f64>,
{
    fn h_decimeters_to_centimeters(&self) -> f64 {
        (*self).into() * 10.0
    }
}

// --------------------------- functionality --------------------------------

pub trait HashMapValuesToHashSet<V>
where
    V: Eq + Hash + Clone,
{
    fn h_hashmap_values_to_hashset(&self) -> HashSet<V>;
}

impl<K, V> HashMapValuesToHashSet<V> for HashMap<K, V>
where
    V: Eq + Hash + Clone,
{
    fn h_hashmap_values_to_hashset(&self) -> HashSet<V> {
        let mut set: HashSet<V> = HashSet::new();
        for (_key, value) in self.iter() {
            set.insert(value.clone());
        }
        set
    }
}

pub trait HashMapKeysToHashSet<K>
where
    K: Eq + Hash + Clone,
{
    fn h_hashmap_keys_to_hashset(&self) -> HashSet<K>;
}

impl<K, V> HashMapKeysToHashSet<K> for HashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn h_hashmap_keys_to_hashset(&self) -> HashSet<K> {
        let mut set: HashSet<K> = HashSet::new();
        for (key, _value) in self.iter() {
            set.insert(key.clone());
        }
        set
    }
}

pub trait ListToHashMap<V> {
    type Key;
    type Value;

    fn h_list_to_hashmap(&self, values: &[V]) -> HashMap<Self::Key, Self::Value>
    where
        Self::Key: Eq + Hash + Clone,
        Self::Value: Clone;
}


impl<K, V> ListToHashMap<V> for [K]
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    type Key = K;
    type Value = V;

    fn h_list_to_hashmap(&self, values: &[V]) -> HashMap<Self::Key, Self::Value> {
        let keys: &[K] = self;

        if keys.len() != values.len() {
            panic!("Keys and values must have the same length");
        }

        let mut set = HashSet::new();
        let mut map = HashMap::new();

        // Check for duplicate keys
        for key in keys.iter() {
            if !set.insert(key.clone()) {
                panic!("Duplicate key found in h_vector_to_hashmap");
            }
        }

        // Build the HashMap
        for i in 0..keys.len() {
            map.insert(keys[i].clone(), values[i].clone());
        }

        map
    }
}

impl<K, V> ListToHashMap<V> for Vec<K>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    type Key = K;
    type Value = V;

    fn h_list_to_hashmap(&self, values: &[V]) -> HashMap<Self::Key, Self::Value> {
        self.as_slice().h_list_to_hashmap(values)
    }
}

pub trait ListToHashSet {
    type Item;

    fn h_list_to_hashset(&self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash + Clone;
}

impl<T> ListToHashSet for [T]
where
    T: Eq + Hash + Clone,
{
    type Item = T;

    fn h_list_to_hashset(&self) -> HashSet<Self::Item> {
        let mut set = HashSet::new();
        for item in self.iter() {
            set.insert(item.clone());
        }
        set
    }
}

impl<T> ListToHashSet for Vec<T>
where
    T: Eq + Hash + Clone,
{
    type Item = T;

    fn h_list_to_hashset(&self) -> HashSet<Self::Item> {
        self.as_slice().h_list_to_hashset()
    }
}

pub trait Tof64 {
    fn h_f64(&self) -> f64;
}

impl<T> Tof64 for T 
where 
    T: Copy + Into<f64>,
{
    fn h_f64(&self) -> f64 {
        (*self).into()
    }
}

pub trait Toi32 {
    fn h_i32(&self) -> i32;
}

impl<T> Toi32 for T 
where 
    T: Copy + Into<i32>,
{
    fn h_i32(&self) -> i32 {
        (*self).into()
    }
}

pub trait ToVecf64 {
    fn h_to_vec_f64(&self) -> Vec<f64>;
}

impl<T> ToVecf64 for Vec<T> 
where 
    T: Copy + Into<f64>,
{
    fn h_to_vec_f64(&self) -> Vec<f64> {
        let mut new_vec: Vec<f64> = Vec::new();
        for i in self {
            new_vec.push((*i).into());
        }
        return new_vec;
    }
}

pub trait ToVeci32 {
    fn h_to_vec_i32(&self) -> Vec<i32>;
}

impl<T> ToVeci32 for Vec<T> 
where 
    T: Copy + Into<i32>,
{
    fn h_to_vec_i32(&self) -> Vec<i32> {
        let mut new_vec: Vec<i32> = Vec::new();
        for i in self {
            new_vec.push((*i).into());
        }
        return new_vec;
    }
}





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

// -------------------------------- Input Terminal ------------------------------

pub fn h_input_data_single_f64(length: i32) -> Vec<f64> { // if length is <= 0; stop the inputs with: "<<"
    let mut count: u32 = 1;
    let mut vec: Vec<f64> = vec![];
    loop {
        if (count - 1) as i32 == length && length > 0 {
            return vec;
        }
        let mut input: String = String::new();
        println!("data point {}: ", count);
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        if length <= 0 && input.trim() == "<<" {
            return vec;
        }
        let num: f64 = input.trim().parse().expect("Please enter a valid number!");
        if num == 0.0 {
            continue;
        }
        vec.push(num);
        count += 1;
    }
}

pub fn h_input_data_single_i32(length: i32) -> Vec<i32> { // if length is <= 0; stop the inputs with: "<<"
    let mut count: u32 = 1;
    let mut vec: Vec<i32> = vec![];
    loop {
        if (count - 1) as i32 == length && length > 0 {
            return vec;
        }
        let mut input: String = String::new();
        println!("data point {}: ", count);
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        if length <= 0 && input.trim() == "<<" {
            return vec;
        }
        let num: i32 = input.trim().parse().expect("Please enter a valid number!");
        if num == 0 {
            continue;
        }
        vec.push(num);
        count += 1;
    }
}


#[derive(PartialEq, Eq, Debug)]
pub enum InputType {
    Lowercase,
    Uppercase,
    Letters, // Lowercase + Uppercase
    Integer,
}

pub trait ValidateInput {
    fn h_validate_input(&self, input_requirements: InputType) -> Result<(), String>;
}

impl ValidateInput for String {
    fn h_validate_input(&self, input_requirements: InputType) -> Result<(), String> {
        if self.is_empty() {
            return Err(String::from("The input is empty"));
        }

        if input_requirements == InputType::Lowercase {
            if !self.chars().all(|c| c.is_ascii_lowercase()) {
                let lowercase: HashSet<char> = ('a'..='z').collect();
                let mut unexcpected: Vec<char> = vec![];
                for c in self.chars() {
                    if !lowercase.contains(&c) {
                        unexcpected.push(c);
                    }
                }
                return Err(format!("Invalid input, lowercase letters only. Found: {:?}", unexcpected));
            }
        }
        else if input_requirements == InputType::Uppercase {
            if !self.chars().all(|c| c.is_ascii_uppercase()) {
                let uppercase: HashSet<char> = ('A'..='Z').collect();
                let mut unexcpected: Vec<char> = vec![];
                for c in self.chars() {
                    if !uppercase.contains(&c) {
                        unexcpected.push(c);
                    }
                }
                return Err(format!("Invalid input, requirement: uppercase letters only. Found: {:?}", unexcpected));
            }
        }
        else if input_requirements == InputType::Letters {
            if !self.chars().all(|c: char| c.is_ascii_uppercase() || c.is_ascii_uppercase()) {
                let characters: HashSet<char> = ('A'..='Z').chain('a'..='z').collect();
                let mut unexcpected: Vec<char> = vec![];
                for c in self.chars() {
                    if !characters.contains(&c) {
                        unexcpected.push(c);
                    }
                }
                return Err(format!("Invalid input, requirement: letters only. Found: {:?}", unexcpected));
            }
        }

        else if input_requirements == InputType::Integer {
            if self.parse::<i32>().is_err() {
                let characters: HashSet<char> = ('0'..='9').collect();
                let mut unexcpected: Vec<char> = vec![];
                for c in self.chars() {
                    if !characters.contains(&c) {
                        unexcpected.push(c);
                    }
                }
                return Err(format!("Invalid input, requirement: whole Integer. Found: {:?}", unexcpected));
            }
        }

        Ok(())
    }
}











