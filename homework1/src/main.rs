use std::env;
use std::process;

use se::logic::config::Config;
use se::logic::error::OperationError;
use se::logic::run;

fn main() {
    // Create configuration for the encryption process,
    // based on the input arguments from the command line, and handle possible errors.
    // Skip 1st argument, program name.
    // Check for the custom error types to, in turn, check for the value of the flag for the help message.
    // If it is set, output produced help message and exit process with the exit code 0.
    let config = match Config::new(env::args().skip(1)) {
        Ok(config) => config,
        Err(e) => {
            match e.downcast::<OperationError>() {
                Ok(value_ref) => {
                    let value = *value_ref;
                    if value.get_help_flag() {
                        println!("{}", value);
                        process::exit(0);
                    }
                    eprintln!("Problem parsing arguments: {}\nEnter \"es(.exe) help\" or \"cargo run help\" to get a help message for more information about the tool.", value);
                    process::exit(64);
                }
                Err(e) => {
                    eprintln!("Problem parsing arguments: {}\nEnter \"es(.exe) help\" or \"cargo run help\" to get a help message for more information about the tool.", e);
                    process::exit(64);
                }
            };
        }
    };

    // Pass the retrieved configuration settings to the main tool logic and handle possible errors.
    if let Err(e) = run(config) {
        eprintln!("Application error: {} Enter \"es(.exe) help\" to get a help message for more information about the tool.", e);
        process::exit(70);
    }
}
