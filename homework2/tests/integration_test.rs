// Crate with integration tests for the tool's logic.
// Tests will mimic "main" function's logic.

use enc::logic::config::ConfigVariant;
use enc::logic::error::OperationError;
use enc::logic::run;

// This function mimics "main" function's logic, but it also accepts test function's name for debugging purposes.
fn mains_alter_ego(args: impl Iterator<Item = String>, test_name: &str) {
    println!("  The logic was run by the test function: {}", test_name);
    // Create configuration for the encryption process, and handle possible errors.
    // Check for the custom error types to, in turn, check for the value of the flag for the help message.
    let config = match ConfigVariant::new(args) {
        Ok(config) => config,
        Err(e) => {
            match e.downcast::<OperationError>() {
                Ok(value_ref) => {
                    let value = *value_ref;
                    if value.get_help_flag() {
                        println!("{}", value);
                        panic!("Expected to create a correct configuration, produced a help message: {}", value);
                    }
                    eprintln!("Problem parsing arguments: {}", value);
                    panic!(
                        "Expected to create a correct configuration, encountered a custom error"
                    );
                }
                Err(e) => {
                    eprintln!("Problem parsing arguments: {}", e);
                    panic!("Expected to create a correct configuration, encountered an error");
                }
            };
        }
    };

    // Pass the retrieved configuration settings to the main tool logic and handle possible errors.
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        panic!("Expected successfully run the tool's logic, encountered an error")
    }
}

// Test logic for Caesar encryption with an output to the console, with correct arguments.
#[test]
fn test_caesar_encrypt_console() {
    let args = ["caesar", "encrypt", "console", "MammaMia", "123"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_caesar_encrypt_console");
}

// Test logic for Caesar decryption with an output to the console and a file, with correct arguments.
#[test]
fn test_caesar_decrypt_both() {
    let args = ["caesar", "decrypt", "both", "C8DCE8E8DCC8E4DC", "123"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_caesar_decrypt_both");
}

// Test logic for Vigenere encryption with an output to the file, with correct arguments.
#[test]
fn test_vigenere_encrypt_file() {
    let args = ["vigenere", "encrypt", "file", "🗝MammaMia", "🔑КрепкийКлюч"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_vigenere_encrypt_file");
}

// Test logic for Vigenere decryption with an output to the console, with correct arguments.
#[test]
fn test_vigenere_decrypt_console() {
    // Result of decryption should be 🗝MammaMia, from Vigenere encryption test.
    let args = [
        "vigenere",
        "decrypt",
        "console",
        "E03E2B2E1DFB3EED31023920",
        "🔑КрепкийКлюч",
    ]
    .iter()
    .map(|s| s.to_string());

    mains_alter_ego(args, "test_vigenere_decrypt_console");
}

// Test logic for Diffie-Hellman key exchange without additional parameters, with an output to the console, with correct arguments.
#[test]
fn test_df_generate_without_parameters_console() {
    let args = ["df", "generate", "console"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_df_generate_without_parameters_console");
}

// Test logic for Diffie-Hellman key exchange with additional parameters, with an output to the console, with correct arguments.
#[test]
fn test_df_generate_with_parameters_console() {
    let args = ["df", "generate", "console", "101", "none", "none", "12345"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_df_generate_with_parameters_console");
}

// Test logic for RSA encryption, with an output to the console, with correct arguments.
#[test]
fn test_rsa_encrypt_console() {
    let args = ["rsa", "encrypt", "console", "Test RSA target string!", "9683922000451682283955009414215846271", "503389953040597954843496152539898795547523683"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_rsa_encrypt_console");
}

// Test logic for RSA encryption, with an output to the console, with correct arguments.
#[test]
fn test_rsa_decrypt_console() {
    let args = ["rsa", "decrypt", "console", "060307010306050108040104060801030907090400010107080201070900080103060301040903090808020501FF030509070901020001000603030301040409000702000706090704050800090401010806080001010904070601", "239227093839837965545527797083977554955436111", "503389953040597954843496152539898795547523683"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_rsa_decrypt_console");
}

// Test logic for RSA key pair generation, with an output to the console, with correct arguments.
#[test]
fn test_rsa_generate_console() {
    let args = ["rsa", "generate", "console"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_rsa_generate_console");
}

// Test logic for RSA public key bruteforce without a custom thread count, with an output to the console, with correct arguments.
#[test]
fn test_rsa_bruteforce_default_console() {
    let args = ["rsa", "bruteforce", "console", "85","268970693"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_rsa_bruteforce_default_console");
}

// Test logic for RSA public key bruteforce with a custom thread count, with an output to the console, with correct arguments.
#[test]
fn test_rsa_bruteforce_custom_console() {
    let args = ["rsa", "bruteforce", "console", "85","268970693", "16"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_rsa_bruteforce_custom_console");
}

// Test logic for the case when there is an incorrect amount of arguments, less than 5. It should panic.
#[test]
#[should_panic]
fn test_incorrect_args_amount_not_enough() {
    let args = ["console", "E03E2B2E1DFB3EED31023920", "🔑КрепкийКлюч"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_incorrect_args_amount_not_enough");
}

// Test logic for the case when there is an incorrect amount of arguments, more than 5. It should panic.
#[test]
#[should_panic]
fn test_incorrect_args_amount_too_much() {
    let args = ["one", "2", "🔑three", "4", "5", "6", "7"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_incorrect_args_amount_too_much");
}

// Test logic for the case when there is no arguments. It should panic.
#[test]
#[should_panic]
fn test_no_args() {
    let mut args = ["THIS_ARG_WILL_BE_SKIPPED"].iter().map(|s| s.to_string());
    args.next();
    args.next();

    mains_alter_ego(args, "test_no_args");
}

// Test logic for the case when there is the "help" argument among others. It should panic.
#[test]
#[should_panic]
fn test_help_arg_and_its_friends() {
    let args = ["SCREAMING_", "SNAKE_", "CASE", "ROCKS!1!111!!!", "help"]
        .iter()
        .map(|s| s.to_string());

    mains_alter_ego(args, "test_help_arg_and_its_friends");
}

// Test logic for the case when there is only "help" argument. It should panic.
#[test]
#[should_panic]
fn test_help_arg_alone() {
    let args = ["help"].iter().map(|s| s.to_string());

    mains_alter_ego(args, "test_help_arg_alone");
}
