use std::io::BufWriter;
use std::str::from_utf8_unchecked;

use crate::logic::error::OperationError;
use crate::logic::output::print_help;

// Tool's configuration variants.
#[derive(Debug, PartialEq, Eq)]
pub enum ConfigVariant {
    Symmetric(ConfigSymmetric),
    DF(ConfigDF),
    RSA(ConfigRSA),
}

// Tool's symmetric cipher configuration.
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigSymmetric {
    pub cipher: Cipher,
    pub mode: Mode,
    pub output: Output,
    pub target: String,
    pub key: String,
}

// Tool's Diffie-Hellman configuration.
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigDF {
    pub cipher: Cipher,
    pub mode: Mode,
    pub output: Output,
    pub shared_prime: Option<String>,
    pub shared_base: Option<String>,
    pub secret_a: Option<String>,
    pub secret_b: Option<String>,
}

// Tool's RSA configuration.
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigRSA {
    pub cipher: Cipher,
    pub mode: Mode,
    pub output: Output,
    pub target: Option<String>,
    pub key_exponent: Option<String>,
    pub key_modulus: Option<String>,
    pub thread_count: Option<String>,
}

// Enumeration of the available ciphers for processing.
#[derive(Debug, PartialEq, Eq)]
pub enum Cipher {
    Caesar,
    Vigenere,
    DiffieHellman,
    RSA,
}

// Enumeration of the available encryption modes for processing.
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Encode,
    Decode,
    Generate,
    Bruteforce,
}

// Enumeration of the available outputs modes for the produced result.
#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Console,
    File,
    Both,
}

impl ConfigVariant {
    // Create a new Config struct, filled with received arguments from the command line.
    // Accepted parameter is trait bound by the Iterator trait, but only Args iterator is expected to be received.
    // More generic solution was implemented for unit-tests, so the method could accept custom iterators.
    pub fn new(args: impl Iterator<Item=String>) -> Result<ConfigVariant, Box<dyn std::error::Error>> {
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

        // Create a new iterator for a separate argument checking.
        let mut arg_iterator = arg_vec.iter();

        // Determine cipher type to use, Caesar, Vigenere, RSA or Diffie-Hellman key exchange algorithm.
        let cipher = match arg_iterator.next() {
            Some(arg) if arg.eq("caesar") => Cipher::Caesar,
            Some(arg) if arg.eq("vigenere") => Cipher::Vigenere,
            Some(arg) if arg.eq("df") => Cipher::DiffieHellman,
            Some(arg) if arg.eq("rsa") => Cipher::RSA,
            _ => return Err(Box::new(OperationError::new("Did not receive an argument for the cipher type or it was incorrect. Correct values: \"caesar\" or \"vigenere\"."))),
        };

        // Check if there is a correct amount of arguments.
        // Do not proceed with operations if there are none or an incorrect amount.
        // Define allowed amounts of arguments for DF and RSA.
        let df_argument_counts = vec![3, 7];
        let rsa_argument_counts = vec![3, 5, 6];
        if arg_vec.len() != 5 && (cipher == Cipher::Caesar || cipher == Cipher::Vigenere) {
            return Err(Box::new(OperationError::new("Did not receive a correct amount of arguments for processing. 5 arguments required for Caesar or Vigenere calculations.")));
        } else if !df_argument_counts.contains(&arg_vec.len()) && cipher == Cipher::DiffieHellman {
            return Err(Box::new(OperationError::new("Did not receive a correct amount of arguments for processing. 3 or 7 arguments required for Diffie-Hellman calculations.")));
        } else if !rsa_argument_counts.contains(&arg_vec.len()) && cipher == Cipher::RSA {
            return Err(Box::new(OperationError::new("Did not receive a correct amount of arguments for processing. 3, 4, 6 arguments required for RSA calculations.")));
        }

        // Determine encryption mode to use, encryption or decryption.
        let mode = match arg_iterator.next() {
            Some(arg) if arg.eq("encrypt") => Mode::Encode,
            Some(arg) if arg.eq("decrypt") => Mode::Decode,
            Some(arg) if arg.eq("generate") => Mode::Generate,
            Some(arg) if arg.eq("bruteforce") => Mode::Bruteforce,
            _ => return Err(Box::new(OperationError::new("Did not receive an argument for the encryption mode or it was incorrect. Correct values: \"encrypt\", \"decrypt\", \"generate\" or \"bruteforce\"."))),
        };

        // Determine output mode to use, output result to the console, file or both.
        let output = match arg_iterator.next() {
            Some(arg) if arg.eq("console") => Output::Console,
            Some(arg) if arg.eq("file") => Output::File,
            Some(arg) if arg.eq("both") => Output::Both,
            _ => return Err(Box::new(OperationError::new("Did not receive an argument for the output mode or it was incorrect. Correct values: \"console\", \"file\" or \"both\"."))),
        };

        // Decide further argument retrieval based on the algorithm and mode requested.
        // After processing is complete, return the prepared config.
        if cipher == Cipher::Caesar || cipher == Cipher::Vigenere {

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

            let symmetric_config = ConfigSymmetric {
                cipher,
                mode,
                output,
                target,
                key,
            };

            return Ok(ConfigVariant::Symmetric(symmetric_config));
        } else if cipher == Cipher::DiffieHellman {

            // If there are no additional parameters, all of them will be randomised.
            if arg_vec.len() == 3 && arg_iterator.next() == None {
                let df_config = ConfigDF {
                    cipher,
                    mode,
                    output,
                    shared_prime: None,
                    shared_base: None,
                    secret_a: None,
                    secret_b: None,
                };

                return Ok(ConfigVariant::DF(df_config));
            } else if arg_vec.len() == 7 {

                // Determine shared prime.
                let shared_prime = match arg_iterator.next() {
                    Some(arg) if arg.eq("none") => None,
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the DF shared prime or it was incorrect. Correct values: \"your own number\" or \"none\"."))),
                };

