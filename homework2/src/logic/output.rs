use std::fs;
use std::io::{BufWriter, Write};

use crate::crypto::diffie_hellman::DiffieHellmanResult;
use crate::crypto::rsa::RsaResult;

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

// Print out calculation result for the Diffie-Hellman calculations into the console.
pub fn print_df_calculation_result(
    handle: &mut impl Write,
    df_result: &DiffieHellmanResult,
) -> Result<(), std::io::Error> {
    writeln!(handle, "The result of the Diffie-Hellman calculations:")?;
    writeln!(handle, "Shared prime: {}", df_result.shared_prime)?;
    writeln!(handle, "Shared base: {}", df_result.shared_base)?;
    writeln!(handle, "Secret A: {}", df_result.secret_a)?;
    writeln!(handle, "Secret B: {}", df_result.secret_b)?;
    writeln!(handle, "Package from A to B: {}", df_result.package_from_a_to_b)?;
    writeln!(handle, "Package from B to A: {}", df_result.package_from_b_to_a)?;
    writeln!(handle, "Result A: {}", df_result.result_a)?;
    writeln!(handle, "Result B: {}", df_result.result_b)?;
    writeln!(handle, "Was the operation successful?: {}", df_result.success)?;

    // Print out buffer.
    handle.flush()?;

    Ok(())
}

