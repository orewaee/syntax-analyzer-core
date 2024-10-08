use crate::cli::colors::{RESET, BLUE};

pub fn with_message(string: &str, message: &str) {
    println!("{BLUE}{}{RESET}", string);
    println!("{BLUE}success: {}{RESET}", message);
}