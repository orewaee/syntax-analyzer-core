pub fn green(string: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", string)
}

pub fn red(string: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", string)
}
