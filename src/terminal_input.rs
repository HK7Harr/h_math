
use std::io;
use std::io::Write;
use std::collections::HashSet;

// -------------------------------- Input Terminal ------------------------------
/// Collects a vector of `f64` values from user input via stdin.
///
/// This function prompts the user to enter floating-point numbers one by one.
/// It supports two modes: collecting a fixed number of inputs or collecting until a quit command.
///
/// # Arguments
///
/// * `length` - An optional `u32` specifying the exact number of inputs to collect.
///   If `None`, inputs are collected until the user types ":q".
///
/// # Behavior
///
/// - Prompts with "data point X: " where X is the current count.
/// - Accepts valid `f64` values.
/// - Ignores invalid inputs and prompts again.
/// - Stops when the specified length is reached (if `length` is `Some`) or when ":q" is entered.
/// - Returns a `Vec<f64>` containing all successfully parsed numbers.
///
/// # Examples
///
/// ```rust,ignore
/// // Collect exactly 3 numbers
/// let data = h_input_data_single_f64(Some(3));
///
/// // Collect until user types ":q"
/// let data = h_input_data_single_f64(None);
/// ```
pub fn h_input_data_single_f64(length: Option<u32>) -> Vec<f64> { 
    let mut count: u32 = 1;
    let mut vec: Vec<f64> = Vec::new();
    loop {
        if length != None {
            if (count - 1) == length.unwrap() && length.unwrap() > 0 {
                return vec;
            }
        }
        let mut input: String = String::new();
        
        println!("data point {}: ", count);
        if io::stdin()
        .read_line(&mut input)
        .is_err() {
            println!("Failed to read line");
            continue;
        }
        if input.trim() == ":q" {
            return vec;
        }
        let num: f64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a floating point number with 64 bits assigned to it");
                continue;
            }
        };
        vec.push(num);
        count += 1;
    }
}

/// Collects a vector of `i32` values from user input via stdin.
///
/// This function prompts the user to enter integer numbers one by one.
/// It supports two modes: collecting a fixed number of inputs or collecting until a quit command.
///
/// # Arguments
///
/// * `length` - An optional `u32` specifying the exact number of inputs to collect.
///   If `None`, inputs are collected until the user types ":q".
///
/// # Behavior
///
/// - Prompts with "data point X: " where X is the current count.
/// - Accepts valid `i32` values.
/// - Ignores invalid inputs and prompts again.
/// - Stops when the specified length is reached (if `length` is `Some`) or when ":q" is entered.
/// - Returns a `Vec<i32>` containing all successfully parsed numbers.
///
/// # Examples
///
/// ```rust,ignore
/// // Collect exactly 3 numbers
/// let data = h_input_data_single_i32(Some(3));
///
/// // Collect until user types ":q"
/// let data = h_input_data_single_i32(None);
/// ```
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
        if io::stdin()
        .read_line(&mut input)
        .is_err() {
            println!("Failed to read line");
            continue;
        }
        if input.trim() == ":q" {
            return vec;
        }
        let num: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter an Integer NAX: 32 bits signed");
                continue;
            }
        };
        vec.push(num);
        count += 1;
    }
}


#[derive(PartialEq, Eq, Debug)]
/// Represents the type of input validation to perform.
///
/// ## Variants
///
/// - `Lowercase`: Validates that the input contains only lowercase ASCII letters (a-z).
/// - `Uppercase`: Validates that the input contains only uppercase ASCII letters (A-Z).
/// - `Letters`: Validates that the input contains only ASCII letters (both lowercase and uppercase).
/// - `Integer`: Validates that the input is a valid integer string that can be parsed to `i32`.
pub enum InputType {
    Lowercase,
    Uppercase,
    Letters, // Lowercase + Uppercase
    Integer,
}




