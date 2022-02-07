use std::io;
use std::io::BufWriter;

use crate::crypto::caesar::{caesar, check_caesar_key};
use crate::crypto::vigenere::vigenere;
use crate::logic::config::{Cipher, Config, Output};
use crate::logic::error::OperationError;
use crate::logic::output::{print_calculation_result, save_calculation_result};

mod output;

pub mod config;

pub mod error;

// Function uniting encryption logic.
// Tests for this function/tool logic can be found in the integration test under "tests" directory.
pub fn run(mut config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Check the chosen cipher and calculate the result.
    let result = if config.cipher == Cipher::Caesar {
        // Check Caesar key, only numbers are accepted.
        if !check_caesar_key(&config.key) {
            return Err(Box::new(OperationError::new("Received incorrect key for Caesar processing, only a number value as a key is accepted.")));
        }
        caesar(&config.mode, &mut config.target, &config.key)?
    } else {
        vigenere(&config.mode, &mut config.target, &config.key)?
    };

    // Get a standard output handle, lock it, wrap into a buffer writer and allocate on heap.
    let stdout = io::stdout();
    let mut handle = Box::new(BufWriter::new(stdout.lock()));

    // Output the produced result in a requested way.
    match config.output {
        // Print out calculation result into the console.
        Output::Console => {
            print_calculation_result(&mut handle, &result)?;
        }
        // Save calculation results into the file.
        Output::File => {
            save_calculation_result(&result)?;
        }
        // Print out calculation result into the console and save it into the file.
        Output::Both => {
            print_calculation_result(&mut handle, &result)?;
            save_calculation_result(&result)?;
        }
    }

    Ok(())
}
