use std::error::Error;
use std::str::from_utf8_unchecked;
use std::sync::mpsc;

use crate::crypto::diffie_hellman::check_parameter_is_numeric;
use crate::crypto::rsa::threadpool::ThreadPool;
use crate::encoding::{string_hex_decode, string_hex_encode};
use crate::logic::bigint::{BigIntSign, ChonkerInt};
use crate::logic::config::Mode;
use crate::logic::error::OperationError;

mod threadpool;

#[derive(Debug, PartialEq, Eq)]
pub struct RsaKeyPair {
    pub public_key_n: ChonkerInt,
    pub public_key_e: ChonkerInt,
    pub private_key_d: ChonkerInt,
}

// An enumeration of results for the main thread from a worker.
pub enum TaskResult {
    Success(BruteforceResult),
    Terminate(OperationError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BruteforceResult {
    pub prime_q: ChonkerInt,
    pub prime_p: ChonkerInt,
    pub public_key_n: ChonkerInt,
    pub public_key_e: ChonkerInt,
    pub private_key_d: ChonkerInt,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RsaResult {
    KeyPair(RsaKeyPair),
    StringResult(String),
    BruteforceRSAResult(BruteforceResult),
}

// Implement default value for RsaResult.
impl Default for RsaResult {
    fn default() -> Self {
        RsaResult::StringResult(String::new())
    }
}

// Constants for RSA block encryption/decryption/bruteforce.
const BLOCK_SIZE: u8 = 16;
const BLOCK_DELIMITER: i8 = 0b11111111_u8 as i8;
const BLOCK_PADDING: u128 = 0b10010000_u8 as u128;
// No assigned value for the extended ASCII.
const BRUTEFORCE_THREAD_COUNT: usize = 8;

pub fn rsa(
    mode: &Mode,
    target: Option<String>,
    key_exponent: Option<String>,
    key_modulus: Option<String>,
    thread_count: Option<String>,
) -> Result<RsaResult, Box<dyn std::error::Error>> {
    let encryption_decryption_clojure =
        |mode: Mode| -> Result<RsaResult, Box<dyn std::error::Error>> {
            let key_exponent = match key_exponent {
                Some(value) => value,
                None => return Err(Box::new(OperationError::new("did not receive a value for the public/private key exponent for the RSA encryption/decryption. Correct value is a positive number."))),
            };

            let key_modulus = match key_modulus {
                Some(value) => value,
                None => return Err(Box::new(OperationError::new("did not receive a value for the key modulus for the RSA encryption/decryption. Correct value is a positive composite number."))),
            };

            // Check if the key exponent and modulus are numeric.
            if let false = check_parameter_is_numeric(&key_modulus) {
                return Err(Box::new(OperationError::new("did not receive a correct value for the public/private key exponent for the RSA encryption/decryption. Correct value is a positive number.")));
            };

            if let false = check_parameter_is_numeric(&key_modulus) {
                return Err(Box::new(OperationError::new("did not receive a correct value for the key modulus for the RSA encryption/decryption. Correct value is a positive composite number.")));
            };

            // Convert the exponent and the modulus into the BigInts.
            let key_exponent = ChonkerInt::from(String::from(&key_exponent));
            let key_modulus = ChonkerInt::from(String::from(&key_modulus));

            // Check if the key modulus is a composite number.
            if let true = key_modulus.is_prime_probabilistic(Some(1)) {
                return Err(Box::new(OperationError::new("did not receive a correct value for the key modulus for the RSA encryption/decryption. Correct value is a positive composite number.")));
            };

            // Check if the modulus is longer than the encryption/decryption block value.
            // The check is preformed only for encryption or decryption requests,
            // for bruteforcing the value is reuqired to be equal or below length 10.
            if (key_modulus.get_vec().len() <= 39) && (mode == Mode::Encode || mode == Mode::Decode) {
                return Err(Box::new(OperationError::new("did not receive a correct value for the key modulus for the RSA encryption/decryption. Correct value is a positive composite number with at least length of 40 or bigger.")));
            }

            // Parameter for encryption or decryption.
            let mut unwrap_target = "".to_string();

            // Parameter for bruteforcing.
            let mut unwrap_thread_count = None;

            // Based on mode unwrap specific key parameters.
            if mode == Mode::Encode || mode == Mode::Decode {
                unwrap_target = match target {
                    Some(value) => value,
                    None => return Err(Box::new(OperationError::new("did not receive a string for for the RSA encryption/decryption. Correct value is a string."))),
                };
            } else if mode == Mode::Bruteforce {
                unwrap_thread_count = match thread_count {
                    Some(thread_count) => {
                        // Check if the thread count parameter is numeric.
                        if let false = check_parameter_is_numeric(&thread_count) {
                            return Err(Box::new(OperationError::new("did not receive a correct value for the thread count for the RSA bruteforcing. Correct value is a positive number in the range 1-64. It can be omitted, or \"none\" can be written instead to use the default value of 8.")));
                        };

                        // Check if the thread count is too long.
                        if thread_count.len() > 2 {
                            return Err(Box::new(OperationError::new("did not receive a correct value for the thread count for the RSA bruteforcing. Correct value is a positive number in the range 1-64. It can be omitted, or \"none\" can be written instead to use the default value of 8.")));
                        };

                        let parsed_count: usize = thread_count.parse()?;

                        Some(parsed_count)
                    }
                    None => None,
                };
            }


            // Encrypt, decrypt or bruteforce the target string.
            match mode {
                Mode::Encode => {
                    let encryption_result = rsa_encrypt(&unwrap_target, &key_exponent, &key_modulus)?;

                    Ok(RsaResult::StringResult(encryption_result))
                }
                Mode::Decode => {
                    let decryption_result = rsa_decrypt(&unwrap_target, &key_exponent, &key_modulus)?;

                    Ok(RsaResult::StringResult(decryption_result))
                }
                Mode::Bruteforce => {
                    let bruteforce_result = rsa_bruteforce(&key_exponent, &key_modulus, unwrap_thread_count)?;

                    Ok(bruteforce_result)
                }
                _ => Err(Box::new(OperationError::new(
                    "error in RSA logic, incorrect handling of mode",
                ))),
            }
        };

    // Determine what to do: encrypt, decrypt or generate a new key pair.
    match mode {
        Mode::Encode => encryption_decryption_clojure(Mode::Encode),
        Mode::Decode => encryption_decryption_clojure(Mode::Decode),
        Mode::Generate => rsa_key_generation(),
        Mode::Bruteforce => encryption_decryption_clojure(Mode::Bruteforce),
    }
}

// Generate a random RSA key pair.
fn rsa_key_generation() -> Result<RsaResult, Box<dyn std::error::Error>> {
    let big_one = ChonkerInt::from(1);
    let prime_q = ChonkerInt::new_prime(&25);
    let mut prime_p = ChonkerInt::new_prime(&21);

    // Regenerate one of the primes to ensure that are distinct.
    while prime_q == prime_p {
        prime_p = ChonkerInt::new_prime(&10);
    }

    // Generate the modulus n, a product of two previously randomly generated primes.
    // Modulus n is a part of the public key.
    let modulus_n = &prime_q * &prime_p;

    // Generate result of Euler's totient function, phi(n) = (p-1)(q-1)
    let phi_n = &(&prime_p - &big_one) * &(&prime_q - &big_one);

    // Generate the exponent e form the range 1 < e < phi(n).
    // Exponent e is a part of the public key. e should not be a factor of n,
    // should be a coprime to phi(n).
    // Note: very often a predetermined value of e = 65537 is used, it does not undermine secrecy,
    // but it increases efficiency of the algorithm.
    // Repeat the public and private exponent generation if the resulting private exponent is negative.
    let mut exponent_e;
    let mut private_key_d;
    loop {
        loop {
            exponent_e = ChonkerInt::new_rand_range_value(&big_one, &phi_n, &BigIntSign::Positive);

            if exponent_e == prime_q || exponent_e == prime_p || (!exponent_e.is_coprime(&phi_n)) {
                continue;
            }
            break;
        }

        // Generate the modular multiplicative inverse d of e,
        // a private key with the extended Euclidean algorithm.
        private_key_d = exponent_e.egcd(&phi_n).self_x;

        if *private_key_d.get_sign() != BigIntSign::Negative {
            break;
        }
    }

    Ok(RsaResult::KeyPair(RsaKeyPair {
        public_key_n: modulus_n,
        public_key_e: exponent_e,
        private_key_d,
    }))
}

// Encrypt the message with a public exponent and a modulus.
fn rsa_encrypt(
    target: &str,
    key_exponent: &ChonkerInt,
    key_modulus: &ChonkerInt,
) -> Result<String, Box<dyn Error>> {
    // Convert string to the vector of unsigned one byte integers.
    let target = target.as_bytes();
    let mut big_int: u128 = 0;

    // Define a vector for encrypted bytes and remainder.
    // The target message is exactly split into 16 byte blocks,
    // if there is a remainder, store it and operate on it separately.
    let mut result_vector: Vec<i8> = vec![];
    let target_chunks = target.chunks_exact(BLOCK_SIZE as usize);
    let remainder = target_chunks.remainder();

    let byte_shift_counter = 8;

    // Loop over the first chunk, store each 8 bits/1 byte of the chunk sequence in a 16 byte unsigned integer.
    // Afterwards, turn the 16 byte integer into the BigInt and proceed with the modpow operation on it,
    // to get the encrypted block.
    // Access the BigInt's vector of decimal digits after encryption and copy them to the resulting vector,
    // delimit each BigInt's vector, corresponding to different chunks of the message.
    for chunk in target_chunks {
        // Store the bytes in the 16 byte unsigned integer.
        for integer in chunk {
            big_int <<= byte_shift_counter;
            big_int |= *integer as u128;
        }

        // Encrypt the produced BigInt.
        // Extract the vector of decimal digits from the BigInt and store it in the result vector with delimiters.
        let encrypted_bigint = ChonkerInt::from(big_int).modpow(key_exponent, key_modulus);
        let mut encrypted_bigint_vec = Vec::from(encrypted_bigint.get_vec());
        result_vector.append(&mut encrypted_bigint_vec);
        result_vector.push(BLOCK_DELIMITER);
    }

    // Check if there is a remainder, if it exists, repeate the process above,
    // but add the predefined padding bytes to the unsigned 16 byte integer at the end.
    if !remainder.is_empty() {
        let remainder_len = remainder.len();
        let padding_len = BLOCK_SIZE as usize - remainder_len;

        // Store the remaining bytes.
        for integer in remainder {
            big_int <<= byte_shift_counter;
            big_int |= *integer as u128;
        }

        // Add padding values.
        for _iteration in 0..padding_len {
            big_int <<= byte_shift_counter;
            big_int |= BLOCK_PADDING;
        }

        // Encrypt the produced BigInt.
        let encrypted_bigint = ChonkerInt::from(big_int).modpow(key_exponent, key_modulus);

        // Extract the vector of decimal digits from the BigInt and store it in the result vector.
        let mut encrypted_bigint_vec = Vec::from(encrypted_bigint.get_vec());
        result_vector.append(&mut encrypted_bigint_vec);
    }

    // Change the type of the bytes in the result vector from signed to the unsigned bytes.
    let result_vector: Vec<u8> = result_vector.iter().map(|int| *int as u8).collect();

    // Encode the vector of bytes into the hex string and return the result.
    string_hex_encode(&result_vector)
}

// Decrypt the message with a private exponent and a modulus.
fn rsa_decrypt(
    target: &str,
    key_exponent: &ChonkerInt,
    key_modulus: &ChonkerInt,
) -> Result<String, Box<dyn Error>> {
    // Convert received hex string into the vector of encrypted one bytes.
    // Split the vector of encrypted bytes into separate vectors of predefined chunks base on the delimiters.
    let decoded_string = string_hex_decode(target)?;
    let mut decrypted_bigint_vec: Vec<u8> = vec![];
    let decoded_string_iterator = decoded_string.split(|int| *int == BLOCK_DELIMITER as u8);

    // Define the 16 byte integer where result of encrypted chunk/number decryption.
    // Define the mutable byte shift to the left and immutable one to the right.
    let mut big_unsigned_integer;
    let mut left_byte_shift_counter = 0u8;
    let right_byte_shift_counter = 120u8;

    // Decrypt the number from the vector of encrypted decimal digits.
    // For each retrieved big integer, split the 16 byte unsigned integer of fused bytes
    // into separate 1 byte unsigned integers and store the result in the final vector of bytes.
    // If the predefined padding value is detected, stop the loops.
    'outer: for bigint in decoded_string_iterator {
        big_unsigned_integer = ChonkerInt::from(bigint)
            .modpow(key_exponent, key_modulus)
            .to_digit();

        for iteration in 0..BLOCK_SIZE {
            let mut big_unsigned_integer_copy = big_unsigned_integer;

            // Extract the target byte and store it in the unsigned 1 byte integer.
            big_unsigned_integer_copy <<= left_byte_shift_counter;
            big_unsigned_integer_copy >>= right_byte_shift_counter;

            let original_byte = big_unsigned_integer_copy as u8;

            // Check if the value equals the predefined padding value, if it does, exit both loops.
            if original_byte == BLOCK_PADDING as u8 {
                break 'outer;
            }

            // Store the extracted byte in the result vector.
            decrypted_bigint_vec.push(original_byte);

            // Prevent subtraction from unsigned 0 at the last iteration.
            if iteration == BLOCK_SIZE - 1 {
                break;
            }

            // Increase the shift to the left, so to clean the bit sequence to the left of the target byte
            // before shifting it to the rightmost position.
            left_byte_shift_counter += 8;
        }

        // Reset the left shift parameter for each new big integer.
        left_byte_shift_counter = 0u8;
    }
    // Convert the vector of unsigned byte integers into the string of UTF-8 characters
    // without checks for the validity of the unicode sequences.
    // Wrap the resulting string and return it.
    let result = unsafe { from_utf8_unchecked(&decrypted_bigint_vec) };

    Ok(String::from(result))
}

// Bruteforce the provided RSA modulus, if successful,
// return calculated primes and new generated exponents for the RSA key pair.
fn rsa_bruteforce(
    key_exponent: &ChonkerInt,
    key_modulus: &ChonkerInt,
    thread_count: Option<usize>,
) -> Result<RsaResult, Box<dyn std::error::Error>> {
    // Check the length of the target modulus for bruteforce.
    // The bruteforcing is fast with the length equal to or below 10 digits, at 12 digits it starts to take 1 minute,
    // the longer it gets, the more time it takes to bruteforce.
    if key_modulus.get_vec().len() > 10 {
        return Err(Box::new(OperationError::new("the requested RSA modulus for bruteforce is longer than 10, after 10 the operation starts taking noticeable amount of time, e.g. it takes about 0.5 min for 12 digit modulus.")));
    }

    // Check the thread count parameter, if it is empty/none, use a default thread count,
    // if it is present, check if it fits into the preset boundary.
    let bruteforce_thread_count = match thread_count {
        None => BRUTEFORCE_THREAD_COUNT,
        Some(thread_count) => {
            // Check for the requested thread count.
            if thread_count > 64 || thread_count == 0 {
                return Err(Box::new(OperationError::new("the requested thread count for brute forcing exceeds 64 or is equal to 0, the amount requested must be a positive number below or equal 64.")));
            }
            thread_count
        }
    };

    // Initialize the thread pool and take a half of the modulus' length.
    let thread_pool = ThreadPool::new(bruteforce_thread_count);
    let key_modulus_half_length = key_modulus.get_vec().len() / 2;

    // Define the ceiling for values to not over-calculate,
    // based on it define starting points for all threads/workers.
    let mut ceiling_limit: Vec<u8> = vec![0; key_modulus_half_length];
    ceiling_limit.fill(9);
    let increment_count_bigint = &ChonkerInt::from(ceiling_limit.as_slice())
        / &ChonkerInt::from(BRUTEFORCE_THREAD_COUNT as u64);

    // A starting point for the first worker that takes the job.
    let start_prime_point = ChonkerInt::from(3);

    // Clojure defining the tasks, executed by the workers.
    let task_clojure = |starting_point: ChonkerInt,
                        key_exponent: ChonkerInt,
                        key_modulus: ChonkerInt,
                        worker_sender: mpsc::Sender<TaskResult>| {
        // Factor the target starting with the given starting point.
        let prime_factors = key_modulus.factor_rsa_modulus(&starting_point);

        // If the vector length is more than two,
        // then the target is not a product of two primes.
        if prime_factors.len() > 2 {
            let _sent_task_result = worker_sender.send(TaskResult::Terminate(OperationError::new("The target RSA modulus for bruteforce is incorrect, it must be a product of two primes. THe received target had more than 2 factors.")));
            return;
        }

        // If the vector length is less than two,
        // then the thread did not find suitable factors in the given range.
        // End the operation of the thread.
        if prime_factors.len() < 2 {
            return;
        }

        let prime_p = prime_factors[0].clone();
        let prime_q = prime_factors[1].clone();
        let big_one = ChonkerInt::from(1);

        // Generate result of Euler's totient function, phi(n) = (p-1)(q-1)
        let phi_n = &(&prime_p - &big_one) * &(&prime_q - &big_one);

        // Check if the provided public exponent is coprime to the phi(n).
        if !key_exponent.is_coprime(&phi_n) {
            let _sent_task_result = worker_sender.send(TaskResult::Terminate(OperationError::new("The target RSA public exponent for bruteforce is incorrect, it must be a coprime to the euler's totient of the bruteforced primes.")));
            return;
        }

        // Generate the private exponent.
        let private_key_d;
        private_key_d = key_exponent.egcd(&phi_n).self_x;

        // Check if the produced private exponent is negative.
        // If it is stop the thread and the whole pool.
        if *private_key_d.get_sign() == BigIntSign::Negative {
            let _sent_task_result = worker_sender.send(TaskResult::Terminate(OperationError::new("The produced private exponent from bruteforce is negative, thus either input parameters are incorrect or there is an error in the algorithm.")));
            return;
        }

        // Package calculated results and send them off to the main thread.
        let bruteforce_result = BruteforceResult {
            prime_q,
            prime_p,
            public_key_n: key_modulus,
            public_key_e: key_exponent,
            private_key_d,
        };

        let _sent_task_result = worker_sender.send(TaskResult::Success(bruteforce_result));
    };

    let big_two = ChonkerInt::from(2);
    let big_one = ChonkerInt::from(1);
    let big_zero = ChonkerInt::new();

    // Create a channel, share the receiver among workers/threads,
    // while the sender part will be utilised by the main thread
    // to listen for the worker results.
    let (worker_sender, main_receiver) = mpsc::channel();

    // Generate the set amount of threads and send them the appropriate task with set starting points.
    for thread in 0..BRUTEFORCE_THREAD_COUNT {
        let mut starting_point =
            &start_prime_point + &(&increment_count_bigint * &ChonkerInt::from(thread as u64));

        // Check if the starting point is odd.
        // If it is, increment by 1.
        // Prime numbers are odd with an exception for 2.
        if &starting_point % &big_two == big_zero {
            starting_point = &starting_point + &big_one;
        }

        let key_exponent = (*key_exponent).clone();
        let key_modulus = (*key_modulus).clone();
        let worker_sender = worker_sender.clone();

        // Create a new worker with the task.
        thread_pool.execute(move || {
            task_clojure(starting_point, key_exponent, key_modulus, worker_sender);
        });
    }

    // Listen for the signals from the threads/workers.
    let received_result = main_receiver.recv()?;

    // Check the received result from a worker/thread.
    match received_result {
        TaskResult::Success(bruteforce_result) => {
            // Testing of the produced values with encryption and decryption of a testing string is not done,
            // because of implementation specifics, the modulus must be equal to 40 digits in length or longer,
            // bu the bruteforcing allows smaller wvalue for bruteforcing.
            // // Test the validity of the bruteforcin result with encryption and decryption of a test string.
            // let test_string = "Test string for RSA bruteforcing.";
            //
            // // Encrypt the test string, in case of errors during the calculation, close the thread.
            // // let encrypted_message = rsa(&Mode::Encode, Some(test_string), Some(bruteforce_result.public_key_e.to_string().as_str()), Some(bruteforce_result.public_key_n.to_string().as_str()));
            // let encrypted_message = rsa_encrypt(test_string, &bruteforce_result.public_key_e, &bruteforce_result.public_key_n);
            //
            // let encrypted_message = match encrypted_message {
            //     Ok(rsa_result) => {
            //         rsa_result
            //     },
            //     Err(err_value) => return Err(Box::new(OperationError::new("failed to encrypt the test string during the test of bruteforcing result (rsa_bruteforce)"))),
            // };
            //
            // // Decrypt the encrypted version of the test message, in case of errors during the calculation, close the thread.
            // // let decrypted_message = rsa(&Mode::Decode, Some(encrypted_message.as_str()), Some(bruteforce_result.private_key_d.to_string().as_str()), Some(bruteforce_result.public_key_n.to_string().as_str()));
            // let decrypted_message = rsa_decrypt(&encrypted_message, &bruteforce_result.private_key_d, &bruteforce_result.public_key_n);
            //
            // let decrypted_message = match decrypted_message {
            //     Ok(rsa_result) => {
            //         rsa_result
            //     },
            //     Err(err_value) => return Err(Box::new(OperationError::new("failed to decrypt the test string during the test of bruteforcing result (rsa_bruteforce)"))),
            // };
            //
            // // If the decrypted message does not equal the initial version,
            // // terminate the thread and the thread pool.
            // if !test_string.eq(decrypted_message.as_str()) {
            //     return Err(Box::new(OperationError::new("decrypted test string does not equal the initial test string, thus either input parameters are incorrect or there is an error in the algorithm (rsa_bruteforce)")));
            // }

            Ok(RsaResult::BruteforceRSAResult(bruteforce_result))
        }
        TaskResult::Terminate(bruteforce_error) => Err(Box::new(bruteforce_error)),
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::crypto::rsa::{
        rsa, rsa_bruteforce, rsa_decrypt, rsa_encrypt, rsa_key_generation, RsaResult,
    };
    use crate::logic::bigint::{BigIntSign, ChonkerInt};
    use crate::logic::config::Mode;

    // Test RSA handling of incorrect input data.
    #[test]
    fn test_rsa_incorrect_input_handling() {
        let mode = Mode::Encode;
        let mut target_message: Option<String> = None;
        let mut exponent = Some("45145151".to_string());
        let mut modulus = Some("578851612".to_string());
        let mut thread_count = None;

        // Test for the absence of the target message string during encryption.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the absence of the target message string during encryption was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Target related error: {}", e),
        }

        target_message = Some("A correct test string.".to_string());
        exponent = None;

        // Test for the absence of the public exponent string during encryption.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the absence of the public exponent string during encryption was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Public exponent related error: {}", e),
        }

        let mut exponent = Some("ABCDE".to_string());

        // Test for the incorrectness of the public exponent string during encryption.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the incorrectness of the public exponent string during encryption was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Public exponent related error: {}", e),
        }

        exponent = Some("45145151".to_string());
        modulus = None;

        // Test for the absence of the modulus string during encryption.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the absence of the modulus string during encryption was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Modulus related error: {}", e),
        }

