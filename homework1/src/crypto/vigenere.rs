use std::error::Error;
use std::str::from_utf8_unchecked;

use crate::crypto::caesar::*;
use crate::encoding::*;
use crate::logic::config::Mode;

pub fn vigenere(mode: &Mode, target: &mut str, key: &str) -> Result<String, Box<dyn Error>> {
    // Turn key string into vector of bytes.
    let key = key.as_bytes();
    let key_len = key.len();

    match mode {
        Mode::Encode => {
            // Convert string to the vector of unsigned one byte integers.
            let target = unsafe { target.as_bytes_mut() };

            // Iterator over key.
            let mut i = 0;

            // Encrypt vector of bytes one by one.
            for char in target.iter_mut() {
                caesar_encrypt_char(char, &key[i]);
                i = (i + 1) % key_len;
            }

            // Encode the vector of bytes into the hex string.
            string_hex_encode(target)
        }
        Mode::Decode => {
            // Convert received hex string into the vector of encrypted one bytes.
            let mut decoded_string = string_hex_decode(target)?;

            // Iterator over key.
            let mut i = 0;

            // Decrypt vector of bytes one by one.
            for char in decoded_string.iter_mut() {
                caesar_decrypt_char(char, &key[i]);
                i = (i + 1) % key_len;
            }

            let result = unsafe { from_utf8_unchecked(&decoded_string) };

            Ok(String::from(result))
        }
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::crypto::vigenere::vigenere;
    use crate::logic::config::Mode;

    // Test Vigenere encryption.
    #[test]
    fn test_vigenere_encryption() {
        let encryption_mode = Mode::Encode;
        let target_original = "TargetText";
        let mut target = String::from(target_original);
        let key = "!АбвгдKey_123";

        // Encrypt target string.
        let encryption_result = vigenere(&encryption_mode, &mut target, key).unwrap();

        println!(
            "  Target for Vigenere encryption: {} (test_vigenere_encryption)",
            target_original
        );
        println!(
            "  Vigenere encryption result: {} (test_vigenere_encryption)",
            encryption_result
        );

        assert_ne!(target_original, encryption_result);
    }

    // Test Vigenere encryption and decryption.
    #[test]
    fn test_vigenere_encryption_and_decryption() {
        let encryption_mode = Mode::Encode;
        let decryption_mode = Mode::Decode;
        let target_original = "TargetText";
        let mut target = String::from(target_original);
        let key = "!АбвгдKey_123";

        // Encrypt target string.
        let encryption_result = vigenere(&encryption_mode, &mut target, &key).unwrap();

        let mut encryption_result_clone = encryption_result.clone();

        // Decrypt target string.
        let decryption_result =
            vigenere(&decryption_mode, &mut encryption_result_clone, &key).unwrap();

        println!(
            "  Target for Vigenere encryption: {} (test_vigenere_encryption_and_decryption)",
            target_original
        );
        println!(
            "  Vigenere encryption result: {} (test_vigenere_encryption_and_decryption)",
            encryption_result
        );
        println!(
            "  Vigenere decryption result: {} (test_vigenere_encryption_and_decryption)",
            decryption_result
        );

        assert_eq!(target_original, decryption_result);
    }
}
