use std::io::BufWriter;
use std::str::from_utf8_unchecked;

use crate::logic::error::OperationError;
use crate::logic::output::print_help;

// Tool's configuration.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub cipher: Cipher,
    pub mode: Mode,
    pub output: Output,
    pub target: String,
    pub key: String,
}

// Enumeration of the available ciphers for processing.
#[derive(Debug, PartialEq, Eq)]
pub enum Cipher {
    Caesar,
    Vigenere,
}

// Enumeration of the available encryption modes for processing.
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Encode,
    Decode,
}

// Enumeration of the available outputs modes for the produced result.
#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Console,
    File,
    Both,
}

impl Config {
    // Create a new Config struct, filled with received arguments from the command line.
    // Accepted parameter is trait bound by the Iterator trait, but only Args iterator is expected to be received.
    // More generic solution was implemented for unit-tests, so the method could accept custom iterators.
    pub fn new(args: impl Iterator<Item = String>) -> Result<Config, Box<dyn std::error::Error>> {
        // Collect arguments and re-iterate them.
        let arg_vec: Vec<String> = args.collect();
        let arg_iterator = arg_vec.iter();

        // Check for the help argument, if it is found, create a help message and return it.
        for arg in arg_iterator {
            if arg.eq("help") {
                // Get a vector of bytes, lock it, wrap into a buffer writer and allocate on heap.
                let mut handle = Box::new(BufWriter::new(Vec::new()));

                // Produce the help message.
                print_help(&mut handle)?;

                // Turn vector of bytes into a string.
                let help_message_ref = handle.get_ref();
                let help_message = unsafe { from_utf8_unchecked(help_message_ref) };

                // Create custom error with the help message and set the help flag.
                let mut help_package = OperationError::new(help_message);
                help_package.set_help_flag();

                // Return the help message wrapped into the custom error type to the main function.
                return Err(Box::new(help_package));
            }
        }

        // Check if there is a correct amount of arguments.
        // Do not proceed with operations if there are none or an incorrect amount.
        // If there is one argument and it is "help", print out help message and terminate program.
        if arg_vec.len() != 5 {
            return Err(Box::new(OperationError::new("Did not receive a correct amount of arguments for processing. 5 arguments required.")));
        }

        // Create a new iterator for a separate argument checking.
        let mut arg_iterator = arg_vec.iter();

        // Determine cipher type to use, Caesar or Vigenere.
        let cipher = match arg_iterator.next() {
            Some(arg) if arg.eq("caesar") => Cipher::Caesar,
            Some(arg) if arg.eq("vigenere") => Cipher::Vigenere,
            _ => return Err(Box::new(OperationError::new("Did not receive an argument for the cipher type or it was incorrect. Correct values: \"caesar\" or \"vigenere\"."))),
        };

        // Determine encryption mode to use, encryption or decryption.
        let mode = match arg_iterator.next() {
            Some(arg) if arg.eq("encrypt") => Mode::Encode,
            Some(arg) if arg.eq("decrypt") => Mode::Decode,
            _ => return Err(Box::new(OperationError::new("Did not receive an argument for the encryption mode or it was incorrect. Correct values: \"encrypt\" or \"decrypt\"."))),
        };

        // Determine output mode to use, output result to the console, file or both.
        let output = match arg_iterator.next() {
            Some(arg) if arg.eq("console") => Output::Console,
            Some(arg) if arg.eq("file") => Output::File,
            Some(arg) if arg.eq("both") => Output::Both,
            _ => return Err(Box::new(OperationError::new("Did not receive an argument for the output mode or it was incorrect. Correct values: \"console\", \"file\" or \"both\"."))),
        };

        // Retrieve a plaintext or a ciphertext for processing.
        let target = match arg_iterator.next() {
            Some(arg) => arg.clone(),
            _ => {
                return Err(Box::new(OperationError::new(
                    "Did not receive a plaintext or a ciphertext for processing.",
                )));
            }
        };

        // Retrieve a key for processing.
        let key = match arg_iterator.next() {
            Some(arg) => arg.clone(),
            _ => {
                return Err(Box::new(OperationError::new(
                    "Did not receive a key for processing.",
                )));
            }
        };

        Ok(Config {
            cipher,
            mode,
            output,
            target,
            key,
        })
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use std::iter::empty;

    use crate::logic::config::{Cipher, Config, Mode, Output};
    use crate::logic::error::OperationError;

    // Test creation of configuration with correct arguments.
    #[test]
    fn test_config_creation() {
        let args_vec = vec!["caesar", "decrypt", "console", "4E626E6E624E6A62", "1"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = Config::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a cofnig struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let cipher = Cipher::Caesar;
        let mode = Mode::Decode;
        let output = Output::Console;

        // Check equality of values between provided arguments and produced config's fields.
        // Check encryption type, caesar/vigenere.
        assert_eq!(config.cipher, cipher);
        // Check encryption mode, encrypt/decrypt.
        assert_eq!(config.mode, mode);
        // Check output mode, console/file/both.
        assert_eq!(config.output, output);
        // Check target string.
        assert_eq!(config.target, *args_vec.get(3).unwrap());
        // Check key.
        assert_eq!(config.key, *args_vec.get(4).unwrap());
    }

    // Test failure of configuration struct creation, when an incorrect amount of arguments passed.
    #[test]
    #[should_panic]
    fn test_config_failure_not_enough_args() {
        let args = ["not", "enough", "args"].iter().map(|s| s.to_string());

        // Try to create a config and retrieve error.
        if let Err(e) = Config::new(args) {
            panic!("{}", e);
        }
    }

    // Test failure of configuration struct creation, when an no arguments are passed.
    #[test]
    #[should_panic]
    fn test_config_failure_no_args() {
        let args = empty();

        // Try to create a config and retrieve error.
        if let Err(e) = Config::new(args) {
            panic!("{}", e);
        }
    }

    // Test of handling of the "help" argument with several other arguments.
    #[test]
    fn test_config_with_help_and_other_args() -> Result<(), Box<dyn std::error::Error>> {
        let args = ["help", "more", "args"].iter().map(|s| s.to_string());

        // Create a new config with the "help" argument.
        match Config::new(args) {
            // Check for the error struct with help message.
            // Return error if a correct config was received.
            Err(e) => {
                match e.downcast::<OperationError>() {
                    // Return Ok if the error contains the help message,
                    // otherwise return encountered errors.
                    Ok(value_ref) => {
                        let value = *value_ref;
                        if value.get_help_flag() {
                            return Ok(());
                        }
                        return Err(Box::new(value));
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(_) => return Err(Box::new(OperationError::new("Received correct config when error with help message was expected. (test_config_with_help_arg)"))),
        }
    }

    // Test of handling of the "help" as the only argument.
    #[test]
    fn test_config_with_only_help_arg() -> Result<(), Box<dyn std::error::Error>> {
        let args = ["help"].iter().map(|s| s.to_string());

        // Create a new config with the "help" argument.
        match Config::new(args) {
            // Check for the error struct with help message.
            // Return error if a correct config was received.
            Err(e) => {
                match e.downcast::<OperationError>() {
                    // Return Ok if the error contains the help message,
                    // otherwise return encountered errors.
                    Ok(value_ref) => {
                        let value = *value_ref;
                        if value.get_help_flag() {
                            return Ok(());
                        }
                        return Err(Box::new(value));
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(_) => return Err(Box::new(OperationError::new("Received correct config when error with help message was expected. (test_config_with_help_arg)"))),
        }
    }
}
