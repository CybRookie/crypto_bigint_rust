use std::fs;
use std::io::Write;

// Print out calculation result into the console.
pub fn print_calculation_result(
    handle: &mut impl Write,
    result: &str,
) -> Result<(), std::io::Error> {
    writeln!(handle, "The result of the calculations:")?;
    writeln!(handle, "{}", result)?;

    // Print out buffer.
    handle.flush()?;

    Ok(())
}

// Save calculation result into the file.
pub fn save_calculation_result(result: &str) -> Result<(), std::io::Error> {
    fs::write("calculation_result.txt", result)?;
    println!("Successfully saved the result of the calculations into \"calculation_result.txt\" file at the location of the program.");
    Ok(())
}

// A function to print out help message to the console.
pub fn print_help(handle: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(
        handle,
        "A command line tool to encrypt/decrypt strings with Caesar or Vigenere ciphers."
    )?;
    writeln!(handle, "Usage pattern: se(.exe) <cipher type> <encryption mode> <output mode> <plaintext or ciphertext> <key>")?;
    writeln!(
        handle,
        "Note: you can use this tool with 'cargo run' instead of tool's binary 'se(.exe)'"
    )?;
    writeln!(handle)?;
    writeln!(handle, "Possible values for the listed arguments:")?;
    writeln!(handle, "    - cipher type: caesar/vigenere,")?;
    writeln!(handle, "    - encryption mode: encrypt/decrypt,")?;
    writeln!(handle, "    - output mode: console/file/both,")?;
    writeln!(
        handle,
        "    - plaintext or ciphertext: \"your text/string/phrase to encrypt or decrypt,\""
    )?;
    writeln!(
        handle,
        "    - key: \"your key to use for encryption or decryption,\""
    )?;
    writeln!(handle)?;
    writeln!(handle, "Notice:")?;
    writeln!(
        handle,
        "    - This tool outputs encrypted information in a hexadecimal encoding format."
    )?;
    writeln!(handle, "    - This tool only accepts ciphertexts for decryption, previousle encrypted with this tool in hex format.")?;
    writeln!(handle, "    - If the \"file\" or \"both \" output mode were chosen then the ciphertext will be saved in the file, named \"ciphertext.txt\".")?;
    writeln!(handle, "    - If the \"file\" or \"both \" output mode were chosen then the ciphertext will be saved in the same location of the tool.")?;
    writeln!(handle, "    - Caesar mode encryption and decryption accept only whole numbers as a key (both positive and negative).")?;
    writeln!(
        handle,
        "    - Vigenere mode encryption and decryption accept any string as a key."
    )?;
    writeln!(handle)?;
    writeln!(handle, "Examples of usage:")?;
    writeln!(
        handle,
        "    - To encrypt a string in Caesar cipher and output the result into the console:"
    )?;
    writeln!(
        handle,
        "    se(.exe) caesar encrypt console ThisIsAMessageToEncrypt 100"
    )?;
    writeln!(
        handle,
        "    - To decrypt a string in Vigenere cipher and output the result into the file:"
    )?;
    writeln!(
        handle,
        "    se(.exe) vigenere decrypt file ThisIsAMessageToDecryptInHEX ThisIsTheUsedKey"
    )?;
    writeln!(handle)?;
    writeln!(
        handle,
        "To trigger this help message pass \"help\" argument:"
    )?;
    writeln!(handle, "    - se(.exe) help")?;
    writeln!(handle)?;

    // Print out buffer.
    handle.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::str::from_utf8_unchecked;
    use std::{fs, io};

    use crate::logic::output::{print_calculation_result, print_help, save_calculation_result};

    // Test the function that outputs a computed result ot the console.
    #[test]
    fn test_calculation_result_console_output() {
        let test_result = "EncryptedOrDecryptedText";
        let mut handle = io::BufWriter::new(Vec::new());

        // Panic if an error was encountered during output of a message to the console.
        if let Err(e) = print_calculation_result(&mut handle, test_result) {
            panic!(
                "Failed to output the result message: {}. (test_calculation_result_console_output)",
                e
            );
        }

        let result_message_ref = handle.get_ref();
        let result_message = unsafe { from_utf8_unchecked(result_message_ref) };

        assert!(
            result_message.contains("The result of the calculations:\nEncryptedOrDecryptedText")
        );
    }

    // Test the function that ouputs a computed result to the file.
    #[test]
    fn test_calculation_result_file_output() {
        let test_result = "EncryptedOrDecryptedText";

        // Panic if an error was encountered during output of a message the file.
        if let Err(e) = save_calculation_result(test_result) {
            panic!(
                "Failed to output the result message: {}. (test_calculation_result_file_output)",
                e
            );
        }

        let file_contents;

        // Read the contents of the produced file, panic if the procedure fails.
        match fs::read_to_string("calculation_result.txt") {
            Ok(contents) => file_contents = contents,
            Err(e) => panic!("Failed to read the contents of the file: {}. (test_calculation_result_file_output)", e),
        }

        // Delete the produced file, panic if the procedure fails.
        if let Err(e) = fs::remove_file("calculation_result.txt") {
            panic!(
                "Failed to remove the produced file: {}. (test_calculation_result_file_output)",
                e
            );
        }

        assert_eq!(test_result, file_contents);
    }

    // Test the function that produces the help message.
    #[test]
    fn test_print_help() {
        let mut handle = io::BufWriter::new(Vec::new());

        // Panic if an error was encountered during creation of the print message.
        if let Err(e) = print_help(&mut handle) {
            panic!(
                "Failed to create the help message: {}. (test_print_help)",
                e
            );
        }

        // Panic if an error was encountered during flush of the print message.
        if let Err(e) = handle.flush() {
            panic!("Failed to flush the help message: {}. (test_print_help)", e);
        }

        let help_message_ref = handle.get_ref();
        let help_message = unsafe { from_utf8_unchecked(help_message_ref) };

        // Check inclusion of the first line.
        assert!(help_message.contains(
            "A command line tool to encrypt/decrypt strings with Caesar or Vigenere ciphers."
        ));
        // Check inclusion of some middle line.
        assert!(help_message.contains("    - If the \"file\" or \"both \" output mode were chosen then the ciphertext will be saved in the file, named \"ciphertext.txt\"."));
        // Check inclusion of the last line.
        assert!(help_message.contains("    - se(.exe) help"));
    }
}
