mod chain;
mod cli;

use crate::chain::chain::is_for;

fn main() {

    let chain = "FOR F21A3 [IAX12, 25, J, 256] := -1 TO +1 DO;";
    // let chain = "FOR F21A3 := -2 TO +1 DO;";

    match is_for(chain, ';') {
        Ok(()) => println!("yes. \"{}\" is chain", chain),
        Err(error) => cli::error::with_message(chain, error.0, error.1)
    }

}
