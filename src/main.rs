mod chain;
mod cli;

use crate::chain::chain::is_for;

fn main() {

    let chain = "FOR F21A3 [I, 25, J, 75] := -1 TO +1 DO;";
    // let chain = "FOR F21A3 := -2 TO +1 DO;";

    match is_for(chain, ';') {
        Ok(()) => println!("yes. it's chain"),
        Err(error) => cli::error::with_message(chain, error.0, error.1)
    }

}
