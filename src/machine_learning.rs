use crate::prelude::*;
use std::iter::zip;




/// A simple Perceptron implementation for binary classification.
/// The Perceptron is a linear classifier that learns a decision boundary to separate two classes.
/// It uses a learning rate (eta) and number of epochs for training.
/// The weights and bias are adjusted during training to minimize classification errors.
/// Example usage:
/// let mut percep = Perceptron::new(0.1, 100, 2);
/// percep.fit(&x_data, &y_labels);
/// let prediction = percep.predict(&input);
pub struct Perceptron {
    eta: f64, // between 0 - 1
    epochs: u32,

    weights: Vec<f64>,
    bias: f64,

    labels_as_nums: Vec<i32>,
    original_labels: Vec<&'static str>,

    unique_labels_as_nums: Vec<i32>,
    unique_original_labels: Vec<&'static str>,
}

impl Perceptron {
    /// Creates a new Perceptron with the given learning rate, number of epochs, and number of features.
    /// The learning rate (eta) should be between 0 and 1.
    /// The number of features (x_features) determines the size of the weight vector.
    /// Example usage:
    /// let percep = Perceptron::new(0.1, 100, 2);
    pub fn new(eta: f64, epochs: u32, x_features: usize) -> Self {
        let mut weights: Vec<f64> = Vec::new();
        for _ in 1..=x_features {
            weights.push(0.0);
        }
        let new: Perceptron = Perceptron { 
            eta: eta,
            epochs: epochs,
            weights: weights,
            bias: 0.0,

            labels_as_nums: Vec::new(),
            original_labels: Vec::new(),

            unique_labels_as_nums: Vec::new(),
            unique_original_labels: Vec::new(),
        };
        new
    }
    /// Trains the Perceptron on the given dataset.
    /// The input data x is a vector of feature vectors, and y is the corresponding labels.
    /// Labels are converted to numeric values for training (0 or 1 for binary classification).
    /// The method updates the weights and bias using the perceptron learning rule.
    /// Example usage:
    /// percep.fit(&x_data, &y_labels);
    pub fn fit<X, Y>(&mut self, x: &Vec<Vec<X>>, y: &Vec<Y>) 
    where 
        X: Copy + Into<f64>,
        Y: Copy + Eq + Into<&'static str>,
    {   
        let mut on_item: &'static str = y[0].into();
        let mut count: i32 = 0;

        self.unique_labels_as_nums.push(count);
        self.unique_original_labels.push(on_item);
        for i in y {
            if (*i).into() == on_item {
                self.labels_as_nums.push(count);
                self.original_labels.push(on_item)
            }
            else {
                count += 1;
                on_item = (*i).into();
                self.labels_as_nums.push(count);
                self.original_labels.push(on_item);
                
                self.unique_labels_as_nums.push(count);
                self.unique_original_labels.push(on_item);
            }
        }

        
        
        
        for _ in 1..=self.epochs as i32 {
            for (xi, target) in zip(x, &self.labels_as_nums) {
                let update: f64 = self.eta * (target - self.predict_num(xi)) as f64;
                self.weights = h_vector_add(&self.weights,
                                            &xi.h_vector_scalar_mult(update));
                self.bias += update;
            }
        }
    }
    /// Computes the net input (weighted sum plus bias) for a given input vector.
    /// This is the decision function before applying the activation.
    /// Formula: net_input = sum(weights[i] * x[i]) + bias
    /// Example usage:
    /// let net = percep.net_input(&input_vector);
    pub fn net_input<X>(&self, x_row: &Vec<X>) -> f64
    where   
        X: Copy + Into<f64>,
    {
        return h_2d_dot_product(&x_row, &self.weights) + self.bias;
    }

    /// Predicts the numeric class (0 or 1) for a given input vector.
    /// Uses the step function: if net_input >= 0, return 1, else 0.
    /// Example usage:
    /// let class_num = percep.predict_num(&input_vector);
    pub fn predict_num<X>(&self, x_row: &Vec<X>) -> i32 
    where 
        X: Copy + Into<f64>,
    {
        if self.net_input(x_row) >= 0.0 {
            1
        } else {
            0
        }
    }
    /// Predicts the original label for a given input vector.
    /// Maps the numeric prediction back to the original string label.
    /// Example usage:
    /// let label = percep.predict(&input_vector);
    pub fn predict<X>(&self, x_row: &Vec<X>) -> &'static str
    where 
        X: Copy + Into<f64>,
    {
        if self.net_input(x_row) >= 0.0 {
            return self.unique_original_labels[1];
        } else {
            return self.unique_original_labels[0];
        }
    }
    pub fn predict_multiple<X>(&self, x_rows: &Vec<Vec<X>>) -> Vec<&'static str>
    where 
        X: Copy + Into<f64>,
    {
        let mut answers: Vec<&'static str> = Vec::new();
        for row in x_rows {
            if self.net_input(row) >= 0.0 {
                answers.push(self.unique_original_labels[1]);
            } else {
                answers.push(self.unique_original_labels[0]);
            }
        }
        return answers;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perceptron_learns_iris_subset() {
        let x: Vec<Vec<f64>> = vec![
            // setosa rows
            vec![5.1, 3.5],
            vec![4.9, 3.0],
            vec![4.7, 3.2],
            vec![4.6, 3.1],
            vec![5.0, 3.6],
            vec![5.4, 3.9],
            vec![4.6, 3.4],
            vec![5.0, 3.4],
            vec![4.4, 2.9],
            vec![4.9, 3.1],
            vec![5.4, 3.7],
            vec![4.8, 3.4],
            vec![4.8, 3.0],
            vec![4.3, 3.0],
            vec![5.8, 4.0],
            // versicolor rows
            vec![7.0, 3.2],
            vec![6.4, 3.2],
            vec![6.9, 3.1],
            vec![5.5, 2.3],
            vec![6.5, 2.8],
            vec![5.7, 2.8],
            vec![6.3, 3.3],
            vec![4.9, 2.4],
            vec![6.6, 2.9],
            vec![5.2, 2.7],
            vec![5.0, 2.0],
            vec![5.9, 3.0],
            vec![6.0, 2.2],
            vec![6.1, 2.9],
            vec![5.6, 2.9],
        ];

        let y: Vec<&'static str> = vec![
            "setosa",      // row 0
            "setosa",      // row 1
            "setosa",      // row 2
            "setosa",      // row 3
            "setosa",      // row 4
            "setosa",      // row 5
            "setosa",      // row 6
            "setosa",      // row 7
            "setosa",      // row 8
            "setosa",      // row 9
            "setosa",      // row 10
            "setosa",      // row 11
            "setosa",      // row 12
            "setosa",      // row 13
            "setosa",      // row 14
            "versicolor",  // row 15
            "versicolor",  // row 16
            "versicolor",  // row 17
            "versicolor",  // row 18
            "versicolor",  // row 19
            "versicolor",  // row 20
            "versicolor",  // row 21
            "versicolor",  // row 22
            "versicolor",  // row 23
            "versicolor",  // row 24
            "versicolor",  // row 25
            "versicolor",  // row 26
            "versicolor",  // row 27
            "versicolor",  // row 28
            "versicolor",  // row 29
        ];

        let mut percep = Perceptron::new(0.1, 100, 2);
        percep.fit(&x, &y);

        for (x, target) in zip(&x, &y) {
            assert_eq!(target, &percep.predict(x));
        }

        assert_eq!(&percep.predict_multiple(&x), &y);
    }
}