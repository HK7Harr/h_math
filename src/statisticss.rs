// -------------------------------- Statistics ---------------------------------

pub trait Average {
    fn h_average(&self) -> f64;
}

impl<T> Average for &[T]
where
    T: Copy + Into<f64>,
{
    fn h_average(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
         let sum: f64 = self.iter().map(|&x| x.into()).sum();
        sum / self.len() as f64
    }
}


pub trait Median {
    fn h_median(&self) -> f64;
}

impl<T> Median for &[T]
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

impl<T> Variance for &[T]
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


// ---------------------------

pub trait ModusMult {
    fn h_modus_mult(&self) -> Vec<f64>;
}

impl<T> ModusMult for &[T]
where
    T: Copy + Into<f64> + PartialEq,
{
    fn h_modus_mult(&self) -> Vec<f64> {
        let mut list_modus: Vec<(f64, i32)> = vec![];
        let mut found_list: Vec<f64> = vec![];

        for &x in *self {
            let val = x.into();
            if !found_list.contains(&val) {
                found_list.push(val);
                list_modus.push((val, 0));
            }
        }

        for &x in *self {
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

