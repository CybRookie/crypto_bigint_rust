use std::error::Error;

use crate::logic::error::OperationError;

// Encode a half of the received byte and return character.
fn encode_part_of_byte(int_half: u8) -> Result<char, Box<dyn Error>> {
    match int_half {
        10..=15 => Ok((int_half + 55) as char),
        0..=9 => Ok((int_half + 48) as char),
        _ => Err(Box::new(OperationError::new("An error was encountered during ciphertext encoding. Try to encrypt/decrypt again. (one_u8_to_hex)"))),
    }
}

// Match 1 byte value to hexadecimal representation.
fn one_u8_to_hex(int: u8) -> Result<Vec<char>, Box<dyn Error>> {
    // Create two variables representing two halves of the byte.
    let int_left_half = int >> 4;
    let int_right_half = (int << 4) >> 4;
    let result = vec![
        // Encode left half of the byte and push the result to the vector of characters.
        encode_part_of_byte(int_left_half)?,
        // Encode right half of the byte and push the result to the vector of characters.
        encode_part_of_byte(int_right_half)?,
    ];

    Ok(result)
}

// Transform string consisting of ciphertext bytes into the hexadecimal string.
pub fn string_hex_encode(string: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut result = Vec::new();

    // Convert every character from the ciphertext into its hexadecimal representation.
    for char in string {
        let mut hex_pair = one_u8_to_hex(*char)?;
        result.append(&mut hex_pair);
    }

    Ok(result.into_iter().collect())
}

// Match hex value to the 1 byte decimal representation.
fn one_hex_to_u8(hex: u8) -> Result<u8, Box<dyn Error>> {
    match hex {
        b'A'..=b'F' => Ok(hex - b'A' + 10),
        b'a'..=b'f' => Ok(hex - b'a' + 10),
        b'0'..=b'9' => Ok(hex - b'0'),
        _ => Err(Box::new(OperationError::new("Received incorrect ciphertext in hexadecimal format for processing, only texts consisting of A-F, a-f and 0-9 values are accepted."))),
    }
}

// Transform string consisting of hex symbols into the vector of decimal integers of one byte.
pub fn string_hex_decode(hex_string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // Check if the received string has an even length.
    if hex_string.chars().count() % 2 != 0 {
        return Err(Box::new(OperationError::new("Received ciphertext in hexadecimal with odd amount for characters, only texts with even amount are accepted.")));
    }

    // Transform hex string into a vector of one byte values.
    let mut decoded_cipher: Vec<u8> = Vec::new();
    let ref_cipher: &[u8] = hex_string.as_ref();

    // Check if the hex string contains allowed values, A-F, a-f and 0-9.
    let int_iter = ref_cipher.iter();
    for char in int_iter {
        match char {
            b'A'..=b'F' => continue,
            b'a'..=b'f' => continue,
            b'0'..=b'9' => continue,
            _ => return Err(Box::new(OperationError::new("Received incorrect ciphertext in hexadecimal format for processing, only texts consisting of A-F, a-f and 0-9 values are accepted."))),
        }
    }

    // Split hex string into the chunks of 2 hex characters and translate them into the decimal representation.
    // First hex symbol is translated and then bits are shifted to the left by 4 bits.
    // Translation of the second hex symbol is added to the first with the bitwise OR.
    let char_vector = ref_cipher.chunks(2).enumerate().map(|(_index, hex_pair)| {
        one_hex_to_u8(hex_pair[0]).unwrap() << 4 | one_hex_to_u8(hex_pair[1]).unwrap()
    });
    for character in char_vector {
        decoded_cipher.push(character as u8);
    }

    Ok(decoded_cipher)
}

// Test module.
#[cfg(test)]
mod tests {
    use std::str::from_utf8_unchecked;

    use crate::encoding::{
        encode_part_of_byte, one_hex_to_u8, one_u8_to_hex, string_hex_decode, string_hex_encode,
    };

    // Test encoding of a half of the received byte and return character,
    // while this half's value is in the range of 0-9.
    #[test]
    fn test_encoding_small_bytes_half() {
        // Binary representation should be '00000100', what equals to decimal '4'.
        let test_int = 4;

        // Considering that the passed value is 4, according to the algorithm,
        // it should increase by 48, the result should be 52.
        let encoding_result = encode_part_of_byte(test_int).unwrap() as u8;

        assert_eq!(encoding_result, 52);
    }

    // Test encoding of a half of the received byte and return character,
    // while this half's value is in the range of 10-15.
    #[test]
    fn test_encoding_big_bytes_half() {
        // Binary representation should be '00001100', what equals to decimal '12'.
        let test_int = 12;

        // Considering that the passed value is 12, according to the algorithm,
        // it should increase by 55, the result should be 67.
        let encoding_result = encode_part_of_byte(test_int).unwrap() as u8;

        assert_eq!(encoding_result, 67);
    }

    // Test encoding of a half of the received byte and return character,
    // while this half's value is is out of suitable ranges of 0-9 and 10-15.
    #[test]
    #[should_panic]
    fn test_encoding_incorrect_bytes_half() {
        // Binary representation should be '01100100', what equals to decimal '100'.
        let test_int = 100;

        // Considering that the passed value is 100, according to the algorithm,
        // an error should be returned as the result.
        if let Err(e) = encode_part_of_byte(test_int) {
            panic!("{}", e);
        }
    }