// Print out calculation result for the Diffie-Hellman calculations into the console.
pub fn print_rsa_calculation_result(
    handle: &mut impl Write,
    rsa_result: &RsaResult,
) -> Result<(), std::io::Error> {

    // Generate an appropriate output, according to the type of RSA result.
    match rsa_result {
        RsaResult::KeyPair(generate_key_pair) => {
            writeln!(handle, "The result of the RSA key pair generation:")?;
            writeln!(handle, "Key modulus n: {}", generate_key_pair.public_key_n)?;
            writeln!(handle, "Public key exponent e: {}", generate_key_pair.public_key_e)?;
            writeln!(handle, "Private key exponent d: {}", generate_key_pair.private_key_d)?;
        }
        RsaResult::StringResult(string_result) => {
            writeln!(handle, "The result of the RSA encryption/decryption calculations:")?;
            writeln!(handle, "Encryption/decryption result: {}", *string_result)?;
        }
        RsaResult::BruteforceRSAResult(bruteforce_result) => {
            writeln!(handle, "The result of the RSA bruteforce calculations:")?;
            writeln!(handle, "Prime q: {}", bruteforce_result.prime_q)?;
            writeln!(handle, "Prime p: {}", bruteforce_result.prime_p)?;
            writeln!(handle, "Key modulus n: {}", bruteforce_result.public_key_n)?;
            writeln!(handle, "Public key exponent e: {}", bruteforce_result.public_key_e)?;
            writeln!(handle, "Private key exponent d: {}", bruteforce_result.private_key_d)?;
        }
    }

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

// A function that consumes the file handle and by dropping it, closes it.
fn close_file(_file_handle: fs::File) {}

// Save calculation result for the Diffie-Hellman calculations into the console.
pub fn save_df_calculation_result(df_result: &DiffieHellmanResult) -> Result<(), std::io::Error> {
    // Create a file or truncate it, write first line.
    let mut file = fs::File::create("calculation_result.txt")?;
    file.write_fmt(format_args!("The result of the Diffie-Hellman calculations:\n"))?;
    close_file(file);

    // Reopen file in appending mode and buffer the handle, after the lines are complete, flush it.
    let mut file_buffer = BufWriter::new(fs::OpenOptions::new().append(true).open("calculation_result.txt")?);
    file_buffer.write_fmt(format_args!("Shared prime: {}\n", df_result.shared_prime))?;
    file_buffer.write_fmt(format_args!("Shared base: {}\n", df_result.shared_base))?;
    file_buffer.write_fmt(format_args!("Secret A: {}\n", df_result.secret_a))?;
    file_buffer.write_fmt(format_args!("Secret B: {}\n", df_result.secret_b))?;
    file_buffer.write_fmt(format_args!("Package from A to B: {}\n", df_result.package_from_a_to_b))?;
    file_buffer.write_fmt(format_args!("Package from B to A: {}\n", df_result.package_from_b_to_a))?;
    file_buffer.write_fmt(format_args!("Result A: {}\n", df_result.result_a))?;
    file_buffer.write_fmt(format_args!("Result B: {}\n", df_result.result_b))?;
    file_buffer.write_fmt(format_args!("Was the operation successful?: {}\n", df_result.success))?;
    file_buffer.flush()?;

    println!("Successfully saved the result of the Diffie-Hellman calculations into \"calculation_result.txt\" file at the location of the program.");

    Ok(())
}

// Save calculation result for the RSA calculations into the console.
pub fn save_rsa_calculation_result(rsa_result: &RsaResult) -> Result<(), std::io::Error> {
    // Create a file or truncate it, write first line.
    let mut file = fs::File::create("calculation_result.txt")?;
    file.write_fmt(format_args!("The result of the RSA calculations.\n"))?;
    close_file(file);

    // Reopen file in appending mode and buffer the handle, after the lines are complete, flush it.
    let mut file_buffer = BufWriter::new(fs::OpenOptions::new().append(true).open("calculation_result.txt")?);

    // Generate an appropriate output, according to the type of RSA result.
    match rsa_result {
        RsaResult::KeyPair(generate_key_pair) => {
            file_buffer.write_fmt(format_args!("The result of the RSA key pair generation:\n"))?;
            file_buffer.write_fmt(format_args!("Key modulus n: {}\n", generate_key_pair.public_key_n))?;
            file_buffer.write_fmt(format_args!("Public key exponent e: {}\n", generate_key_pair.public_key_e))?;
            file_buffer.write_fmt(format_args!("Private key exponent d: {}\n", generate_key_pair.private_key_d))?;
        }
        RsaResult::StringResult(string_result) => {
            file_buffer.write_fmt(format_args!("The result of the RSA encryption/decryption calculations:\n"))?;
            file_buffer.write_fmt(format_args!("Encryption/decryption result: {}\n", *string_result))?;
        }
        RsaResult::BruteforceRSAResult(bruteforce_result) => {
            file_buffer.write_fmt(format_args!("The result of the RSA bruteforce calculations:\n"))?;
            file_buffer.write_fmt(format_args!("Prime q: {}\n", bruteforce_result.prime_q))?;
            file_buffer.write_fmt(format_args!("Prime p: {}\n", bruteforce_result.prime_p))?;
            file_buffer.write_fmt(format_args!("Key modulus n: {}\n", bruteforce_result.public_key_n))?;
            file_buffer.write_fmt(format_args!("Public key exponent e: {}\n", bruteforce_result.public_key_e))?;
            file_buffer.write_fmt(format_args!("Private key exponent d: {}\n", bruteforce_result.private_key_d))?;
        }
    }

    file_buffer.flush()?;

    println!("Successfully saved the result of the RSA calculations into \"calculation_result.txt\" file at the location of the program.");

    Ok(())
}

// A function to print out help message to the console.
pub fn print_help(handle: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(handle, "A command line tool to encrypt/decrypt strings with Caesar, Vigenere or RSA ciphers, or establish a key exchange with Diffie-Hellman algorithm.")?;
    writeln!(handle, "Usage pattern:")?;
    writeln!(handle, "    - For symmetric encryption with Caesar or Vigenere: enc(.exe) <cipher type> <encryption mode> <output mode> <plaintext or ciphertext> <key>")?;
    writeln!(handle, "    - For Diffie-Hellman algorithm: enc(.exe) <cipher type> generate <output mode> <none or shared prime> <none or shared base> <none or secret A> <none or secret B>")?;
    writeln!(handle, "    - For RSA encryption/decryption: enc(.exe) <cipher type> <encryption mode> <output mode> <plaintext or ciphertext> <public or private exponent> <public modulus>")?;
    writeln!(handle, "    - For RSA key pair generation: enc(.exe) <cipher type> generate <output mode>")?;
    writeln!(handle, "    - For RSA public key bruteforcing: enc(.exe) <cipher type> generate <output mode> <public or private exponent> <public modulus> <empty or a custom amount of threads>")?;
    writeln!(handle, "Note: you can use this tool with \"cargo run\" instead of tool's binary \"enc(.exe)\"")?;
    writeln!(handle)?;
    writeln!(handle, "Possible values for the listed arguments:")?;
    writeln!(handle, "    - cipher type: caesar/vigenere/rsa/df,")?;
    writeln!(handle, "    - encryption mode: encrypt/decrypt/generate/bruteforce,")?;
    writeln!(handle, "    - output mode: console/file/both,")?;
    writeln!(handle, "    - plaintext or ciphertext: \"your text/string/phrase to encrypt or decrypt\",")?;
    writeln!(handle, "    - key: \"your key to use for encryption or decryption\",")?;
    writeln!(handle, "    - shared prime/shared base/secret A/secret B: \"your appropriate numeric values for Diffie-Hellman algorithm or \"none\" to generate a random value\",")?;
    writeln!(handle, "    - public or private exponent/public modulus: \"your appropriate numeric values for RSA cipher\",")?;
    writeln!(handle, "    - amount of threads: \"your custom of number of threads, values from 1 to 64 are allowed, or \"none\", or leave it empty to use a default value of 8\",")?;
    writeln!(handle)?;
    writeln!(handle, "Notice:")?;
    writeln!(handle, "    - This tool outputs encrypted information in a hexadecimal encoding format.")?;
    writeln!(handle, "    - This tool only accepts ciphertexts for decryption, previously encrypted with this tool in hex format.")?;
    writeln!(handle, "    - If the \"file\" or \"both \" output mode were chosen then the ciphertext will be saved in the file, named \"ciphertext.txt\".")?;
    writeln!(handle, "    - If the \"file\" or \"both \" output mode were chosen then the ciphertext will be saved in the same location of the tool.")?;
    writeln!(handle, "    - Caesar mode encryption and decryption accept only whole numbers as a key (both positive and negative).")?;
    writeln!(handle, "    - Vigenere mode encryption and decryption accept any string as a key.")?;
    writeln!(handle, "    - If you use \"cargo run\" command to run the program, it is advised to run int with \"--release\" parameter to speed up calculations.")?;
    writeln!(handle)?;
    writeln!(handle, "Examples of usage:")?;
    writeln!(handle, "    - To encrypt a string in Caesar cipher and output the result into the console:")?;
    writeln!(handle, "    enc(.exe) caesar encrypt console ThisIsAMessageToEncrypt 100")?;
    writeln!(handle, "    - To decrypt a string in Vigenere cipher and output the result into the file:")?;
    writeln!(handle, "    enc(.exe) vigenere decrypt file ThisIsAMessageToDecryptInHEX ThisIsTheUsedKey")?;
    writeln!(handle, "    - To generate Diffie-Hellman values:")?;
    writeln!(handle, "    enc(.exe) df generate file")?;
    writeln!(handle, "    enc(.exe) df generate console none none none none")?;
    writeln!(handle, "    enc(.exe) df generate console none 123 none 12345")?;
    writeln!(handle, "    - To encrypt with RSA cipher:")?;
    writeln!(handle, "    enc(.exe) rsa encrypt console \"Target string!\" 12 19784619")?;
    writeln!(handle, "    - To generate an RSA key pair:")?;
    writeln!(handle, "    enc(.exe) rsa generate console")?;
    writeln!(handle, "    - To bruteforce a public RSA key:")?;
    writeln!(handle, "    enc(.exe) rsa bruteforce both 12 19784619")?;
    writeln!(handle, "    enc(.exe) rsa bruteforce both 12 19784619 32")?;
    writeln!(handle)?;
    writeln!(handle, "To trigger this help message pass \"help\" argument:")?;
    writeln!(handle, "    - enc(.exe) help")?;
    writeln!(handle)?;

    // Print out buffer.
    handle.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, io};
    use std::io::Write;
    use std::str::from_utf8_unchecked;

    // Add a crate to run specific test sequentially, e.g. to run file modifying test in a sequence.
    use serial_test::serial;

    use crate::crypto::diffie_hellman::DiffieHellmanResult;
    use crate::crypto::rsa::{BruteforceResult, RsaKeyPair, RsaResult};
    use crate::logic::bigint::ChonkerInt;
    use crate::logic::output::{print_calculation_result, print_df_calculation_result, print_help, print_rsa_calculation_result, save_calculation_result, save_df_calculation_result, save_rsa_calculation_result};

    // Test the function that outputs a computed result of the symmetric ciphers to the console.
    #[test]
    fn test_symmetric_calculation_result_console_output() {
        let test_result = "EncryptedOrDecryptedText";
        let mut handle = io::BufWriter::new(Vec::new());

        // Panic if an error was encountered during output of a message to the console.
        if let Err(e) = print_calculation_result(&mut handle, test_result) {
            panic!(
                "Failed to output the result message from a symmetric cipher: {}. (test_symmetric_calculation_result_console_output)",
                e
            );
        }

        let result_message_ref = handle.get_ref();
        let result_message = unsafe { from_utf8_unchecked(result_message_ref) };

        assert!(
            result_message.contains("The result of the calculations:\nEncryptedOrDecryptedText")
        );
    }

    // Test the function that ouputs a computed result of the symmetric ciphers to the file.
    #[test]
    #[serial]
    fn test_symmetric_calculation_result_file_output() {
        let test_result = "EncryptedOrDecryptedText";

        // Panic if an error was encountered during output of a message the file.
        if let Err(e) = save_calculation_result(test_result) {
            panic!(
                "Failed to save the result message from a symmetric cipher: {}. (test_symmetric_calculation_result_file_output)",
                e
            );
        }

        let file_contents;

        // Read the contents of the produced file, panic if the procedure fails.
        match fs::read_to_string("calculation_result.txt") {
            Ok(contents) => file_contents = contents,
            Err(e) => panic!("Failed to read the contents of the file: {}. (test_symmetric_calculation_result_file_output)", e),
        }

        // Delete the produced file, panic if the procedure fails.
        if let Err(e) = fs::remove_file("calculation_result.txt") {
            panic!(
                "Failed to remove the produced file: {}. (test_symmetric_calculation_result_file_output)",
                e
            );
        }

        assert_eq!(test_result, file_contents);
    }

    // Test the function that outputs a computed result of the DF algorithm to the console.
    #[test]
    fn test_df_calculation_result_console_output() {
        let test_result = DiffieHellmanResult {
            shared_prime: ChonkerInt::from(String::from("6331500943")),
            shared_base: ChonkerInt::from(String::from("33530")),
            secret_a: ChonkerInt::from(String::from("614842067695921615776914492033052920130845385386020346228402639809162015232875540829522077340269791144539698780492499495612027557332283006609256641139780780317628663030773355
        6411597370715491090659980583649982643141463339114832003654176356893821022100118556316754857822235268961392684184951595378194268980784826601317860609544840475618026842771887550684169201
        1644933575088826425638789591936961155246037386969900811911695403303334029607933251239069356595493592306160680788239826728901265888023256841658912969785025931515473724612482821378578162
        6398647391653256112049832720877186174954533762080430369117225005619358022480719027080148986072083977903949635634616489062552457425774431519704408906516330132075107904956833837218948456
        93967")),
            secret_b: ChonkerInt::from(String::from("664548053793703207938312794234108351013217917927678584828415585495428831992013519090586806994255428025757839534990428481538563524224539429347138403809762604829791969095934599
        1007670982655405500444189781890510497317408832317567369057307952536188706306476488638827376051063573446843489233301063473545853100336638868150560620248347061689877494846744722359155428
        8504955565307490502191130494183049681247188175596558888096865558369731708332394219910593183873003108992317274454127848975223591769450541802244727768976679416970173181490068832726061546
        8582369984940245142332861319142161325034900101788309903947558216162780714077240019939769179574330406529607962254553152630303736908762532369516851693378879094120312722680132559767798388
        7442249484192894544229036742084494862478127381827437040902917773290943965301803822990563639650577978808164560843883186781741455231333101029906597150022872357610579172598559678444362442
        1428432")),
            package_from_a_to_b: ChonkerInt::from(String::from("2985467518")),
            package_from_b_to_a: ChonkerInt::from(String::from("2620722778")),
            result_a: ChonkerInt::from(String::from("3828477390")),
            result_b: ChonkerInt::from(String::from("3828477390")),
            success: true,
        };
        let mut handle = io::BufWriter::new(Vec::new());

        // Panic if an error was encountered during output of a message to the console.
        if let Err(e) = print_df_calculation_result(&mut handle, &test_result) {
            panic!(
                "Failed to output the DF result message: {}. (test_df_calculation_result_console_output)",
                e
            );
        }

        let result_message_ref = handle.get_ref();
        let result_message = unsafe { from_utf8_unchecked(result_message_ref) };

        assert!(
            result_message.contains("The result of the Diffie-Hellman calculations:\nShared prime:")
        );
    }

    // Test the function that outputs a computed result of the DF algorithm to the file.
    #[test]
    #[serial]
    fn test_df_calculation_result_file_output() {
        let test_result = DiffieHellmanResult {
            shared_prime: ChonkerInt::from(String::from("6331500943")),
            shared_base: ChonkerInt::from(String::from("33530")),
            secret_a: ChonkerInt::from(String::from("614842067695921615776914492033052920130845385386020346228402639809162015232875540829522077340269791144539698780492499495612027557332283006609256641139780780317628663030773355
        6411597370715491090659980583649982643141463339114832003654176356893821022100118556316754857822235268961392684184951595378194268980784826601317860609544840475618026842771887550684169201
        1644933575088826425638789591936961155246037386969900811911695403303334029607933251239069356595493592306160680788239826728901265888023256841658912969785025931515473724612482821378578162
        6398647391653256112049832720877186174954533762080430369117225005619358022480719027080148986072083977903949635634616489062552457425774431519704408906516330132075107904956833837218948456
        93967")),
            secret_b: ChonkerInt::from(String::from("664548053793703207938312794234108351013217917927678584828415585495428831992013519090586806994255428025757839534990428481538563524224539429347138403809762604829791969095934599
        1007670982655405500444189781890510497317408832317567369057307952536188706306476488638827376051063573446843489233301063473545853100336638868150560620248347061689877494846744722359155428
        8504955565307490502191130494183049681247188175596558888096865558369731708332394219910593183873003108992317274454127848975223591769450541802244727768976679416970173181490068832726061546
        8582369984940245142332861319142161325034900101788309903947558216162780714077240019939769179574330406529607962254553152630303736908762532369516851693378879094120312722680132559767798388
        7442249484192894544229036742084494862478127381827437040902917773290943965301803822990563639650577978808164560843883186781741455231333101029906597150022872357610579172598559678444362442
        1428432")),
            package_from_a_to_b: ChonkerInt::from(String::from("2985467518")),
            package_from_b_to_a: ChonkerInt::from(String::from("2620722778")),
            result_a: ChonkerInt::from(String::from("3828477390")),
            result_b: ChonkerInt::from(String::from("3828477390")),
            success: true,
        };

        // Panic if an error was encountered during output of a message the file.
        if let Err(e) = save_df_calculation_result(&test_result) {
            panic!(
                "Failed to save the DF result message to a file: {}. (test_df_calculation_result_file_output)",
                e
            );
        }

        let file_contents;

        // Read the contents of the produced file, panic if the procedure fails.
        match fs::read_to_string("calculation_result.txt") {
            Ok(contents) => file_contents = contents,
            Err(e) => panic!("Failed to read the contents of the file: {}. (test_df_calculation_result_file_output)", e),
        }

        // Delete the produced file, panic if the procedure fails.
        if let Err(e) = fs::remove_file("calculation_result.txt") {
            panic!(
                "Failed to remove the produced file: {}. (test_df_calculation_result_file_output)",
                e
            );
        }

        assert!(
            file_contents.contains("The result of the Diffie-Hellman calculations:\nShared prime:")
        );
    }

    // Test the function that outputs a computed keypair result of the RSA algorithm to the console.
    #[test]
    fn test_rsa_calculation_keypair_result_console_output() {
        let test_result = RsaResult::KeyPair(RsaKeyPair {
            public_key_n: ChonkerInt::from(String::from("2877480840864488227166946252682128647397404497")),
            public_key_e: ChonkerInt::from(String::from("1")),
            private_key_d: ChonkerInt::from(String::from("1")),
        });
        let mut handle = io::BufWriter::new(Vec::new());

        // Panic if an error was encountered during output of a message to the console.
        if let Err(e) = print_rsa_calculation_result(&mut handle, &test_result) {
            panic!(
                "Failed to output the RSA result message: {}. (test_rsa_calculation_keypair_result_console_output)",
                e
            );
        }

        let result_message_ref = handle.get_ref();
        let result_message = unsafe { from_utf8_unchecked(result_message_ref) };

        assert!(
            result_message.contains("The result of the RSA key pair generation:\nKey modulus n:")
        );
    }

    // Test the function that outputs a computed keypair result of the RSA algorithm to the file.
    #[test]
    #[serial]
    fn test_rsa_calculation_keypair_result_file_output() {
        let test_result = RsaResult::KeyPair(RsaKeyPair {
            public_key_n: ChonkerInt::from(String::from("2877480840864488227166946252682128647397404497")),
            public_key_e: ChonkerInt::from(String::from("1")),
            private_key_d: ChonkerInt::from(String::from("1")),
        });

        // Panic if an error was encountered during output of a message the file.
        if let Err(e) = save_rsa_calculation_result(&test_result) {
            panic!(
                "Failed to save the DF result message to a file: {}. (test_rsa_calculation_keypair_result_file_output)",
                e
            );
        }

        let file_contents;

        // Read the contents of the produced file, panic if the procedure fails.
        match fs::read_to_string("calculation_result.txt") {
            Ok(contents) => file_contents = contents,
            Err(e) => panic!("Failed to read the contents of the file: {}. (test_rsa_calculation_keypair_result_file_output)", e),
        }

        // Delete the produced file, panic if the procedure fails.
        if let Err(e) = fs::remove_file("calculation_result.txt") {
            panic!(
                "Failed to remove the produced file: {}. (test_rsa_calculation_keypair_result_file_output)",
                e
            );
        }

        assert!(
            file_contents.contains("The result of the RSA key pair generation:\nKey modulus n:")
        );
    }

    // Test the function that outputs a computed string result of the RSA algorithm to the console.
    #[test]
    fn test_rsa_calculation_string_result_console_output() {
        let test_result = RsaResult::StringResult(String::from("Test string."));
        let mut handle = io::BufWriter::new(Vec::new());

        // Panic if an error was encountered during output of a message to the console.
        if let Err(e) = print_rsa_calculation_result(&mut handle, &test_result) {
            panic!(
                "Failed to output the RSA result message: {}. (test_rsa_calculation_string_result_console_output)",
                e
            );
        }

        let result_message_ref = handle.get_ref();
        let result_message = unsafe { from_utf8_unchecked(result_message_ref) };

        assert!(
            result_message.contains("The result of the RSA encryption/decryption calculations:\nEncryption/decryption result:")
        );
    }

    // Test the function that outputs a computed string result of the RSA algorithm to the file.
    #[test]
    #[serial]
    fn test_rsa_calculation_string_result_file_output() {
        let test_result = RsaResult::StringResult(String::from("Test string."));

        // Panic if an error was encountered during output of a message the file.
        if let Err(e) = save_rsa_calculation_result(&test_result) {
            panic!(
                "Failed to save the DF result message to a file: {}. (test_rsa_calculation_string_result_file_output)",
                e
            );
        }

        let file_contents;

        // Read the contents of the produced file, panic if the procedure fails.
        match fs::read_to_string("calculation_result.txt") {
            Ok(contents) => file_contents = contents,
            Err(e) => panic!("Failed to read the contents of the file: {}. (test_rsa_calculation_string_result_file_output)", e),
        }

        // Delete the produced file, panic if the procedure fails.
        if let Err(e) = fs::remove_file("calculation_result.txt") {
            panic!(
                "Failed to remove the produced file: {}. (test_rsa_calculation_string_result_file_output)",
                e
            );
        }

        assert!(
            file_contents.contains("The result of the RSA encryption/decryption calculations:\nEncryption/decryption result:")
        );
    }

    // Test the function that outputs a computed bruteforce result of the RSA algorithm to the console.
    #[test]
    fn test_rsa_calculation_bruteforce_result_console_output() {
        let test_result = RsaResult::BruteforceRSAResult(BruteforceResult {
            prime_q: ChonkerInt::from(String::from("24907")),
            prime_p: ChonkerInt::from(String::from("10799")),
            public_key_n: ChonkerInt::from(String::from("268970693")),
            public_key_e: ChonkerInt::from(String::from("85")),
            private_key_d: ChonkerInt::from(String::from("88590349")),
        });
        let mut handle = io::BufWriter::new(Vec::new());

        // Panic if an error was encountered during output of a message to the console.
        if let Err(e) = print_rsa_calculation_result(&mut handle, &test_result) {
            panic!(
                "Failed to output the RSA result message: {}. (test_rsa_calculation_string_result_console_output)",
                e
            );
        }

        let result_message_ref = handle.get_ref();
        let result_message = unsafe { from_utf8_unchecked(result_message_ref) };

        assert!(
            result_message.contains("The result of the RSA bruteforce calculations:\nPrime q:")
        );
    }

    // Test the function that outputs a computed bruteforce result of the RSA algorithm to the file.
    #[test]
    #[serial]
    fn test_rsa_calculation_bruteforce_result_file_output() {
        let test_result = RsaResult::BruteforceRSAResult(BruteforceResult {
            prime_q: ChonkerInt::from(String::from("24907")),
            prime_p: ChonkerInt::from(String::from("10799")),
            public_key_n: ChonkerInt::from(String::from("268970693")),
            public_key_e: ChonkerInt::from(String::from("85")),
            private_key_d: ChonkerInt::from(String::from("88590349")),
        });

        // Panic if an error was encountered during output of a message the file.
        if let Err(e) = save_rsa_calculation_result(&test_result) {
            panic!(
                "Failed to save the DF result message to a file: {}. (test_rsa_calculation_bruteforce_result_file_output)",
                e
            );
        }

        let file_contents;

        // Read the contents of the produced file, panic if the procedure fails.
        match fs::read_to_string("calculation_result.txt") {
            Ok(contents) => file_contents = contents,
            Err(e) => panic!("Failed to read the contents of the file: {}. (test_rsa_calculation_bruteforce_result_file_output)", e),
        }

        // Delete the produced file, panic if the procedure fails.
        if let Err(e) = fs::remove_file("calculation_result.txt") {
            panic!(
                "Failed to remove the produced file: {}. (test_rsa_calculation_bruteforce_result_file_output)",
                e
            );
        }

        assert!(
            file_contents.contains("The result of the RSA bruteforce calculations:\nPrime q:")
        );
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
            "A command line tool to encrypt/decrypt strings with Caesar, Vigenere or RSA ciphers, or establish a key exchange with Diffie-Hellman algorithm."
        ));
        // Check inclusion of some middle line.
        assert!(help_message.contains("    - If the \"file\" or \"both \" output mode were chosen then the ciphertext will be saved in the file, named \"ciphertext.txt\"."));
        // Check inclusion of the last line.
        assert!(help_message.contains("    - enc(.exe) help"));
    }
}