/// # ValidateInput Trait
///
/// This trait provides validation for string inputs based on specified requirements defined by `InputType`.
///
/// ## Supported Input Types
///
/// - `Lowercase`: Only lowercase ASCII letters (a-z)
/// - `Uppercase`: Only uppercase ASCII letters (A-Z)
/// - `Letters`: Both lowercase and uppercase ASCII letters
/// - `Integer`: Valid integer strings that can be parsed to `i32`
///
/// ## Returns
///
/// - `Ok(())` if the input is valid according to the specified `InputType`
/// - `Err(String)` with a descriptive error message if the input is invalid
///
/// ## Examples
///
/// ```rust,ignore
/// use h_math::terminal_input::{ValidateInput, InputType};
///
/// let input = "hello".to_string();
/// assert!(input.h_validate_input(InputType::Lowercase).is_ok());
///
/// let input = "Hello".to_string();
/// assert!(input.h_validate_input(InputType::Lowercase).is_err());
///
/// let input = "123".to_string();
/// assert!(input.h_validate_input(InputType::Integer).is_ok());
///
/// let input = "abc123".to_string();
/// assert!(input.h_validate_input(InputType::Integer).is_err());
/// ```
pub trait ValidateInput {
    /// Validates the string input based on the given `InputType` requirements.
    ///
    /// # Arguments
    ///
    /// * `input_requirements` - The type of validation to perform, specified by `InputType`.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the input meets the requirements.
    /// * `Err(String)` with details on why the input is invalid.
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





/// Collects multiple parsed values from stdin into a `Vec`, with two calling modes.
///
/// ---
///
/// ## Variants
///
/// ### 1. Stop-string mode
/// ```rust
/// h_input!($print, $in_type, $stop_string, $err_msg)
/// ```
/// Reads lines until the user enters a specific sentinel string.
///
/// | Parameter | Type | Description |
/// |---|---|---|
/// | `$print` | `block` | Code block that prints a prompt before each read |
/// | `$in_type` | `ty` | The type to parse each line into (e.g. `f64`, `i32`) |
/// | `$stop_string` | `expr` | A `&str` that stops the loop when entered exactly |
/// | `$err_msg` | `expr` | Message printed when parsing fails |
///
/// ---
///
/// ### 2. Fixed-count mode
/// ```rust
/// h_input!($print, $in_type, $amount_of_inputs, $err_msg)
/// ```
/// Reads exactly `n` successfully parsed values.
///
/// | Parameter | Type | Description |
/// |---|---|---|
/// | `$print` | `block` | Code block that prints a prompt before each read |
/// | `$in_type` | `ty` | The type to parse each line into |
/// | `$amount_of_inputs` | `expr` | Number of valid inputs to collect before stopping |
/// | `$err_msg` | `expr` | Message printed when parsing fails — line is **not** counted |
///
/// ---
///
/// ## Behaviour
///
/// - **Bad reads** (`read_line` error): prints `"Failed to read line"` and retries — does not count toward total in fixed-count mode.
/// - **Parse failures**: prints `$err_msg` and retries — input is discarded and not added to the result.
/// - **Stop-string** (variant 1): compared against the trimmed line; match breaks immediately with whatever has been collected so far.
/// - **Returns** `Vec<$in_type>` — always, even if empty.
///
/// ---
///
/// ## Examples
///
/// ```rust,ignore
/// // Collect f64s until user types "done"
/// let values = h_input!(
///     { print!("Enter a number (or 'done' to stop): "); },
///     f64,
///     "done",
///     "Not a valid number, try again."
/// );
///
/// // Collect exactly 3 i32s
/// let values = h_input!(
///     { print!("Enter integer: "); },
///     i32,
///     3u32,
///     "Expected an integer."
/// );
/// ```
#[macro_export]
macro_rules! h_input {
    ($print:block, $in_type:ty, $stop_string:expr, $err_msg:expr) => {
        {
            let mut ret: Vec<$in_type> = Vec::new();
            loop {
                let mut input = String::new();
                $print
                io::stdout().flush().unwrap(); 
                if io::stdin()
                .read_line(&mut input)
                .is_err() {
                    println!("Failed to read line");
                    continue;
                }
                if input.trim() == $stop_string {
                    break
                }
                let num: $in_type = match input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!($err_msg);
                        continue;
                    }
                };
                ret.push(num);
            }
            ret
        }
    };
    ($print:block, $in_type:ty, $amount_of_inputs:expr, $err_msg:expr) => {
        {
            let mut ret: Vec<$in_type> = Vec::new();
            let mut count: usize = 0;
            loop {
                if count >= $amount_of_inputs as usize {
                    break
                }
                let mut input = String::new();
                $print
                if io::stdin()
                .read_line(&mut input)
                .is_err() {
                    println!("Failed to read line");
                    continue;
                }
                
                let num: $in_type = match input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!($err_msg);
                        continue;
                    }
                };
                count += 1;
                ret.push(num);
            }
            ret
        }
    };
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
