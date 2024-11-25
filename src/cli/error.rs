use crate::cli::colors::{RESET, RED, BLUE};

pub fn with_message(string: &str, index: usize, message: &str) {
    let string = if string.len() == index { format!("{} ", string) } else { string.to_string() };

    let right = &string[..index];
    let wrong = &string[index..(index + 1)];
    let other = &string[(index + 1)..];

    println!("{BLUE}{}{RESET}{}{}", right, wrong, other);
    println!("{RED}{}└─ {} ({}){RESET}", " ".repeat(index), message, index);
}