        modulus = Some("ABCDE".to_string());

        // Test for the incorrectness of the modulus string during encryption.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the incorrectness of the modulus string during encryption was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Modulus related error: {}", e),
        }

        modulus = Some("57885161".to_string());

        // Test for the modulus being a composite number during encryption.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the modulus being a composite number during encryption was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Modulus related error: {}", e),
        }

        modulus = Some("578851612".to_string());

        // Test for the modulus being too short, having equal or less than 39 digits during encryption.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the modulus being too short, having equal or less than 39 digits during encryption was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Modulus related error: {}", e),
        }

        thread_count = Some("65".to_string());

        // Test for the thread count being too big, being over 64.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the thread count being too big, being over 64 was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Thread count related error: {}", e),
        }

        thread_count = Some("0".to_string());

        // Test for the thread count being zero.
        match rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()) {
            Ok(_) => panic!("somehow generated a result, while the error for the thread count being zero was desired (test_rsa_incorrect_input_handling)"),
            Err(e) => println!("Thread count related error: {}", e),
        }
    }

    // Test RSA handling of correct input data.
    #[test]
    fn test_rsa_correct_input_handling() {
        let mut mode = Mode::Generate;
        let mut target_message: Option<String> = None;
        let mut exponent = Some("45145151".to_string());
        let mut modulus = Some("".to_string());
        let mut thread_count = None;

        // Test random RSA keypair generation with absent or incorrect data besides the mode..
        let _keypair = rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()).unwrap();

        target_message = Some("Test string.".to_string());
        exponent = None;
        modulus = None;

        // Test random RSA keypair generation with absent or incorrect data besides the mode..
        let _keypair = rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()).unwrap();

        // A key pair that was produced separately by the rsa_key_generation() function.
        // Randomly generated RSA public/private key modulus n: 441982524952231918609144409818894577105184461
        // Randomly generated RSA public key exponent e: 6119931580888508280272762765
        // Randomly generated RSA private key exponent d: 3257209244777795983999918284178604218550597

        mode = Mode::Encode;
        exponent = Some("6119931580888508280272762765".to_string());
        modulus = Some("441982524952231918609144409818894577105184461".to_string());

        // Test the target string encryption with correct parameters. Print out the encrypted string, it is also encoded in hexadecimal format.
        let encryption_result = rsa(&mode, target_message.clone(), exponent.clone(), modulus.clone(), thread_count.clone()).unwrap();

        let encrypted_string = match encryption_result {
            RsaResult::KeyPair(_) => panic!("somehow generated a random RSA key pair, while the encrypted string was desired (test_rsa_correct_input_handling)"),
            RsaResult::StringResult(encrypted_string) => {
                println!("The encrypted string: {}", encrypted_string);
                encrypted_string
            }
            RsaResult::BruteforceRSAResult(_) => panic!("somehow generated an RSA bruteforce result, while the encrypted string was desired (test_rsa_correct_input_handling)"),
        };

        mode = Mode::Decode;
        exponent = Some("3257209244777795983999918284178604218550597".to_string());

        // Result of the encryption of the "Test string." with previous parameters.
        // let encrypted_string = Some("0109020000060607080608020405030409090304010309000708090108070900050901080503010803");

        // Test the target string decryption with correct parameters. Print out the decrypted string and comapre it with the original.
        let decryption_result = rsa(&mode, Some(encrypted_string.clone()), exponent.clone(), modulus.clone(), thread_count.clone()).unwrap();

        let decrypted_string = match decryption_result {
            RsaResult::KeyPair(_) => panic!("somehow generated a random RSA key pair, while the decrypted string was desired (test_rsa_correct_input_handling)"),
            RsaResult::StringResult(decrypted_string) => {
                println!("The decrypted string: {}", decrypted_string);
                decrypted_string
            }
            RsaResult::BruteforceRSAResult(_) => panic!("somehow generated an RSA bruteforce result, while the encrypted string was desired (test_rsa_correct_input_handling)"),
        };

        // Check the original string with the decrypted string.
        assert_eq!(target_message.unwrap(), decrypted_string);

        // Test requests for bruteforcing.
        mode = Mode::Bruteforce;
        exponent = Some("85".to_string());
        modulus = Some("268970693".to_string());
        thread_count = None; // Rely on default 8 threads/workers.

        let target_modulus = ChonkerInt::from(268970693);
        let target_public_exponent = ChonkerInt::from(85);
        let private_key_comparison = ChonkerInt::from(88590349);

        let bruteforce_result = rsa(&mode, Some(encrypted_string.clone()), exponent.clone(), modulus.clone(), thread_count.clone()).unwrap();

        let rsa_package = match bruteforce_result {
            RsaResult::BruteforceRSAResult(rsa_result) => rsa_result,
            _ => panic!(
                "error in the algorithm, did not compute a bruteforce result (test_rsa_bruteforce)"
            ),
        };

        println!("Bruteforce result package: {:?}", rsa_package);
        println!("Bruteforce prime p: {}", rsa_package.prime_p);
        println!("Bruteforce prime q: {}", rsa_package.prime_q);

        assert_eq!(rsa_package.public_key_n, target_modulus);
        assert_eq!(rsa_package.public_key_e, target_public_exponent);
        assert_eq!(rsa_package.private_key_d, private_key_comparison);

        // Repeat the bruteforcing request with a custom amount of threads.
        thread_count = Some("24".to_string()); // Rely on default 8 threads/workers.
        let bruteforce_result = rsa(&mode, Some(encrypted_string.clone()), exponent.clone(), modulus.clone(), thread_count.clone()).unwrap();

        let rsa_package = match bruteforce_result {
            RsaResult::BruteforceRSAResult(rsa_result) => rsa_result,
            _ => panic!(
                "error in the algorithm, did not compute a bruteforce result (test_rsa_bruteforce)"
            ),
        };

        println!("Bruteforce result package: {:?}", rsa_package);
        println!("Bruteforce prime p: {}", rsa_package.prime_p);
        println!("Bruteforce prime q: {}", rsa_package.prime_q);

        // let private_key_comparison = ChonkerInt::from(2485);
        let private_key_comparison = ChonkerInt::from(88590349);

        assert_eq!(rsa_package.public_key_n, target_modulus);
        assert_eq!(rsa_package.public_key_e, target_public_exponent);
        assert_eq!(rsa_package.private_key_d, private_key_comparison);
    }

    // Test RSA random key pair generation.
    #[test]
    fn test_rsa_key_pair_random_generation() {
        let rsa_generation_result = rsa_key_generation().unwrap();

        match rsa_generation_result {
            RsaResult::KeyPair(key_pair) => {
                println!("Randomly generated RSA public/private key modulus n: {}", key_pair.public_key_n);
                println!("Randomly generated RSA public key exponent e: {}", key_pair.public_key_e);
                println!("Randomly generated RSA private key exponent d: {}", key_pair.private_key_d);
                assert_eq!(*key_pair.private_key_d.get_sign(), BigIntSign::Positive);
            }
            RsaResult::StringResult(_) => panic!("produced a string from encryption/decryption instead of a randomly generated key pair (test_rsa_key_pair_random_generation)"),
            RsaResult::BruteforceRSAResult(_) => panic!("somehow generated an RSA bruteforce result, while the encrypted string was desired (test_rsa_correct_input_handling)"),
        }
    }

    // Test RSA encryption and decryption of the target data string.
    #[test]
    fn test_rsa_encryption_and_decryption() {
        let target_string = "String for RSA encryption and decryption test.";
        let rsa_generation_result = rsa_key_generation().unwrap();

        let rsa_key_pair = match rsa_generation_result {
            RsaResult::KeyPair(key_pair) => {
                key_pair
            }
            RsaResult::StringResult(_) => panic!("produced a string from encryption/decryption instead of a randomly generated key pair (test_rsa_encryption_and_decryption)"),
            RsaResult::BruteforceRSAResult(_) => panic!("somehow generated an RSA bruteforce result, while the encrypted string was desired (test_rsa_correct_input_handling)"),
        };

        println!("RSA key pair used: {:?}", rsa_key_pair);
        println!(
            "Randomly generated RSA public/private key modulus n: {}",
            rsa_key_pair.public_key_n
        );
        println!(
            "Randomly generated RSA public key exponent e: {}",
            rsa_key_pair.public_key_e
        );
        println!(
            "Randomly generated RSA private key exponent d: {}",
            rsa_key_pair.private_key_d
        );
        let encryption_result = rsa_encrypt(
            target_string,
            &rsa_key_pair.public_key_e,
            &rsa_key_pair.public_key_n,
        )
            .unwrap();

        let decryption_result = rsa_decrypt(
            &encryption_result,
            &rsa_key_pair.private_key_d,
            &rsa_key_pair.public_key_n,
        )
            .unwrap();

        println!(
            "Target string for RSA encryption and decryption: {}",
            target_string
        );
        println!("RSA encryption result: {}", encryption_result);
        println!("RSA decryption result: {}", decryption_result);

        assert_eq!(target_string, decryption_result);
    }

    // Test RSA brute force.
    #[test]
    fn test_rsa_bruteforce() {
        // Modulus length 5.
        // Randomly generated RSA public/private key modulus n: 30221
        // Randomly generated RSA public key exponent e: 3589
        // Randomly generated RSA private key exponent d: 2485

        // Modulus length 10. Took ~ 1 s.
        // Randomly generated RSA public/private key modulus n: 268970693
        // Randomly generated RSA public key exponent e: 85
        // Randomly generated RSA private key exponent d: 88590349

        // Modulus length 12. Took ~1 min to brute force.
        // Randomly generated RSA public/private key modulus n: 343791989081
        // Randomly generated RSA public key exponent e: 49
        // Randomly generated RSA private key exponent d: 154355050441

        // Modulus length 20.
        // Randomly generated RSA public/private key modulus n: 22136415989430223339
        // Randomly generated RSA public key exponent e: 48517897146637569
        // Randomly generated RSA private key exponent d: 1048878608988043305

        // Modulus length 46.
        // Randomly generated RSA public/private key modulus n: 2037436796323626547583399318666087453147364877
        // Randomly generated RSA public key exponent e: 43450959902815584332855827
        // Randomly generated RSA private key exponent d: 759919301373385809531821631701110587528750619

        // let target_modulus = ChonkerInt::from(30221);
        // let target_public_exponent = ChonkerInt::from(3589);
        let target_modulus = ChonkerInt::from(268970693);
        let target_public_exponent = ChonkerInt::from(85);
        // let target_modulus = ChonkerInt::from(22136415989430223339u128);
        // let target_public_exponent = ChonkerInt::from(48517897146637569u128);
        let thread_count = None; // Rely on default 8 threads/workers.

        let bruteforce_result = rsa_bruteforce(&target_public_exponent, &target_modulus, thread_count).unwrap();

        // Retest with a custom amount of workers/threads.
        let thread_count = Some(32);
        let bruteforce_result = rsa_bruteforce(&target_public_exponent, &target_modulus, thread_count).unwrap();

        let rsa_package = match bruteforce_result {
            RsaResult::BruteforceRSAResult(rsa_result) => rsa_result,
            _ => panic!(
                "error in the algorithm, did not compute a bruteforce result (test_rsa_bruteforce)"
            ),
        };

        println!("Bruteforce result package: {:?}", rsa_package);
        println!("Bruteforce prime p: {}", rsa_package.prime_p);
        println!("Bruteforce prime q: {}", rsa_package.prime_q);

        // let private_key_comparison = ChonkerInt::from(2485);
        let private_key_comparison = ChonkerInt::from(88590349);
        // let private_key_comparison = ChonkerInt::from(1048878608988043305u128);

        assert_eq!(rsa_package.public_key_n, target_modulus);
        assert_eq!(rsa_package.public_key_e, target_public_exponent);
        assert_eq!(rsa_package.private_key_d, private_key_comparison);
    }
}
