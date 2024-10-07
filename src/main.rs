mod chain;
mod cli;

use crate::chain::xyz::is_xyz;

fn main() {
    let items = vec!["xyz", "xxxyyasdz;", "xyyyyzzz;"];
    for item in items {
        match is_xyz(item) {
            Ok((x, y, z)) => println!("{} is xyz. x = {}, y = {}, z = {}", item, x, y, z),
            Err(error) => cli::error::with_message(item, error.0, error.1)
        }
    }
}
