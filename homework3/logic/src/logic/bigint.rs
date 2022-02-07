use crate::logic::error::OperationError;

pub mod addition;
pub mod comparison;
pub mod conversion;
pub mod division;
pub mod exponentiation;
pub mod factor;
pub mod gcd;
pub mod modulus;
pub mod multiplication;
pub mod negation;
pub mod prime;
pub mod randomisation;
pub mod subtraction;

// Definitions for a custom BigInt.
// This module contains definition of the BigInt struct,
// supporting structs/enums, constants, functions, some general methods and traits.
// This file also contains all the code (commented out) in one place before refactor, modification
// and split into separate modulus.

// Inspired from a Medium post: https://medium.com/@sukantk3.4/arbitrary-precision-arithmetic-1c3f9737734
// Paper on Bernikel Zielger’s recursive division algorithm https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.565&rep=rep1&type=pdf
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣧⠀⠀⠀⢰⡿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡟⡆⠀⠀⣿⡇⢻⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠀⣿⠀⢰⣿⡇⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡄⢸⠀⢸⣿⡇⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⡇⢸⡄⠸⣿⡇⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⢸⡅⠀⣿⢠⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣥⣾⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⡿⡿⣿⣿⡿⡅⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠉⠀⠉⡙⢔⠛⣟⢋⠦⢵⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣄⠀⠀⠁⣿⣯⡥⠃⠀⢳⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⡇⠀⠀⠀⠐⠠⠊⢀⠀⢸⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⡿⠀⠀⠀⠀⠀⠈⠁⠀⠀⠘⣿⣄⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣷⡀⠀⠀⠀
// ⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣧⠀⠀
// ⠀⠀⠀⡜⣭⠤⢍⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⢛⢭⣗⠀
// ⠀⠀⠀⠁⠈⠀⠀⣀⠝⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠠⠀⠀⠰⡅
// ⠀⠀⠀⢀⠀⠀⡀⠡⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠔⠠⡕⠀
// ⠀⠀⠀⠀⣿⣷⣶⠒⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠀⠀⠀⠀
// ⠀⠀⠀⠀⠘⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠈⢿⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠊⠉⢆⠀⠀⠀⠀
// ⠀⢀⠤⠀⠀⢤⣤⣽⣿⣿⣦⣀⢀⡠⢤⡤⠄⠀⠒⠀⠁⠀⠀⠀⢘⠔⠀⠀⠀⠀
// ⠀⠀⠀⡐⠈⠁⠈⠛⣛⠿⠟⠑⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠉⠑⠒⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀

// Constant declaring radix/base of separate digits in the BigInt's vector;
// and value difference between decimal numbers and their ASCII character representation.
const RADIX: i8 = 10;
const ASCII_DIFF: i8 = 48;

// Enumeration determining BigInt's sign.
#[derive(Debug, PartialEq, Eq)]
pub enum BigIntSign {
    Positive,
    Zero,
    Negative,
}

// Define BigInt struct, storing separate digits in 1 byte signed integers in an array,
// in a little endian format.
#[derive(Debug, PartialEq, Eq)]
pub struct ChonkerInt {
    digits: Vec<i8>,
    sign: BigIntSign,
}

// Retrieve remainder of the digit after adjusting it to radix.
fn clip(digit: i8) -> i8 {
    digit.rem_euclid(RADIX)
}

// Retrieve overflow of the digit.
fn overflow(digit: i8) -> i8 {
    (digit - clip(digit)) / RADIX
}

// Implement methods for BigInt.
impl ChonkerInt {
    // Initialize an empty BigInt.
    pub fn new() -> ChonkerInt {
        let sign = BigIntSign::Zero;
        let digits = vec![];

        ChonkerInt { digits, sign }
    }

    // Get an immutable reference to the internal vector of digits.
    pub fn get_vec(&self) -> &[i8] {
        &self.digits
    }

    // Get an immutable reference to the internal sign value.
    pub fn get_sign(&self) -> &BigIntSign {
        &self.sign
    }

    // Normalize BigInt, remove leading zeros.
    fn normalize(&mut self) {
        // Check if there are any digits in the vector.
        if self.digits.is_empty() {
            return;
        }

        let mut digit_index = self.digits.len() - 1;

        while self.digits[digit_index] == 0 {
            self.digits.remove(digit_index);
            if digit_index == 0 {
                return;
            }
            digit_index -= 1;
        }
    }

    // Push a new digit to the vector of digits.
    fn push(&mut self, digit: i8) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the digit for insertion is in a valid range of 0-9.
        match digit {
            0..=9 => {
                self.digits.push(digit);
                Ok(())
            }
            _ => Err(Box::new(OperationError::new("Did not receive a correct digit for insertion into the BigInt's vector. Allowed values are in range of 0-9."))),
        }
    }

    // Push vector of digits.
    // Note: leading zeros are allowed.
    fn push_vec(&mut self, digits_slice: &[i8]) {
        // Check if the vector is added to an empty/zero BigInt.
        if *self == ChonkerInt::new() {
            self.set_positive_sign();
        }

        let mut digits_vec = Vec::from(digits_slice);

        self.digits.append(&mut digits_vec);
    }

    // Set a positive sign.
    fn set_negative_sign(&mut self) {
        self.sign = BigIntSign::Negative;
    }

    // Set a negative sign.
    fn set_positive_sign(&mut self) {
        self.sign = BigIntSign::Positive;
    }

    // Splice/concatenate vectors of two BigInts.
    fn splice(&mut self, mut other: ChonkerInt) {
        // Check if the BigInt is added to an empty/zero BigInt.
        if *self == ChonkerInt::new() {
            match other.sign {
                BigIntSign::Positive => self.set_positive_sign(),
                BigIntSign::Negative => self.set_negative_sign(),
                _ => (),
            }
        }

        self.digits.append(&mut other.digits);
    }
}

// Implement default value for BigInt. It is empty/zero by default.
impl Default for ChonkerInt {
    fn default() -> Self {
        Self::new()
    }
}

