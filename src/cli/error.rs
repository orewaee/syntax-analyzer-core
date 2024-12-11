use crate::cli::colors::{RESET, RED, BLUE};

pub fn with_message(string: &str, index: usize, message: &str) {
    let string = if string.len() == index { format!("{} ", string) } else { string.to_string() };

    let right = &string[..index];
    let wrong = &string[index..(index + 1)];
    let other = &string[(index + 1)..];

    println!("{BLUE}{}{RESET}{}{}", right, wrong, other);
    println!("{RED}{}└─ {} ({}){RESET}", " ".repeat(index), message, index);
}

pub fn with_message_plain(string: &str, index: usize, message: &str) -> String {
    let string = if string.len() == index { format!("{} ", string) } else { string.to_string() };

    let right = &string[..index];
    let wrong = &string[index..(index + 1)];
    let other = &string[(index + 1)..];

    format!("{}{}{}\n{}└─ {} ({})", right, wrong, other, " ".repeat(index), message, index)
}

pub fn with_message_html(string: &str, index: usize, message: &str) -> String {
    let string = if string.len() == index { format!("{} ", string) } else { string.to_string() };

    let right = &string[..index];
    let wrong = &string[index..(index + 1)];
    let other = &string[(index + 1)..];

    println!("right: \"{right}\"");
    println!("wrong: \"{wrong}\"");
    println!("other: \"{other}\"");

    format!(
        "<span class='right'>{}</span><span class='wrong'>{}</span>{}<br><span class='wrong'>{}└─ {} ({})</span>",
        right.replace(" ", "&nbsp;"), wrong.replace(" ", "&nbsp;"), other.replace(" ", "&nbsp;"),
        "&nbsp;".repeat(right.len()), message, index
    )
}
