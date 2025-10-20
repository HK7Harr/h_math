use core::panic;

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

// ---------------------------

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

// ---------------------------

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