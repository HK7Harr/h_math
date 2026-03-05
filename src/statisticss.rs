// -------------------------------- Statistics ---------------------------------

/// Calculates the mean (average) of a dataset. The mean is calculated by summing all the values and dividing by the number of observations.
/// If the dataset is empty, the function returns 0.0 to avoid division by zero.
/// Example usage:
/// let data = vec![1.0, 2.0, 3.0];
/// let mean = data.h_mean();
/// The result will be 2.0, because (1.0 + 2.0 + 3.0) / 3 = 6.0 / 3 = 2.0.
/// If the dataset were empty (vec![]), the result would be 0.0, because there are no values to average.
/// If the dataset were vec![1.0, 2.0, 3.0, 4.0], the result would be 2.5, because (1.0 + 2.0 + 3.0 + 4.0) / 4 = 10.0 / 4 = 2.5.
/// If the dataset were vec![1.0, 2.0, 3.0, 4.0, 5.0], the result would be 3.0, because (1.0 + 2.0 + 3.0 + 4.0 + 5.0) / 5 = 15.0 / 5 = 3.0.
/// If the dataset were vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], the result would be 3.5, because (1.0 + 2.0 + 3.0 + 4.0 + 5.0 + 6.0) / 6 = 21.0 / 6 = 3.5.
/// If the dataset were vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0], the result would be 4.0, because (1.0 + 2.0 + 3.0 + 4.0 + 5.0 + 6.0 + 7.0) / 7 = 28.0 / 7 = 4.0.
pub trait Mean {
    fn h_mean(&self) -> f64;
}

impl<T> Mean for [T]
where
    T: Copy + Into<f64>,
{
    fn h_mean(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
         let sum: f64 = self.iter().map(|&x| x.into()).sum();
        sum / self.len() as f64
    }
}

/// Calculates the median of a dataset. The median is the middle value when the data is sorted.
/// If there is an even number of observations, the median is the average of the two middle values.
/// If the dataset is empty, the function returns 0.0.
/// Example usage:
/// let data = vec![3.0, 1.0, 2.0];
/// let median = data.h_median();
/// 
/// The result will be 2.0, because when the data is sorted (1.0, 2.0, 3.0), the middle value is 2.0.
/// If the dataset were vec![3.0, 1.0, 2.0, 4.0], the result would be 2.5, because when the data is sorted 
/// (1.0, 2.0, 3.0, 4.0),
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



/// Calculates the population variance of a dataset. 
/// The population variance is calculated by dividing the sum of squared differences from the mean by n,
/// where n is the number of observations in the dataset. This is used when you have data for the entire population. 
/// If there are fewer than 2 observations, the function returns 0.0 to avoid division by zero.
/// Example usage:
/// let data = vec![1.0, 2.0, 3.0];
/// let variance = data.h_population_variance();
/// The result will be 0.6666666666666666, because the mean is 2.0, 
/// and the squared differences from the mean are (1-2)^2 = 1, (2-2)^2 = 0, (3-2)^2 = 1.
pub trait PopulationVariance {
    fn h_population_variance(&self) -> f64;
}

impl<T> PopulationVariance for [T]
where
    T: Copy + Into<f64>,
{
    fn h_population_variance(&self) -> f64 {
        let n = self.len();
        if n < 2 {
            return 0.0;
        }

        let mean: f64 = self.h_mean();

        self.iter()
            .map(|&x| {
                let diff = x.into() - mean;
                diff * diff
            })
            .sum::<f64>() / n as f64
    }
}


/// Calculates the sample variance of a dataset. 
/// The sample variance is calculated by dividing the sum of squared differences from the mean by (n - 1),
/// where n is the number of observations in the dataset.
/// This is used when you have a sample of a population and want to estimate the population variance. 
/// If there are fewer than 2 observations, the function returns 0.0 to avoid division by zero.
/// Example usage:
/// let data = vec![1.0, 2.0, 3.0];
/// let variance = data.h_sample_variance();
/// The result will be 1.0, because the mean is 2.0, 
/// and the squared differences from the mean are (1-2)^2 = 1, (2-2)^2 = 0, (3-2)^2 = 1. 
/// The sum of squared differences is 1 + 0 + 1 = 2, and dividing by (n - 1) = (3 - 1) = 2 
/// gives a sample variance of 1.0.
pub trait SampleVariance {
    fn h_sample_variance(&self) -> f64;
}

impl<T> SampleVariance for [T]
where
    T: Copy + Into<f64>,
{
    fn h_sample_variance(&self) -> f64 {
        let n = self.len();
        if n < 2 {
            return 0.0;
        }

        let mean: f64 = self.h_mean();

        self.iter()
            .map(|&x| {
                let diff = x.into() - mean;
                diff * diff
            })
            .sum::<f64>() / (n - 1) as f64
    }
}


// ---------------------------

/// Calculates the mode(s) of a dataset. If there are multiple modes, all of them will be returned in a vector.
/// If there is no mode (i.e., all values are unique), an empty vector will be returned.
/// Example usage:
/// let data = vec![1.0, 2.0, 2.0, 3.0];
/// let modes = data.h_modus_mult();
/// The result will be vec![2.0], because 2.0 is the most frequently occurring value in the dataset. 
/// If the dataset were vec![1.0, 2.0, 3.0], the result would be an empty vector, because there is no mode (all values are unique). 
/// If the dataset were vec![1.0, 1.0, 2.0, 2.0], the result would be vec![1.0, 2.0], 
/// because both 1.0 and 2.0 are modes (they both occur with the same highest frequency).
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

/// Calculates the standard deviation of a dataset. The standard deviation is the square root of the variance.
/// The population standard deviation is calculated using the population variance, 
/// while the sample standard deviation is calculated using the sample variance.
/// If there are fewer than 2 observations, the function returns 0.0 to avoid division by zero.
/// Example usage:
/// let data = vec![1.0, 2.0, 3.0];
/// let std_dev_population = data.h_std_dev_population();
/// let std_dev_sample = data.h_std_dev_sample();
/// The result for std_dev_population will be 0.816496580927726, because the population variance is 0.6666666666666666, and the square root of 0.6666666666666666 is approximately 0.816496580927726.

pub trait StdDevPopulation {
    fn h_std_dev_population(&self) -> f64;
}

impl<T> StdDevPopulation for [T] 
where 
    T: Copy + Into<f64>,
{
    fn h_std_dev_population(&self) -> f64 {
        let variance = self.h_sample_variance();
        variance.sqrt()
    }
}

/// Calculates the standard deviation of a dataset. The standard deviation is the square root of the variance.
/// The population standard deviation is calculated using the population variance, 
/// while the sample standard deviation is calculated using the sample variance.
/// If there are fewer than 2 observations, the function returns 0.0 to avoid division by zero.
/// Example usage:
/// let data = vec![1.0, 2.0, 3.0];
/// let std_dev_population = data.h_std_dev_population();
/// let std_dev_sample = data.h_std_dev_sample();
/// The result for std_dev_sample will be 1.0, because the sample variance is 1.0, and the square root of 1.0 is 1.0.
/// If the dataset were vec![1.0, 2.0, 3.0, 4.0], the result for std_dev_sample would be approximately 1.2909944487358056,
pub trait StdDevSample {
    fn h_std_dev_sample(&self) -> f64;
}

impl<T> StdDevSample for [T] 
where 
    T: Copy + Into<f64>,
{
    fn h_std_dev_sample(&self) -> f64 {
        let variance = self.h_sample_variance();
        variance.sqrt()
    }
}












































