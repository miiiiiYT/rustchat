use std::io::{self, BufRead, Write};

use ansi_control_codes::control_sequences::CUU;

/// Reads a line of input from the user.
///
/// # Returns
///
/// The user's input as a `String`.
pub fn read_input() -> Option<String> {
    // stolen from https://github.com/miiiiiyt/calc.rs
    let stdin = io::stdin();
    let line = stdin.lock().lines().next(); // TODO: implement panic safe
    if line.is_some() {
        return line.unwrap().ok()
    } else {
        return None
    }
}

/// Prompts the user for input and returns the input as a `String`.
///
/// # Arguments
///
/// * `prompt` - The prompt to display to the user.
///
/// # Returns
///
/// The user's input as a `String`.
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt); // the flush here is needed, in order to print the prompt 
    let _ = io::stdout().flush();
    let input = read_input();
    print!("{}",CUU(None));
    let _ = io::stdout().flush();
    if input.is_some() {
        return input.unwrap()
    } else {
        return String::new()
    }
}