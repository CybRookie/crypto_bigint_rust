// BigInt module regarding conversion from a specific data type to BigInt and vice versa.

use std::fmt::{Display, Formatter};
use std::str::from_utf8_unchecked;

use num_traits::{PrimInt, Signed, Zero};

use crate::logic::bigint::{BigIntSign, ChonkerInt, ASCII_DIFF};

// Implement conversion methods for BigInt.
// Turn BigInt into a string consisting of its digits in big endian format.
impl Display for ChonkerInt {
    // Produce a string representing the number inside the BigInt.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Check if the BigInt is zero.
        if (*self) == ChonkerInt::new() {
            return write!(f, "0");
        }

        let mut digits_vec = self.digits.clone();
        digits_vec.reverse();

        let modified_digits_vec: Vec<u8> = digits_vec
            .iter_mut()
            .map(|x| (*x + ASCII_DIFF) as u8)
            .collect();

        let digits_string = unsafe { from_utf8_unchecked(&modified_digits_vec) };

        let mut result;
        // If the BigInt is negative, add minus/hyphen to the result string.
        result = String::from(digits_string);
        if self.sign == BigIntSign::Negative {
            let minus = '-';
            result.insert(0, minus);
        }

        write!(f, "{}", result)
    }
}

// Turn BigInt into a 16 byte unsigned integer consiting of its digits in big endian format.
impl ChonkerInt {
    pub fn to_digit(&self) -> u128 {
        // Check if the BigInt is zero.
        if (*self) == ChonkerInt::new() {
            return 0u128;
        }

        // Check if the BigInt is too big for the 16 byte unsigned integer.
        if self > &ChonkerInt::from(u128::MAX) {
            panic!("the target BigInt for conversion into the unsgned 16 byte integer is too long/big (ChonkerInt::to_digit)")
        }

        // Define the initial base of 1 that will be multiplied by 10 for each digit.
        let mut base = 1;
        let digits_iter = self.digits.iter();
        let mut result_integer: u128 = 0;
        let max_base_value = (10_u128).pow(38);

        // Multiply each digit by the appropriate base/order and store the result in the result integer.
        // Check if the base hit the limit, if it did, exit the loop.
        for digit in digits_iter {
            result_integer += *digit as u128 * base;

            if base == max_base_value {
                break;
            }

            base *= 10;
        }

        result_integer
    }
}

// Conversion of an integer into BigInt.
fn digit_convert(int: &u128) -> Result<Vec<i8>, Box<dyn std::error::Error>> {
    let mut base = 10;
    let max_base_value = (10_u128).pow(38);
    let mut prev_base = 10;
    let mut remainder = *int % base;
    let mut prev_remainder = remainder;
    let mut digit;
    // Create a result vector and add the first element.
    let mut result_vec: Vec<i8> = vec![remainder as i8];

    base *= 10;
    while remainder != *int {
        // Calculate remainder with the current base and then calculate the target digit with it.
        remainder = *int % base;
        digit = (remainder - prev_remainder) / prev_base;

        result_vec.push(digit as i8);

        prev_remainder = remainder;
        prev_base = base;

        // Check if the base hit the limit, if it did,
        // calculate the last digit most significant separately and exit the loop.
        if base == max_base_value {
            digit = (*int - prev_remainder) / prev_base;
            result_vec.push(digit as i8);
            break;
        }
        base *= 10;
    }

    Ok(result_vec)
}

// Conversions from specific integer data types.
// Convert a 16 byte signed integer into a BigInt.
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
//         digit_vector_produce(&unsigned_int, sign)
//     }
// }

// Convert a 16 byte unsigned integer into a BigInt.
// impl From<u128> for ChonkerInt {
//     fn from(int: u128) -> ChonkerInt {
//         // Check for zero.
//         if int == 0 {
//             return ChonkerInt::new();
//         }
//
//         let mut sign = BigIntSign::Positive;
//
//         digit_vector_produce(&int, sign)
//     }
// }

