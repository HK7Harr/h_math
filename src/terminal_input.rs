
use std::io;

use std::collections::HashSet;

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
