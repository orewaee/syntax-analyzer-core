mod chain;
mod cli;

use crate::chain::constant::is_constant;

fn main() {
    let items = vec!["128", "99 ", "256", "256 ", "2asd32 "];
    for item in items {
        match is_constant(item) {
            Ok(()) => println!("{} is constant", item),
            Err(error) => cli::error::with_message(item, error.0, error.1)
        }
    }
}
