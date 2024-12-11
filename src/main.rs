mod core;
mod semantics;
mod cli;

use crate::core::analyzer::analyze;

fn main() {
    let chains: Vec<&str> = vec![
        "FOR F21A3 [IAX12345, 25, J, 256] := -1 TO 1 BY 1 DO;",
        "FOR I := " /* +10 TO +20 BY +100 DO; */
    ];

    for chain in chains {
        match analyze(chain, ';') {
            Ok(semantics) => println!("yes. \"{}\" is chain. semantics: {semantics}", chain),
            Err(error) => cli::error::with_message(chain, error.0, error.1)
        }
    }
}
