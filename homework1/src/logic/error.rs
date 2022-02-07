use std::fmt;
use std::fmt::Formatter;

// Define own error type for handling... unhappy accidents; and derive Debug trait.
#[derive(Debug, Default)]
pub struct OperationError {
    err_message: String,
    help_message_flag: bool,
}

impl OperationError {
    pub fn new(msg: &str) -> OperationError {
        OperationError {
            err_message: String::from(msg),
            help_message_flag: Default::default(),
        }
    }

    // Set the flag to true if a help message was requested.
    pub fn set_help_flag(&mut self) {
        self.help_message_flag = true;
    }

    // Get the flag for the help message.
    pub fn get_help_flag(&self) -> bool {
        self.help_message_flag
    }
}

// Implement Display trait for possible formatting.
impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err_message)
    }
}

// Implement Error trait for the custom error type.
impl std::error::Error for OperationError {}

// Test module.
#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::logic::error::OperationError;

    // Test creation of the custom error struct.
    #[test]
    fn test_error_struct_creation() {
        let error_message = "this is a test error message";

        let error_struct = OperationError::new(error_message);

        assert_eq!(error_message, error_struct.err_message);
    }

    // Test setup and retrieval of the help flag on an error struct.
    #[test]
    fn test_error_help_flag_setup() {
        let error_message = "this is a test error message";

        let mut error_struct = OperationError::new(error_message);

        error_struct.set_help_flag();

        let flag = error_struct.get_help_flag();

        assert_eq!(flag, true);
    }

    // Test implementation of the Display trait on the custom error struct.
    #[test]
    fn test_error_struct_display_trait() {
        let error_message = "this is a test error message";

        let error_struct = OperationError::new(error_message);

        assert_eq!(
            format!("The error struct's contents: {}.", error_struct),
            "The error struct's contents: this is a test error message."
        );
        // println!("Display of the error struct: {}", error_struct);
    }

    // Test implementation of the Error trait on the custom error struct.
    #[test]
    fn test_error_struct_error_trait() {
        let error_message = "this is a test error message";

        let error_struct = OperationError::new(error_message);

        // Accept and return a reference to a trait object containing a value, implementing Error trait.
        fn accept_args_with_error_trait(error: Box<dyn Error>) -> Box<dyn Error> {
            error
        }

        let boxed_error = Box::new(error_struct);
        let _boxed_error_new = accept_args_with_error_trait(boxed_error);
    }
}
