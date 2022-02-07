// Program's library.
#![allow(warnings)]

// Module containing Caesar/Vigenere encryption/decryption operations.
mod crypto;

// Module containing encoding/decoding into/from hexadecimal format.
mod encoding;

// Tool's logic.
pub mod logic;

// Pre-refactor code in one place, below.
// Without later applied improvements/modifications/fixes.

// use std::{env, io, fs};
// use std::error::Error;
// use std::io::{BufWriter, Write};
// use std::str::from_utf8_unchecked;
// use std::fmt;
// use std::fmt::Formatter;
//
// // Tool's configuration.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Config {
//     cipher: Cipher,
//     mode: Mode,
//     output: Output,
//     target: String,
//     key: String,
// }
//
// // Enumeration of the available ciphers for processing.
// #[derive(Debug, PartialEq, Eq)]
// enum Cipher {
//     Caesar,
//     Vigenere,
// }
//
// // Enumeration of the available encryption modes for processing.
// #[derive(Debug, PartialEq, Eq)]
// enum Mode {
//     Encode,
//     Decode,
// }
//
// // Enumeration of the available outputs modes for the produced result.
// #[derive(Debug, PartialEq, Eq)]
// enum Output {
//     Console,
//     File,
//     Both,
// }
//
// // Define own error type for handling... errors.
// #[derive(Debug)]
// struct OperationError(String);
//
// impl OperationError {
//     fn new(msg: &str) -> OperationError {
//         OperationError(String::from(msg))
//     }
// }
//
// // Implement Display trait for possible formatting.
// impl fmt::Display for OperationError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
//
// // Implement Error trait for the custom error type.
// impl std::error::Error for OperationError {
//     fn description(&self) -> &str {
//         &self.0
//     }
// }
//
// impl Config {
//     // Create a new Config struct, filled with received argument from the command line.
//     pub fn new(mut args: env::Args) -> Result<Config, Box<dyn std::error::Error>> {
//         // Skip 1st argument, program name.
//         args.next();
//
//         // Collect arguments and re-iterate them.
//         let arg_vec: Vec<String> = args.collect();
//         let arg_iterator = arg_vec.iter();
//
//         // Clone iterator for a separate argument checking.
//         // let mut args_clone = args.cloned();
//
//         // Check for the help argument.
//         for arg in arg_iterator {
//             if arg.eq("help") {
//                 // Print out help message.
//                 print_help()?;
//             }
//         }
//
//         // Clone iterator for a separate argument checking.
//         // let mut args_clone = args.cloned();
//         let arg_iterator = arg_vec.iter();
//         // Check if there are any passed arguments. Do not proceed with operations if there are none.
//         if arg_iterator.count() == 0 {
//             return Err(Box::new(OperationError::new("Did not receive any arguments for processing. Enter \"es(.exe) help\" to get a help message for more information about the tool.")));
//         }
//         let mut arg_iterator = arg_vec.iter();
//         // Determine cipher type to use, Caesar or Vigenere.
//         let cipher = match arg_iterator.next() {
//             Some(arg) if arg.eq("caesar") => Cipher::Caesar,
//             Some(arg) if arg.eq("vigenere") => Cipher::Vigenere,
//             _ => return Err(Box::new(OperationError::new("Did not receive an argument for the cipher type or it was incorrect. Correct values: \"caesar\" or \"vigenere\". Enter \"es(.exe) help\" to get a help message for more information about the tool."))),
//         };
//
//         // Determine encryption mode to use, encryption or decryption.
//         let mode = match arg_iterator.next() {
//             Some(arg) if arg.eq("encrypt") => Mode::Encode,
//             Some(arg) if arg.eq("decrypt") => Mode::Decode,
//             _ => return Err(Box::new(OperationError::new("Did not receive an argument for the encryption mode or it was incorrect. Correct values: \"encrypt\" or \"decrypt\". Enter \"es(.exe) help\" to get a help message for more information about the tool."))),
//         };
//
//         // Determine output mode to use, output result to the console, file or both.
//         let output = match arg_iterator.next() {
//             Some(arg) if arg.eq("console") => Output::Console,
//             Some(arg) if arg.eq("file") => Output::File,
//             Some(arg) if arg.eq("both") => Output::Both,
//             _ => return Err(Box::new(OperationError::new("Did not receive an argument for the output mode or it was incorrect. Correct values: \"console\", \"file\" or \"both\". Enter \"es(.exe) help\" to get a help message for more information about the tool."))),
//         };
//
//         // Retrieve a plaintext or a ciphertext for processing.
//         let target = match arg_iterator.next() {
//             Some(arg) => arg.clone(),
//             _ => return Err(Box::new(OperationError::new("Did not receive a plaintext or a ciphertext for processing. Enter \"es(.exe) help\" to get a help message for more information about the tool."))),
//         };
//
//         // Retrieve a key for processing.
//         let key = match arg_iterator.next() {
//             Some(arg) => arg.clone(),
//             _ => return Err(Box::new(OperationError::new("Did not receive a key for processing. Enter \"es(.exe) help\" to get a help message for more information about the tool."))),
//         };
//
//         Ok(Config {
//             cipher,
//             mode,
//             output,
//             target,
//             key,
//         })
//     }
// }
//
// // A function to print out help message to the console.
// fn print_help() -> Result<String, Box<dyn std::error::Error>> {
//     // let stdout = io::stdout()?;
//     // let mut handle = io::BufWriter::new(stdout.lock())?;
//
//     // Get a standard output handle, lock it, wrap into a buffer writer and allocate on heap.
//     let stdout = io::stdout();
//     let mut handle = Box::new(BufWriter::new(stdout.lock()));
//
//     writeln!(handle, "A command line tool to encrypt/decrypt strings with Caesar or Vigenere ciphers.")?;
//     writeln!(handle, "Usage pattern: se(.exe) <cipher type> <encryption mode> <output mode> <plaintext or ciphertext> <key>")?;
//     writeln!(handle)?;
//     writeln!(handle, "Possible values for the listed arguments:")?;
//     writeln!(handle, "    - cipher type: caesar/vigenere,")?;
//     writeln!(handle, "    - encryption mode: encrypt/decrypt,")?;
//     writeln!(handle, "    - output mode: console/file/both,")?;
//     writeln!(handle, "    - plaintext or ciphertext: \"your text/string/phrase to encrypt or decrypt,\"")?;
//     writeln!(handle, "    - key: \"your key to use for encryption or decryption,\"")?;
//     writeln!(handle)?;
//     writeln!(handle, "Notice:")?;
//     writeln!(handle, "    - This tool outputs encrypted information in a hexadecimal encoding format.")?;
//     writeln!(handle, "    - This tool only accepts ciphertexts for decryption, previousle encrypted with this tool in hex format.")?;
//     writeln!(handle, "    - If the \"file\" or \"both \" output mode were chosen then the ciphertext will be saved in the file, named \"ciphertext.txt\".")?;
//     writeln!(handle, "    - If the \"file\" or \"both \" output mode were chosen then the ciphertext will be saved in the same location of the tool.")?;
//     writeln!(handle, "    - Caesar mode encryption and decryption accept only whole numbers as a key (both positive and negative).")?;
//     writeln!(handle, "    - Vigenere mode encryption and decryption accept any string as a key.")?;
//     writeln!(handle)?;
//     writeln!(handle, "Examples of usage:")?;
//     writeln!(handle, "    - To encrypt a string in Caesar cipher and output the result into the console:")?;
//     writeln!(handle, "    se(.exe) caesar encrypt console ThisIsAMessageToEncrypt 100")?;
//     writeln!(handle, "    - To decrypt a string in Vigenere cipher and output the result into the file:")?;
//     writeln!(handle, "    se(.exe) vigenere decrypt file ThisIsAMessageToDecryptInHEX ThisIsTheUsedKey")?;
//     writeln!(handle)?;
//     writeln!(handle, "To trigger this help message pass \"help\" argument:")?;
//     writeln!(handle, "    - se(.exe) help")?;
//     writeln!(handle)?;
//
//     // Print out buffer.
//     handle.flush()?;
//
//     // Exit program.
//     std::process::exit(0);
// }
//
// // Function uniting encryption logic.
// pub fn run(mut config: Config) -> Result<(), Box<dyn std::error::Error>> {
//
//     // Check the chosen cipher and calculate the result.
//     let result = if *(&config.cipher) == Cipher::Caesar {
//         // Check Caesar key, only numbers are accepted.
//         if !check_caesar_key(&config.key) {
//             return Err(Box::new(OperationError::new("Received incorrect key for Caesar processing, only a number value as a key is accepted. Enter \"es(.exe) help\" to get a help message for more information about the tool.")));
//         }
//         caesar(&config.mode, &mut config.target, &config.key)?
//     } else {
//         vigenere(&config.mode, &mut config.target, &config.key)?
//     };
//
//     // Output the produced result in a requested way.
//     match config.output {
//         // Print out calculation result into the console.
//         Output::Console => {
//             print_calculation_result(&result)?;
//         },
//         // Save calculation results into the file.
//         Output::File => {
//             save_calculation_result(&result)?;
//         },
//         // Print out calculation result into the console and save it into the file.
//         Output::Both => {
//             print_calculation_result(&result)?;
//             save_calculation_result(&result)?;
//         },
//     }
//
//     Ok(())
// }
//
// // Print out calculation result into the console.
// fn print_calculation_result(result: &str) -> Result<(), std::io::Error> {
//     // Get a standard output handle, lock it, wrap into a buffer writer and allocate on heap.
//     let stdout = io::stdout();
//     let mut handle = Box::new(BufWriter::new(stdout.lock()));
//     writeln!(handle, "The result of the calculations:")?;
//     writeln!(handle, "{}", result)?;
//
//     // Print out buffer.
//     handle.flush()?;
//
//     Ok(())
// }
//
// // Save calculation result into the file.
// fn save_calculation_result(result: &str) -> Result<(), std::io::Error> {
//     fs::write("calculation_result.txt", result)?;
//     println!("Successfully saved the result of the calculations into \"calculation_result.txt\" file at the location of the program.");
//     Ok(())
// }
//
// // Check if the Caesar key numeric and shorter than 39 number integer, if so return True, otherwise False.
// fn check_caesar_key(key: &str) -> bool {
//     for char in key.chars(){
//         if !char.is_numeric() {
//             return false
//         }
//     }
//     if key.len() >= 39 {
//         return false
//     }
//     true
// }
//
// // Function to encrypt or decrypt the target sting under Caesar cipher.
// fn caesar(mode: &Mode, target: &mut str, key: &str) -> Result<String, Box<dyn Error>> {
//     // Byte has only 256 variations, considering the algorithm used,
//     // there is no need for key number bigger than 256;
//     // the euclidean modulus is calculated to account for possible negative entries instead of
//     // C-like remainder "%" operation.
//     let key: i128 = key.parse()?;
//     let key = key.rem_euclid(256);
//     let key = key as u8;
//
//     match mode {
//         Mode::Encode => {
//             // Convert string to the vector of unsigned one byte integers.
//             let target = unsafe {target.as_bytes_mut()};
//
//             // Encrypt vector of bytes one by one.
//             for char in target.iter_mut() {
//                 caesar_encrypt_char(char, &key);
//             }
//
//             // Encode the vector of bytes into the hex string.
//             string_hex_encode(target)
//
//             // !!!TEMP
//             // Ok(String::from(unsafe {from_utf8_unchecked(target)}))
//         },
//         Mode::Decode => {
//             // Convert received hex string into the vector of encrypted one bytes.
//             let mut decoded_string = string_hex_decode(target)?;
//
//             // !!!TEMP
//             // let mut decoded_string = unsafe {target.as_bytes_mut()};
//
//             // Decrypt vector of bytes one by one.
//             for char in decoded_string.iter_mut() {
//                 caesar_decrypt_char(char, &key);
//             }
//
//             let result = unsafe {from_utf8_unchecked(&decoded_string)};
//
//             Ok(String::from(result))
//         },
//     }
// }
//
// // Encrypt provided character.
// fn caesar_encrypt_char(target_char: &mut u8, key: &u8) {
//     // Cast target value from unsigned one byte integer to the two byte signed integer,
//     // add key value to the target value, if the result exceeds 256,
//     // subtract 256. Turn the temporary value back into one byte unsigned integer and
//     // replace with it the original value.
//     let mut wide_char = target_char.clone() as i16;
//     wide_char += *key as i16;
//     if wide_char >= 256 {
//         wide_char -= 256;
//     }
//     let temp_char = wide_char as u8;
//     std::mem::replace(target_char, temp_char);
// }
//
// // Decrypt provided character.
// fn caesar_decrypt_char(target_char: &mut u8, key: &u8) {
//     // Opposite operations to the encryption are done.
//     let mut wide_char = target_char.clone() as i16;
//     wide_char -= *key as i16;
//     if wide_char < 0 {
//         wide_char += 256;
//     }
//     let temp_char = wide_char as u8;
//     std::mem::replace(target_char, temp_char);
// }
//
// // Match 1 byte value to hexadecimal representation.
// fn one_u8_to_hex(int: u8) ->  Result<Vec<char>, Box<dyn Error>> {
//     // Create two variables representing two halves of the byte.
//     let int_left_half = int >> 4;
//     let int_right_half = (int << 4) >> 4;
//     let mut result = Vec::new();
//
//     // Encode left half of the byte and push the result to the vector of characters.
//     match int_left_half {
//         10..=15 => result.push((int_left_half + 31) as char),
//         0..=9 => result.push((int_left_half + 30) as char),
//         _ => return Err(Box::new(OperationError::new("An error was encountered during ciphertext encoding. Try to encrypt/decrypt again. (one_u8_to_hex)"))),
//     };
//
//     // Encode right half of the byte and push the result to the vector of characters.
//     match int_right_half {
//         10..=15 => result.push((int_right_half + 31) as char),
//         0..=9 => result.push((int_right_half + 30) as char),
//         _ => return Err(Box::new(OperationError::new("An error was encountered during ciphertext encoding. Try to encrypt/decrypt again. (one_u8_to_hex)"))),
//     };
//
//     Ok(result)
// }
//
// // Transform string consisting of ciphertext bytes into the hexadecimal string.
// fn string_hex_encode(string: &[u8]) -> Result<String, Box<dyn Error>> {
//     let mut result = Vec::new();
//
//     // Convert every character from the ciphertext into its hexadecimal representation.
//     for char in string {
//         let mut hex_pair = one_u8_to_hex(*char)?;
//         result.append(&mut hex_pair);
//     }
//
//     Ok(result.into_iter().collect())
// }
//
// // Match hex value to the 1 byte decimal representation.
// fn one_hex_to_u8(hex: u8) -> Result<u8, Box<dyn Error>> {
//     match hex {
//         b'A'..=b'F' => Ok(hex - b'A' + 10),
//         b'a'..=b'f' => Ok(hex - b'a' + 10),
//         b'0'..=b'9' => Ok(hex - b'0'),
//         // _ => Box::new(Err("Incorrect hex symbol. (one_hex_to_u8)")),
//         _ => Err(Box::new(OperationError::new("Received incorrect ciphertext in hexadecimal format for processing, only texts consisting of A-F, a-f and 0-9 values are accepted. Enter \"es(.exe) help\" to get a help message for more information about the tool."))),
//     }
// }
//
// // Transform string consisting of hex symbols into the vector of decimal integers of one byte.
// fn string_hex_decode(hex_string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
//     if hex_string.chars().count() % 2 != 0 {
//         return Err(Box::new(OperationError::new("Received ciphertext in hexadecimal with odd amount for characters, only texts with even amount are accepted. Enter \"es(.exe) help\" to get a help message for more information about the tool.")));
//     }
//
//     let mut decoded_cipher: Vec<u8> = Vec::new();
//     let ref_cipher: &[u8] = hex_string.as_ref();
//     // Split hex string into the chunks of 2 hex characters and translate them into the decimal representation.
//     // First hex symbol is translated and then bits are shifted to the left by 4 bits.
//     // Translation of the second hex symbol is added to the first with the bitwise OR.
//     let char_vector = ref_cipher.chunks(2).enumerate().map(|(_index, hex_pair)| {
//         one_hex_to_u8(hex_pair[0]).unwrap() << 4 | one_hex_to_u8(hex_pair[1]).unwrap()
//     });
//     for character in char_vector {
//         decoded_cipher.push(character as u8);
//     }
//     Ok(decoded_cipher)
// }
//
// fn vigenere(mode: &Mode, target: &mut str, key: &str) -> Result<String, Box<dyn Error>> {
//     // Turn key string into vector of bytes.
//     let key = key.as_bytes();
//     let key_len = key.len();
//
//     match mode {
//         Mode::Encode => {
//             // Convert string to the vector of unsigned one byte integers.
//             let target = unsafe { target.as_bytes_mut() };
//
//             // Iterator over key.
//             let mut i = 0;
//
//             // Encrypt vector of bytes one by one.
//             for char in target.iter_mut() {
//                 caesar_encrypt_char(char, &key[i]);
//                 i = (i + 1) % key_len;
//             }
//
//             // Encode the vector of bytes into the hex string.
//             string_hex_encode(target)
//         },
//         Mode::Decode => {
//             // Convert received hex string into the vector of encrypted one bytes.
//             let mut decoded_string = string_hex_decode(target)?;
//
//             // Iterator over key.
//             let mut i = 0;
//
//             // Decrypt vector of bytes one by one.
//             for char in decoded_string.iter_mut() {
//                 caesar_decrypt_char(char, &key[i]);
//                 i = (i + 1) % key_len;
//             }
//
//             let result = unsafe {from_utf8_unchecked(&decoded_string)};
//
//             Ok(String::from(result))
//         },
//     }
// }
