use std::error::Error;
use std::str::from_utf8_unchecked;

use crate::encoding::*;
use crate::logic::config::Mode;

// Check if the Caesar key numeric and shorter than 39 number integer, if so return True, otherwise False.
pub fn check_caesar_key(key: &str) -> bool {
    let mut char_iter = key.chars();

    // Check if the first character is minus/hyphen or a number.
    // If so, proceed with further checking.
    if let Some(char) = char_iter.next() {
        if !(char == '-' || char.is_numeric()) {
            return false;
        }
    }

    // Check if every other character is numeric.
    for char in char_iter {
        if !char.is_numeric() {
            return false;
        }
    }

    // Check the length of the key, if it too long to handle, deny it.
    if key.len() >= 39 {
        return false;
    }

    true
}

// Function to encrypt or decrypt the target sting under Caesar cipher.
pub fn caesar(mode: &Mode, target: &mut str, key: &str) -> Result<String, Box<dyn Error>> {
    // Byte has only 256 variations, considering the algorithm used,
    // there is no need for key number bigger than 256;
    // the euclidean modulus is calculated to account for possible negative entries instead of
    // C-like remainder "%" operation.
    let key: i128 = key.parse()?;
    let key = key.rem_euclid(256);
    let key = key as u8;

    match mode {
        Mode::Encode => {
            // Convert string to the vector of unsigned one byte integers.
            let target = unsafe { target.as_bytes_mut() };

            // Encrypt vector of bytes one by one.
            for char in target.iter_mut() {
                caesar_encrypt_char(char, &key);
            }

            // Encode the vector of bytes into the hex string.
            string_hex_encode(target)
        }
        Mode::Decode => {
            // Convert received hex string into the vector of encrypted one bytes.
            let mut decoded_string = string_hex_decode(target)?;

            // Decrypt vector of bytes one by one.
            for char in decoded_string.iter_mut() {
                caesar_decrypt_char(char, &key);
            }

            let result = unsafe { from_utf8_unchecked(&decoded_string) };

            Ok(String::from(result))
        }
    }
}

// Encrypt provided character.
pub fn caesar_encrypt_char(target_char: &mut u8, key: &u8) {
    // Cast target value from unsigned one byte integer to the two byte signed integer,
    // add key value to the target value, if the result exceeds 256,
    // subtract 256. Turn the temporary value back into one byte unsigned integer and
    // replace with it the original value.
    let mut wide_char = *target_char as i16;
    wide_char += *key as i16;
    if wide_char >= 256 {
        wide_char -= 256;
    }
    let temp_char = wide_char as u8;
    let _ = std::mem::replace(target_char, temp_char);
}

// Decrypt provided character.
pub fn caesar_decrypt_char(target_char: &mut u8, key: &u8) {
    // Opposite operations to the encryption are done.
    let mut wide_char = *target_char as i16;
    wide_char -= *key as i16;
    if wide_char < 0 {
        wide_char += 256;
    }
    let temp_char = wide_char as u8;
    let _ = std::mem::replace(target_char, temp_char);
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::crypto::caesar::{
        caesar, caesar_decrypt_char, caesar_encrypt_char, check_caesar_key,
    };
    use crate::logic::config::Mode;

    // Test Caesar encryption.
    #[test]
    fn test_caesar_encryption() {
        // Setup environment.
        let mode = Mode::Encode;
        let target_original = "TestString123";
        let mut target = String::from(target_original);
        let key = "18903427";

        // Encrypt target string.
        let encryption_result = caesar(&mode, &mut target, &key).unwrap();

        println!(
            "  Original target string: {} (test_caesar_encryption)",
            target_original
        );
        println!(
            "  Result of encryption and decryption: {} (test_caesar_encryption)",
            encryption_result
        );

        assert_ne!(target_original, encryption_result);
    }

    // Test Caesar encryption and decryption.
    #[test]
    fn test_caesar_encryption_and_decryption() {
        // Setup environment.
        let encryption_mode = Mode::Encode;
        let decryption_mode = Mode::Decode;
        let target_original = "TestString123";
        let mut target = String::from(target_original);
        let key = "18903427";

        // Encrypt target string.
        let encryption_result = caesar(&encryption_mode, &mut target, &key).unwrap();

        let mut encryption_result_clone = encryption_result.clone();

        // Decrypt target string.
        let decryption_result =
            caesar(&decryption_mode, &mut encryption_result_clone, &key).unwrap();

        println!(
            "  Target for Caesar encryption: {} (test_caesar_encryption_and_decryption)",
            target_original
        );
        println!(
            "  Caesar encryption result: {} (test_caesar_encryption_and_decryption)",
            encryption_result
        );
        println!(
            "  Caesar decryption result: {} (test_caesar_encryption_and_decryption)",
            decryption_result
        );

        assert_eq!(target_original, decryption_result);
    }

    // Test Caesar key check mechanism on positive numbers.
    #[test]
    fn test_caesar_key_check_positive() {
        let key = "18903427";

        assert!(check_caesar_key(&key));
    }

    // Test Caesar key check mechanism on negative numbers.
    #[test]
    fn test_caesar_key_check_negative() {
        let key = "-18903427";

        assert!(check_caesar_key(&key));
    }

    // Test Caesar key check mechanism on incorrect input.
    #[test]
    fn test_caesar_key_check_incorrect() {
        let key = "-ThisIsAKeyForCaesarCipher";

        assert!(!check_caesar_key(&key));
    }

    // Test Caesar character encryption.
    #[test]
    fn test_caesar_character_encryption() {
        let key = 120;
        // Character 'A' has a decimal value of 65.
        let char_original = 'A' as u8;
        let mut char = char_original;

        // Encrypt the char.
        caesar_encrypt_char(&mut char, &key);

        println!(
            "  Target for Caesar character encryption: {} (test_caesar_character_encryption)",
            char_original
        );
        println!(
            "  Caesar character encryption result: {} (test_caesar_character_encryption)",
            char
        );

        assert_ne!(char_original, char);
        assert_eq!(char_original + key, char);
    }

    // Test Caesar character decryption.
    #[test]
    fn test_caesar_character_decryption() {
        let key = 120;
        // Character 'A' has a decimal value of 65.
        let char_original = 'A' as u8;
        let mut char = char_original;

        // Encrypt the char.
        caesar_decrypt_char(&mut char, &key);

        println!(
            "  Target for Caesar character decryption: {} (test_caesar_character_decryption)",
            char_original
        );
        println!(
            "  Caesar character decryption result: {} (test_caesar_character_decryption)",
            char
        );

        // Calculate decryption result manually for comparison.
        let temp_char = char_original as i16;
        let temp_key = key as i16;
        let check_result: i16 = temp_char - temp_key + 256;
        let check_result = check_result as u8;

        assert_ne!(char_original, char);
        assert_eq!(check_result, char);
    }
}
