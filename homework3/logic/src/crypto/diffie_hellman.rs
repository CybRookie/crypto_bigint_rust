use std::error::Error;

// Import required randomisation items.
use rand::Rng;

use crate::logic::bigint::{BigIntSign, ChonkerInt};
use crate::logic::error::OperationError;

pub struct DiffieHellmanResult {
    pub shared_prime: ChonkerInt,
    pub shared_base: ChonkerInt,
    pub secret_a: ChonkerInt,
    pub secret_b: ChonkerInt,
    pub package_from_a_to_b: ChonkerInt,
    pub package_from_b_to_a: ChonkerInt,
    pub result_a: ChonkerInt,
    pub result_b: ChonkerInt,
    pub success: bool,
}

// Implement default value for DiffieHellmanResult.
impl Default for DiffieHellmanResult {
    fn default() -> Self {
        DiffieHellmanResult {
            shared_prime: Default::default(),
            shared_base: Default::default(),
            secret_a: Default::default(),
            secret_b: Default::default(),
            package_from_a_to_b: Default::default(),
            package_from_b_to_a: Default::default(),
            result_a: Default::default(),
            result_b: Default::default(),
            success: false,
        }
    }
}

struct DiffieHellmanParameters {
    shared_prime: ChonkerInt,
    shared_base: ChonkerInt,
    secret_a: ChonkerInt,
    secret_b: ChonkerInt,
}

// Diffie-Hellman implementation.
pub fn diffie_hellman(
    shared_prime: Option<String>,
    shared_base: Option<String>,
    secret_a: Option<String>,
    secret_b: Option<String>,
) -> Result<DiffieHellmanResult, Box<dyn Error>> {
    let parameters = check_df_parameters(&shared_prime, &shared_base, &secret_a, &secret_b)?;

    let package_from_a_to_b = parameters
        .shared_base
        .modpow(&parameters.secret_a, &parameters.shared_prime);
    let package_from_b_to_a = parameters
        .shared_base
        .modpow(&parameters.secret_b, &parameters.shared_prime);
    let result_a = package_from_b_to_a.modpow(&parameters.secret_a, &parameters.shared_prime);
    let result_b = package_from_a_to_b.modpow(&parameters.secret_b, &parameters.shared_prime);
    let success = result_a == result_b;

    Ok(DiffieHellmanResult {
        shared_prime: parameters.shared_prime,
        shared_base: parameters.shared_base,
        secret_a: parameters.secret_a,
        secret_b: parameters.secret_b,
        package_from_a_to_b,
        package_from_b_to_a,
        result_a,
        result_b,
        success,
    })
}

