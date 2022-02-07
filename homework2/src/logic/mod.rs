use std::io;
use std::io::BufWriter;

use crate::crypto::caesar::{caesar, check_caesar_key};
use crate::crypto::diffie_hellman::diffie_hellman;
use crate::crypto::rsa::rsa;
use crate::crypto::vigenere::vigenere;
use crate::logic::config::{Cipher, ConfigVariant, Output};
use crate::logic::error::OperationError;
use crate::logic::output::{print_calculation_result, print_df_calculation_result, print_rsa_calculation_result, save_calculation_result, save_df_calculation_result, save_rsa_calculation_result};

mod output;

pub mod config;

pub mod error;

pub mod bigint;

// Function uniting encryption logic.
// Tests for this function/tool logic can be found in the integration test under "tests" directory.
pub fn run(config: ConfigVariant) -> Result<(), Box<dyn std::error::Error>> {
    let mut symmetric_result = String::new();
    let mut df_result = Default::default();
    let mut rsa_result = Default::default();
    let cipher_mode;
    let mut output_mode = Output::File;

    // Determine the appropriate action according to the type of configuration and store the results of calculations.
    match config {
        ConfigVariant::Symmetric(mut symmetric_config) => {
            // Check the chosen cipher and calculate the result.
            symmetric_result = if symmetric_config.cipher == Cipher::Caesar {
                // Store cipher and output mode.
                cipher_mode = Cipher::Caesar;
                output_mode = symmetric_config.output;

                // Check Caesar key, only numbers are accepted.
                if !check_caesar_key(&symmetric_config.key) {
                    return Err(Box::new(OperationError::new("Received incorrect key for Caesar processing, only a number value as a key is accepted.")));
                }
                caesar(&symmetric_config.mode, &mut symmetric_config.target, &symmetric_config.key)?
            } else {
                cipher_mode = Cipher::Vigenere;
                vigenere(&symmetric_config.mode, &mut symmetric_config.target, &symmetric_config.key)?
            };
        }
        ConfigVariant::DF(df_config) => {
            // Store cipher and output mode.
            cipher_mode = Cipher::DiffieHellman;
            output_mode = df_config.output;

            let shared_prime = df_config.shared_prime;
            let shared_base = df_config.shared_base;
            let secret_a = df_config.secret_a;
            let secret_b = df_config.secret_b;

            df_result = diffie_hellman(shared_prime, shared_base, secret_a, secret_b)?
        }
        ConfigVariant::RSA(rsa_config) => {
            // Store cipher and output mode.
            cipher_mode = Cipher::RSA;
            output_mode = rsa_config.output;

            let target = rsa_config.target;
            let key_exponent = rsa_config.key_exponent;
            let key_modulus = rsa_config.key_modulus;
            let thread_count = rsa_config.thread_count;

            rsa_result = rsa(&rsa_config.mode, target, key_exponent, key_modulus, thread_count)?;
        }
    }


    // Get a standard output handle, lock it, wrap into a buffer writer and allocate on heap.
    let stdout = io::stdout();
    let mut handle = Box::new(BufWriter::new(stdout.lock()));

    // Output the produced result in a requested way.
    // And match the output according to the result types.
    match cipher_mode {
        Cipher::DiffieHellman => {
            // Produce an output for Diffie-Hellman algorithm.
            match output_mode {
                // Print out calculation result into the console.
                Output::Console => {
                    print_df_calculation_result(&mut handle, &df_result)?;
                }
                // Save calculation results into the file.
                Output::File => {
                    save_df_calculation_result(&df_result)?;
                }
                // Print out calculation result into the console and save it into the file.
                Output::Both => {
                    print_df_calculation_result(&mut handle, &df_result)?;
                    save_df_calculation_result(&df_result)?;
                }
            }
        }
        Cipher::RSA => {
            // Produce an output for RSA algorithm.
            match output_mode {
                // Print out calculation result into the console.
                Output::Console => {
                    print_rsa_calculation_result(&mut handle, &rsa_result)?;
                }
                // Save calculation results into the file.
                Output::File => {
                    save_rsa_calculation_result(&rsa_result)?;
                }
                // Print out calculation result into the console and save it into the file.
                Output::Both => {
                    print_rsa_calculation_result(&mut handle, &rsa_result)?;
                    save_rsa_calculation_result(&rsa_result)?;
                }
            }
        }
        _ => {
            // Produce an output for Caesar or Vigenere ciphers.
            match output_mode {
                // Print out calculation result into the console.
                Output::Console => {
                    print_calculation_result(&mut handle, &symmetric_result)?;
                }
                // Save calculation results into the file.
                Output::File => {
                    save_calculation_result(&symmetric_result)?;
                }
                // Print out calculation result into the console and save it into the file.
                Output::Both => {
                    print_calculation_result(&mut handle, &symmetric_result)?;
                    save_calculation_result(&symmetric_result)?;
                }
            }
        }
    }


    Ok(())
}