// Accept the 16 byte unsigned integer, produce the vector of unsigned bytes,
// representing separate digits and use it in BigInt construction.
fn digit_vector_produce(int: &u128, sign: BigIntSign) -> ChonkerInt {
    // Define BigInt's vector, check for the integer being zero.
    let digits = if *int == 0 {
        vec![]
    } else {
        digit_convert(int).unwrap()
    };

    ChonkerInt { digits, sign }
}

// Convert a generic integer into a BigInt.
impl From<u128> for ChonkerInt {
    fn from(int: u128) -> ChonkerInt {
        generic_from_unsigned_integer(int)
    }
}

impl From<u64> for ChonkerInt {
    fn from(int: u64) -> ChonkerInt {
        generic_from_unsigned_integer(int)
    }
}

impl From<u32> for ChonkerInt {
    fn from(int: u32) -> ChonkerInt {
        generic_from_unsigned_integer(int)
    }
}

impl From<u16> for ChonkerInt {
    fn from(int: u16) -> ChonkerInt {
        generic_from_unsigned_integer(int)
    }
}

impl From<u8> for ChonkerInt {
    fn from(int: u8) -> ChonkerInt {
        generic_from_unsigned_integer(int)
    }
}

impl From<i128> for ChonkerInt {
    fn from(int: i128) -> ChonkerInt {
        generic_from_signed_integer(int)
    }
}

impl From<i64> for ChonkerInt {
    fn from(int: i64) -> ChonkerInt {
        generic_from_signed_integer(int)
    }
}

impl From<i32> for ChonkerInt {
    fn from(int: i32) -> ChonkerInt {
        generic_from_signed_integer(int)
    }
}

impl From<i16> for ChonkerInt {
    fn from(int: i16) -> ChonkerInt {
        generic_from_signed_integer(int)
    }
}

impl From<i8> for ChonkerInt {
    fn from(int: i8) -> ChonkerInt {
        generic_from_signed_integer(int)
    }
}

// For generic operations: num-traits = "0.2.14" crate should be added to the Cargo.toml
// Convert a generic integer into a BigInt.
// This function is used under from functions for specific types under specific traits.
fn generic_from_unsigned_integer<T>(int: T) -> ChonkerInt
where
    T: PrimInt + Zero,
{
    // Check for zero.
    if int == T::zero() {
        return ChonkerInt::new();
    }

    let sign = BigIntSign::Positive;
    let unsigned_int = int.to_u128().unwrap_or(0);

    // Define BigInt's vector, check for the integer being zero.
    digit_vector_produce(&unsigned_int, sign)
}

// Convert a generic signed integer into a BigInt.
// This function is used under from functions for specific types under specific traits.
fn generic_from_signed_integer<T>(int: T) -> ChonkerInt
where
    T: PrimInt + Signed + Zero,
{
    // Check for zero.
    if int == T::zero() {
        return ChonkerInt::new();
    }

    let mut sign = BigIntSign::Zero;

    // Determine the sign of the target and save it.
    if int.is_negative() {
        sign = BigIntSign::Negative;
    } else {
        sign = BigIntSign::Positive;
    }

    let unsigned_int = (int.abs()).to_u128().unwrap_or(0);

    // Define BigInt's vector, check for the integer being zero.
    digit_vector_produce(&unsigned_int, sign)
}