fn check_df_parameters(
    shared_prime: &Option<String>,
    shared_base: &Option<String>,
    secret_a: &Option<String>,
    secret_b: &Option<String>,
) -> Result<DiffieHellmanParameters, Box<dyn Error>> {
    // Initiate RNG. For possible value length randomisation.
    let mut rng = rand::thread_rng();

    // Check inputs, are they correct/incorrect, are they empty.
    // If they are empty, randomly generate required numbers.
    // Check shared prime.
    let shared_prime = match shared_prime {
        Some(value) => {
            // Check the received value, is it numeric and a prime.
            match check_parameter_is_numeric(value) {
                true => {
                    // Check if the numeric value is prime.
                    let candidate = ChonkerInt::from(String::from(value));

                    // Check the length of the received number, if it is too long, deny the service.
                    let number_length = candidate.get_vec().len();

                    if number_length > 100 {
                        return Err(Box::new(OperationError::new("the received candidate has a length more than 100, primality test will take a significant amount of time, choose a smaller value. Correct value is a prime number with the length under 100.")));
                    }

                    let number_of_trials = if number_length < 25 {
                        Some(20)
                    } else if (25..50).contains(&number_length) {
                        Some(10)
                    } else if (50..75).contains(&number_length) {
                        Some(3)
                    } else { // number_length >= 75 && number_length <= 100
                        Some(1)
                    };

                    if candidate.is_prime_probabilistic(number_of_trials) {
                        candidate
                    } else {
                        return Err(Box::new(OperationError::new("the received candidate number is not a prime, according to Miller-Rabin primality test. Correct value is a prime number with the length under 100.")));
                    }
                }
                false => return Err(Box::new(OperationError::new("did no receive a correct shared prime for the Diffie-Hellman calculation. Correct value is a prime number with the length under 100."))),
            }
        }
        None => {
            // Generate a random prime of random length in the range of 5-10.
            let random_length: u64 = rng.gen_range(5..=10);
            ChonkerInt::new_prime(&random_length)
        }
    };

    // Check shared base.
    let shared_base = match shared_base {
        Some(value) => {
            match check_parameter_is_numeric(value) {
                true => {
                    // Check if the numeric value is a primitive root to the shared base.
                    let candidate = ChonkerInt::from(String::from(value));

                    match candidate.is_primitive_root(&shared_prime) {
                        true => candidate,
                        false => return Err(Box::new(OperationError::new("the received candidate number is not a primitive root to the provided prime, did no receive a correct shared base for the Diffie-Hellman calculation. Correct value is a primitive root to the shared prime."))),
                    }
                }
                false => return Err(Box::new(OperationError::new("did no receive a correct shared base for the Diffie-Hellman calculation. Correct value is a primitive root to the shared prime."))),
            }
        }
        None => {
            // Generate a random primitive root to the shared prime..
            shared_prime.new_primitive_root()
        }
    };

    // Check secret value A.
    let secret_a = match secret_a {
        Some(value) => {
            // Check the received value, is it numeric.
            match check_parameter_is_numeric(value) {
                true => {
                    ChonkerInt::from(String::from(value))
                }
                false => return Err(Box::new(OperationError::new("did no receive a correct value for the peer A for the Diffie-Hellman calculation. Correct value is a positive number."))),
            }
        }
        None => {
            // Generate a random number of random length in the range of 500-1000.
            let random_length: u64 = rng.gen_range(500..=1000);
            ChonkerInt::new_rand(&random_length, &BigIntSign::Positive)
        }
    };

    // Check secret value B.
    let secret_b = match secret_b {
        Some(value) => {
            // Check the received value, is it numeric.
            match check_parameter_is_numeric(value) {
                true => {
                    ChonkerInt::from(String::from(value))
                }
                false => return Err(Box::new(OperationError::new("did no receive a correct value for the peer B for the Diffie-Hellman calculation. Correct value is a positive number."))),
            }
        }
        None => {
            // Generate a random number of random length in the range of 500-1000.
            let random_length: u64 = rng.gen_range(500..=1000);
            ChonkerInt::new_rand(&random_length, &BigIntSign::Positive)
        }
    };

    // Generate and return a package of parameters for Diffie-Hellman algorithm calculations.
    Ok(DiffieHellmanParameters {
        shared_prime,
        shared_base,
        secret_a,
        secret_b,
    })
}

