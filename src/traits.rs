
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
        if self.is_empty() {
            return 0.0;
        }
        let mut sorted = self.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 1 {
            sorted[mid]
        } else {
            (sorted[mid - 1] + sorted[mid]) / 2.0
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
        if self.is_empty() {
            return 0.0;
        }
        let mut sorted = self.to_vec();
        sorted.sort();

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 1 {
            sorted[mid] as f64
        } else {
            (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
        }
    }
}

impl Median for Vec<i32> {
    fn h_median(&self) -> f64 {
        self.as_slice().h_median()
    }
}

pub trait Sum {
    type Output;
    fn h_sum(&self) -> Self::Output;
}

impl Sum for [f64] {
    type Output = f64;
    fn h_sum(&self) -> Self::Output {
        if self.is_empty() {return 0.0;}
        let mut sum: f64 = 0.0;
        for i in self {
            sum += i;
        }
        sum
    }
}

impl Sum for Vec<f64> {
    type Output = f64;
    fn h_sum(&self) -> Self::Output {
        self.as_slice().h_sum()
    }
}

impl Sum for [i32] {
    type Output = i32;
    fn h_sum(&self) -> Self::Output {
        if self.is_empty() {return 0;}
        let mut sum: i32 = 0;
        for i in self {
            sum += i;
        }
        sum
    }
}

impl Sum for Vec<i32> {
    type Output = i32;
    fn h_sum(&self) -> Self::Output {
        self.as_slice().h_sum()
    }
}

pub trait Variance {
    type Output;
    fn h_variance(&self) -> Self::Output;
}
impl Variance for [f64] {
    type Output = f64;
    fn h_variance(&self) -> Self::Output {
        let sorted: Vec<f64> = self.to_vec();
        sorted[sorted.len()-1] - sorted[0]
    }
}
impl Variance for Vec<f64> {
    type Output = f64;
    fn h_variance(&self) -> Self::Output {
        self.as_slice().h_variance()
    }
}
impl Variance for [i32] {
    type Output = i32;
    fn h_variance(&self) -> Self::Output {
        let sorted: Vec<i32> = self.to_vec();
        sorted[sorted.len()-1] - sorted[0]
    }
}
impl Variance for Vec<i32> {
    type Output = i32;
    fn h_variance(&self) -> Self::Output {
        self.as_slice().h_variance()
    }
}

pub trait ModusMult {
    type Output;
    fn h_modus_mult(&self) -> Self::Output;
}
impl ModusMult for [f64] {
    type Output = Vec<f64>;
    fn h_modus_mult(&self) -> Self::Output {
        let mut list_modus: Vec<(f64, i32)> = vec![];
        let mut found_list: Vec<f64> = vec![];
        
        for i in self {
            if !found_list.contains(i) {
                found_list.push(*i);
                list_modus.push((*i, 0));
            }
        }
        for i in self {
            let x: f64 = *i;
            for j in &mut list_modus {
                if j.0 == x {
                    j.1 += 1;
                }
            }
        }
        let mut max_count: (i32, i32) = (0, 0); // (index, count)
        let mut index: i32 = 0;
        for i in &list_modus {
            if i.1 > max_count.1 {
                max_count = (index, i.1);
            }
            index += 1;
        }
        if max_count.1 == 1 {
            return vec![];
        }
        let mut modus_list: Vec<f64> = vec![];
        for i in &list_modus {
            if i.1 == max_count.1 {
                modus_list.push(i.0);
            }
        }
        return modus_list;
    }
}

impl ModusMult for Vec<f64> {
    type Output = Vec<f64>;
    fn h_modus_mult(&self) -> Self::Output {
        self.as_slice().h_modus_mult()
    }
}

impl ModusMult for [i32] {
    type Output = Vec<i32>;
    fn h_modus_mult(&self) -> Self::Output {
        let mut list_modus: Vec<(i32, i32)> = vec![];
        let mut found_list: Vec<i32> = vec![];
        
        for i in self {
            if !found_list.contains(i) {
                found_list.push(*i);
                list_modus.push((*i, 0));
            }
        }
        for i in self {
            let x: i32 = *i;
            for j in &mut list_modus {
                if j.0 == x {
                    j.1 += 1;
                }
            }
        }
        let mut max_count: (i32, i32) = (0, 0); // (index, count)
        let mut index: i32 = 0;
        for i in &list_modus {
            if i.1 > max_count.1 {
                max_count = (index, i.1);
            }
            index += 1;
        }
        if max_count.1 == 1 {
            return vec![];
        }
        let mut modus_list: Vec<i32> = vec![];
        for i in &list_modus {
            if i.1 == max_count.1 {
                modus_list.push(i.0);
            }
        }
        return modus_list;
    }
}

impl ModusMult for Vec<i32> {
    type Output = Vec<i32>;
    fn h_modus_mult(&self) -> Self::Output {
        self.as_slice().h_modus_mult()
    }
}

