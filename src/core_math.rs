
// ------------------------------------ Core math ------------------------------------

pub trait Factorial {
    fn h_factorial(&self) -> u64;
}

impl<T> Factorial for T 
where 
    T: Copy + PartialOrd + Into<u32> + Default
{
    fn h_factorial(&self) -> u64 {
        if *self < T::default() {
            panic!("Factorial is not defined for negative numbers");
        }
        let mut result: u64 = 1;
        for i in 1..=(*self).into() as u64 {
            result *= i;
        }
        result
    }
}


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

pub fn h_arrange_vec<I>(start: I, stop: I, step: I) -> Vec<f64>
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

pub fn h_pi() -> f64 {
    return std::f64::consts::PI;
}