pub fn check_parameter_is_numeric(parameter: &str) -> bool {
    let char_iter = parameter.chars();

    // Check if every character is numeric.
    for char in char_iter {
        if !char.is_numeric() {
            return false;
        }
    }

    true
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::crypto::diffie_hellman::{
        check_df_parameters, check_parameter_is_numeric, diffie_hellman,
    };
    use crate::logic::bigint::ChonkerInt;

    // Test Diffie-Hellman algorithm.
    #[test]
    fn test_diffie_hellman() {
        // Correct values.
        let test_shared_prime = "13".to_string();
        let test_shared_base = "7".to_string();
        let test_secret_a = "12323".to_string();
        let test_secret_b = "42398472".to_string();

        let result = diffie_hellman(
            Some(test_shared_prime.clone()),
            Some(test_shared_base.clone()),
            Some(test_secret_a.clone()),
            Some(test_secret_b.clone()),
        )
            .unwrap();
        let package_from_a_to_b = ChonkerInt::from(String::from("7")).modpow(
            &ChonkerInt::from(String::from("12323")),
            &ChonkerInt::from(String::from("13")),
        );
        let package_from_b_to_a = ChonkerInt::from(String::from("7")).modpow(
            &ChonkerInt::from(String::from("42398472")),
            &ChonkerInt::from(String::from("13")),
        );
        let result_a = package_from_b_to_a.modpow(
            &ChonkerInt::from(String::from("12323")),
            &ChonkerInt::from(String::from("13")),
        );
        let result_b = package_from_a_to_b.modpow(
            &ChonkerInt::from(String::from("42398472")),
            &ChonkerInt::from(String::from("13")),
        );

        assert_eq!(
            ChonkerInt::from(test_shared_prime),
            result.shared_prime
        );
        assert_eq!(
            ChonkerInt::from(test_shared_base),
            result.shared_base
        );
        assert_eq!(
            ChonkerInt::from(test_secret_a),
            result.secret_a
        );
        assert_eq!(
            ChonkerInt::from(test_secret_b),
            result.secret_b
        );
        assert_eq!(package_from_a_to_b, result.package_from_a_to_b);
        assert_eq!(package_from_b_to_a, result.package_from_b_to_a);
        assert_eq!(result_a, result.result_a);
        assert_eq!(result_b, result.result_b);
        assert_eq!(true, result.success);

        // Incorrect shared prime.
        let test_shared_prime = Some("562457128101735933004861281229980320017117423199759147390620125871795298609636011392770132344949088969751961333591438819671223807833285269283606992239".to_string());
        let test_shared_base = Some("7".to_string());
        let test_secret_a = Some("12323".to_string());
        let test_secret_b = Some("42398472".to_string());

        let result = match diffie_hellman(
            test_shared_prime.clone(),
            test_shared_base.clone(),
            test_secret_a.clone(),
            test_secret_b.clone(),
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(!result);

        // Incorrect shared base.
        let test_shared_prime = Some("13".to_string());
        let test_shared_base = Some("abcd!".to_string());

        let result = match diffie_hellman(
            test_shared_prime.clone(),
            test_shared_base.clone(),
            test_secret_a.clone(),
            test_secret_b.clone(),
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(!result);

        // Incorrect secret A.
        let test_shared_base = Some("7".to_string());
        let test_secret_a = Some("Abcd!".to_string());

        let result = match diffie_hellman(
            test_shared_prime.clone(),
            test_shared_base.clone(),
            test_secret_a.clone(),
            test_secret_b.clone(),
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(!result);

        // Incorrect secret B.
        let test_secret_a = Some("-12323".to_string());
        let test_secret_b = Some("?abcd".to_string());

        let result = match diffie_hellman(
            test_shared_prime.clone(),
            test_shared_base.clone(),
            test_secret_a.clone(),
            test_secret_b.clone(),
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(!result);

        // Random values.
        let test_shared_prime = None;
        let test_shared_base = None;
        let test_secret_a = None;
        let test_secret_b = None;

        let result = match diffie_hellman(
            test_shared_prime,
            test_shared_base,
            test_secret_a,
            test_secret_b,
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(result);
    }

    // Test check of the Diffie-Hellman parameters, are they suitable for further calculations.
    #[test]
    fn test_df_parameters() {
        // Correct values.
        let test_shared_prime = Some("13".to_string());
        let test_shared_base = Some("7".to_string());
        let test_secret_a = Some("12323".to_string());
        let test_secret_b = Some("42398472".to_string());

        let result = match check_df_parameters(
            &test_shared_prime,
            &test_shared_base,
            &test_secret_a,
            &test_secret_b,
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(result);

        // Incorrect shared prime.
        let test_shared_prime = Some("3415".to_string());

        let result = match check_df_parameters(
            &test_shared_prime,
            &test_shared_base,
            &test_secret_a,
            &test_secret_b,
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(!result);

        // Incorrect shared base.
        let test_shared_prime = Some("13".to_string());
        // let test_shared_base = Some("0"); // Panics, as it should.
        //
        // let result = match check_df_parameters(&test_shared_prime, &test_shared_base, &test_secret_a, &test_secret_b) {
        //     Ok(_) => true,
        //     Err(_) => false,
        // };
        //
        // assert!(!result);

        // Incorrect secret A.
        let test_shared_base = Some("7".to_string());
        let test_secret_a = Some("Abcd!".to_string());

        let result = match check_df_parameters(
            &test_shared_prime,
            &test_shared_base,
            &test_secret_a,
            &test_secret_b,
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(!result);

        // Incorrect secret B.
        let test_secret_a = Some("-12323".to_string());
        let test_secret_b = Some("?abcd".to_string());

        let result = match check_df_parameters(
            &test_shared_prime,
            &test_shared_base,
            &test_secret_a,
            &test_secret_b,
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(!result);

        // Random values.
        let test_shared_prime = None;
        let test_shared_base = None;
        let test_secret_a = None;
        let test_secret_b = None;

        let result = match check_df_parameters(
            &test_shared_prime,
            &test_shared_base,
            &test_secret_a,
            &test_secret_b,
        ) {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(result);
    }

    // Test check of the Diffie-Hellman parameter, is it numeric.
    #[test]
    fn test_df_parameter_numeric_check() {
        let test_string1 = "TestString";
        let test_string2 = "0";
        let test_string3 = "-12323";
        let test_string4 = "42398472";
        let test_string5 = "$^&!*#Onspw";

        assert!(!check_parameter_is_numeric(test_string1));
        assert!(check_parameter_is_numeric(test_string2));
        assert!(!check_parameter_is_numeric(test_string3));
        assert!(check_parameter_is_numeric(test_string4));
        assert!(!check_parameter_is_numeric(test_string5));
    }
}
