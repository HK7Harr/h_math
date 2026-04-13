
use std::io;

use std::collections::HashSet;

// -------------------------------- Input Terminal ------------------------------

pub fn h_input_data_single_f64(length: Option<u32>) -> Vec<f64> { // if length is <= 0; stop the inputs with: "<<"
    let mut count: u32 = 1;
    let mut vec: Vec<f64> = vec![];
    loop {
        if length != None {
            if (count - 1) == length.unwrap() && length.unwrap() > 0 {
                return vec;
            }
        }
        let mut input: String = String::new();
        println!("data point {}: ", count);
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        if input.trim() == ":q" {
            return vec;
        }
        let num: f64 = input.trim().parse().expect("Please enter a floating point number of 64 signed bits");
        if num == 0.0 {
            continue;
        }
        vec.push(num);
        count += 1;
    }
}

pub fn h_input_data_single_i32(length: Option<u32>) -> Vec<i32> { 
    let mut count: u32 = 1;
    let mut vec: Vec<i32> = vec![];
    loop {
        if length != None {
            if (count - 1) == length.unwrap() && length.unwrap() > 0 {
                return vec;
            }
        }
        let mut input: String = String::new();
        println!("data point {}: ", count);
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        if input.trim() == ":q" {
            return vec;
        }
        let num: i32 = input.trim().parse().expect("Please enter an Integer NAX: 32 bits unsigned");
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

/// Validates a string input based on the specified requirements.
/// The trait checks if the input string meets the criteria defined by the InputType enum.
/// Returns Ok(()) if valid, or Err with a descriptive message if invalid.
/// Supported types: Lowercase, Uppercase, Letters (both cases), Integer.
/// Example usage:
/// let input = "hello".to_string();
/// let result = input.h_validate_input(InputType::Lowercase);
/// // Returns Ok(()) since "hello" is all lowercase.
/// let input = "Hello".to_string();
/// let result = input.h_validate_input(InputType::Lowercase);
/// // Returns Err with message about uppercase characters found.
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
            if !self.chars().all(|c: char| c.is_ascii_uppercase() || c.is_ascii_lowercase()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_lowercase_valid() {
        let input = "hello".to_string();
        assert!(input.h_validate_input(InputType::Lowercase).is_ok());
    }

    #[test]
    fn test_validate_input_lowercase_invalid() {
        let input = "Hello".to_string();
        assert!(input.h_validate_input(InputType::Lowercase).is_err());
    }

    #[test]
    fn test_validate_input_uppercase_valid() {
        let input = "HELLO".to_string();
        assert!(input.h_validate_input(InputType::Uppercase).is_ok());
    }

    #[test]
    fn test_validate_input_uppercase_invalid() {
        let input = "Hello".to_string();
        assert!(input.h_validate_input(InputType::Uppercase).is_err());
    }

    #[test]
    fn test_validate_input_letters_valid() {
        let input = "HelloWorld".to_string();
        assert!(input.h_validate_input(InputType::Letters).is_ok());
    }

    #[test]
    fn test_validate_input_letters_invalid() {
        let input = "Hello123".to_string();
        assert!(input.h_validate_input(InputType::Letters).is_err());
    }

    #[test]
    fn test_validate_input_integer_valid() {
        let input = "123".to_string();
        assert!(input.h_validate_input(InputType::Integer).is_ok());
    }

    #[test]
    fn test_validate_input_integer_invalid() {
        let input = "123a".to_string();
        assert!(input.h_validate_input(InputType::Integer).is_err());
    }

    #[test]
    fn test_validate_input_empty() {
        let input = "".to_string();
        assert!(input.h_validate_input(InputType::Lowercase).is_err());
    }
}