// Convert a string of digits, possibly with a minus/hyphnen sign, into a BigInt.
impl From<String> for ChonkerInt {
    fn from(string: String) -> ChonkerInt {
        let mut char_iter = string.chars();
        let mut sign = BigIntSign::Positive;
        let mut digits = string.clone();

        // Check if the first character is minus/hyphen or a number.
        // If so, proceed with further checking.
        // If not, return an empty BigInt.
        if let Some(char) = char_iter.next() {
            if !(char == '-' || char.is_numeric()) {
                return ChonkerInt {
                    digits: vec![],
                    sign: BigIntSign::Zero,
                };
            } else if char == '-' {
                // If the first char was negative and others numbers,
                // remove the first hyphen char and change sign's value to negative.
                sign = BigIntSign::Negative;
                digits.remove(0);
            }
        }

        // Check if every other character is numeric.
        // If the input string is incorrect, return an empty BigInt.
        for char in char_iter {
            if !char.is_numeric() {
                return ChonkerInt {
                    digits: vec![],
                    sign: BigIntSign::Zero,
                };
            }
        }

        // Check for leading zeros, if present, remove them.
        // Check if the first char was a minus/hyphen in the original string, if so, skip it.
        let mut char_iter = string.chars();
        if sign == BigIntSign::Negative {
            char_iter.next();
        }
        for char in char_iter {
            if char == '0' {
                digits.remove(0);
            } else {
                break;
            }
        }

        // Check if the string consisted only from zeros, if so, return an empty BigInt.
        let digits_length = digits.len();
        if digits_length == 0 {
            return ChonkerInt {
                digits: vec![],
                sign: BigIntSign::Zero,
            };
        }

        // Convert string into a vector/slice of one byte unsigned integers, containing digits of a numbers.
        // Subtract 48 from ASCII/UTF-8 representation of integers to get true integers.
        let mut digits: Vec<i8> = unsafe { digits.as_bytes_mut() }
            .iter_mut()
            .map(|x| (*x - (ASCII_DIFF as u8)) as i8)
            .collect();
        digits.reverse();

        ChonkerInt { digits, sign }
    }
}