// Implement cloning of the BigInt.
impl Clone for ChonkerInt {
    fn clone(&self) -> Self {
        ChonkerInt {
            digits: self.digits.clone(),
            sign: match self.sign {
                BigIntSign::Positive => BigIntSign::Positive,
                BigIntSign::Zero => BigIntSign::Zero,
                BigIntSign::Negative => BigIntSign::Negative,
            },
        }
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::{BigIntSign, ChonkerInt};

    // Test creation/construction of an empty/zero BigInt.
    #[test]
    fn test_empty_bigint_construction() {
        let bigint = ChonkerInt::new();

        let comparison_empty_digits_vector: Vec<i8> = vec![];

        // Check digit vector contents, the vector should be empty.
        assert_eq!(*bigint.get_vec(), comparison_empty_digits_vector);

        // Check sign value, the sign value should be BigIntSign::Zero.
        assert_eq!(*bigint.get_sign(), BigIntSign::Zero);
    }

    // Test retrieval of a reference to the BigInt's vector of digits.
    #[test]
    fn test_bigint_digits_vector_retrieval() {
        let bigint = ChonkerInt::from(String::from("123"));

        // Note, digits in the BigInt are stored in little endian format,
        // when requested for reference, vector is not adjusted for big endian format.
        let comparison_digits_vector: Vec<i8> = vec![3, 2, 1];

        assert_eq!(*bigint.get_vec(), comparison_digits_vector);
    }

    // Test retrieval of a reference to the BigInt's sign.
    #[test]
    fn test_bigint_sign_retrieval() {
        let positive_bigint = ChonkerInt::from(String::from("1"));
        let negative_bigint = ChonkerInt::from(String::from("-1"));
        let zero_bigint = ChonkerInt::from(String::from("0"));

        assert_eq!(*positive_bigint.get_sign(), BigIntSign::Positive);
        assert_eq!(*negative_bigint.get_sign(), BigIntSign::Negative);
        assert_eq!(*zero_bigint.get_sign(), BigIntSign::Zero);
    }

    // Test BigInt normalization and digit insertion.
    #[test]
    fn test_bigint_normalization_and_insertion() {
        let mut bigint = ChonkerInt::from(String::from("123"));
        let comparison_bigint = ChonkerInt::from(String::from("123"));
        // Add leading zeros to the vector of digits.
        for _x in 0..10 {
            let _ = bigint.push(0);
        }
        // Normalize BigInt to remove leading zeros.
        bigint.normalize();

        assert_eq!(bigint, comparison_bigint);
    }

    // Test BigInt vector/slice insertion.
    #[test]
    fn test_bigint_vector_insertion() {
        let mut bigint = ChonkerInt::from(12345);
        let mut bigint_empty = ChonkerInt::from(0);

        // Note: leading zeros are preserved, the result is not normalized.
        let digits_vector = vec![1, 2, 3, 0];

        // Mind little endian.
        bigint.push_vec(&digits_vector);
        bigint_empty.push_vec(&digits_vector);

        let mut comparison_bigint1 = ChonkerInt::new();
        let comparison_digits_vector = vec![5, 4, 3, 2, 1, 1, 2, 3, 0];
        comparison_bigint1.push_vec(&comparison_digits_vector);
        comparison_bigint1.set_positive_sign();

        let mut comparison_bigint2 = ChonkerInt::new();
        comparison_bigint2.push_vec(&digits_vector);
        comparison_bigint2.set_positive_sign();

        assert_eq!(bigint, comparison_bigint1);
        assert_eq!(bigint_empty, comparison_bigint2);
    }

    // Test of BigInt's change of signs.
    #[test]
    fn test_bigint_sign_change() {
        // Check transition of negative to positive.
        let negative_bigint = ChonkerInt::from(String::from("-100"));
        let mut negated_negative_bigint = ChonkerInt::from(String::from("-100"));
        negated_negative_bigint.set_positive_sign();

        // Check transition of positive to negative.
        let positive_bigint = ChonkerInt::from(String::from("100"));
        let mut negated_positive_bigint = ChonkerInt::from(String::from("100"));
        negated_positive_bigint.set_negative_sign();

        // Check transition of neutral zero to positive zero.
        let mut positivized_zero_bigint = ChonkerInt::from(String::from("0"));
        positivized_zero_bigint.set_positive_sign();

        // Check transition of neutral zero to negative zero.
        let mut negated_zero_bigint = ChonkerInt::from(String::from("0"));
        negated_zero_bigint.set_negative_sign();

        // Check signs of non-zero numbers.
        assert_eq!(negative_bigint, negated_positive_bigint);
        assert_eq!(positive_bigint, negated_negative_bigint);

        // Check signs of zeros.
        assert_eq!(*positivized_zero_bigint.get_sign(), BigIntSign::Positive);
        assert_eq!(*negated_zero_bigint.get_sign(), BigIntSign::Negative);
    }

    // Test BigInt default value generation.
    #[test]
    fn test_bigint_default() {
        let bigint_default: ChonkerInt = Default::default();
        let bigint_comparison = ChonkerInt::new();

        assert_eq!(bigint_default, bigint_comparison);
    }

    // Test BigInt cloning.
    #[test]
    fn test_bigint_cloning() {
        let original_bigint_positive = ChonkerInt::from(String::from("123123123"));
        let original_bigint_negative = ChonkerInt::from(-123123123);
        let original_bigint_empty = ChonkerInt::new();

        // Create clones with deep copying.
        let clone_bigint_positive = original_bigint_positive.clone();
        let clone_bigint_negative = original_bigint_negative.clone();
        let clone_bigint_empty = original_bigint_empty.clone();

        assert_eq!(original_bigint_positive, clone_bigint_positive);
        assert_eq!(original_bigint_negative, clone_bigint_negative);
        assert_eq!(original_bigint_empty, clone_bigint_empty);
    }

    // Test BigInt splicing/concatenation.
    #[test]
    fn test_bigint_splicing() {
        let mut target_bigint1 = ChonkerInt::from(123);
        let mut target_bigint2 = ChonkerInt::from(101);
        let mut target_bigint_empty = ChonkerInt::from(0);

        let target_for_splicing_clone0 = ChonkerInt::from(45);
        let target_for_splicing_clone1 = ChonkerInt::from(01);
        let target_for_splicing_clone3 = ChonkerInt::from(0);

        // Mind little endian.
        target_bigint1.splice(target_for_splicing_clone0);
        target_bigint2.splice(target_for_splicing_clone3);
        target_bigint_empty.splice(target_for_splicing_clone1);

        assert_eq!(target_bigint1, ChonkerInt::from(45123));
        assert_eq!(target_bigint2, ChonkerInt::from(101));
        assert_eq!(target_bigint_empty, ChonkerInt::from(1));
    }
}

// // Constant declaring radix/base of separate digits in the BigInt's vector;
// // and value difference between decimal numbers and their ASCII character representation.
// const RADIX: i8 = 10;
// const ASCII_DIFF: i8 = 48;
//
// // Enumeration determining BigInt's sign.
// #[derive(Debug, PartialEq, Eq)]
// enum BigIntSign {
//     Positive,
//     Zero,
//     Negative,
// }
//
// // Define BigInt struct, storing separate digits in 1 byte signed integers in an array,
// // in a little endian format.
// #[derive(Debug, PartialEq, Eq)]
// struct ChonkerInt {
//     digits: Vec<i8>,
//     sign: BigIntSign,
// }
//
// // Retrieve remainder of the digit after adjusting it to radix.
// fn clip(digit: i8) -> i8 {
//     digit.rem_euclid(RADIX)
// }
//
// // Retrieve overflow of the digit.
// fn overflow(digit: i8) -> i8 {
//     (digit - clip(digit)) / RADIX
// }
//
// // Conversion of an integer into BigInt.
// fn digit_convert(int: &u128) -> Result<Vec<i8>, Box<dyn std::error::Error>> {
//     let mut base = 10;
//     let max_base_value = (10 as u128).pow(38);
//     let mut prev_base = 10;
//     let mut remainder = *int % base;
//     let mut prev_remainder = remainder;
//     let mut digit;
//     // Create a result vector and add the first element.
//     let mut result_vec: Vec<i8> = vec![remainder as i8];
//
//     base *= 10;
//     while remainder != *int {
//         // Calculate remainder with the current base and then calculate the target digit with it.
//         remainder = *int % base;
//         digit = (remainder - prev_remainder) / prev_base;
//
//         result_vec.push(digit as i8);
//
//         prev_remainder = remainder;
//         prev_base = base;
//
//         // Check if the base hit the limit to prevent overflow.
//         if base == max_base_value {
//             return Err(Box::new(OperationError::new(
//                 "Hit the possible overflow during conversion of an integer into a BigInt.",
//             )));
//         }
//
//         base *= 10;
//     }
//
//     Ok(result_vec)
// }
//
// // Implement methods for BigInt.
// impl ChonkerInt {
//     // Initialize an empty BigInt.
//     fn new() -> ChonkerInt {
//         let sign = BigIntSign::Zero;
//         let digits = vec![];
//
//         ChonkerInt { digits, sign }
//     }
//
//     // Initialize a randomly filled BigInt.
//     fn new_rand(length: &u64, sign: &BigIntSign) -> ChonkerInt {
//         if *length == 0 {
//             panic!("requested length for random bigint generation is 0, nothing to generate");
//         }
//
//         let mut rng = rand::thread_rng();
//         let mut bigint = ChonkerInt::new();
//
//         // Assign the requested sign.
//         match *sign {
//             BigIntSign::Positive => bigint.set_positive_sign(),
//             BigIntSign::Negative => bigint.set_negative_sign(),
//             BigIntSign::Zero => panic!("zeros are not randomly generated"),
//         }
//
//         let main_length = *length - 1;
//         let mut digit: i8;
//
//         // Fill the empty BigInt with the requested amount of random digits in the range of 0-9.
//         for _iteration in 0..main_length {
//             digit = rng.gen_range(0..=9);
//             bigint.push(digit);
//         }
//
//         // Ensure that the leading/last digit is not zero. Generate it separately.
//         digit = rng.gen_range(1..=9);
//         bigint.push(digit);
//
//         bigint
//     }
//
//     // Initialize a randomly filled BigInt from the provided range of lengths.
//     fn new_rand_range_len(start: &u64, end: &u64, sign: &BigIntSign) -> ChonkerInt {
//         if *start == 0 || *end == 0 {
//             panic!("start or end length boundary for the random BigInt generation is zero, nothing to generate (ChonkerInt::new_rand_range_len)");
//         }
//
//         if *start > *end {
//             panic!("provided incorrect boundaries for the random BigInt generation, starting boundary must be lower or equal to the ending one (ChonkerInt::new_rand_range_len)");
//         }
//
//         let mut rng = rand::thread_rng();
//         let mut bigint = ChonkerInt::new();
//
//         // Assign requested sign.
//         match *sign {
//             BigIntSign::Positive => bigint.set_positive_sign(),
//             BigIntSign::Negative => bigint.set_negative_sign(),
//             BigIntSign::Zero => panic!("zeros are not randomly generated"),
//         }
//
//         // Randomly generate the length of the BigInt from the provided range.
//         let main_length = (rng.gen_range((*start)..=(*end))) - 1;
//
//         let mut digit: i8;
//
//         // Fill the empty BigInt with the requested amount of random digits in the range of 0-9.
//         for _iteration in 0..main_length {
//             digit = rng.gen_range(0..=9);
//             bigint.push(digit);
//         }
//
//         // Ensure that the leading/last digit is not zero. Generate it separately.
//         digit = rng.gen_range(1..=9);
//         bigint.push(digit);
//
//         bigint
//     }
//
//     // Initialize a randomly filled BigInt from the provided range of lengths.
//     fn new_rand_range_value(start: &ChonkerInt, end: &ChonkerInt, sign: &BigIntSign) -> ChonkerInt {
//         let big_zero = ChonkerInt::new();
//
//         // Check if either of boundaries is zero.
//         if *start == big_zero || *end == big_zero {
//             panic!("start or end length boundary for the random BigInt generation is zero, nothing to generate (ChonkerInt::new_rand_range_value)");
//         }
//
//         // Check if either of boundaries is negative.
//         if *start < big_zero || *end < big_zero {
//             panic!("start or end length boundary for the random BigInt generation is negative, nothing to generate (ChonkerInt::new_rand_range_value)");
//         }
//
//         // Check if starting boundary is bigger than the ending boundary.
//         if *start >= *end {
//             panic!("provided incorrect boundaries for the random BigInt generation, starting boundary must be lower and not equal to the ending one (ChonkerInt::new_rand_range_value)");
//         }
//
//         // Check requested sign.
//         if (*sign) == BigIntSign::Zero {
//             panic!("zeros are not randomly generated");
//         }
//
//         let mut rng = rand::thread_rng();
//         let mut bigint = ChonkerInt::new();
//         bigint.set_positive_sign();
//
//         // Randomly generate the BigInt from the provided range of values.
//         loop {
//             bigint = ChonkerInt::new_rand_range_len(&(start.digits.len() as u64), &(end.digits.len() as u64), &BigIntSign::Positive);
//
//             // Check if the generated value is in between the requested boundaries.
//             if (bigint > (*start)) && (bigint < (*end)) {
//                 // Assign requested sign.
//                 match *sign {
//                     BigIntSign::Positive => bigint.set_positive_sign(),
//                     BigIntSign::Negative => bigint.set_negative_sign(),
//                     _ => (),
//                 }
//
//                 return bigint;
//             } else {
//                 bigint = ChonkerInt::new();
//                 bigint.set_positive_sign();
//             }
//         }
//     }
//
//     // Initialize a randomly filled prime BigInt.
//     // Test for primality is based on the Miller-Rabin probabilistic test. 10 trials are done.
//     fn new_prime(length: &u64) -> ChonkerInt {
//         if *length == 0 {
//             panic!("requested length for random bigint generation is 0, nothing to generate");
//         }
//
//         let mut rng = rand::thread_rng();
//         let mut bigint = ChonkerInt::new();
//         bigint.set_positive_sign();
//
//         let main_length = *length - 2;
//         let mut digit: i8;
//         let least_significant_candidates: Vec<i8> = vec![1, 3, 5, 7, 9];
//
//         loop {
//             // Ensure that the produced BigInt is odd, by limiting the least significant values to odd ones:
//             // 1, 3, 5, 7, 9.
//             digit = *(least_significant_candidates.choose(&mut rng).unwrap());
//             bigint.push(digit);
//
//             // Fill the empty BigInt with the requested amount of random digits in the range of 0-9.
//             for _iteration in 0..main_length {
//                 digit = rng.gen_range(0..=9);
//                 bigint.push(digit);
//             }
//             // Ensure that the leading/last digit is not zero. Generate it separately.
//             digit = rng.gen_range(1..=9);
//             bigint.push(digit);
//
//             if bigint.is_prime_probabilistic(Some(10)) {
//                 break;
//             } else {
//                 bigint = ChonkerInt::new();
//                 bigint.set_positive_sign();
//             }
//         }
//
//         bigint
//     }
//
//     // Implement exponentiation operation.
//     // It is accomplished with the use of exponentiation by squaring algorithm, in an iterative form.
//     // More about the idea: https://en.wikipedia.org/wiki/Exponentiation_by_squaring
//     fn pow(&self, power: &ChonkerInt) -> ChonkerInt {
//         let mut base = (*self).clone();
//         let mut power = (*power).clone();
//         let zero_bigint = ChonkerInt::new();
//         let big_one = ChonkerInt::from(1);
//         let big_two = ChonkerInt::from(2);
//
//         // If the base is zero, return zero.
//         if *self == zero_bigint {
//             return zero_bigint;
//         }
//
//         let mut result = ChonkerInt::from(1);
//
//         // Check if the power is zero, one, positive or negative and take according action.
//         if power == zero_bigint {
//             return big_one;
//         } else if power == big_one {
//             return (*self).clone();
//         } else if power > zero_bigint {
//             while power > zero_bigint {
//                 // If the power is odd, split it in half and multiply base by itself.
//                 if &power % &big_two == big_one {
//                     result = &result * &base;
//                 }
//
//                 base = &base * &base;
//                 power = &power / &big_two;
//             }
//         } else if power < zero_bigint {
//             return zero_bigint;
//         }
//
//         result
//     }
//
//     // Implement modular exponentiation with Right-to-left binary which includes memory efficient method.
//     fn modpow(&self, power: &ChonkerInt, modulus: &ChonkerInt) -> ChonkerInt {
//         let mut base = (*self).clone();
//         let mut power = (*power).clone();
//         let modulus = (*modulus).clone();
//
//         let zero_bigint = ChonkerInt::new();
//         let big_one = ChonkerInt::from(1);
//         let big_two = ChonkerInt::from(2);
//
//         // If the base is zero, return zero.
//         if *self == zero_bigint {
//             return zero_bigint;
//         }
//
//         let mut result = ChonkerInt::from(1);
//         base = &base % &modulus;
//
//         // Check if the power is zero, one, positive or negative and take according action.
//         if power == zero_bigint {
//             return big_one;
//         } else if power == big_one {
//             return (*self).clone();
//         } else if power > zero_bigint {
//
//             loop {
//                 if (&power % &big_two) == big_one {
//                     result = &result * &base;
//                     result = &result % &modulus;
//                 }
//
//                 if power == big_one {
//                     return result;
//                 }
//
//                 power = &power / &big_two;
//                 base = &base * &base;
//                 base = &base % &modulus;
//             }
//         } else if power < zero_bigint {
//             return zero_bigint;
//         }
//
//         result
//     }
//
//     // Get an immutable reference to the internal vector of digits.
//     fn get_vec(&self) -> &[i8] {
//         &self.digits
//     }
//
//     // Get an immutable reference to the internal sign value.
//     fn get_sign(&self) -> &BigIntSign {
//         &self.sign
//     }
//
//     // Normalize BigInt, remove leading zeros.
//     fn normalize(&mut self) {
//         // Check if there are any digits in the vector.
//         if self.digits.len() == 0 {
//             return;
//         }
//
//         let mut digit_index = self.digits.len() - 1;
//
//         while self.digits[digit_index] == 0 {
//             self.digits.remove(digit_index);
//             if digit_index == 0 {
//                 return;
//             }
//             digit_index -= 1;
//         }
//     }
//
//     // Push a new digit to the vector of digits.
//     fn push(&mut self, digit: i8) -> Result<(), Box<dyn std::error::Error>> {
//         // Check if the digit for insertion is in a valid range of 0-9.
//         match digit {
//             0..=9 => {
//                 self.digits.push(digit);
//                 Ok(())
//             }
//             _ => Err(Box::new(OperationError::new("Did not receive a correct digit for insertion into the BigInt's vector. Allowed values are in range of 0-9."))),
//         }
//     }
//
//     // Push vector of digits.
//     // Note: leading zeros are allowed.
//     fn push_vec(&mut self, digits_slice: &[i8]) {
//         // Check if the vector is added to an empty/zero BigInt.
//         if *self == ChonkerInt::new() {
//             self.set_positive_sign();
//         }
//
//         let mut digits_vec = Vec::from(digits_slice);
//
//         self.digits.append(&mut digits_vec);
//     }
//
//     // Set a positive sign.
//     fn set_negative_sign(&mut self) {
//         self.sign = BigIntSign::Negative;
//     }
//
//     // Set a negative sign.
//     fn set_positive_sign(&mut self) {
//         self.sign = BigIntSign::Positive;
//     }
//
//     // Splice/concatenate vectors of two BigInts.
//     fn splice(&mut self, mut other: ChonkerInt) {
//         // Check if the BigInt is added to an empty/zero BigInt.
//         if *self == ChonkerInt::new() {
//             match other.sign {
//                 BigIntSign::Positive => self.set_positive_sign(),
//                 BigIntSign::Negative => self.set_negative_sign(),
//                 _ => (),
//             }
//         }
//
//         self.digits.append(&mut other.digits);
//     }
//
//     // Produce a string representing the number inside the BigInt.
//     fn to_string(&self) -> String {
//         // Check if the BigInt is zero.
//         if (*self) == ChonkerInt::new() {
//             return String::from("0");
//         }
//
//         let mut digits_vec = self.digits.clone();
//         digits_vec.reverse();
//
//         let modified_digits_vec: Vec<u8> = digits_vec
//             .iter_mut()
//             .map(|x| (*x + ASCII_DIFF) as u8)
//             .collect();
//
//         let digits_string = unsafe { from_utf8_unchecked(&modified_digits_vec) };
//
//         let mut result;
//         // If the BigInt is negative, add minus/hyphnen to the result string.
//         if self.sign == BigIntSign::Negative {
//             result = String::from(digits_string);
//             let minus = '-';
//             result.insert(0, minus);
//         } else {
//             result = String::from(digits_string);
//         }
//         result
//     }
//
//     // Check if this BigInt is a prime number, works only with the natural numbers.
//     // Returns true, when the number is a prime one, false otherwise.
//     // Time complexity is O(sqrt(N)), check of the even numbers is skipped.
//     fn is_prime(&self) -> bool {
//         // Return false if the BigInt is negative, zero or one.
//         if *self == ChonkerInt::from(1) {
//             return false;
//         } else if *self == ChonkerInt::new() || self.sign == BigIntSign::Zero {
//             return false;
//         } else if self.sign == BigIntSign::Negative {
//             return false;
//         }
//
//         let mut factor = ChonkerInt::from(5);
//         let big_zero = ChonkerInt::new();
//         let big_two = ChonkerInt::from(2);
//         let big_three = ChonkerInt::from(3);
//         let big_six = ChonkerInt::from(6);
//
//         // Check if the target number is 2 or 3, which are primes.
//         if (*self == big_two) || (*self == big_three) {
//             return true;
//         }
//
//         // Check if the target is even, divisible by even numbers,
//         // or if it is divisible by 3.
//         if (self % &big_two == big_zero) || (self % &big_three == big_zero) {
//             return false;
//         }
//
//         // Loop will cover odd values from 3 to sqrt(self)
//         // Equality is allowed for the cases, such as 4 * 4 = 16,
//         // when the target number may have a doubled factor.
//         while (&factor * &factor) <= (*self) {
//             // Check if the factor divides self without leaving a remainder.
//             if (self % &factor == big_zero) || (self % &(&factor + &big_two) == big_zero) {
//                 // This means that self has an odd factor in between 3 and sqrt(self),
//                 // so it is not a prime number.
//                 return false;
//             }
//
//             factor = &factor + &big_six;
//         }
//
//         true
//     }
//
//     // Miller - Rabin primality test. Bottle-necked by the exponentiation when big primes are checked.
//     // Running complexity is O(k log3n).
//     // More information: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
//     fn is_prime_probabilistic(&self, number_of_trials: Option<u64>) -> bool {
//         // A number of tests to run.
//         let number_of_trials = number_of_trials.unwrap_or(40);
//         let mut target_original = (*self).clone();
//
//         // Return false if the BigInt is negative, zero or one.
//         if *self == ChonkerInt::from(1) {
//             return false;
//         } else if *self == ChonkerInt::new() || self.sign == BigIntSign::Zero {
//             return false;
//         } else if self.sign == BigIntSign::Negative {
//             return false;
//         }
//
//         let mut factor = ChonkerInt::from(5);
//         let big_zero = ChonkerInt::new();
//         let big_one = ChonkerInt::from(1);
//         let big_two = ChonkerInt::from(2);
//         let big_three = ChonkerInt::from(3);
//         let big_four = ChonkerInt::from(4);
//         let big_six = ChonkerInt::from(6);
//
//         // Check if the target number is 2 or 3, which are primes.
//         if (*self == big_two) || (*self == big_three) {
//             return true;
//         }
//
//         // Check if the target is even, divisible by even numbers,
//         // or if it is divisible by 3.
//         if (self % &big_two == big_zero) || (self % &big_three == big_zero) {
//             return false;
//         }
//
//         // 2^s * d + 1 = n , d - odd; d = (n - 1) / 2^s
//         let target_one = &target_original - &big_one;
//         let mut d = target_one.clone();
//         let mut s = ChonkerInt::new();
//
//         while &d % &big_two == big_one {
//             d = &d / &big_two;
//             s = &s + &big_one;
//         }
//
//         let mut base;
//         let mut trial_result;
//
//         // Testing loop/witness loop.
//         'outer: for _iteration in 0..number_of_trials {
//             // Generate a random base, a possible witness or a liar, from the range 2 - (self - 2)
//             base = ChonkerInt::new_rand_range_value(&big_two, &(&target_original - &big_two), &BigIntSign::Positive);
//             trial_result = base.modpow(&d, &target_original);
//
//             // Check the trial result, if it is equals 1 or (self - 1), proceed to the next trials,
//             // otherwise continue the current one.
//             if (trial_result == big_one) || (trial_result == target_one)  {
//                 continue;
//             }
//
//             let mut s_clone = s.clone();
//
//             // Increase the base, take a power of 2 of the base, while decreasing the s exponent by one
//             // and take modulus of the original target.
//             // If the calculation result equals (self - 1), proceed to the next trial,
//             // otherwise the target is a composite number.
//             while s_clone > big_zero {
//                 trial_result = trial_result.modpow(&big_two, &target_original);
//                 if  trial_result == target_one {
//                     continue 'outer;
//                 }
//
//                 s_clone = &s_clone - &big_one;
//             }
//
//             return false;
//         }
//
//         true
//     }
//
//     // A recursive function to find the greatest common divisor.
//     fn gcd(&self, other: &ChonkerInt) -> ChonkerInt {
//         let big_zero = ChonkerInt::new();
//
//         // Check arguments for zeros.
//         if *self == big_zero || *other == big_zero {
//             return big_zero;
//         }
//
//         if *self < *other {
//             return other.gcd(self);
//         } else if (self % other) == big_zero {
//             return (*other).clone();
//         } else {
//             return other.gcd(&(self % other));
//         }
//     }
// }
//
// // Implement cloning of the BigInt.
// impl Clone for ChonkerInt {
//     fn clone(&self) -> Self {
//         ChonkerInt {
//             digits: self.digits.clone(),
//             sign: match self.sign {
//                 BigIntSign::Positive => BigIntSign::Positive,
//                 BigIntSign::Zero => BigIntSign::Zero,
//                 BigIntSign::Negative => BigIntSign::Negative,
//             },
//         }
//     }
// }
//
// // Convert a 16 byte signed integer into a BigInt.
// impl From<i128> for ChonkerInt {
//     fn from(int: i128) -> ChonkerInt {
//         // Check for zero.
//         if int == 0 {
//             return ChonkerInt::new();
//         }
//
//         let mut sign = BigIntSign::Zero;
//
//         // Determine the sign of the target and save it.
//         if int.is_negative() {
//             sign = BigIntSign::Negative;
//         } else {
//             sign = BigIntSign::Positive;
//         }
//
//         let unsigned_int = (int.abs()) as u128;
//
//         // Define BigInt's vector, check for the integer being zero.
//         let digits = if unsigned_int == 0 {
//             vec![]
//         } else {
//             digit_convert(&unsigned_int).unwrap()
//         };
//
//         ChonkerInt { digits, sign }
//     }
// }
//
// // Convert a string of digits signed integer into a BigInt.
// impl From<String> for ChonkerInt {
//     fn from(string: String) -> ChonkerInt {
//         let mut char_iter = string.chars();
//         let mut sign = BigIntSign::Positive;
//         let mut digits = string.clone();
//
//         // Check if the first character is minus/hyphen or a number.
//         // If so, proceed with further checking.
//         // If not, return an empty BigInt.
//         if let Some(char) = char_iter.next() {
//             if !(char == '-' || char.is_numeric()) {
//                 return ChonkerInt {
//                     digits: vec![],
//                     sign: BigIntSign::Zero,
//                 };
//             } else if char == '-' {
//                 // If the first char was negative and others numbers,
//                 // remove the first hyphen char and change sign's value to negative.
//                 sign = BigIntSign::Negative;
//                 digits.remove(0);
//             }
//         }
//
//         // Check if every other character is numeric.
//         // If the input string is incorrect, return an empty BigInt.
//         for char in char_iter {
//             if !char.is_numeric() {
//                 return ChonkerInt {
//                     digits: vec![],
//                     sign: BigIntSign::Zero,
//                 };
//             }
//         }
//
//         // Check for leading zeros, if present, remove them.
//         // Check if the first char was a minus/hyphen in the original string, if so, skip it.
//         let mut char_iter = string.chars();
//         if sign == BigIntSign::Negative {
//             char_iter.next();
//         }
//         for char in char_iter {
//             if char == '0' {
//                 digits.remove(0);
//             } else {
//                 break;
//             }
//         }
//
//         // Check if the string consisted only from zeros, if so, return an empty BigInt.
//         let digits_length = digits.len();
//         if digits_length == 0 {
//             return ChonkerInt {
//                 digits: vec![],
//                 sign: BigIntSign::Zero,
//             };
//         }
//
//         // Convert string into a vector/slice of one byte unsigned integers, containing digits of a numbers.
//         // Subtract 48 from ASCII/UTF-8 representation of integers to get true integers.
//         let mut digits: Vec<i8> = unsafe { digits.as_bytes_mut() }
//             .iter_mut()
//             .map(|x| (*x - (ASCII_DIFF as u8)) as i8)
//             .collect();
//         digits.reverse();
//
//         ChonkerInt { digits, sign }
//     }
// }
//
// // Implement comparison operators "<", "<=", ">", ">=" for the BigInt.
// impl PartialOrd for ChonkerInt {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         // Compare signs, if they are equal, proceed with length and content checks.
//         // If both are zero/empty, return equal ordering.
//         if self.digits.is_empty() && other.digits.is_empty() {
//             return Some(Ordering::Equal);
//         }
//
//         if self.sign == other.sign {
//             // If the lengths are equal compare separate digits in the vector.
//             // Reminder: digits are stored in little endian format.
//             if self.digits.len() == other.digits.len() {
//                 let mut self_char_index = self.digits.len() - 1;
//                 let mut other_char_index = other.digits.len() - 1;
//
//                 while self_char_index >= 0 {
//                     if self.digits[self_char_index] < other.digits[other_char_index] {
//                         return Some(Ordering::Less);
//                     } else if self.digits[self_char_index] > other.digits[other_char_index] {
//                         return Some(Ordering::Greater);
//                     }
//
//                     // Check for index being zero, if allowed,
//                     // to proceed an attempt at subtraction against unsigned zero would happen.
//                     if self_char_index == 0 {
//                         break;
//                     }
//
//                     self_char_index -= 1;
//                     other_char_index -= 1;
//                 }
//
//                 return Some(Ordering::Equal);
//                 // If the self length is less that other's return Ordering variant Less, Greater otherwise.
//             } else if self.digits.len() < other.digits.len() {
//                 return Some(Ordering::Less);
//             } else {
//                 return Some(Ordering::Greater);
//             }
//         } else if self.sign == BigIntSign::Negative
//             && (other.sign == BigIntSign::Positive || other.sign == BigIntSign::Zero)
//         {
//             return Some(Ordering::Less);
//         } else if self.sign == BigIntSign::Positive
//             && (other.sign == BigIntSign::Negative || other.sign == BigIntSign::Zero)
//         {
//             return Some(Ordering::Greater);
//         } else if self.sign == BigIntSign::Zero && other.sign == BigIntSign::Positive {
//             return Some(Ordering::Less);
//         } else if self.sign == BigIntSign::Zero && other.sign == BigIntSign::Negative {
//             return Some(Ordering::Greater);
//         }
//
//         // If the signs, lengths and digits were equal, return Ordering variant of Equal.
//         Some(Ordering::Equal)
//     }
// }
//
// // Implement addition "+" operator for the BigInt.
// impl<'a, 'b> Add<&'b ChonkerInt> for &'a ChonkerInt {
//     type Output = ChonkerInt;
//
//     fn add(self, other: &'b ChonkerInt) -> Self::Output {
//         // If self is empty/zero, return other BigInt.
//         if self.sign == BigIntSign::Zero {
//             return (*other).clone();
//         }
//
//         // If other is empty/zero, return self.
//         if other.sign == BigIntSign::Zero {
//             return (*self).clone();
//         }
//
//         // Check the signs of both operands, if they are not the same, determine an appropriate operation.
//         // If the signs are the same and negative, negate them and recall addition operation,
//         // then negate the retrieved result back to negative.
//         if self.sign != other.sign {
//             return if self.sign == BigIntSign::Positive {
//                 self - &(-other) // +self - (-other) = self - other
//             } else {
//                 other - &(-self) // other - (-self) = other - self
//             };
//         } else if self.sign == BigIntSign::Negative {
//             let sum_of_negated_bigints = &(-self) + &(-other);
//             let sum_ref: &ChonkerInt = &sum_of_negated_bigints;
//             let result: ChonkerInt = sum_ref.neg();
//             return result; // (-self) + (-other) = -(self + other)
//         }
//
//         let mut result = ChonkerInt::new();
//
//         let mut last_digit_overflow = 0;
//         let mut self_offset = 0;
//         let mut self_length = self.digits.len();
//         let mut other_offset = 0;
//         let mut other_length = other.digits.len();
//
//         // Determine the longer/bigger BigInt and calculate addition.
//         // If vector were of different lengths, finish operation on the digits of the longer vector/BigInt with a second loop.
//         if (*self > *other) || (*self == *other) {
//             while other_offset < other_length {
//                 add_digits(
//                     &self.digits,
//                     &other.digits,
//                     &mut self_offset,
//                     &mut other_offset,
//                     &mut result,
//                     &mut last_digit_overflow,
//                 );
//             }
//             while self_offset < self_length {
//                 add_digit_and_overflow(
//                     &self.digits,
//                     &mut self_offset,
//                     &mut result,
//                     &mut last_digit_overflow,
//                 );
//             }
//         } else {
//             while self_offset < self_length {
//                 add_digits(
//                     &self.digits,
//                     &other.digits,
//                     &mut self_offset,
//                     &mut other_offset,
//                     &mut result,
//                     &mut last_digit_overflow,
//                 );
//             }
//             while other_offset < other_length {
//                 add_digit_and_overflow(
//                     &other.digits,
//                     &mut other_offset,
//                     &mut result,
//                     &mut last_digit_overflow,
//                 );
//             }
//         }
//
//         // Check for a possible remaining overflow.
//         if last_digit_overflow > 0 {
//             result.push(last_digit_overflow);
//         }
//
//         // Set a positive sing of the resulting BigInt.
//         result.set_positive_sign();
//
//         result
//     }
// }
//
// // Addition of two passed digits.
// fn add_digits(
//     one_vec: &Vec<i8>,
//     other_vec: &Vec<i8>,
//     one_offset: &mut usize,
//     other_offset: &mut usize,
//     result: &mut ChonkerInt,
//     last_digit_overflow: &mut i8,
// ) {
//     // Calculate sum of digits.
//     let mut sum = (*one_vec)[*one_offset] + (*other_vec)[*other_offset] + (*last_digit_overflow);
//
//     // Check for the overflow.
//     *last_digit_overflow = overflow(sum);
//     sum = clip(sum);
//
//     result.push(sum);
//     *one_offset += 1;
//     *other_offset += 1;
// }
//
// // Addition of one passed digit and a result slot.
// fn add_digit_and_overflow(
//     one_vec: &Vec<i8>,
//     one_offset: &mut usize,
//     result: &mut ChonkerInt,
//     last_digit_overflow: &mut i8,
// ) {
//     // Calculate sum of digits.
//     let mut sum = (*one_vec)[*one_offset] + (*last_digit_overflow);
//
//     // Check for the overflow.
//     *last_digit_overflow = overflow(sum);
//     sum = clip(sum);
//
//     result.push(sum);
//     *one_offset += 1;
// }
//
// // Implement subtraction "-" operator for the BigInt.
// impl<'a, 'b> Sub<&'b ChonkerInt> for &'a ChonkerInt {
//     type Output = ChonkerInt;
//
//     fn sub(self, other: &'b ChonkerInt) -> Self::Output {
//         // If self is empty/zero, return other BigInt with a possible change of its sign.
//         if self.sign == BigIntSign::Zero {
//             if (other.sign == BigIntSign::Negative) || (other.sign == BigIntSign::Positive) {
//                 let mut other_clone = (*other).clone();
//                 let mut negated_clone = other_clone.neg();
//                 return negated_clone;
//             }
//         }
//
//         // If other is empty/zero, return self.
//         if other.sign == BigIntSign::Zero {
//             return (*self).clone();
//         }
//
//         // Check the signs of both operands, if they are not the same, determine an appropriate operation.
//         // If the signs are the same and negative, negate them and recall subtraction operation,
//         // then negate the retrieved result.
//         if self.sign != other.sign {
//             return if self.sign == BigIntSign::Positive {
//                 self + &(-other) // +self + (-(-other)) = self + other
//             } else {
//                 let sum_result: ChonkerInt = &(-self) + other;
//                 sum_result.neg()
//                 // -(&(-self) + other) // -((-(-self)) + other) = -(self + other)
//             };
//         } else if self.sign == BigIntSign::Negative {
//             let diff_result: ChonkerInt = &(-self) - &(-other);
//             return diff_result.neg();
//             // return -((-self) - (-other)); // -((-(-self)) - (-(-other))) = -(self - other)
//         }
//
//         let mut result = ChonkerInt::new();
//
//         let mut last_digit_underflow = 0;
//         let mut self_offset = 0;
//         let mut self_length = self.digits.len();
//         let mut other_offset = 0;
//         let mut other_length = other.digits.len();
//
//         // Determine the longer/bigger BigInt and calculate subtraction and a sign.
//         // If vector were of different lengths, finish operation on the digits of the longer vector/BigInt with a second loop.
//         if *self > *other {
//             while other_offset < other_length {
//                 subtract_digits(
//                     &self.digits,
//                     &other.digits,
//                     &mut self_offset,
//                     &mut other_offset,
//                     &mut result,
//                     &mut last_digit_underflow,
//                 );
//             }
//             while self_offset < self_length {
//                 subtract_digit_and_underflow(
//                     &self.digits,
//                     &mut self_offset,
//                     &mut result,
//                     &mut last_digit_underflow,
//                 );
//             }
//             result.set_positive_sign();
//         } else if *self < *other {
//             while self_offset < self_length {
//                 subtract_digits(
//                     &other.digits,
//                     &self.digits,
//                     &mut other_offset,
//                     &mut self_offset,
//                     &mut result,
//                     &mut last_digit_underflow,
//                 )
//             }
//             while other_offset < other_length {
//                 subtract_digit_and_underflow(
//                     &other.digits,
//                     &mut other_offset,
//                     &mut result,
//                     &mut last_digit_underflow,
//                 );
//             }
//             result.set_negative_sign();
//         } else {
//             // Both are equal, return and empty/zero BigInt.
//             return result;
//         }
//
//         result.normalize();
//
//         result
//     }
// }
//
// // Subtract of two passed digits.
// fn subtract_digits(
//     minuend_vec: &Vec<i8>,
//     subtrahend_vec: &Vec<i8>,
//     minuend_offset: &mut usize,
//     subtrahend_offset: &mut usize,
//     result: &mut ChonkerInt,
//     last_digit_underflow: &mut i8,
// ) {
//     // Calculate sum of digits.
//     let mut difference = (*minuend_vec)[*minuend_offset]
//         - (*subtrahend_vec)[*subtrahend_offset]
//         - (*last_digit_underflow);
//
//     // Nullify the underflow from previous operation after being used.
//     *last_digit_underflow = 0;
//
//     // Check for the difference being negative, if it is, set underflow.
//     if difference.is_negative() {
//         *last_digit_underflow = 1;
//         difference += RADIX;
//     }
//
//     result.push(difference);
//     *minuend_offset += 1;
//     *subtrahend_offset += 1;
// }
//
// // Subtract of one passed digit and a result slot.
// fn subtract_digit_and_underflow(
//     one_vec: &Vec<i8>,
//     one_offset: &mut usize,
//     result: &mut ChonkerInt,
//     last_digit_underflow: &mut i8,
// ) {
//     // Calculate sum of digits.
//     let mut difference = (*one_vec)[*one_offset] - (*last_digit_underflow);
//
//     // Nullify the underflow from previous operation after being used.
//     *last_digit_underflow = 0;
//
//     // Check for the difference being negative, if it is, set underflow.
//     if difference.is_negative() {
//         *last_digit_underflow = 1;
//         difference += RADIX;
//     }
//
//     result.push(difference);
//     *one_offset += 1;
// }
//
// // Implement multiplication "*" operator for the BigInt.
// impl<'a, 'b> Mul<&'b ChonkerInt> for &'a ChonkerInt {
//     // impl Mul for ChonkerInt {
//     type Output = ChonkerInt;
//     // type Output = Self;
//
//     fn mul(self, rhs: &'b ChonkerInt) -> Self::Output {
//         // fn mul(self, rhs: Self) -> Self::Output {
//
//         let positive_one = ChonkerInt::from(1);
//         let negative_one = ChonkerInt::from(-1);
//
//         // If self or rhs is empty/zero, return rhs empty/zero BigInt.
//         // If self or rhs is 1 or -1, return opposite value with, possibly, an opposite sign.
//         if self.sign == BigIntSign::Zero || rhs.sign == BigIntSign::Zero {
//             return ChonkerInt::new();
//         } else if *self == positive_one {
//             // If self multiplier is a positive 1, return rhs multiplier clone.
//             return (*rhs).clone();
//         } else if *self == negative_one {
//             // If self multiplier is a negative 1, return a negated rhs multiplier clone.
//             let mut negated_rhs = (*rhs).clone();
//             match negated_rhs.sign {
//                 BigIntSign::Positive => negated_rhs.sign = BigIntSign::Negative,
//                 BigIntSign::Negative => negated_rhs.sign = BigIntSign::Positive,
//                 BigIntSign::Zero => {
//                     panic!("error in the bigint multiplication, skipped zero check")
//                 }
//             }
//             return negated_rhs;
//         } else if *rhs == positive_one {
//             // If rhs multiplier is a positive 1, return self multiplier clone.
//             return (*self).clone();
//         } else if *rhs == negative_one {
//             // If rhs multiplier is a negative 1, return a negated self multiplier clone.
//             let mut negated_self = (*self).clone();
//             match negated_self.sign {
//                 BigIntSign::Positive => negated_self.sign = BigIntSign::Negative,
//                 BigIntSign::Negative => negated_self.sign = BigIntSign::Positive,
//                 BigIntSign::Zero => {
//                     panic!("error in the bigint multiplication, skipped zero check")
//                 }
//             }
//             return negated_self;
//         }
//
//         let mut result = ChonkerInt::new();
//
//         let mut last_digit_overflow = 0;
//         let mut self_offset = 0;
//         let mut self_length = self.digits.len();
//         let mut rhs_offset = 0;
//         let mut rhs_length = rhs.digits.len();
//         let mut partial_product_bigint: ChonkerInt;
//         let mut partial_product_digit: i8;
//
//         // Calculate intermediate/partial products and add them together to get the final product.
//         while rhs_offset < rhs_length {
//             // Calculate a partial product. Iterate one digit of the rhs BigInt over all digits of the self BigInt
//             // and shift the result with zeros.
//
//             // Construct a temporary BigInt for the partial product.
//             partial_product_bigint = ChonkerInt::new();
//             partial_product_bigint.set_positive_sign();
//
//             // println!("Constructed partial product bigint: {:?}", partial_product_bigint);
//
//             // Shift partial product by a number of zeros, which aligns with the iteration number over rhs BigInt.
//             for _x in 0..rhs_offset {
//                 partial_product_bigint.push(0);
//             }
//
//             // println!("Partial product bigint after shifting: {:?}", partial_product_bigint);
//
//             while self_offset < self_length {
//                 partial_product_digit =
//                     self.digits[self_offset] * rhs.digits[rhs_offset] + last_digit_overflow;
//
//                 // Check a partial product for overflow.
//                 last_digit_overflow = overflow(partial_product_digit);
//                 partial_product_digit = clip(partial_product_digit);
//
//                 partial_product_bigint.push(partial_product_digit);
//
//                 self_offset += 1;
//             }
//
//             // Reset index for the self BigInt, to iterate over it again during next iteration of the outer loop.
//             self_offset = 0;
//
//             // println!("Partial product bigint after product calculation: {:?}", partial_product_bigint);
//
//             // If there is an overflow add as a last digit.
//             // Considering that a number length after multiplication equals the sum of lengths of both operands,
//             // and the target is multiplied only by one digit at a time, the result will be longer only by 1 digit.
//             // Nullify overflow after its addition.
//             if last_digit_overflow > 0 {
//                 partial_product_bigint.push(last_digit_overflow);
//                 last_digit_overflow = 0;
//             }
//
//             // println!("Partial product bigint after final overflow addition: {:?}", partial_product_bigint);
//
//             // Add a partial product to the total final resulting product.
//             result = &result + &partial_product_bigint;
//
//             // println!("Product after addition of the partial product: {:?}\n\n", result);
//
//             rhs_offset += 1;
//         }
//
//         // Check the signs of both operands, if they are not the same, the resulting sign is negative.
//         // By default the sign is positive.
//         // If the signs are the same and positive, the resulting sign is positive.
//         // If the signs are the same and negative, the resulting sign is positive.
//         if self.sign != rhs.sign {
//             result.set_negative_sign();
//         }
//
//         result
//     }
// }
//
// // Implement division "/" operator for the BigInt
// // Division and remainder calculation were achieved with Quotient Estimation Algorithm,
// // based on on Lemma 2 from Bernikel Zielger’s recursive division algorithm paper.
// // Algorithm says that if A (the dividend) is at most l digits longer than B (the divisor) and B is normalized
// // (more than half the value if all digits were max. value, e.g. 444 is not normalized because it is less than 999/2).
// // A is also less than RADIX*B.
// // Quotient has to fit into one digit under the RADIX. In decimal system, allowed values would be in the range of 0-9.
// // Then you can estimate the quotient to be the division of A be B,
// // where we exclude the lowest k digits of both numbers (where k is B’s length minus l).
// // This estimate is at most two more than the actual quotient.
//
// // Example: In the decimal system: 378,546÷78,356.
// //
// //     378,546 is (atmost) 1 digit longer than 78,356 (l = 1).
// //     Hence, k = 5 – 1=4, where 5 = length(78,356) and 1= l.
// //     78,356 is normalized — it’s greater than half of 99999.
// //     Clearly, 378,546 is less than 10 times 78,356 (and 10 is our radix); hence, this is a valid use case for us.
// //
// // Hence, our quotient estimate will be division of 37/7 (excluding lower k digits) which is 5. Now,
// // 5 is at most 2 more than the actual quotient — hence, q can be 3, 4, or 5.
// //
// //     78,356*5 = 391,780 (wrong)
// //     78,356*4 = 313,424 (right, where remainder r is 65122)
// //     don’t check for 3 now — we found q already.
// //
// // Normalization of B is ensured with fractional equivalency.
// //
// // We need to multiply A and B by a number such that B becomes normalized, if not already.
// // That number is equal Math.floor(radix / (B._digits[0] + 1)) in code.
// // This ensures that the first digit of B is as high as it can get without increasing the number of digits.
//
// impl<'a, 'b> Div<&'b ChonkerInt> for &'a ChonkerInt {
//     type Output = ChonkerInt;
//
//     fn div(self, rhs: &'b ChonkerInt) -> Self::Output {
//         // Check for division by zero, if the divisor is zero, panic.
//         if *rhs == ChonkerInt::new() {
//             panic!("attempt to divide by zero (ChonkerInt::div())");
//         }
//
//         // Clone dividend and divisor, make them absolute for comparisons.
//         let mut absolute_dividend = (*self).clone();
//         absolute_dividend.set_positive_sign();
//         let mut absolute_divisor = (*rhs).clone();
//         absolute_divisor.set_positive_sign();
//
//         // Compare the lengths/values of the dividend and divisor.
//         // If self/dividend is smaller that the divisor, return empty/zero BigInt as a quotient/result.
//         // If self/dividend is equal to the divisor, return 1 or -1 as a BigInt.
//         // If self/dividend is bigger that the divisor, proceed with calculations.
//         if absolute_dividend < absolute_divisor {
//             return ChonkerInt::new();
//         } else if *self == *rhs && *self == absolute_divisor {
//             // If self/dividend and rhs/divisor are positive.
//             return ChonkerInt::from(1);
//         } else if *self != *rhs && *self == absolute_divisor {
//             // If self/dividend is positive, and rhs/divisor is negative.
//             return ChonkerInt::from(-1);
//         } else if *self == *rhs && *self != absolute_divisor {
//             // If self/dividend and rhs/divisor are negative.
//             return ChonkerInt::from(1);
//         } else if *self != *rhs && absolute_dividend == *rhs {
//             // If self/dividend is negative, and rhs/divisor is positive.
//             return ChonkerInt::from(-1);
//         }
//
//         let mut quotient = ChonkerInt::new();
//
//         let mut cut_dividend = ChonkerInt::new();
//         cut_dividend.set_positive_sign();
//
//         // Compare lengths of the dividend and divisor and normalization of the most significant digits.
//         // Length of the dividend should not be more than length of the divisor + 1.
//         // If it is, cut the dividend.
//         if (self.digits.len() > rhs.digits.len() + 1)
//             || (self.digits.len() == rhs.digits.len() + 1
//             && self.digits[self.digits.len() - 1] > rhs.digits[rhs.digits.len() - 1])
//             && (rhs.digits[rhs.digits.len() - 1] < 5)
//         {
//             // Cut the dividend to the smaller size of divisor's length with or without additional digit,
//             // calculate the quotient digit and the remainder that will be used as the dividend,
//             // and then repeat the process by adding digits from the original dividend to the remainders one by one in the loop.
//
//             let mut dividend_index = self.digits.len();
//             // let mut difference = rhs.digits.len() + 1;
//             let mut difference;
//
//             // Check normalization of the divisor's most significant digit and compare it to the divident's digit,
//             // if divisor's digit is not normalized, and it is smaller than the dividend's one,
//             // make the difference/required length for the cut dividend equal to the divisor's length;
//             // otherwise add 1 to it.
//             if (self.digits[self.digits.len() - 1] > rhs.digits[rhs.digits.len() - 1])
//                 && (rhs.digits[rhs.digits.len() - 1] < 5)
//             {
//                 difference = rhs.digits.len();
//             } else {
//                 difference = rhs.digits.len() + 1;
//             }
//
//             let cut_dividend_splice =
//                 &self.digits[(dividend_index - difference)..=(dividend_index - 1)];
//
//             for digit in cut_dividend_splice.iter().rev() {
//                 cut_dividend.digits.insert(0, *digit);
//             }
//
//             let (mut quotient_digit, remainder_digit) =
//                 quotient_estimation_algorithm(&cut_dividend, rhs);
//
//             if quotient_digit.digits.len() > 1 {
//                 quotient_digit.digits.reverse();
//             }
//
//             quotient.push_vec(&quotient_digit.digits);
//
//             cut_dividend = remainder_digit.clone();
//
//             dividend_index -= difference;
//
//             difference = 1;
//
//             // Loop over the dividend's digits one by one and add them to the remainder from the previous iteration.
//             while dividend_index > 0 {
//                 let cut_dividend_splice =
//                     &self.digits[(dividend_index - difference)..=(dividend_index - 1)];
//
//                 // Add remaining digits form the dividend to the remainder from previous division operation.
//                 // Preserve little endian from the dividend with reverse of the iterator over the dividend.
//                 for digit in cut_dividend_splice.iter().rev() {
//                     cut_dividend.digits.insert(0, *digit);
//                 }
//                 cut_dividend.set_positive_sign();
//                 cut_dividend.normalize();
//
//                 // Check if the dividend became shorter/smaller than the divisor, if so,
//                 // add a quotient digit of zero and proceed to the next iteration.
//                 if cut_dividend < absolute_divisor {
//                     quotient.push(0);
//                     dividend_index -= difference;
//                     continue;
//                 }
//
//                 let (mut quotient_digit, remainder_digit) =
//                     quotient_estimation_algorithm(&cut_dividend, rhs);
//
//                 // If quotient has several digit, then it stored in the little endian by default.
//                 // But separate quotient digits are stored in the big endian format.
//                 // As the result, if the partial quotient has several digits by itself,
//                 // it has to be reversed.
//                 if quotient_digit.digits.len() > 1 {
//                     quotient_digit.digits.reverse();
//                 }
//
//                 // Save the quotient digit.
//                 quotient.push_vec(&quotient_digit.digits);
//
//                 // Use the remainder digit(-s) for the new partial dividend.
//                 // Reverse digits from little endian to big endian.
//                 cut_dividend = remainder_digit.clone();
//
//                 dividend_index -= difference;
//             }
//
//             // Digits of the quotient were stored in big endian during calculation, reverse the vector of digits.
//             quotient.digits.reverse();
//         } else {
//             // If lengths of dividend and divisor are equal or have a difference of 1.
//             let (quotient_digit, _remainder_digit) = quotient_estimation_algorithm(self, rhs);
//             // Save the quotient digit.
//             quotient.push_vec(&quotient_digit.digits);
//         }
//
//         // Determine the sign of the quotient.
//         // Check the signs of both operands, if they are not the same, the resulting sign is negative.
//         // By default the sign is zero, check if it should be change wiht the digits vector's length.
//         // If the signs are the same and positive, the resulting sign is positive.
//         // If the signs are the same and negative, the resulting sign is positive.
//         if quotient.digits.len() > 0 {
//             if self.sign != rhs.sign {
//                 quotient.set_negative_sign();
//             } else {
//                 quotient.set_positive_sign();
//             }
//         }
//
//         quotient
//     }
// }
//
// // Calculate division or modulus depending on the mode.
// // Dividend should be bigger than the divisor, thus dividend should be longer or equal in length to the divisor.
// // The function returns separate digits of the quotient or the remainder,
// // in form of a tuple: (quotient, remainder)
// fn quotient_estimation_algorithm(
//     dividend: &ChonkerInt,
//     divisor: &ChonkerInt,
// ) -> (ChonkerInt, ChonkerInt) {
//     // Make dividends and divisors absolute, positive.
//     let mut dividend_original = (*dividend).clone();
//     dividend_original.set_positive_sign();
//     let mut dividend = (*dividend).clone();
//     dividend.set_positive_sign();
//     let mut divisor_original = (*divisor).clone();
//     divisor_original.set_positive_sign();
//     let mut divisor = (*divisor).clone();
//     divisor.set_positive_sign();
//
//     // Normalize divisor and calculate the coefficient for the fractional equivalency.
//     let coefficient = RADIX / (divisor.digits[divisor.digits.len() - 1] + 1);
//
//     // Check if the calculated equivalency bigger than 1, if it is,
//     // use it to increase dividend and divisor.
//     if coefficient > 1 {
//         let bigint_coefficient = ChonkerInt::from(coefficient as i128);
//         dividend = &dividend * &bigint_coefficient;
//         divisor = &divisor * &bigint_coefficient;
//     }
//
//     let mut quotient;
//     let remainder;
//
//     // Calculate quotient estimate. If dividend's length is equal or longer by 1 than the divisor's.
//     if (dividend.digits.len()) == (divisor.digits.len() + 1) {
//         quotient = ChonkerInt::from(
//             ((dividend.digits[dividend.digits.len() - 1] * RADIX
//                 + dividend.digits[dividend.digits.len() - 2])
//                 / divisor.digits[divisor.digits.len() - 1]) as i128,
//         );
//     } else if dividend.digits.len() == divisor.digits.len() {
//         quotient = ChonkerInt::from(
//             (dividend.digits[dividend.digits.len() - 1] / divisor.digits[divisor.digits.len() - 1])
//                 as i128,
//         );
//     } else {
//         panic!("dividend has to be equal in length or longer by 1 than the divisor (division_or_modulus_calculation)")
//     }
//
//     // Calculate a product between calculated quotient modified divisor.
//     let mut check_quotient_product = &quotient * &divisor_original;
//
//     // Create an empty/zero BigInt for comparison with the estimated quotient,
//     // if they are equal, create a BigInt, containing zero and return as the quotient.
//     let empty_bigint = ChonkerInt::new();
//
//     // Check the quotient estimate, if it does not suit, reduce it by 1 up to 3 times.
//     // If it does suit, exit the loop.
//     for delta in 0..=3 {
//         check_quotient_product.set_positive_sign();
//         if quotient == empty_bigint {
//             break;
//         }
//         // if check_quotient_product <= dividend  {
//         if check_quotient_product <= dividend_original {
//             break;
//         }
//
//         quotient = &quotient - &ChonkerInt::from(1);
//         check_quotient_product = &check_quotient_product - &divisor_original;
//
//         // There is an error in the algorithm, if after three reductions in the quotient,
//         // quotient is still unsuitable and we reach this point.
//         if delta == 3 {
//             panic!("error with the division algorithm (division_or_modulus_calculation)")
//         }
//     }
//
//     remainder = &dividend_original - &(&quotient * &divisor_original);
//
//     if quotient == empty_bigint {
//         quotient.digits.push(0);
//     }
//     (quotient, remainder)
// }
//
// // Bruteforce method of calculating the division, the bigger the quotient the more time it takes to compute.
// // Time requirement increases exponentially, to compute very big quotients.
// // While recursive method provides immediate answer, bruteforce could not calculate the result for several minutes.
// fn bruteforce_division(dividend: &ChonkerInt, divisor: &ChonkerInt) -> (ChonkerInt, ChonkerInt) {
//     // Make dividends and divisors absolute, positive.
//     let mut dividend_original = (*dividend).clone();
//     dividend_original.set_positive_sign();
//     let mut dividend = (*dividend).clone();
//     dividend.set_positive_sign();
//     let mut divisor_original = (*divisor).clone();
//     divisor_original.set_positive_sign();
//     let mut divisor = (*divisor).clone();
//     divisor.set_positive_sign();
//
//     let zero_bigint = ChonkerInt::new();
//     let mut quotient = ChonkerInt::new();
//
//     while dividend >= zero_bigint {
//         dividend = &dividend - &divisor;
//         quotient = &quotient + &ChonkerInt::from(String::from("1"));
//         println!("{:?}", dividend);
//         println!("{:?}", quotient);
//     }
//
//     if dividend.sign == BigIntSign::Negative {
//         quotient = &quotient - &ChonkerInt::from(String::from("1"));
//     }
//
//     let remainder = &dividend_original - &(&divisor_original * &quotient);
//
//     (quotient, remainder)
// }
//
// // Implement modulus "%" operator for the BigInt. The sign of the result follows the divisor.
// // The implementation is a copy of the division implementation, except it returns the remainder instead of quotient,
// // which is not even stored.
// impl<'a, 'b> Rem<&'b ChonkerInt> for &'a ChonkerInt {
//     type Output = ChonkerInt;
//
//     fn rem(self, rhs: &'b ChonkerInt) -> Self::Output {
//         // Check for division by zero, if the divisor is zero, panic.
//         if *rhs == ChonkerInt::new() {
//             panic!("attempt to divide/take modulus by zero (ChonkerInt::rem())");
//         }
//
//         // Clone dividend and divisor, make them absolute for comparisons.
//         let mut absolute_dividend = (*self).clone();
//         absolute_dividend.set_positive_sign();
//         let mut absolute_divisor = (*rhs).clone();
//         absolute_divisor.set_positive_sign();
//
//         // Compare the lengths/values of the dividend and divisor.
//         // If self/dividend is smaller that the divisor, either the dividend or the sum of  the divisor with the dividend.
//         if absolute_dividend < absolute_divisor {
//             if self.sign == BigIntSign::Negative && rhs.sign == BigIntSign::Positive {
//                 return rhs + self;
//             } else if self.sign == BigIntSign::Positive && rhs.sign == BigIntSign::Negative {
//                 return rhs + self;
//             } else if self.sign == BigIntSign::Negative && rhs.sign == BigIntSign::Negative {
//                 return (*self).clone();
//             }
//             // Both dividend and divisor are positive.
//             return (*self).clone();
//         }
//
//         let mut remainder;
//         let mut cut_dividend = ChonkerInt::new();
//         cut_dividend.set_positive_sign();
//
//         // Compare lengths of the dividend and divisor and normalization of the most significant digits.
//         // Length of the dividend should not be more than length of the divisor + 1.
//         // If it is, cut the dividend.
//         if (self.digits.len() > rhs.digits.len() + 1)
//             || (self.digits.len() == rhs.digits.len() + 1
//             && self.digits[self.digits.len() - 1] > rhs.digits[rhs.digits.len() - 1])
//             && (rhs.digits[rhs.digits.len() - 1] < 5)
//         {
//             // Cut the dividend to the smaller size of divisor's length with or without additional digit,
//             // calculate the remainder that will be used as the dividend,
//             // and then repeat the process by adding digits from the original dividend to the remainders one by one in the loop.
//
//             let mut dividend_index = self.digits.len();
//             // let mut difference = rhs.digits.len() + 1;
//             let mut difference;
//
//             // Check normalization of the divisor's most significant digit and compare it to the divident's digit,
//             // if divisor's digit is not normalized, and it is smaller than the dividend's one,
//             // make the difference/required length for the cut dividend equal to the divisor's length;
//             // otherwise add 1 to it.
//             if (self.digits[self.digits.len() - 1] > rhs.digits[rhs.digits.len() - 1])
//                 && (rhs.digits[rhs.digits.len() - 1] < 5)
//             {
//                 difference = rhs.digits.len();
//             } else {
//                 difference = rhs.digits.len() + 1;
//             }
//
//             let cut_dividend_splice =
//                 &self.digits[(dividend_index - difference)..=(dividend_index - 1)];
//
//             for digit in cut_dividend_splice.iter().rev() {
//                 cut_dividend.digits.insert(0, *digit);
//             }
//
//             let (_quotient_digit, remainder_digit) =
//                 quotient_estimation_algorithm(&cut_dividend, rhs);
//
//             cut_dividend = remainder_digit.clone();
//
//             dividend_index -= difference;
//
//             difference = 1;
//
//             // Loop over the dividend's digits one by one and add them to the remainder from the previous iteration.
//             while dividend_index > 0 {
//                 let cut_dividend_splice =
//                     &self.digits[(dividend_index - difference)..=(dividend_index - 1)];
//
//                 // Add remaining digits form the dividend to the remainder from previous division operation.
//                 // Preserve little endian from the dividend with reverse of the iterator over the dividend.
//                 for digit in cut_dividend_splice.iter().rev() {
//                     cut_dividend.digits.insert(0, *digit);
//                 }
//                 cut_dividend.set_positive_sign();
//                 cut_dividend.normalize();
//
//                 // Check if the dividend became shorter/smaller than the divisor, if so,
//                 // skip the iteration.
//                 if cut_dividend < absolute_divisor {
//                     dividend_index -= difference;
//                     continue;
//                 }
//
//                 let (_quotient_digit, remainder_digit) =
//                     quotient_estimation_algorithm(&cut_dividend, rhs);
//
//                 // Use the remainder digit(-s) for the new partial dividend.
//                 // Reverse digits from little endian to big endian.
//                 cut_dividend = remainder_digit.clone();
//
//                 dividend_index -= difference;
//             }
//
//             // Clone the remainder.
//             remainder = cut_dividend.clone();
//         } else {
//             // If lengths of dividend and divisor are equal or have a difference of 1.
//             let (_quotient_digit, remainder_digit) = quotient_estimation_algorithm(self, rhs);
//             // Clone the remainder.
//             remainder = remainder_digit.clone();
//         }
//
//         // This is an implementaion of the modulo operation, not the remainder,
//         // thus the final sign follows the sign of the divisor.
//         // Check for the sign of the dividend,
//         // if it is negative and the remainder is not a zero,
//         // make the remainder negative and add 1 divisor to it.
//         if remainder != ChonkerInt::new() {
//             if self.sign == BigIntSign::Negative && rhs.sign == BigIntSign::Positive {
//                 remainder.set_negative_sign();
//                 remainder = &remainder + rhs;
//             } else if self.sign == BigIntSign::Positive && rhs.sign == BigIntSign::Negative {
//                 remainder = &remainder + rhs;
//             } else if self.sign == BigIntSign::Negative && rhs.sign == BigIntSign::Negative {
//                 remainder.set_negative_sign();
//             }
//         }
//
//         // Cut the leading zeros.
//         remainder.normalize();
//         remainder
//     }
// }
//
// // Implement negation "-" operator for the BigInt.
// // impl Neg for ChonkerInt {
// impl Neg for &ChonkerInt {
//     // type Output = Self;
//     type Output = ChonkerInt;
//
//     fn neg(self) -> Self::Output {
//         match self.sign {
//             BigIntSign::Negative => ChonkerInt {
//                 digits: self.digits.clone(),
//                 sign: BigIntSign::Positive,
//             },
//             BigIntSign::Positive => ChonkerInt {
//                 digits: self.digits.clone(),
//                 sign: BigIntSign::Negative,
//             },
//             BigIntSign::Zero => ChonkerInt::new(),
//         }
//     }
// }
//
// // Test module.
// #[cfg(test)]
// mod tests {
//     use std::ops::{Add, Neg};
//
//     use crate::logic::bigint::{
//         add_digit_and_overflow, add_digits, BigIntSign, ChonkerInt,
//         digit_convert, subtract_digit_and_underflow, subtract_digits,
//     };
//     use crate::logic::error::OperationError;
//
//     // Test creation/construction of an empty/zero BigInt.
//     #[test]
//     fn test_empty_bigint_construction() {
//         let bigint = ChonkerInt::new();
//
//         let comparison_empty_digits_vector: Vec<i8> = vec![];
//
//         // Check digit vector contents, the vector should be empty.
//         assert_eq!(*bigint.get_vec(), comparison_empty_digits_vector);
//
//         // Check sign value, the sign value should be BigIntSign::Zero.
//         assert_eq!(*bigint.get_sign(), BigIntSign::Zero);
//     }
//
//     // Test creation/construction of a filled BigInt with random digits.
//     #[test]
//     fn test_random_bigint_construction() {
//         let requested_length: u64 = 418256; // Reasonable limit, considering O(n) time complexity. Length: 6-7.
//         let requested_zero_length: u64 = 0;
//         let requested_positive_sign = BigIntSign::Positive;
//         let requested_negative_sign = BigIntSign::Negative;
//         let requested_zero_sign = BigIntSign::Zero;
//
//         let random_positive_bigint = ChonkerInt::new_rand(&requested_length, &requested_positive_sign);
//         let random_negative_bigint = ChonkerInt::new_rand(&requested_length, &requested_negative_sign);
//
//         // let random_zero_bigint1 = ChonkerInt::new_rand(&requested_zero_length, &requested_negative_sign);
//         // let random_zero_bigint1 = ChonkerInt::new_rand(&requested_length, &requested_zero_sign);
//
//         assert_eq!(random_positive_bigint.sign, requested_positive_sign);
//         assert_eq!(random_positive_bigint.digits.len(), requested_length as usize);
//         assert_eq!(random_negative_bigint.sign, requested_negative_sign);
//         assert_eq!(random_negative_bigint.digits.len(), requested_length as usize);
//     }
//
//     // Test creation/construction of random prime BigInt.
//     #[test]
//     fn test_random_prime_bigint_construction() {
//         let requested_length: u64 = 12; // Reasonable limit, considering O(n) time complexity. Length: max 11-12.
//         let requested_zero_length: u64 = 0;
//
//         let random_prime_bigint = ChonkerInt::new_prime(&requested_length);
//
//         // let random_zero_bigint = ChonkerInt::new_rand(&requested_zero_length);
//
//         assert_eq!(random_prime_bigint.sign, BigIntSign::Positive);
//         assert_eq!(random_prime_bigint.digits.len(), requested_length as usize);
//         assert!(random_prime_bigint.is_prime());
//     }
//
//     // Test creation/construction of random prime BigInt from the provided range of lengths.
//     #[test]
//     fn test_random_bigint_range_length_construction() {
//         let requested_length_start: u64 = 1; // Reasonable limit, considering O(n) time complexity. Length: max 11-12.
//         let requested_length_end: u64 = 12;
//         let requested_zero_length: u64 = 0;
//         let requested_positive_sign = BigIntSign::Positive;
//         let requested_negative_sign = BigIntSign::Negative;
//         let requested_zero_sign = BigIntSign::Zero;
//
//         let random_positive_bigint = ChonkerInt::new_rand_range_len(&requested_length_start, &requested_length_end, &requested_positive_sign);
//         let random_negative_bigint = ChonkerInt::new_rand_range_len(&requested_length_start, &requested_length_end, &requested_negative_sign);
//
//         // let random_zero_bigint1 = ChonkerInt::new_rand_range_len(&requested_length_start, &requested_length_end, &requested_negative_sign);
//         // let random_zero_bigint1 = ChonkerInt::new_rand_range_len(&requested_length_start, &requested_length_end, &requested_zero_sign);
//
//         assert_eq!(random_positive_bigint.sign, requested_positive_sign);
//         assert!((1..=12).contains(&(random_positive_bigint.digits.len())));
//         assert_eq!(random_negative_bigint.sign, requested_negative_sign);
//         assert!((1..=12).contains(&(random_negative_bigint.digits.len())));
//     }
//
//     // Test creation/construction of random prime BigInt from the provided range of values.
//     #[test]
//     fn test_random_prime_bigint_range_value_construction() {
//         let requested_value_start = ChonkerInt::from(1);
//         let requested_value_end = ChonkerInt::from(1234567);
//         let requested_zero_value = 0;
//         let requested_positive_sign = BigIntSign::Positive;
//         let requested_negative_sign = BigIntSign::Negative;
//         let requested_zero_sign = BigIntSign::Zero;
//
//         let random_positive_bigint = ChonkerInt::new_rand_range_value(&requested_value_start, &requested_value_end, &requested_positive_sign);
//         let mut random_negative_bigint = ChonkerInt::new_rand_range_value(&requested_value_start, &requested_value_end, &requested_negative_sign);
//
//         // let random_zero_bigint1 = ChonkerInt::new_rand_range_value(&requested_value_start, &requested_value_end, &requested_negative_sign);
//         // let random_zero_bigint1 = ChonkerInt::new_rand_range_value(&requested_value_start, &requested_value_end, &requested_zero_sign);
//
//         assert_eq!(random_positive_bigint.sign, requested_positive_sign);
//         assert!(random_positive_bigint > requested_value_start && random_positive_bigint < requested_value_end);
//         assert_eq!(random_negative_bigint.sign, requested_negative_sign);
//         random_negative_bigint.set_positive_sign();
//         assert!(random_positive_bigint > requested_value_start && random_positive_bigint < requested_value_end);
//     }
//
//     // Test BigInt's power operation.
//     #[test]
//     fn test_bigint_pow_operation() {
//         // Positive BigInts.
//         let positive_bigint = ChonkerInt::from(String::from("13"));
//
//         // Negative BigInts.
//         let negative_bigint = ChonkerInt::from(String::from("-8"));
//
//         // Very big BigInts. If you take more, result will be very long and
//         // calculations will take a long time even with exponetiation by squaring.
//         let positive_very_big_bigint = ChonkerInt::from(String::from("714"));
//         let negative_very_big_bigint = ChonkerInt::from(String::from("-831"));
//
//         // Empty/zero BigInt.
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//
//         // Check exponentiation of positive BigInt.
//         let positive_into_positive_exponentiation_result =
//             ChonkerInt::from(String::from("302875106592253"));
//         assert_eq!(
//             positive_bigint.pow(&positive_bigint),
//             positive_into_positive_exponentiation_result
//         );
//
//         // Check negative exponentiation of negative BigInts.
//         let negative_into_negative_exponentiation_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             negative_bigint.pow(&negative_bigint),
//             negative_into_negative_exponentiation_result
//         );
//
//         // Check mixed exponentiation.
//         let positive_into_negative_exponentiation_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             positive_bigint.pow(&negative_bigint),
//             positive_into_negative_exponentiation_result
//         );
//         let negative_into_positive_exponentiation_result =
//             ChonkerInt::from(String::from("-549755813888"));
//         assert_eq!(
//             negative_bigint.pow(&positive_bigint),
//             negative_into_positive_exponentiation_result
//         );
//
//         // Check zero exponentiation.
//         let positive_into_zero_exponentiation_result = ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             positive_bigint.pow(&zero_bigint),
//             positive_into_zero_exponentiation_result
//         );
//         let negative_into_zero_exponentiation_result = ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             negative_bigint.pow(&zero_bigint),
//             negative_into_zero_exponentiation_result
//         );
//
//         // Check exponentiation of zero BigInt.
//         let zero_exponentiation_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             zero_bigint.pow(&positive_bigint),
//             zero_exponentiation_result
//         );
//         assert_eq!(
//             zero_bigint.pow(&negative_bigint),
//             zero_exponentiation_result
//         );
//
//         // Check very big BigInt exponentiation.
//         let exponentiation_by_big_positive_result = ChonkerInt::from(String::from("2267532639227975293484649359822622477267689237579697617906192701053898488585679615971899020889703769424853593769941894099705006188147791222757056131368726677466232928799681558371072954085332934432778999746503487442415093710752191893125909910080172104071186196202318808905629441412179120109245122253741155406625147838162586256820577038206529709540777905603717397005471625810253776105763148136621464827535515291812707058866519618646290545419919167807400141040627830812565548535385001713064127210523318363986953541795681288429435802412989826355065278574119348352219099460207465524775197105990445932679942909728731805572486726083245008659975610583638158014896578631266675555540161864816514746194270973162359461556173014105269004989142077146806003668299107697188717385561871063129881589233956646361289"));
//         assert_eq!(
//             positive_bigint.pow(&positive_very_big_bigint),
//             exponentiation_by_big_positive_result
//         );
//         assert_eq!(
//             positive_very_big_bigint.pow(&negative_very_big_bigint),
//             zero_bigint
//         );
//         let big_negative_exponentiation_by_big_positive_result = ChonkerInt::from(String::from("393668786826267553077076474140299370880918568633563724842666026887776380400692507101958767814164298006252398130266405011634320074245315113033300793421272121694777985497897167256840060270989737120689785782511428409896396048195430068938559250888431303594273773458951383706995409676453086456521994456295614684097413271167286341695248290067965403698108928941471664208182091242033334793930243716022436130436364911194491778238188273770470802023623079742059009766543879957296830437165202578267998722610234076548932922293704169105785971273074051301109900762041708423339290740902499110645226372903295714474590337870063400159212122982099331841277519357737923455372909102252600266400418575947091730422665647428116914539158466916493885084085612950657386804563036551513189590436669087936432673802292033752746111866688630990125757428824046174459252601860997315161352243407845749473077838394751025949100852484995533549035660758414837703125558423697084106243864495746715761436733376868148766657885493971607216332569267922294404814980854030044970158555648288715014317812616390609047193586114161512702965856387218837858166052100553898789139804660797970747108950883914128113314021672344327825955132390747948842969862107736457509838732942623385512250976704094261050980772256073544253081541031345079031032364601997324753773880108846682540347154574904882318554809375741408296121927147437160030649395305425180669121870859453237154694425736638073587779704911653563977141631560940520786057038882281455819435104785227726034385354135947100172064790387713080298235953549473155071842855112924615851579027373517736475699137515166693637491228088704452538669947537597739225539365158053926355837721785585790095248003005310035036534010451112841933386560090762859855053853912069020851284005299437700326199339888487499713149192634147472202811307482794085250467256565249587688716142208770285689499321039876392577357106678155028658660245414611378631571628503673365981639037164007646301143945969330141469137765067899977984941827961563471054094969640836506834793455965606891806616235299067618810916324150922380957399772315521"));
//         assert_eq!(
//             negative_very_big_bigint.pow(&positive_very_big_bigint),
//             big_negative_exponentiation_by_big_positive_result
//         );
//         assert_eq!(
//             negative_very_big_bigint.pow(&negative_very_big_bigint),
//             zero_bigint
//         );
//     }
//
//     // Test BigInt's modular exponentiation operation.
//     #[test]
//     fn test_bigint_modpow_operation() {
//         // Positive BigInts.
//         let positive_bigint = ChonkerInt::from(String::from("13786234"));
//         let bigint_modulus = ChonkerInt::from(String::from("45"));
//         let bigint_negative_modulus = ChonkerInt::from(String::from("-45"));
//
//         // Negative BigInts.
//         let negative_bigint = ChonkerInt::from(String::from("-8"));
//
//         // Empty/zero BigInt.
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//
//         // Check exponentiation of positive BigInt.
//         let positive_into_positive_exponentiation_result =
//             ChonkerInt::from(String::from("16"));
//         assert_eq!(
//             positive_bigint.modpow(&positive_bigint, &bigint_modulus),
//             positive_into_positive_exponentiation_result
//         );
//
//         // Check exponentiation of positive BigInt with negative BigInt.
//         let positive_into_negative_exponentiation_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             positive_bigint.modpow(&negative_bigint, &bigint_modulus),
//             positive_into_negative_exponentiation_result
//         );
//
//         // Check exponentiation of positive BigInt with zero BigInt.
//         let positive_into_zero_exponentiation_result =
//             ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             positive_bigint.modpow(&zero_bigint, &bigint_modulus),
//             positive_into_zero_exponentiation_result
//         );
//
//         // Check exponentiation of negative BigInt with positive BigInt.
//         let negative_into_positive_exponentiation_result =
//             ChonkerInt::from(String::from("19"));
//         assert_eq!(
//             negative_bigint.modpow(&positive_bigint, &bigint_modulus),
//             negative_into_positive_exponentiation_result
//         );
//
//         // Check exponentiation of negative BigInt with negative BigInt.
//         let negative_into_negative_exponentiation_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             negative_bigint.modpow(&negative_bigint, &bigint_modulus),
//             negative_into_negative_exponentiation_result
//         );
//
//         // Check exponentiation of negative BigInt with zero BigInt.
//         let negative_into_zero_exponentiation_result =
//             ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             negative_bigint.modpow(&zero_bigint, &bigint_modulus),
//             negative_into_zero_exponentiation_result
//         );
//
//         // Repeat the same tests with negative modulus.
//
//         // Check exponentiation of positive BigInt.
//         let positive_into_positive_exponentiation_result =
//             ChonkerInt::from(String::from("16"));
//         // assert_eq!(
//         //     positive_bigint.modpow(&positive_bigint, &bigint_negative_modulus),
//         //     positive_into_positive_exponentiation_result
//         // );
//
//         // Check exponentiation of positive BigInt with negative BigInt.
//         let positive_into_negative_exponentiation_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             positive_bigint.modpow(&negative_bigint, &bigint_negative_modulus),
//             positive_into_negative_exponentiation_result
//         );
//
//         // Check exponentiation of positive BigInt with zero BigInt.
//         let positive_into_zero_exponentiation_result =
//             ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             positive_bigint.modpow(&zero_bigint, &bigint_negative_modulus),
//             positive_into_zero_exponentiation_result
//         );
//
//         // Check exponentiation of negative BigInt with positive BigInt.
//         let negative_into_positive_exponentiation_result =
//             ChonkerInt::from(String::from("19"));
//         // assert_eq!(
//         //     negative_bigint.modpow(&positive_bigint, &bigint_negative_modulus),
//         //     negative_into_positive_exponentiation_result
//         // );
//
//         // Check exponentiation of negative BigInt with negative BigInt.
//         let negative_into_negative_exponentiation_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             negative_bigint.modpow(&negative_bigint, &bigint_negative_modulus),
//             negative_into_negative_exponentiation_result
//         );
//
//         // Check exponentiation of negative BigInt with zero BigInt.
//         let negative_into_zero_exponentiation_result =
//             ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             negative_bigint.modpow(&zero_bigint, &bigint_negative_modulus),
//             negative_into_zero_exponentiation_result
//         );
//     }
//
//     // Test retrieval of a reference to the BigInt's vector of digits.
//     #[test]
//     fn test_bigint_digits_vector_retrieval() {
//         let bigint = ChonkerInt::from(String::from("123"));
//
//         // Note, digits in the BigInt are stored in little endian format,
//         // when requested for reference, vector is not adjusted for big endian format.
//         let comparison_digits_vector: Vec<i8> = vec![3, 2, 1];
//
//         assert_eq!(*bigint.get_vec(), comparison_digits_vector);
//     }
//
//     // Test retrieval of a reference to the BigInt's sign.
//     #[test]
//     fn test_bigint_sign_retrieval() {
//         let positive_bigint = ChonkerInt::from(String::from("1"));
//         let negative_bigint = ChonkerInt::from(String::from("-1"));
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//
//         assert_eq!(*positive_bigint.get_sign(), BigIntSign::Positive);
//         assert_eq!(*negative_bigint.get_sign(), BigIntSign::Negative);
//         assert_eq!(*zero_bigint.get_sign(), BigIntSign::Zero);
//     }
//
//     // Test BigInt normalization and digit insertion.
//     #[test]
//     fn test_bigint_normalization_and_insertion() {
//         let mut bigint = ChonkerInt::from(String::from("123"));
//         let comparison_bigint = ChonkerInt::from(String::from("123"));
//         // Add leading zeros to the vector of digits.
//         for x in 0..10 {
//             bigint.push(0);
//         }
//         // Normalize BigInt to remove leading zeros.
//         bigint.normalize();
//
//         assert_eq!(bigint, comparison_bigint);
//     }
//
//     // Test BigInt vector/slice insertion.
//     #[test]
//     fn test_bigint_vector_insertion() {
//         let mut bigint = ChonkerInt::from(12345);
//         let mut bigint_empty = ChonkerInt::from(0);
//
//         // Note: leading zeros are preserved, the result is not normalized.
//         let digits_vector = vec![1, 2, 3, 0];
//
//         // Mind little endian.
//         bigint.push_vec(&digits_vector);
//         bigint_empty.push_vec(&digits_vector);
//
//         let mut comparison_bigint1 = ChonkerInt::new();
//         let comparison_digits_vector = vec![5, 4, 3, 2, 1, 1, 2, 3, 0];
//         comparison_bigint1.push_vec(&comparison_digits_vector);
//         comparison_bigint1.set_positive_sign();
//
//         let mut comparison_bigint2 = ChonkerInt::new();
//         comparison_bigint2.push_vec(&digits_vector);
//         comparison_bigint2.set_positive_sign();
//
//         assert_eq!(bigint, comparison_bigint1);
//         assert_eq!(bigint_empty, comparison_bigint2);
//     }
//
//     // Test of BigInt's change of signs.
//     #[test]
//     fn test_bigint_sign_change() {
//         // Check transition of negative to positive.
//         let negative_bigint = ChonkerInt::from(String::from("-100"));
//         let mut negated_negative_bigint = ChonkerInt::from(String::from("-100"));
//         negated_negative_bigint.set_positive_sign();
//
//         // Check transition of positive to negative.
//         let positive_bigint = ChonkerInt::from(String::from("100"));
//         let mut negated_positive_bigint = ChonkerInt::from(String::from("100"));
//         negated_positive_bigint.set_negative_sign();
//
//         // Check transition of neutral zero to positive zero.
//         let mut positivized_zero_bigint = ChonkerInt::from(String::from("0"));
//         positivized_zero_bigint.set_positive_sign();
//
//         // Check transition of neutral zero to negative zero.
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//         let mut negated_zero_bigint = ChonkerInt::from(String::from("0"));
//         negated_zero_bigint.set_negative_sign();
//
//         // Check signs of non-zero numbers.
//         assert_eq!(negative_bigint, negated_positive_bigint);
//         assert_eq!(positive_bigint, negated_negative_bigint);
//
//         // Check signs of zeros.
//         assert_eq!(*positivized_zero_bigint.get_sign(), BigIntSign::Positive);
//         assert_eq!(*negated_zero_bigint.get_sign(), BigIntSign::Negative);
//     }
//
//     // Test BigInt cloning.
//     #[test]
//     fn test_bigint_cloning() {
//         let original_bigint_positive = ChonkerInt::from(String::from("123123123"));
//         let original_bigint_negative = ChonkerInt::from(-123123123);
//         let original_bigint_empty = ChonkerInt::new();
//
//         // Create clones with deep copying.
//         let clone_bigint_positive = original_bigint_positive.clone();
//         let clone_bigint_negative = original_bigint_negative.clone();
//         let clone_bigint_empty = original_bigint_empty.clone();
//
//         assert_eq!(original_bigint_positive, clone_bigint_positive);
//         assert_eq!(original_bigint_negative, clone_bigint_negative);
//         assert_eq!(original_bigint_empty, clone_bigint_empty);
//     }
//
//     // Test BigInt splicing/concatenation.
//     #[test]
//     fn test_bigint_splicing() {
//         let mut target_bigint1 = ChonkerInt::from(123);
//         let mut target_bigint2 = ChonkerInt::from(101);
//         let mut target_bigint_empty = ChonkerInt::from(0);
//
//         let target_for_splicing_clone0 = ChonkerInt::from(45);
//         let target_for_splicing_clone1 = ChonkerInt::from(01);
//         let target_for_splicing_clone3 = ChonkerInt::from(0);
//
//         // Mind little endian.
//         target_bigint1.splice(target_for_splicing_clone0);
//         target_bigint2.splice(target_for_splicing_clone3);
//         target_bigint_empty.splice(target_for_splicing_clone1);
//
//         assert_eq!(target_bigint1, ChonkerInt::from(45123));
//         assert_eq!(target_bigint2, ChonkerInt::from(101));
//         assert_eq!(target_bigint_empty, ChonkerInt::from(1));
//     }
//
//     // Test BigInt to string conversion.
//     #[test]
//     fn test_bigint_to_string_conversion() {
//         let positive_bigint = ChonkerInt::from(012300);
//         let negative_bigint = ChonkerInt::from(-012300);
//         let zero_bigint1 = ChonkerInt::new();
//         let zero_bigint2 = ChonkerInt::from(0);
//         let zero_bigint3 = ChonkerInt::from(String::from("0"));
//
//         let positive_bigint_string = String::from("12300");
//         let negative_bigint_string = String::from("-12300");
//         let zero_bigint_string = String::from("0");
//
//         assert_eq!(positive_bigint.to_string(), positive_bigint_string);
//         assert_eq!(negative_bigint.to_string(), negative_bigint_string);
//         assert_eq!(zero_bigint1.to_string(), zero_bigint_string);
//         assert_eq!(zero_bigint2.to_string(), zero_bigint_string);
//         assert_eq!(zero_bigint3.to_string(), zero_bigint_string);
//     }
//
//     // Test the methods checking the BigInt, if it is a prime number. Iterative and probabilistic approaches.
//     #[test]
//     fn test_bigint_is_prime() {
//         let negative_bigint = ChonkerInt::from(-123534);
//         let big_one = ChonkerInt::from(1);
//         let big_zero = ChonkerInt::new();
//
//         let bigint_not_prime1 = ChonkerInt::from(String::from(
//             "4231689648728034761024109348723094713208529386505712",
//         ));
//         let bigint_not_prime2 = ChonkerInt::from(String::from("9231689641731777"));
//         let bigint_not_prime3 = ChonkerInt::from(String::from("4920945105274017443"));
//         let bigint_not_prime4 = ChonkerInt::from(String::from("1963760928849712729"));
//         let bigint_not_prime5 = ChonkerInt::from(String::from("91913571835595342720975337196553217435917295531"));
//         let bigint_not_prime6 = ChonkerInt::from(String::from("612805246882657062501053758885060201204312902577192906873202382957257140215187148278954238693446490196431299436910286231"));
//         let small_bigint_not_prime = ChonkerInt::from(String::from("4230"));
//         let bigint_prime1 = ChonkerInt::from(String::from("57885161"));
//         let bigint_prime2 = ChonkerInt::from(String::from("7434295549380978012839955681932280624399018262337538072234694442121788312959451806126604174504220901"));
//         let bigint_prime3 = ChonkerInt::from(String::from("14043145053387290701740553217226309216528545163443"));
//         let bigint_prime4 = ChonkerInt::from(String::from("87989249816948596463490931421050640230915364594808832915245649794766910416355601009344627352778928041250895432095896869913026898577325779505914260099267273891723698030806841435326256829587373811552723"));
//         let bigint_prime5 = ChonkerInt::from(String::from("562457128101735933004861281229980320017117423199759147390620125871795298609636011392770132344949088969751961333591438819671223807833285269283606992239"));
//         let bigint_prime6 = ChonkerInt::from(String::from("1000000000000066600000000000001"));
//         let bigint_prime7 = ChonkerInt::from(String::from("299572883"));
//         let bigint_prime8 = ChonkerInt::from(String::from("1894964749"));
//         let bigint_prime9 = ChonkerInt::from(String::from("69954509893")); // Length 11.
//         let bigint_prime10 = ChonkerInt::from(String::from("855111008179")); // Length 12.
//         let bigint_prime11 = ChonkerInt::from(String::from("78139"));
//         let small_bigint_prime = ChonkerInt::from(String::from("7"));
//
//         // Iterative approach, with O(sqrt(n) runtime complexity. Guaranteed outcome,
//         // but takes significantly more time with big inputs.
//         assert!(!negative_bigint.is_prime());
//         assert!(!big_one.is_prime());
//         assert!(!big_zero.is_prime());
//         assert!(!bigint_not_prime1.is_prime());
//         assert!(!bigint_not_prime2.is_prime());
//         assert!(!small_bigint_not_prime.is_prime());
//         assert!(bigint_prime1.is_prime());
//         // assert!(bigint_prime2.is_prime());
//         // assert!(bigint_prime3.is_prime());
//         // assert!(bigint_prime4.is_prime());
//         // assert!(bigint_prime5.is_prime());
//         // assert!(bigint_prime6.is_prime());
//         assert!(bigint_prime7.is_prime());
//         // assert!(bigint_prime8.is_prime());
//         // assert!(bigint_prime9.is_prime());
//         // assert!(bigint_prime10.is_prime());
//         assert!(small_bigint_prime.is_prime());
//
//         // Probabilistic approach, with O(k log3n) runtime complexity.
//         // If a composite value is detected - it is 100% composite,
//         // if a prime value is assumed - it is not 100% guranteed.
//         // Significantly faster, speed also depends on the amount of trials,
//         // there are 40 by default.
//         assert!(!negative_bigint.is_prime_probabilistic(None));
//         assert!(!big_one.is_prime_probabilistic(None));
//         assert!(!big_zero.is_prime_probabilistic(None));
//         assert!(!bigint_not_prime1.is_prime_probabilistic(None));
//         assert!(!bigint_not_prime2.is_prime_probabilistic(None));
//         assert!(!bigint_not_prime3.is_prime_probabilistic(None));
//         assert!(!bigint_not_prime4.is_prime_probabilistic(None));
//         assert!(!bigint_not_prime5.is_prime_probabilistic(Some(2)));
//         assert!(!bigint_not_prime6.is_prime_probabilistic(Some(4)));
//         assert!(!small_bigint_not_prime.is_prime_probabilistic(None));
//         assert!(bigint_prime1.is_prime_probabilistic(None));
//         assert!(bigint_prime2.is_prime_probabilistic(Some(2)));
//         assert!(bigint_prime3.is_prime_probabilistic(Some(3)));
//         // assert!(bigint_prime4.is_prime_probabilistic(Some(2)));
//         assert!(bigint_prime5.is_prime_probabilistic(Some(2)));
//         // assert!(bigint_prime6.is_prime_probabilistic(Some(6)));
//         assert!(bigint_prime7.is_prime_probabilistic(None));
//         assert!(bigint_prime8.is_prime_probabilistic(None));
//         assert!(bigint_prime9.is_prime_probabilistic(None));
//         assert!(bigint_prime10.is_prime_probabilistic(None));
//         assert!(bigint_prime11.is_prime_probabilistic(None));
//         assert!(small_bigint_prime.is_prime_probabilistic(Some(2)));
//     }
//
//     // Test the method computign the greatest common divisor between two BigInts.
//     #[test]
//     fn test_bigint_gcd() {
//         let bigint1 = ChonkerInt::new_rand(&13, &BigIntSign::Positive);
//         let bigint2 = ChonkerInt::new_rand(&98, &BigIntSign::Positive);
//
//         println!("bigint1: {}", bigint1.to_string());
//         println!("bigint1 len: {}", bigint1.digits.len());
//         println!("bigint2: {}", bigint2.to_string());
//         println!("bigint2 len: {}", bigint2.digits.len());
//
//         let result = bigint1.gcd(&bigint2);
//
//         println!("Result: {}", result.to_string());
//     }
//
//     // Test u128 integer conversion into a BigInt
//     #[test]
//     fn test_digit_conversion() {
//         let target: u128 = 123_123_123_123_123_123_123_123_123_123_123_123_1;
//         let result = match digit_convert(&target) {
//             Ok(result) => result,
//             Err(e) => {
//                 match e.downcast::<OperationError>() {
//                     Ok(value) => panic!("    An overflow occurred during conversion of an u128 integer into a BigInt: {}. (test_digit_conversion)", value),
//                     Err(e) => panic!("    An error was encountered during conversion of an u128 integer into a BigInt: {}. (test_digit_conversion)", e),
//                 }
//             }
//         };
//
//         // let comparison_vec: Vec<i8> = vec![3,4,0,2,8,2,3,6,6,9,2,0,9,3,8,4,6,3,4,6,3,3,7,4,6,0,7,4,3,1,7,6,8,2,1,1,4,5,5,];
//         let mut comparison_vec: Vec<i8> = vec![
//             1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2,
//             3, 1, 2, 3, 1, 2, 3, 1,
//         ];
//         &comparison_vec.reverse();
//
//         assert_eq!(result, comparison_vec);
//     }
//
//     // Test string conversion into a BigInt
//     #[test]
//     fn test_string_conversion() {
//         let target = String::from("-00000000000000123123123123123123123123123123123123123123123123123123123123123123123123123123123123123123");
//         let other_target = String::from("100000");
//         let result = ChonkerInt::from(target);
//         let other_result = ChonkerInt::from(other_target);
//
//         // let comparison_vec: Vec<i8> = vec![3,4,0,2,8,2,3,6,6,9,2,0,9,3,8,4,6,3,4,6,3,3,7,4,6,0,7,4,3,1,7,6,8,2,1,1,4,5,5,];
//         let mut comparison_vec: Vec<i8> = vec![
//             1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2,
//             3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1,
//             2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
//             1, 2, 3,
//         ];
//         &comparison_vec.reverse();
//         let mut other_comparison_vec: Vec<i8> = vec![1, 0, 0, 0, 0, 0];
//         &other_comparison_vec.reverse();
//
//         assert_eq!(*result.get_vec(), comparison_vec);
//         assert_eq!(*other_result.get_vec(), other_comparison_vec);
//     }
//
//     // Test comparisons of BigInts.
//     #[test]
//     fn test_bigint_comparison() {
//         let positive_bigint = ChonkerInt::from(String::from("123"));
//         let positive_bigint_clone = ChonkerInt::from(String::from("123"));
//         let negative_bigint = ChonkerInt::from(String::from("-123"));
//         let zero_bigint = ChonkerInt::new();
//
//         assert!(positive_bigint > negative_bigint);
//         assert!(positive_bigint > zero_bigint);
//         assert!(negative_bigint < positive_bigint);
//         assert!(negative_bigint < zero_bigint);
//         assert!(zero_bigint < positive_bigint);
//         assert!(zero_bigint > negative_bigint);
//         assert!(positive_bigint >= positive_bigint_clone);
//         assert!(positive_bigint <= positive_bigint_clone);
//     }
//
//     // Test of BigInt's addition operation.
//     #[test]
//     fn test_bigint_addition() {
//         // Positive BigInts.
//         let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
//         let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));
//
//         // Negative BigInts.
//         let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
//         let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));
//
//         // Empty/zero BigInt.
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//
//         // Check addition of positive BigInts. Try different positions.
//         let positive_addition_result = ChonkerInt::from(String::from("123423"));
//         assert_eq!(
//             &positive_bigger_bigint + &positive_smaller_bigint,
//             positive_addition_result
//         );
//         assert_eq!(
//             &positive_smaller_bigint + &positive_bigger_bigint,
//             positive_addition_result
//         );
//
//         // Check addition of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
//         let bigger_positive_smaller_negative_addition_result =
//             ChonkerInt::from(String::from("-1130000"));
//         assert_eq!(
//             &positive_bigger_bigint + &negative_smaller_bigint,
//             bigger_positive_smaller_negative_addition_result
//         );
//         assert_eq!(
//             &negative_smaller_bigint + &positive_bigger_bigint,
//             bigger_positive_smaller_negative_addition_result
//         );
//
//         // Check addition of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
//         let smaller_positive_bigger_negative_addition_result =
//             ChonkerInt::from(String::from("11078"));
//         assert_eq!(
//             &positive_smaller_bigint + &negative_bigger_bigint,
//             smaller_positive_bigger_negative_addition_result
//         );
//         assert_eq!(
//             &negative_bigger_bigint + &positive_smaller_bigint,
//             smaller_positive_bigger_negative_addition_result
//         );
//
//         // Check addition of negative BigInts. Try different positions.
//         let negative_addition_result = ChonkerInt::from(String::from("-1242345"));
//         assert_eq!(
//             &negative_bigger_bigint + &negative_smaller_bigint,
//             negative_addition_result
//         );
//         assert_eq!(
//             &negative_smaller_bigint + &negative_bigger_bigint,
//             negative_addition_result
//         );
//
//         // Check addition of positive BigInts with an empty/zero BigInt. Try different positions.
//         let bigger_positive_empty_addition_result = ChonkerInt::from(String::from("100000"));
//         assert_eq!(
//             &positive_bigger_bigint + &zero_bigint,
//             bigger_positive_empty_addition_result
//         );
//         let smaller_positive_empty_addition_result = ChonkerInt::from(String::from("23423"));
//         assert_eq!(
//             &zero_bigint + &positive_smaller_bigint,
//             smaller_positive_empty_addition_result
//         );
//
//         // Check addition of negative BigInts with an empty/zero BigInt. Try different positions.
//         let bigger_negative_empty_addition_result = ChonkerInt::from(String::from("-12345"));
//         assert_eq!(
//             &negative_bigger_bigint + &zero_bigint,
//             bigger_negative_empty_addition_result
//         );
//         let smaller_negative_empty_addition_result = ChonkerInt::from(String::from("-1230000"));
//         assert_eq!(
//             &zero_bigint + &negative_smaller_bigint,
//             smaller_negative_empty_addition_result
//         );
//     }
//
//     // Test addition of two digits.
//     #[test]
//     fn test_digits_addition() {
//         let one_vec1: Vec<i8> = vec![1];
//         let one_vec2: Vec<i8> = vec![2];
//         let mut one_offset1 = 0;
//         let mut one_offset2 = 0;
//         let other_vec1: Vec<i8> = vec![9];
//         let other_vec2: Vec<i8> = vec![5];
//         let mut other_offset1 = 0;
//         let mut other_offset2 = 0;
//         let mut result1 = ChonkerInt::new();
//         let mut result2 = ChonkerInt::new();
//         let mut last_digit_overflow1 = 0;
//         let mut last_digit_overflow2 = 0;
//
//         add_digits(
//             &one_vec1,
//             &other_vec1,
//             &mut one_offset1,
//             &mut other_offset1,
//             &mut result1,
//             &mut last_digit_overflow1,
//         );
//         add_digits(
//             &one_vec2,
//             &other_vec2,
//             &mut one_offset2,
//             &mut other_offset2,
//             &mut result2,
//             &mut last_digit_overflow2,
//         );
//
//         // First test of addition, 1+9
//         assert_eq!((*result1.get_vec())[0], 0);
//         assert_eq!(last_digit_overflow1, 1);
//         assert_eq!(one_offset1, 1);
//         assert_eq!(other_offset1, 1);
//
//         // Second test of addition, 0+5
//         assert_eq!((*result2.get_vec())[0], 7);
//         assert_eq!(last_digit_overflow2, 0);
//         assert_eq!(one_offset2, 1);
//         assert_eq!(other_offset2, 1);
//     }
//
//     // Test addition of a digit with an overflow.
//     #[test]
//     fn test_digit_and_overflow_addition() {
//         let one_vec1: Vec<i8> = vec![9];
//         let one_vec2: Vec<i8> = vec![0];
//         let mut one_offset1 = 0;
//         let mut one_offset2 = 0;
//         let mut result1 = ChonkerInt::new();
//         let mut result2 = ChonkerInt::new();
//         let mut last_digit_overflow1 = 1;
//         let mut last_digit_overflow2 = 0;
//
//         add_digit_and_overflow(
//             &one_vec1,
//             &mut one_offset1,
//             &mut result1,
//             &mut last_digit_overflow1,
//         );
//         add_digit_and_overflow(
//             &one_vec2,
//             &mut one_offset2,
//             &mut result2,
//             &mut last_digit_overflow2,
//         );
//
//         // First test of addition, 9 + 1
//         assert_eq!((*result1.get_vec())[0], 0);
//         assert_eq!(last_digit_overflow1, 1);
//         assert_eq!(one_offset1, 1);
//
//         // Second test of addition, 0 + 0
//         assert_eq!((*result2.get_vec())[0], 0);
//         assert_eq!(last_digit_overflow2, 0);
//         assert_eq!(one_offset2, 1);
//     }
//
//     // Test of BigInt's subtraction operation.
//     #[test]
//     fn test_bigint_subtraction() {
//         // Positive BigInts.
//         let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
//         let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));
//
//         // Negative BigInts.
//         let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
//         let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));
//
//         // Empty/zero BigInt.
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//
//         // Check subtraction from itself.
//         let subtraction_from_itself = ChonkerInt::new();
//         assert_eq!(
//             &positive_smaller_bigint - &positive_smaller_bigint,
//             subtraction_from_itself
//         );
//
//         // Check subtraction of positive BigInts. Try different positions.
//         let positive_subtraction_smaller_from_bigger_result =
//             ChonkerInt::from(String::from("76577"));
//         assert_eq!(
//             &positive_bigger_bigint - &positive_smaller_bigint,
//             positive_subtraction_smaller_from_bigger_result
//         );
//         let positive_subtraction_bigger_from_smaller_result =
//             ChonkerInt::from(String::from("-76577"));
//         assert_eq!(
//             &positive_smaller_bigint - &positive_bigger_bigint,
//             positive_subtraction_bigger_from_smaller_result
//         );
//
//         // Check subtraction of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
//         let from_bigger_positive_bigger_negative_subtraction_result =
//             ChonkerInt::from(String::from("1330000"));
//         assert_eq!(
//             &positive_bigger_bigint - &negative_smaller_bigint,
//             from_bigger_positive_bigger_negative_subtraction_result
//         );
//         let bigger_positive_from_bigger_negative_subtraction_result =
//             ChonkerInt::from(String::from("-1330000"));
//         assert_eq!(
//             &negative_smaller_bigint - &positive_bigger_bigint,
//             bigger_positive_from_bigger_negative_subtraction_result
//         );
//
//         // Check subtraction of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
//         let from_smaller_positive_bigger_negative_subtraction_result =
//             ChonkerInt::from(String::from("35768"));
//         assert_eq!(
//             &positive_smaller_bigint - &negative_bigger_bigint,
//             from_smaller_positive_bigger_negative_subtraction_result
//         );
//         let smaller_positive_from_bigger_negative_subtraction_result =
//             ChonkerInt::from(String::from("-35768"));
//         assert_eq!(
//             &negative_bigger_bigint - &positive_smaller_bigint,
//             smaller_positive_from_bigger_negative_subtraction_result
//         );
//
//         // Check subtraction of negative BigInts. Try different positions.
//         let smaller_from_bigger_negative_subtraction_result =
//             ChonkerInt::from(String::from("1217655"));
//         assert_eq!(
//             &negative_bigger_bigint - &negative_smaller_bigint,
//             smaller_from_bigger_negative_subtraction_result
//         );
//         let bigger_from_smaller_negative_subtraction_result =
//             ChonkerInt::from(String::from("-1217655"));
//         assert_eq!(
//             &negative_smaller_bigint - &negative_bigger_bigint,
//             bigger_from_smaller_negative_subtraction_result
//         );
//
//         // Check subtraction of positive BigInts with an empty/zero BigInt. Try different positions.
//         let from_bigger_positive_empty_subtraction_result =
//             ChonkerInt::from(String::from("100000"));
//         assert_eq!(
//             &positive_bigger_bigint - &zero_bigint,
//             from_bigger_positive_empty_subtraction_result
//         );
//         let smaller_positive_from_empty_subtraction_result =
//             ChonkerInt::from(String::from("-23423"));
//         assert_eq!(
//             &zero_bigint - &positive_smaller_bigint,
//             smaller_positive_from_empty_subtraction_result
//         );
//
//         // Check subtraction of negative BigInts with an empty/zero BigInt. Try different positions.
//         let from_bigger_negative_empty_subtraction_result =
//             ChonkerInt::from(String::from("-12345"));
//         assert_eq!(
//             &negative_bigger_bigint - &zero_bigint,
//             from_bigger_negative_empty_subtraction_result
//         );
//         let smaller_negative_from_empty_subtraction_result =
//             ChonkerInt::from(String::from("1230000"));
//         assert_eq!(
//             &zero_bigint - &negative_smaller_bigint,
//             smaller_negative_from_empty_subtraction_result
//         );
//     }
//
//     // Test subtraction of two digits.
//     #[test]
//     fn test_digits_subtraction() {
//         let one_vec1: Vec<i8> = vec![9];
//         let one_vec2: Vec<i8> = vec![2];
//         let mut one_offset1 = 0;
//         let mut one_offset2 = 0;
//         let other_vec1: Vec<i8> = vec![1];
//         let other_vec2: Vec<i8> = vec![5];
//         let mut other_offset1 = 0;
//         let mut other_offset2 = 0;
//         let mut result1 = ChonkerInt::new();
//         let mut result2 = ChonkerInt::new();
//         let mut last_digit_underflow1 = 0;
//         let mut last_digit_underflow2 = 0;
//
//         subtract_digits(
//             &one_vec1,
//             &other_vec1,
//             &mut one_offset1,
//             &mut other_offset1,
//             &mut result1,
//             &mut last_digit_underflow1,
//         );
//         subtract_digits(
//             &one_vec2,
//             &other_vec2,
//             &mut one_offset2,
//             &mut other_offset2,
//             &mut result2,
//             &mut last_digit_underflow2,
//         );
//
//         // First test of subtraction, 9-1
//         assert_eq!((*result1.get_vec())[0], 8);
//         assert_eq!(last_digit_underflow1, 0);
//         assert_eq!(one_offset1, 1);
//         assert_eq!(other_offset1, 1);
//
//         // Second test of subtraction, 2-5
//         assert_eq!((*result2.get_vec())[0], 7);
//         assert_eq!(last_digit_underflow2, 1);
//         assert_eq!(one_offset2, 1);
//         assert_eq!(other_offset2, 1);
//     }
//
//     // Test subtraction of a digit with and underflow.
//     #[test]
//     fn test_digit_and_underflow_subtraction() {
//         let one_vec1: Vec<i8> = vec![9];
//         let one_vec2: Vec<i8> = vec![0];
//         let mut one_offset1 = 0;
//         let mut one_offset2 = 0;
//         let mut result1 = ChonkerInt::new();
//         let mut result2 = ChonkerInt::new();
//         let mut last_digit_underflow1 = 0;
//         let mut last_digit_underflow2 = 1;
//
//         subtract_digit_and_underflow(
//             &one_vec1,
//             &mut one_offset1,
//             &mut result1,
//             &mut last_digit_underflow1,
//         );
//         subtract_digit_and_underflow(
//             &one_vec2,
//             &mut one_offset2,
//             &mut result2,
//             &mut last_digit_underflow2,
//         );
//
//         // First test of subtraction, 9 - 0
//         assert_eq!((*result1.get_vec())[0], 9);
//         assert_eq!(last_digit_underflow1, 0);
//         assert_eq!(one_offset1, 1);
//
//         // Second test of subtraction, 0 - 1
//         assert_eq!((*result2.get_vec())[0], 9);
//         assert_eq!(last_digit_underflow2, 1);
//         assert_eq!(one_offset2, 1);
//     }
//
//     // Test multiplication of two BigInts.
//     #[test]
//     fn test_bigint_multiplication() {
//         // Positive BigInts.
//         let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
//         let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));
//
//         // Negative BigInts.
//         let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
//         let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));
//
//         // Empty/zero BigInt.
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//
//         // Check multiplication of positive BigInts. Try different positions.
//         let positive_multiplication_result = ChonkerInt::from(String::from("2342300000"));
//         assert_eq!(
//             &positive_bigger_bigint * &positive_smaller_bigint,
//             positive_multiplication_result
//         );
//         assert_eq!(
//             &positive_smaller_bigint * &positive_bigger_bigint,
//             positive_multiplication_result
//         );
//
//         // Check multiplication of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
//         let bigger_positive_smaller_negative_multiplication_result =
//             ChonkerInt::from(String::from("-123000000000"));
//         assert_eq!(
//             &positive_bigger_bigint * &negative_smaller_bigint,
//             bigger_positive_smaller_negative_multiplication_result
//         );
//         assert_eq!(
//             &negative_smaller_bigint * &positive_bigger_bigint,
//             bigger_positive_smaller_negative_multiplication_result
//         );
//
//         // Check multiplication of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
//         let smaller_positive_bigger_negative_multiplication_result =
//             ChonkerInt::from(String::from("-289156935"));
//         assert_eq!(
//             &positive_smaller_bigint * &negative_bigger_bigint,
//             smaller_positive_bigger_negative_multiplication_result
//         );
//         assert_eq!(
//             &negative_bigger_bigint * &positive_smaller_bigint,
//             smaller_positive_bigger_negative_multiplication_result
//         );
//
//         // Check multiplication of negative BigInts. Try different positions.
//         let negative_multiplication_result = ChonkerInt::from(String::from("15184350000"));
//         assert_eq!(
//             &negative_bigger_bigint * &negative_smaller_bigint,
//             negative_multiplication_result
//         );
//         assert_eq!(
//             &negative_smaller_bigint * &negative_bigger_bigint,
//             negative_multiplication_result
//         );
//
//         // Check multiplication of positive BigInts with an empty/zero BigInt. Try different positions.
//         let bigger_and_smaller_positive_empty_multiplication_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &positive_bigger_bigint * &zero_bigint,
//             bigger_and_smaller_positive_empty_multiplication_result
//         );
//         assert_eq!(
//             &zero_bigint * &positive_smaller_bigint,
//             bigger_and_smaller_positive_empty_multiplication_result
//         );
//
//         // Check multiplication of negative BigInts with an empty/zero BigInt. Try different positions.
//         let bigger_and_smaller_negative_empty_multiplication_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_bigger_bigint * &zero_bigint,
//             bigger_and_smaller_negative_empty_multiplication_result
//         );
//         assert_eq!(
//             &zero_bigint * &negative_smaller_bigint,
//             bigger_and_smaller_negative_empty_multiplication_result
//         );
//     }
//
//     // Test division of two BigInts.
//     #[test]
//     fn test_bigint_division() {
//         // Positive BigInts.
//         let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
//         let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));
//
//         // Negative BigInts.
//         let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
//         let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));
//
//         // Very big BigInts.
//         let positive_bigger_very_big_bigint = ChonkerInt::from(String::from("4379853178597859156740573149857154310578942357435678165781568134756871356187956143975358713583915634785431658143560178536107563147805634807561348506134"));
//         let positive_smaller_very_big_bigint = ChonkerInt::from(String::from(
//             "7142756019471983982475239851587182390573438756286598175918",
//         ));
//         let negative_bigger_very_big_bigint = ChonkerInt::from(String::from(
//             "-3714856173245610358671095834519578134957135871390587314982",
//         ));
//         let negative_smaller_very_big_bigint = ChonkerInt::from(String::from("-7846518746531895729834723194263984236421304673218561384612384623198412894123506123859123452319048712958714309584104712340823408213842130948"));
//
//         // Empty/zero BigInt.
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//
//         // Check division of positive BigInts. Try different positions.
//         let positive_bigger_by_smaller_division_result = ChonkerInt::from(String::from("4"));
//         assert_eq!(
//             &positive_bigger_bigint / &positive_smaller_bigint,
//             positive_bigger_by_smaller_division_result
//         );
//         let positive_smaller_by_bigger_division_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &positive_smaller_bigint / &positive_bigger_bigint,
//             positive_smaller_by_bigger_division_result
//         );
//
//         // Check division of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
//         let bigger_positive_by_smaller_negative_division_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &positive_bigger_bigint / &negative_smaller_bigint,
//             bigger_positive_by_smaller_negative_division_result
//         );
//         let smaller_negative_by_bigger_positive_division_result =
//             ChonkerInt::from(String::from("-12"));
//         assert_eq!(
//             &negative_smaller_bigint / &positive_bigger_bigint,
//             smaller_negative_by_bigger_positive_division_result
//         );
//
//         // Check division of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
//         let smaller_positive_by_bigger_negative_division_result =
//             ChonkerInt::from(String::from("-1"));
//         assert_eq!(
//             &positive_smaller_bigint / &negative_bigger_bigint,
//             smaller_positive_by_bigger_negative_division_result
//         );
//         let bigger_negative_by_smaller_positive_division_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_bigger_bigint / &positive_smaller_bigint,
//             bigger_negative_by_smaller_positive_division_result
//         );
//
//         // Check division of negative BigInts. Try different positions.
//         let negative_bigger_by_smaller_division_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_bigger_bigint / &negative_smaller_bigint,
//             negative_bigger_by_smaller_division_result
//         );
//         let negative_smaller_by_bigger_division_result = ChonkerInt::from(String::from("99"));
//         assert_eq!(
//             &negative_smaller_bigint / &negative_bigger_bigint,
//             negative_smaller_by_bigger_division_result
//         );
//
//         // Check division of positive BigInts with an empty/zero BigInt. Try different positions.
//         // let positive_bigger_by_empty_division_result = ChonkerInt::from(String::from("0"));
//         // assert_eq!(&positive_bigger_bigint / &zero_bigint, positive_bigger_by_empty_division_result);
//
//         let empty_by_positive_smaller_division_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &zero_bigint / &positive_smaller_bigint,
//             empty_by_positive_smaller_division_result
//         );
//
//         // Check division of negative BigInts with an empty/zero BigInt. Try different positions.
//         // let negative_bigger_by_empty_division_result = ChonkerInt::from(String::from("0"));
//         // assert_eq!(&negative_bigger_bigint / &zero_bigint, negative_bigger_by_empty_division_result);
//
//         let empty_by_negative_smaller_division_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &zero_bigint / &negative_smaller_bigint,
//             empty_by_negative_smaller_division_result
//         );
//
//         // Check division of very BigInts.
//         let positive_very_big_bigger_by_negative_very_big_smaller_result =
//             ChonkerInt::from(String::from("-558190621864"));
//         assert_eq!(
//             &positive_bigger_very_big_bigint / &negative_smaller_very_big_bigint,
//             positive_very_big_bigger_by_negative_very_big_smaller_result
//         );
//         let positive_very_big_smaller_by_negative_very_big_smaller_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &positive_smaller_very_big_bigint / &negative_smaller_very_big_bigint,
//             positive_very_big_smaller_by_negative_very_big_smaller_result
//         );
//         let positive_very_big_bigger_by_negative_very_big_bigger_result = ChonkerInt::from(String::from("-1179010161992692051385290940889169636086403689958542693814405492871089212617077532843158926503"));
//         assert_eq!(
//             &positive_bigger_very_big_bigint / &negative_bigger_very_big_bigint,
//             positive_very_big_bigger_by_negative_very_big_bigger_result
//         );
//         let positive_very_big_smaller_by_negative_very_big_bigger_result =
//             ChonkerInt::from(String::from("-1"));
//         assert_eq!(
//             &positive_smaller_very_big_bigint / &negative_bigger_very_big_bigint,
//             positive_very_big_smaller_by_negative_very_big_bigger_result
//         );
//
//         let negative_very_big_bigger_by_positive_very_big_smaller_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_bigger_very_big_bigint / &positive_smaller_very_big_bigint,
//             negative_very_big_bigger_by_positive_very_big_smaller_result
//         );
//         let negative_very_big_smaller_by_positive_very_big_smaller_result = ChonkerInt::from(String::from("-1098528176678773945133770019243427742596676897941676790359156611511232741298674199"));
//         assert_eq!(
//             &negative_smaller_very_big_bigint / &positive_smaller_very_big_bigint,
//             negative_very_big_smaller_by_positive_very_big_smaller_result
//         );
//         let negative_very_big_bigger_by_positive_very_big_bigger_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_bigger_very_big_bigint / &positive_bigger_very_big_bigint,
//             negative_very_big_bigger_by_positive_very_big_bigger_result
//         );
//         let negative_very_big_smaller_by_positive_very_big_bigger_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_bigger_very_big_bigint / &positive_bigger_very_big_bigint,
//             negative_very_big_smaller_by_positive_very_big_bigger_result
//         );
//         //971101161992692513852994889169636864368995854269381445492871892126177753284315892653
//         // Check division of very big positive BigInts.
//         let positive_very_big_bigger_by_positive_very_big_bigger_result =
//             ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             &positive_bigger_very_big_bigint / &positive_bigger_very_big_bigint,
//             positive_very_big_bigger_by_positive_very_big_bigger_result
//         );
//         let positive_very_big_bigger_by_positive_very_big_smaller_result = ChonkerInt::from(String::from("613188126075967005949151232496730640186830021148675211669412879242084924584975233197896711416"));
//         assert_eq!(
//             &positive_bigger_very_big_bigint / &positive_smaller_very_big_bigint,
//             positive_very_big_bigger_by_positive_very_big_smaller_result
//         );
//         let positive_very_big_smaller_by_positive_very_big_smaller_result =
//             ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             &positive_smaller_very_big_bigint / &positive_smaller_very_big_bigint,
//             positive_very_big_smaller_by_positive_very_big_smaller_result
//         );
//         let positive_very_big_smaller_by_positive_very_big_bigger_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &positive_smaller_very_big_bigint / &positive_bigger_very_big_bigint,
//             positive_very_big_smaller_by_positive_very_big_bigger_result
//         );
//
//         // Check division of very big negative BigInts.
//         let negative_very_big_bigger_by_negative_very_big_bigger_result =
//             ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             &negative_bigger_very_big_bigint / &negative_bigger_very_big_bigint,
//             negative_very_big_bigger_by_negative_very_big_bigger_result
//         );
//         let negative_very_big_bigger_by_negative_very_big_smaller_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_bigger_very_big_bigint / &negative_smaller_very_big_bigint,
//             negative_very_big_bigger_by_negative_very_big_smaller_result
//         );
//         let negative_very_big_smaller_by_negative_very_big_smaller_result =
//             ChonkerInt::from(String::from("1"));
//         assert_eq!(
//             &negative_smaller_very_big_bigint / &negative_smaller_very_big_bigint,
//             negative_very_big_smaller_by_negative_very_big_smaller_result
//         );
//         let negative_very_big_smaller_by_negative_very_big_bigger_result = ChonkerInt::from(String::from("2112199875473649343041361462004034608444019134994613761100327207438381586024335294"));
//         assert_eq!(
//             &negative_smaller_very_big_bigint / &negative_bigger_very_big_bigint,
//             negative_very_big_smaller_by_negative_very_big_bigger_result
//         );
//
//         //                                                             [3, 5, 6, 2, 9, 8, 5, 1, 3, 4, 8, 2, 3, 5, 7, 7, 7, 1, 6, 2, 1, 2, 9, 8, 1, 7, 8, 2, 9, 4, 5, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 4, 6, 8, 6, 3, 6, 9, 6, 1, 9, 8, 8, 4, 9, 9, 2, 5, 8, 3, 1, 5, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 9, 7, 1, 1],
//         //                               [3, 0, 5, 6, 2, 9, 8, 5, 1, 3, 4, 8, 2, 3, 5, 7, 7, 0, 7, 1, 6, 2, 1, 2, 9, 8, 0, 1, 7, 8, 2, 9, ! 4, 5, 0, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 0, 4, 6, 8, 0, 6, 3, 6, 9, 6, 1, 9, 8, 8, 0, 4, 9, 0, 9, 2, 5, 8, 3, 1, 5, 0, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 0, 9, 7, 1, 1]
//         // [1, 9, 4, 5, 9, 3, 8, 8, 2, 4, 2, 2, 7, 6, 6, 1, 6, 7, 2, 5, 3, 5, 0, 9, 3, 8, 4, 3, 3, 5, 6, 8, 2, 3, 1, 3, 9, 7, 0, 1, 7, 8, ! 4, 5, 0, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 0, 4, 6, 8, 0, 6, 3, 6, 9, 6, 1, 9, 8, 8, 0, 4, 9, 0, 9, 2, 5, 8, 3, 1, 5, 0, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 0, 9, 7, 1, 1]
//         // [1, 9, 4, 5, 9, 3, 8, 8, 2, 4, 2, 2, 7, 6, 6, 1, 6, 7, 2, 5, 3, 5, 0, 9, 3, 8, 4, 3, 3, 5, 6, 8, 2, 3, 1, 3, 9, 7, 0, 1, 7, 8, ! 4, 5, 0, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 0, 4, 6, 8, 0, 6, 3, 6, 9, 6, 1, 9, 8, 8, 0, 4, 9, 0, 9, 2, 5, 8, 3, 1, 5, 0, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 0, 9, 7, 1, 1]
//         // [1, 9, 4, 5, 9, 3, 8, 8, 2, 4, 2, 2, 7, 6, 6, 1, 6, 7, 2, 5, 3, 5, 0, 9, 3, 8, 4, 3, 3, 5, 6, 8, 2, 3, 1, 3, 9, 7, 0, 1, 7, 8, ! 4, 5, 0, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 0, 4, 6, 8, 0, 6, 3, 6, 9, 6, 1, 9, 8, 8, 0, 4, 9, 0, 9, 2, 5, 8, 3, 1, 5, 0, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 0, 9, 7, 1, 1]
//         // 19459388242276616725350938433568231397017845044183962458599863046806369619880490925831502962991610109711
//     }
//
//     // Test modulus division of two BigInts.
//     #[test]
//     fn test_bigint_modulus_division() {
//         // Positive BigInts.
//         let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
//         let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));
//
//         // Negative BigInts.
//         let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
//         let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));
//
//         // Very big BigInts.
//         let positive_bigger_very_big_bigint = ChonkerInt::from(String::from("4379853178597859156740573149857154310578942357435678165781568134756871356187956143975358713583915634785431658143560178536107563147805634807561348506134"));
//         let positive_smaller_very_big_bigint = ChonkerInt::from(String::from(
//             "7142756019471983982475239851587182390573438756286598175918",
//         ));
//         let negative_bigger_very_big_bigint = ChonkerInt::from(String::from(
//             "-3714856173245610358671095834519578134957135871390587314982",
//         ));
//         let negative_smaller_very_big_bigint = ChonkerInt::from(String::from("-7846518746531895729834723194263984236421304673218561384612384623198412894123506123859123452319048712958714309584104712340823408213842130948"));
//
//         // Empty/zero BigInt.
//         let zero_bigint = ChonkerInt::from(String::from("0"));
//
//         // Check modulus division of positive BigInts. Try different positions.
//         let positive_bigger_by_smaller_modulus_division_result =
//             ChonkerInt::from(String::from("6308"));
//         assert_eq!(
//             &positive_bigger_bigint % &positive_smaller_bigint,
//             positive_bigger_by_smaller_modulus_division_result
//         );
//         let positive_smaller_by_bigger_modulus_division_result =
//             ChonkerInt::from(String::from("23423"));
//         assert_eq!(
//             &positive_smaller_bigint % &positive_bigger_bigint,
//             positive_smaller_by_bigger_modulus_division_result
//         );
//
//         // Check modulus division of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
//         let bigger_positive_by_smaller_negative_modulus_division_result =
//             ChonkerInt::from(String::from("-1130000"));
//         assert_eq!(
//             &positive_bigger_bigint % &negative_smaller_bigint,
//             bigger_positive_by_smaller_negative_modulus_division_result
//         );
//         let smaller_negative_by_bigger_positive_modulus_division_result =
//             ChonkerInt::from(String::from("70000"));
//         assert_eq!(
//             &negative_smaller_bigint % &positive_bigger_bigint,
//             smaller_negative_by_bigger_positive_modulus_division_result
//         );
//
//         // Check modulus division of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
//         let smaller_positive_by_bigger_negative_modulus_division_result =
//             ChonkerInt::from(String::from("-1267"));
//         assert_eq!(
//             &positive_smaller_bigint % &negative_bigger_bigint,
//             smaller_positive_by_bigger_negative_modulus_division_result
//         );
//         let bigger_negative_by_smaller_positive_modulus_division_result =
//             ChonkerInt::from(String::from("11078"));
//         assert_eq!(
//             &negative_bigger_bigint % &positive_smaller_bigint,
//             bigger_negative_by_smaller_positive_modulus_division_result
//         );
//
//         // Check modulus division of negative BigInts. Try different positions.
//         let negative_bigger_by_smaller_modulus_division_result =
//             ChonkerInt::from(String::from("-12345"));
//         assert_eq!(
//             &negative_bigger_bigint % &negative_smaller_bigint,
//             negative_bigger_by_smaller_modulus_division_result
//         );
//         let negative_smaller_by_bigger_modulus_division_result =
//             ChonkerInt::from(String::from("-7845"));
//         assert_eq!(
//             &negative_smaller_bigint % &negative_bigger_bigint,
//             negative_smaller_by_bigger_modulus_division_result
//         );
//
//         // Check modulus division of positive BigInts with an empty/zero BigInt. Try different positions.
//         // let positive_bigger_by_empty_modulus_division_result = ChonkerInt::from(String::from("0"));
//         // assert_eq!(&positive_bigger_bigint % &zero_bigint, positive_bigger_by_empty_division_result);
//
//         let empty_by_positive_smaller_modulus_division_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &zero_bigint % &positive_smaller_bigint,
//             empty_by_positive_smaller_modulus_division_result
//         );
//
//         // Check modulus division of negative BigInts with an empty/zero BigInt. Try different positions.
//         // let negative_bigger_by_empty_modulus_division_result = ChonkerInt::from(String::from("0"));
//         // assert_eq!(&negative_bigger_bigint % &zero_bigint, negative_bigger_by_empty_division_result);
//
//         let empty_by_negative_smaller_modulus_division_result = ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &zero_bigint % &negative_smaller_bigint,
//             empty_by_negative_smaller_modulus_division_result
//         );
//
//         // Check modulus division of very BigInts.
//         let positive_very_big_bigger_by_negative_very_big_smaller_result = ChonkerInt::from(String::from("-4160032753209023619198251321622880242821224555894280046814442463878143415701119919859842686220966394065360364693594909227653167239533471886"));
//         assert_eq!(
//             &positive_bigger_very_big_bigint % &negative_smaller_very_big_bigint,
//             positive_very_big_bigger_by_negative_very_big_smaller_result
//         );
//         let positive_very_big_smaller_by_negative_very_big_smaller_result = ChonkerInt::from(String::from("-7846518746531895729834723194263984236421304673218561384612384623198412894123506116716367432847064730483474457996922321767384651927243955030"));
//         assert_eq!(
//             &positive_smaller_very_big_bigint % &negative_smaller_very_big_bigint,
//             positive_very_big_smaller_by_negative_very_big_smaller_result
//         );
//         let positive_very_big_bigger_by_negative_very_big_bigger_result = ChonkerInt::from(
//             String::from("-2040600818200247843220407180540756771951755806741487576794"),
//         );
//         assert_eq!(
//             &positive_bigger_very_big_bigint % &negative_bigger_very_big_bigint,
//             positive_very_big_bigger_by_negative_very_big_bigger_result
//         );
//         let positive_very_big_smaller_by_negative_very_big_bigger_result = ChonkerInt::from(
//             String::from("-286956327019236734866951817451973879340832986494576454046"),
//         );
//         assert_eq!(
//             &positive_smaller_very_big_bigint % &negative_bigger_very_big_bigint,
//             positive_very_big_smaller_by_negative_very_big_bigger_result
//         );
//
//         let negative_very_big_bigger_by_positive_very_big_smaller_result = ChonkerInt::from(
//             String::from("3427899846226373623804144017067604255616302884896010860936"),
//         );
//         assert_eq!(
//             &negative_bigger_very_big_bigint % &positive_smaller_very_big_bigint,
//             negative_very_big_bigger_by_positive_very_big_smaller_result
//         );
//         let negative_very_big_smaller_by_positive_very_big_smaller_result = ChonkerInt::from(
//             String::from("4316350245186309090610264867148023910380230603937925784652"),
//         );
//         assert_eq!(
//             &negative_smaller_very_big_bigint % &positive_smaller_very_big_bigint,
//             negative_very_big_smaller_by_positive_very_big_smaller_result
//         );
//         let negative_very_big_bigger_by_positive_very_big_bigger_result = ChonkerInt::from(String::from("4379853178597859156740573149857154310578942357435678165781568134756871356187956143975358713580200778612186047784889082701587985012848498936170761191152"));
//         assert_eq!(
//             &negative_bigger_very_big_bigint % &positive_bigger_very_big_bigint,
//             negative_very_big_bigger_by_positive_very_big_bigger_result
//         );
//         let negative_very_big_smaller_by_positive_very_big_bigger_result = ChonkerInt::from(String::from("4379853178597859156740573149857154310578942357435678165781568134756871356187956143975358713580200778612186047784889082701587985012848498936170761191152"));
//         assert_eq!(
//             &negative_bigger_very_big_bigint % &positive_bigger_very_big_bigint,
//             negative_very_big_smaller_by_positive_very_big_bigger_result
//         );
//
//         // Check modulus division of very big positive BigInts.
//         let positive_very_big_bigger_by_positive_very_big_bigger_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &positive_bigger_very_big_bigint % &positive_bigger_very_big_bigint,
//             positive_very_big_bigger_by_positive_very_big_bigger_result
//         );
//         let positive_very_big_bigger_by_positive_very_big_smaller_result = ChonkerInt::from(
//             String::from("5918573268533236639722045796608111873199720905564901626246"),
//         );
//         assert_eq!(
//             &positive_bigger_very_big_bigint % &positive_smaller_very_big_bigint,
//             positive_very_big_bigger_by_positive_very_big_smaller_result
//         );
//         let positive_very_big_smaller_by_positive_very_big_smaller_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &positive_smaller_very_big_bigint % &positive_smaller_very_big_bigint,
//             positive_very_big_smaller_by_positive_very_big_smaller_result
//         );
//         let positive_very_big_smaller_by_positive_very_big_bigger_result = ChonkerInt::from(
//             String::from("7142756019471983982475239851587182390573438756286598175918"),
//         );
//         assert_eq!(
//             &positive_smaller_very_big_bigint % &positive_bigger_very_big_bigint,
//             positive_very_big_smaller_by_positive_very_big_bigger_result
//         );
//
//         // Check modulus division of very big negative BigInts.
//         let negative_very_big_bigger_by_negative_very_big_bigger_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_bigger_very_big_bigint % &negative_bigger_very_big_bigint,
//             negative_very_big_bigger_by_negative_very_big_bigger_result
//         );
//         let negative_very_big_bigger_by_negative_very_big_smaller_result = ChonkerInt::from(
//             String::from("-3714856173245610358671095834519578134957135871390587314982"),
//         );
//         assert_eq!(
//             &negative_bigger_very_big_bigint % &negative_smaller_very_big_bigint,
//             negative_very_big_bigger_by_negative_very_big_smaller_result
//         );
//         let negative_very_big_smaller_by_negative_very_big_smaller_result =
//             ChonkerInt::from(String::from("0"));
//         assert_eq!(
//             &negative_smaller_very_big_bigint % &negative_smaller_very_big_bigint,
//             negative_very_big_smaller_by_negative_very_big_smaller_result
//         );
//         let negative_very_big_smaller_by_negative_very_big_bigger_result = ChonkerInt::from(
//             String::from("-1368537949400640214032806262892480125625296555619084556240"),
//         );
//         assert_eq!(
//             &negative_smaller_very_big_bigint % &negative_bigger_very_big_bigint,
//             negative_very_big_smaller_by_negative_very_big_bigger_result
//         );
//     }
//
//     // Test change of BigInt's sign, when it is empty.
//     #[test]
//     fn test_empty_bigint_negation() {
//         let negated_empty_bigint = ChonkerInt::new().neg();
//
//         assert_eq!(ChonkerInt::new(), negated_empty_bigint);
//     }
//
//     // Test change of BigInt's sign, when it is negative.
//     #[test]
//     fn test_negative_bigint_negation() {
//         let negated_negative_bigint = ChonkerInt::from(String::from("-123")).neg();
//         let comparison_positive_bigint = ChonkerInt::from(String::from("123"));
//
//         assert_eq!(comparison_positive_bigint, negated_negative_bigint);
//     }
//
//     // Test change of BigInt's sign, when it is positive.
//     #[test]
//     fn test_positive_bigint_negation() {
//         let negated_positive_bigint = ChonkerInt::from(String::from("123")).neg();
//         let comparison_negative_bigint = ChonkerInt::from(String::from("-123"));
//
//         assert_eq!(comparison_negative_bigint, negated_positive_bigint);
//     }
// }
