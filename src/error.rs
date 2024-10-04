fn green(string: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", string)
}

fn red(string: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", string)
}

pub fn with_message(string: &str, index: usize, message: &str) {
    let rigth = &string[..index];
    let wrong = &string[index..(index + 1)];
    let other = &string[(index + 1)..];

    println!("{}{}{}", green(rigth), red(wrong), other);
    println!("{}{}", " ".repeat(index), red("^"));

    let error = format!("error: {}", message);
    println!("{}", red(error.as_str()));
}
