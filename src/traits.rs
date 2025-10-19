

pub trait Average {
    fn h_average(&self) -> f64;
}

impl Average for [f64] {
    fn h_average(&self) -> f64 {
        if self.is_empty() { return 0.0; }
        let sum: f64 = self.iter().copied().sum();
        sum / self.len() as f64
    }
}

impl Average for Vec<f64> {
    fn h_average(&self) -> f64 {
        self.as_slice().h_average()
    }
}

impl Average for [i32] {
    fn h_average(&self) -> f64 {
        if self.is_empty() { return 0.0; }
        let sum: i32 = self.iter().copied().sum();
        sum as f64 / self.len() as f64
    }
}

impl Average for Vec<i32> {
    fn h_average(&self) -> f64 {
        self.as_slice().h_average()
    }
}

pub trait Median {
    fn h_median(&self) -> f64;
}

impl Median for [f64] {
    fn h_median(&self) -> f64 {
        if self.len() % 2 == 1 {
            return self[(self.len()-1)/2];
        }
        else {
            return (self[self.len()/2] + self[(self.len()/2)-1])/2.0;
        }
    }
}

impl Median for Vec<f64> {
    fn h_median(&self) -> f64 {
        self.as_slice().h_median()
    }
}

impl Median for [i32] {
    fn h_median(&self) -> f64 {
        if self.len() % 2 == 1 {
            return self[(self.len()-1)/2] as f64;
        }
        else {
            return (self[self.len()/2] + self[(self.len()/2)-1]) as f64/2.0;
        }
    }
}

impl Median for Vec<i32> {
    fn h_median(&self) -> f64 {
        self.as_slice().h_median()
    }
}