// Convert a one byte slice of digits into a positive BigInt.
// It is assumed that the target one byte slice contains digits in the little endian format,
// and values from the range of 0-9.
impl From<&[u8]> for ChonkerInt {
    fn from(slice: &[u8]) -> ChonkerInt {
        let mut digit_iter = slice.iter();
        let sign = BigIntSign::Positive;
        let mut digits: Vec<i8> = digit_iter.map(|digit| *digit as i8).collect();
        digit_iter = slice.iter();

        // Check if the numbers are in range of 0-9.
        let digit_range = 0..=9;
        for digit in digit_iter {
            if !digit_range.contains(digit) {
                panic!("the one byte slice for BigInt conversions contained values out of allowed range of 0-9 (From<&[u8]> for ChonkerInt)");
            }
        }

        // Check for leading zeros, if present, remove them.
        // Reverse digits to the big endian before check and back after.
        digits.reverse();
        let digit_iter = slice.iter().rev();
        for digit in digit_iter {
            if *digit == 0u8 {
                digits.remove(0);
            } else {
                break;
            }
        }
        digits.reverse();

        // Check if the vector consisted only from zeros, if so, return an empty BigInt.
        if digits.is_empty() {
            return ChonkerInt {
                digits: vec![],
                sign: BigIntSign::Zero,
            };
        }

        ChonkerInt { digits, sign }
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::conversion::digit_convert;
    use crate::logic::bigint::ChonkerInt;
    use crate::logic::error::OperationError;

    // Test BigInt to string conversion.
    #[test]
    fn test_bigint_to_string_conversion() {
        let positive_bigint = ChonkerInt::from(012300);
        let negative_bigint = ChonkerInt::from(-012300);
        let zero_bigint1 = ChonkerInt::new();
        let zero_bigint2 = ChonkerInt::from(0);
        let zero_bigint3 = ChonkerInt::from(String::from("0"));

        let positive_bigint_string = String::from("12300");
        let negative_bigint_string = String::from("-12300");
        let zero_bigint_string = String::from("0");

        assert_eq!(positive_bigint.to_string(), positive_bigint_string);
        assert_eq!(negative_bigint.to_string(), negative_bigint_string);
        assert_eq!(zero_bigint1.to_string(), zero_bigint_string);
        assert_eq!(zero_bigint2.to_string(), zero_bigint_string);
        assert_eq!(zero_bigint3.to_string(), zero_bigint_string);
    }

    // Test BigInt to unsigned 16 byte integer conversion.
    #[test]
    fn test_bigint_to_unsigned_integer_conversion() {
        let positive_bigint_max = ChonkerInt::from(u128::MAX);
        let positive_bigint = ChonkerInt::from(012300);
        let negative_bigint = ChonkerInt::from(-012300);
        let zero_bigint1 = ChonkerInt::new();
        let zero_bigint2 = ChonkerInt::from(0);
        let zero_bigint3 = ChonkerInt::from(String::from("0"));

        let positive_bigint_number_max = u128::MAX;
        let positive_bigint_number = 12300u128;
        let negative_bigint_number = 12300u128;
        let zero_bigint_number = 0u128;

        assert_eq!(positive_bigint_max.to_digit(), positive_bigint_number_max);
        assert_eq!(positive_bigint.to_digit(), positive_bigint_number);
        assert_eq!(negative_bigint.to_digit(), negative_bigint_number);
        assert_eq!(zero_bigint1.to_digit(), zero_bigint_number);
        assert_eq!(zero_bigint2.to_digit(), zero_bigint_number);
        assert_eq!(zero_bigint3.to_digit(), zero_bigint_number);
    }

    // Test u128 integer conversion into a BigInt
    #[test]
    fn test_digit_conversion() {
        let target: u128 = 123_123_123_123_123_123_123_123_123_123_123_123_1;
        let result = match digit_convert(&target) {
            Ok(result) => result,
            Err(e) => {
                match e.downcast::<OperationError>() {
                    Ok(value) => panic!("    An overflow occurred during conversion of an u128 integer into a BigInt: {}. (test_digit_conversion)", value),
                    Err(e) => panic!("    An error was encountered during conversion of an u128 integer into a BigInt: {}. (test_digit_conversion)", e),
                }
            }
        };

        // let comparison_vec: Vec<i8> = vec![3,4,0,2,8,2,3,6,6,9,2,0,9,3,8,4,6,3,4,6,3,3,7,4,6,0,7,4,3,1,7,6,8,2,1,1,4,5,5,];
        let mut comparison_vec: Vec<i8> = vec![
            1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2,
            3, 1, 2, 3, 1, 2, 3, 1,
        ];
        comparison_vec.reverse();

        assert_eq!(result, comparison_vec);
    }

    // Test string conversion into a BigInt
    #[test]
    fn test_string_conversion() {
        let target = String::from("-00000000000000123123123123123123123123123123123123123123123123123123123123123123123123123123123123123123");
        let other_target = String::from("100000");
        let result = ChonkerInt::from(target);
        let other_result = ChonkerInt::from(other_target);

        // let comparison_vec: Vec<i8> = vec![3,4,0,2,8,2,3,6,6,9,2,0,9,3,8,4,6,3,4,6,3,3,7,4,6,0,7,4,3,1,7,6,8,2,1,1,4,5,5,];
        let mut comparison_vec: Vec<i8> = vec![
            1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2,
            3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1,
            2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
            1, 2, 3,
        ];
        comparison_vec.reverse();
        let mut other_comparison_vec: Vec<i8> = vec![1, 0, 0, 0, 0, 0];
        other_comparison_vec.reverse();

        assert_eq!(*result.get_vec(), comparison_vec);
        assert_eq!(*other_result.get_vec(), other_comparison_vec);
    }

    // Test one byte slice conversion into a BigInt
    #[test]
    fn test_unsigned_one_byte_slice_conversion() {
        let mut target: Vec<u8> = Vec::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
            1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2,
            3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1,
            2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
        ]);
        target.reverse();
        let mut other_target: Vec<u8> = Vec::from([1, 0, 0, 0, 0, 0]);
        other_target.reverse();
        let result = ChonkerInt::from(target.as_slice());
        let other_result = ChonkerInt::from(other_target.as_slice());

        // let comparison_vec: Vec<i8> = vec![3,4,0,2,8,2,3,6,6,9,2,0,9,3,8,4,6,3,4,6,3,3,7,4,6,0,7,4,3,1,7,6,8,2,1,1,4,5,5,];
        let mut comparison_vec: Vec<i8> = vec![
            1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2,
            3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1,
            2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
            1, 2, 3,
        ];
        comparison_vec.reverse();
        let mut other_comparison_vec: Vec<i8> = vec![1, 0, 0, 0, 0, 0];
        other_comparison_vec.reverse();

        assert_eq!(*result.get_vec(), comparison_vec);
        assert_eq!(*other_result.get_vec(), other_comparison_vec);
    }
}