    // Test encoding of a half of a byte to its hexadecimal representation.
    #[test]
    fn test_one_u8_to_hex() {
        // Binary representation should be '01100100', what equals to decimal '100'.
        let test_int = 100;

        // Encode the whole byte in to its hexadecimal representation.
        let encoding_result = match one_u8_to_hex(test_int) {
            Ok(result) => result,
            Err(e) => panic!("Failed to encode a byte into its hexadecimal representation: {} (test_one_u8_to_hex)", e),
        };

        println!(
            "  Target byte for hexadecimal encoding: {} (test_one_u8_to_hex)",
            test_int
        );
        println!(
            "  Result of the byte to hexadecimal encoding: {:?} (test_one_u8_to_hex)",
            encoding_result
        );

        // Decimal 100 has a value of 64 in hexadecimal.
        let comparison_vec = vec!['6', '4'];

        assert_eq!(encoding_result, comparison_vec);
    }

    // Test encoding of a whole string (in form of vector of bytes) to its hexadecimal representation.
    #[test]
    fn test_string_hex_encoding() {
        let mut test_string = String::from("ThisIsATestString");
        // Convert string to the vector of unsigned one byte integers.
        let target = unsafe { test_string.as_bytes_mut() };

        let encoding_result = match string_hex_encode(target) {
            Ok(result) => result,
            Err(e) => panic!("Failed to encode a string into its hexadecimal representation: {} (test_string_hex_encoding)", e),
        };

        println!(
            "  Target string for hexadecimal encoding: {} (test_string_hex_encoding)",
            test_string
        );
        println!(
            "  Result of the string to hexadecimal encoding: {:?} (test_string_hex_encoding)",
            encoding_result
        );

        // Hexadecimal representation of the target string.
        let comparison_encoded_hex_string = "5468697349734154657374537472696E67";

        assert_ne!(test_string, encoding_result);
        assert_eq!(encoding_result, comparison_encoded_hex_string);
    }

    // Test decoding a byte from its capital letter hexadecimal representation.
    #[test]
    fn test_one_capital_letter_hex_to_u8() {
        let test_char = 'A' as u8;

        let decoding_result = match one_hex_to_u8(test_char) {
            Ok(result) => result,
            Err(e) => panic!("Failed to decode a byte from its capital letter hexadecimal representation: {} (test_one_capital_letter_hex_to_u8)", e),
        };

        println!("  Target char from capital letter hexadecimal decoding: {} (test_one_capital_letter_hex_to_u8)", test_char);
        println!("  Result of the char from capital letter hexadecimal decoding: {:?} (test_one_capital_letter_hex_to_u8)", decoding_result);

        // Hexadecimal 'A' has a value of 10 in decimal.
        let comparison_int = 10;

        assert_eq!(comparison_int, decoding_result);
    }

    // Test decoding a byte from its minuscule letter hexadecimal representation.
    #[test]
    fn test_one_minuscule_letter_hex_to_u8() {
        let test_char = 'a' as u8;

        let decoding_result = match one_hex_to_u8(test_char) {
            Ok(result) => result,
            Err(e) => panic!("Failed to decode a byte from its minuscule letter hexadecimal representation: {} (test_one_minuscule_letter_hex_to_u8)", e),
        };

        println!("  Target char from minuscule letter hexadecimal decoding: {} (test_one_minuscule_letter_hex_to_u8)", test_char);
        println!("  Result of the char from  minuscule letter hexadecimal decoding: {:?} (test_one_minuscule_letter_hex_to_u8)", decoding_result);

        // Hexadecimal 'a' has a value of 10 in decimal.
        let comparison_int = 10;

        assert_eq!(comparison_int, decoding_result);
    }

    // Test decoding a byte from its incorrect hexadecimal representation.
    #[test]
    #[should_panic]
    fn test_one_incorrect_hex_to_u8() {
        let test_char = 'X' as u8;

        // Considering that allowed character are: A-F, a-f and 0-9;
        // an error should be returned as the result.
        if let Err(e) = one_hex_to_u8(test_char) {
            panic!("{}", e)
        }
    }

    // Test decoding of a whole string from its hexadecimal representation.
    #[test]
    fn test_string_hex_decoding() {
        // This hexadecimal string should be decoded into the "ThisIsATestString" string.
        let test_string = "5468697349734154657374537472696E67";

        let decoding_result = match string_hex_decode(test_string) {
            Ok(result) => result,
            Err(e) => panic!("Failed to decode a string from its hexadecimal representation: {} (test_string_hex_decoding)", e),
        };

        println!(
            "  Target string for hexadecimal decoding: {} (test_string_hex_decoding)",
            test_string
        );
        println!(
            "  Result of the string from hexadecimal decoding: {:?} (test_string_hex_decoding)",
            decoding_result
        );

        // UTF-8 representation of the target string.
        let comparison_decoded_hex_string = "ThisIsATestString";
        let decoding_result_string = unsafe { from_utf8_unchecked(&decoding_result) };

        assert_ne!(test_string, decoding_result_string);
        assert_eq!(decoding_result_string, comparison_decoded_hex_string);
    }
}