                // Determine shared base.
                let shared_base = match arg_iterator.next() {
                    Some(arg) if arg.eq("none") => None,
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the DF shared base or it was incorrect. Correct values: \"your own number\" or \"none\"."))),
                };

                // Determine secret A.
                let secret_a = match arg_iterator.next() {
                    Some(arg) if arg.eq("none") => None,
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the DF secret A or it was incorrect. Correct values: \"your own number\" or \"none\"."))),
                };

                // Determine secret B.
                let secret_b = match arg_iterator.next() {
                    Some(arg) if arg.eq("none") => None,
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the DF secret B or it was incorrect. Correct values: \"your own number\" or \"none\"."))),
                };

                // Collect the config and send it off,
                let df_config = ConfigDF {
                    cipher,
                    mode,
                    output,
                    shared_prime,
                    shared_base,
                    secret_a,
                    secret_b,
                };

                return Ok(ConfigVariant::DF(df_config));
            } else {
                return Err(Box::new(OperationError::new("Error with Diffie-Hellman configuration logic.")));
            }
        } else if cipher == Cipher::RSA {

            // If there are no additional parameters, required ones will be randomised.
            if arg_vec.len() == 3 && arg_iterator.next() == None && mode == Mode::Generate {

                // If there are no additional parameters, required ones will be randomised.
                let rsa_config = ConfigRSA {
                    cipher,
                    mode,
                    output,
                    target: None,
                    key_exponent: None,
                    key_modulus: None,
                    thread_count: None,
                };
                return Ok(ConfigVariant::RSA(rsa_config));
            } else if arg_vec.len() == 5 && mode == Mode::Bruteforce {

                // Determine RSA exponent.
                let key_exponent = match arg_iterator.next() {
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the RSA exponent or it was incorrect. Correct values: \"your own positive number\"."))),
                };

                // Determine RSA modulus.
                let key_modulus = match arg_iterator.next() {
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the RSA modulus or it was incorrect. Correct values: \"your own positive composite number\"."))),
                };

                // Collect the config and send it off,
                let rsa_config = ConfigRSA {
                    cipher,
                    mode,
                    output,
                    target: None,
                    key_exponent,
                    key_modulus,
                    thread_count: None,
                };
                return Ok(ConfigVariant::RSA(rsa_config));
            } else if arg_vec.len() == 6 && mode == Mode::Bruteforce {

                // Determine RSA exponent.
                let key_exponent = match arg_iterator.next() {
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the RSA exponent or it was incorrect. Correct values: \"your own positive number\"."))),
                };

                // Determine RSA modulus.
                let key_modulus = match arg_iterator.next() {
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the RSA modulus or it was incorrect. Correct values: \"your own positive composite number\"."))),
                };

                // Determine RSA thread count.
                let thread_count = match arg_iterator.next() {
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the RSA thread count or it was incorrect. Correct values: \"your own positive number in the range of 1-64\"."))),
                };

                // Collect the config and send it off,
                let rsa_config = ConfigRSA {
                    cipher,
                    mode,
                    output,
                    target: None,
                    key_exponent,
                    key_modulus,
                    thread_count,
                };

                return Ok(ConfigVariant::RSA(rsa_config));
            } else if arg_vec.len() == 6 && (mode == Mode::Encode || mode == Mode::Decode) {

                // Determine RSA target for encryption or decryption.
                let target = match arg_iterator.next() {
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the RSA target for encryption or encryption or it was incorrect. Correct values: \"your own text for encryption\" or \"the produced hex before for decryption\"."))),
                };

                // Determine RSA exponent.
                let key_exponent = match arg_iterator.next() {
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the RSA exponent or it was incorrect. Correct values: \"your own positive number\"."))),
                };

                // Determine RSA modulus.
                let key_modulus = match arg_iterator.next() {
                    Some(arg) => Some(arg.clone()),
                    _ => return Err(Box::new(OperationError::new("Did not receive an argument for the RSA modulus or it was incorrect. Correct values: \"your own positive composite number\"."))),
                };

                // Collect the config and send it off,
                let rsa_config = ConfigRSA {
                    cipher,
                    mode,
                    output,
                    target,
                    key_exponent,
                    key_modulus,
                    thread_count: None,
                };

                return Ok(ConfigVariant::RSA(rsa_config));
            } else {
                return Err(Box::new(OperationError::new("Error with RSA configuration logic.")));
            }
        }

        Err(Box::new(OperationError::new("Error with the configuration logic.")))
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use std::iter::empty;

    use crate::logic::config::{Cipher, ConfigVariant, Mode, Output};
    use crate::logic::error::OperationError;

    // Test creation of configuration with correct arguments for symmetric algorithms.
    #[test]
    fn test_symmetric_config_creation() {
        // Test Caesar decryption.
        let args_vec = vec!["caesar", "decrypt", "console", "4E626E6E624E6A62", "1"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let config = match config {
            ConfigVariant::Symmetric(symmetric_config) => symmetric_config,
            ConfigVariant::DF(_) => panic!("    A symmetric configuration was expected, but received DF config. (test_config_creation)"),
            ConfigVariant::RSA(_) => panic!("    A symmetric configuration was expected, but received RSA config. (test_config_creation)"),
        };

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

    // Test creation of configuration with correct arguments for Diffie-Hellman algorithm,
    // without optional ones.
    #[test]
    fn test_df_config_creation_no_optional_args() {
        // Test DF algorithm minimum arguments for random generation.
        let args_vec = vec!["df", "generate", "console"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let config = match config {
            ConfigVariant::Symmetric(_) => panic!("    A DF configuration was expected, but received symmetric config. (test_config_creation)"),
            ConfigVariant::DF(df_config) => df_config,
            ConfigVariant::RSA(_) => panic!("    A DF configuration was expected, but received RSA config. (test_config_creation)"),
        };

        let cipher = Cipher::DiffieHellman;
        let mode = Mode::Generate;
        let output = Output::Console;
        let shared_prime = None;
        let shared_base = None;
        let secret_a = None;
        let secret_b = None;

        // Check equality of values between provided arguments and produced config's fields.
        // Check encryption type, DF.
        assert_eq!(config.cipher, cipher);
        // Check encryption mode, does not matter for the DF.
        assert_eq!(config.mode, mode);
        // Check output mode, console/file/both.
        assert_eq!(config.output, output);
        // Check shared prime.
        assert_eq!(config.shared_prime, shared_prime);
        // Check shared base.
        assert_eq!(config.shared_base, shared_base);
        // Check secret A.
        assert_eq!(config.secret_a, secret_a);
        // Check secret B.
        assert_eq!(config.secret_b, secret_b);
    }

    // Test creation of configuration with correct arguments for Diffie-Hellman algorithm,
    // with filled arguments.
    #[test]
    fn test_df_config_creation_all_optional_args() {
        // Test DF algorithm with filled optional arguments..
        let args_vec = vec!["df", "generate", "console", "5", "2", "1", "3"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let config = match config {
            ConfigVariant::Symmetric(_) => panic!("    A DF configuration was expected, but received symmetric config. (test_config_creation)"),
            ConfigVariant::DF(df_config) => df_config,
            ConfigVariant::RSA(_) => panic!("    A DF configuration was expected, but received RSA config. (test_config_creation)"),
        };

        let cipher = Cipher::DiffieHellman;
        let mode = Mode::Generate;
        let output = Output::Console;
        let shared_prime = Option::Some(String::from("5"));
        let shared_base = Option::Some(String::from("2"));
        let secret_a = Option::Some(String::from("1"));
        let secret_b = Option::Some(String::from("3"));

        // Check equality of values between provided arguments and produced config's fields.
        // Check encryption type, DF.
        assert_eq!(config.cipher, cipher);
        // Check encryption mode, does not matter for the DF.
        assert_eq!(config.mode, mode);
        // Check output mode, console/file/both.
        assert_eq!(config.output, output);
        // Check shared prime.
        assert_eq!(config.shared_prime, shared_prime);
        // Check shared base.
        assert_eq!(config.shared_base, shared_base);
        // Check secret A.
        assert_eq!(config.secret_a, secret_a);
        // Check secret B.
        assert_eq!(config.secret_b, secret_b);
    }

    // Test creation of configuration with correct arguments for Diffie-Hellman algorithm,
    // with partially filled arguments.
    #[test]
    fn test_df_config_creation_partial_optional_args() {
        // Test DF algorithm with partially filled optional arguments.
        let args_vec = vec!["df", "generate", "console", "none", "2", "none", "3"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let config = match config {
            ConfigVariant::Symmetric(_) => panic!("    A DF configuration was expected, but received symmetric config. (test_config_creation)"),
            ConfigVariant::DF(df_config) => df_config,
            ConfigVariant::RSA(_) => panic!("    A DF configuration was expected, but received RSA config. (test_config_creation)"),
        };

        let cipher = Cipher::DiffieHellman;
        let mode = Mode::Generate;
        let output = Output::Console;
        let shared_prime = None;
        let shared_base = Option::Some(String::from("2"));
        let secret_a = None;
        let secret_b = Option::Some(String::from("3"));

        // Check equality of values between provided arguments and produced config's fields.
        // Check encryption type, DF.
        assert_eq!(config.cipher, cipher);
        // Check encryption mode, does not matter for the DF.
        assert_eq!(config.mode, mode);
        // Check output mode, console/file/both.
        assert_eq!(config.output, output);
        // Check shared prime.
        assert_eq!(config.shared_prime, shared_prime);
        // Check shared base.
        assert_eq!(config.shared_base, shared_base);
        // Check secret A.
        assert_eq!(config.secret_a, secret_a);
        // Check secret B.
        assert_eq!(config.secret_b, secret_b);
    }

    // Test creation of configuration with more incorrect arguments for Diffie-Hellman algorithm.
    #[test]
    #[should_panic]
    fn test_df_config_creation_more_incorrect_args() {
        // Test DF error, when there is too much arguments.
        let args_vec = vec!["df", "123", "console", "abcd", "abcd", "abcd", "abcd", "abcd"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }
    }

    // Test creation of configuration less arguments for Diffie-Hellman algorithm.
    #[test]
    #[should_panic]
    fn test_df_config_creation_less_incorrect_args() {
        // Test DF error, when there is not enough arguments.
        let args_vec = vec!["df"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }
    }

    // Test creation of configuration with correct arguments for RSA algorithm,
    // without optional ones for random generation of a key pair.
    #[test]
    fn test_rsa_generate_config_creation_no_optional_args() {
        // Test RSA algorithm with random algorithm without providing args.
        let args_vec = vec!["rsa", "generate", "console"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let config = match config {
            ConfigVariant::Symmetric(_) => panic!("    An RSA configuration was expected, but received symmetric config. (test_config_creation)"),
            ConfigVariant::DF(_) => panic!("    An RSA configuration was expected, but received DF config. (test_config_creation)"),
            ConfigVariant::RSA(rsa_config) => rsa_config,
        };

        let cipher = Cipher::RSA;
        let mode = Mode::Generate;
        let output = Output::Console;
        let target = None;
        let key_exponent = None;
        let key_modulus = None;
        let thread_count = None;

        // Check equality of values between provided arguments and produced config's fields.
        // Check encryption type, DF.
        assert_eq!(config.cipher, cipher);
        // Check encryption mode, does not matter for the DF.
        assert_eq!(config.mode, mode);
        // Check output mode, console/file/both.
        assert_eq!(config.output, output);
        // Check shared the target for encryption/decryption.
        assert_eq!(config.target, target);
        // Check the RSA exponent for encryption/decryption.
        assert_eq!(config.key_exponent, key_exponent);
        // Check the RSA modulus.
        assert_eq!(config.key_modulus, key_modulus);
        // Check the thread count.
        assert_eq!(config.thread_count, thread_count);
    }

    // Test creation of configuration with incorrect arguments for RSA algorithm,
    // without optional ones for encryption.
    #[test]
    #[should_panic]
    fn test_rsa_encryption_config_creation_no_optional_args() {
        // Test RSA algorithm with random algorithm without providing args.
        let args_vec = vec!["rsa", "encrypt", "file"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }
    }

    // Test creation of configuration with incorrect arguments for RSA algorithm,
    // without optional ones for bruteforce.
    #[test]
    #[should_panic]
    fn test_rsa_bruteforce_config_creation_no_optional_args() {
        // Test RSA algorithm with random algorithm without providing args.
        let args_vec = vec!["rsa", "bruteforce", "both"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }
    }

    // Test creation of configuration with incorrect arguments for RSA algorithm,
    // with not enough arguments provided.
    #[test]
    #[should_panic]
    fn test_rsa_incorrect_config_creation_not_enough_args() {
        // Test RSA algorithm with random algorithm without providing args.
        let args_vec = vec!["rsa"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }
    }

    // Test creation of configuration with incorrect arguments for RSA algorithm,
    // with too much arguments provided.
    #[test]
    #[should_panic]
    fn test_rsa_incorrect_config_creation_too_much_args() {
        // Test RSA algorithm with random algorithm without providing args.
        let args_vec = vec!["rsa", "rsa", "rsa", "rsa", "rsa", "rsa", "rsa", "rsa", "rsa"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }
    }

    // Test creation of configuration with correct arguments for RSA algorithm,
    // with ones for encryption.
    #[test]
    fn test_rsa_encrypt_config_creation_correct_args() {
        // Test RSA algorithm with random algorithm without providing args.
        let args_vec = vec!["rsa", "encrypt", "console", "target", "exponent", "modulus"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let config = match config {
            ConfigVariant::Symmetric(_) => panic!("    An RSA configuration was expected, but received symmetric config. (test_config_creation)"),
            ConfigVariant::DF(_) => panic!("    An RSA configuration was expected, but received DF config. (test_config_creation)"),
            ConfigVariant::RSA(rsa_config) => rsa_config,
        };

        let cipher = Cipher::RSA;
        let mode = Mode::Encode;
        let output = Output::Console;
        let target = Some(String::from("target"));
        let key_exponent = Some(String::from("exponent"));
        let key_modulus = Some(String::from("modulus"));
        let thread_count = None;

        // Check equality of values between provided arguments and produced config's fields.
        // Check encryption type, DF.
        assert_eq!(config.cipher, cipher);
        // Check encryption mode, does not matter for the DF.
        assert_eq!(config.mode, mode);
        // Check output mode, console/file/both.
        assert_eq!(config.output, output);
        // Check shared the target for encryption/decryption.
        assert_eq!(config.target, target);
        // Check the RSA exponent for encryption/decryption.
        assert_eq!(config.key_exponent, key_exponent);
        // Check the RSA modulus.
        assert_eq!(config.key_modulus, key_modulus);
        // Check the thread count.
        assert_eq!(config.thread_count, thread_count);
    }

    // Test creation of configuration with correct arguments for RSA algorithm,
    // with ones for bruteforcing of a public key without a custom amount of threads.
    #[test]
    fn test_rsa_bruteforce_no_custom_threads_config_creation_correct_args() {
        // Test RSA algorithm with random algorithm without providing args.
        let args_vec = vec!["rsa", "bruteforce", "console", "exponent", "modulus"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let config = match config {
            ConfigVariant::Symmetric(_) => panic!("    An RSA configuration was expected, but received symmetric config. (test_config_creation)"),
            ConfigVariant::DF(_) => panic!("    An RSA configuration was expected, but received DF config. (test_config_creation)"),
            ConfigVariant::RSA(rsa_config) => rsa_config,
        };

        let cipher = Cipher::RSA;
        let mode = Mode::Bruteforce;
        let output = Output::Console;
        let target = None;
        let key_exponent = Some(String::from("exponent"));
        let key_modulus = Some(String::from("modulus"));
        let thread_count = None;

        // Check equality of values between provided arguments and produced config's fields.
        // Check encryption type, DF.
        assert_eq!(config.cipher, cipher);
        // Check encryption mode, does not matter for the DF.
        assert_eq!(config.mode, mode);
        // Check output mode, console/file/both.
        assert_eq!(config.output, output);
        // Check shared the target for encryption/decryption.
        assert_eq!(config.target, target);
        // Check the RSA exponent for encryption/decryption.
        assert_eq!(config.key_exponent, key_exponent);
        // Check the RSA modulus.
        assert_eq!(config.key_modulus, key_modulus);
        // Check the thread count.
        assert_eq!(config.thread_count, thread_count);
    }

    // Test creation of configuration with correct arguments for RSA algorithm,
    // with ones for bruteforcing of a public key without a custom amount of threads.
    #[test]
    fn test_rsa_bruteforce_custom_threads_config_creation_correct_args() {
        // Test RSA algorithm with random algorithm without providing args.
        let args_vec = vec!["rsa", "bruteforce", "console", "exponent", "modulus", "custom_amount_of_threads"];
        let args = args_vec.iter().map(|s| s.to_string());

        let config = ConfigVariant::new(args);

        // Check if config was not successfully created.
        if let Err(e) = config {
            panic!("    An error was encountered during creation of a config struct in a test: {}. (test_config_creation)", e);
        }

        let config = config.unwrap();

        let config = match config {
            ConfigVariant::Symmetric(_) => panic!("    An RSA configuration was expected, but received symmetric config. (test_config_creation)"),
            ConfigVariant::DF(_) => panic!("    An RSA configuration was expected, but received DF config. (test_config_creation)"),
            ConfigVariant::RSA(rsa_config) => rsa_config,
        };

        let cipher = Cipher::RSA;
        let mode = Mode::Bruteforce;
        let output = Output::Console;
        let target = None;
        let key_exponent = Some(String::from("exponent"));
        let key_modulus = Some(String::from("modulus"));
        let thread_count = Some(String::from("custom_amount_of_threads"));

        // Check equality of values between provided arguments and produced config's fields.
        // Check encryption type, DF.
        assert_eq!(config.cipher, cipher);
        // Check encryption mode, does not matter for the DF.
        assert_eq!(config.mode, mode);
        // Check output mode, console/file/both.
        assert_eq!(config.output, output);
        // Check shared the target for encryption/decryption.
        assert_eq!(config.target, target);
        // Check the RSA exponent for encryption/decryption.
        assert_eq!(config.key_exponent, key_exponent);
        // Check the RSA modulus.
        assert_eq!(config.key_modulus, key_modulus);
        // Check the thread count.
        assert_eq!(config.thread_count, thread_count);
    }

    // Test failure of configuration struct creation, when an incorrect amount of arguments passed.
    #[test]
    #[should_panic]
    fn test_config_failure_not_enough_args() {
        let args = ["not", "enough", "args"].iter().map(|s| s.to_string());

        // Try to create a config and retrieve error.
        if let Err(e) = ConfigVariant::new(args) {
            panic!("{}", e);
        }
    }

    // Test failure of configuration struct creation, when no arguments are passed.
    #[test]
    #[should_panic]
    fn test_config_failure_no_args() {
        let args = empty();

        // Try to create a config and retrieve error.
        if let Err(e) = ConfigVariant::new(args) {
            panic!("{}", e);
        }
    }

    // Test of handling of the "help" argument with several other arguments.
    #[test]
    fn test_config_with_help_and_other_args() -> Result<(), Box<dyn std::error::Error>> {
        let args = ["help", "more", "args"].iter().map(|s| s.to_string());

        // Create a new config with the "help" argument.
        match ConfigVariant::new(args) {
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
        match ConfigVariant::new(args) {
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
