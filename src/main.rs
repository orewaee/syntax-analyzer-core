mod constant;
mod cli;

use crate::constant::is_constant;

fn main() {
    let items = vec!["1", "257", "128 "];
    for item in items {
        match is_constant(item) {
            Ok(()) => println!("{} is constant", item),
            Err(error) => cli::error::with_message(item, error.0, error.1)
        }
    }
}
