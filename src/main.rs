mod chain;
mod cli;

use crate::chain::chain::is_for;
use crate::chain::constant::is_constant;

fn main() {
    /*
    let items = vec!["128", "99 ", "256", "256 ", "2asd32 "];
    for item in items {
        match is_constant(item) {
            Ok(()) => cli::success::with_message(item, "is constant"),
            Err(error) => cli::error::with_message(item, error.0, error.1)
        }
        println!();
    }
    */
    let item = "f or;";

    match is_for(item) {
        Ok(()) => println!("done"),
        Err(error) => cli::error::with_message(item, error.0, error.1)
    }

}
