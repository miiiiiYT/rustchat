use std::io::{self, BufRead};

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
    io::Write::flush(&mut io::stdout()).expect("flush failed!"); // TODO: implement panic safe flush
    let input = read_input();
    if input.is_some() {
        return input.unwrap()
    } else {
        return String::new()
    }
}