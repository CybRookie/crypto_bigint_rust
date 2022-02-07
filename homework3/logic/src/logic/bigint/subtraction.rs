// BigInt module regarding subtraction of BigInts.

use std::cmp::Ordering;
use std::ops::{Neg, Sub};

use crate::logic::bigint::{BigIntSign, ChonkerInt, RADIX};

// Implement subtraction "-" operator for the BigInt.
// Subtraction is done with school style long subtraction.
impl<'a, 'b> Sub<&'b ChonkerInt> for &'a ChonkerInt {
    type Output = ChonkerInt;

    fn sub(self, other: &'b ChonkerInt) -> Self::Output {
        // If self is empty/zero, return other BigInt with a possible change of its sign.
        if (self.sign == BigIntSign::Zero)
            && ((other.sign == BigIntSign::Negative) || (other.sign == BigIntSign::Positive))
        {
            let other_clone = (*other).clone();
            let negated_clone = other_clone.neg();
            return negated_clone;
        }

        // If other is empty/zero, return self.
        if other.sign == BigIntSign::Zero {
            return (*self).clone();
        }

        // Check the signs of both operands, if they are not the same, determine an appropriate operation.
        // If the signs are the same and negative, negate them and recall subtraction operation,
        // then negate the retrieved result.
        if self.sign != other.sign {
            return if self.sign == BigIntSign::Positive {
                self + &(-other) // +self + (-(-other)) = self + other
            } else {
                let sum_result: ChonkerInt = &(-self) + other;
                sum_result.neg()
                // -(&(-self) + other) // -((-(-self)) + other) = -(self + other)
            };
        } else if self.sign == BigIntSign::Negative {
            let diff_result: ChonkerInt = &(-self) - &(-other);
            return diff_result.neg();
            // return -((-self) - (-other)); // -((-(-self)) - (-(-other))) = -(self - other)
        }

        let mut result = ChonkerInt::new();

        let mut last_digit_underflow = 0;
        let mut self_offset = 0;
        let self_length = self.digits.len();
        let mut other_offset = 0;
        let other_length = other.digits.len();

        // Determine the longer/bigger BigInt and calculate subtraction and a sign.
        // If vector were of different lengths, finish operation on the digits of the longer vector/BigInt with a second loop.
        match self.cmp(other) {
            Ordering::Less => {
                while self_offset < self_length {
                    subtract_digits(
                        &other.digits,
                        &self.digits,
                        &mut other_offset,
                        &mut self_offset,
                        &mut result,
                        &mut last_digit_underflow,
                    )
                }
                while other_offset < other_length {
                    subtract_digit_and_underflow(
                        &other.digits,
                        &mut other_offset,
                        &mut result,
                        &mut last_digit_underflow,
                    );
                }
                result.set_negative_sign();
            }
            Ordering::Equal => {
                // Both are equal, return and empty/zero BigInt.
                return result;
            }
            Ordering::Greater => {
                while other_offset < other_length {
                    subtract_digits(
                        &self.digits,
                        &other.digits,
                        &mut self_offset,
                        &mut other_offset,
                        &mut result,
                        &mut last_digit_underflow,
                    );
                }
                while self_offset < self_length {
                    subtract_digit_and_underflow(
                        &self.digits,
                        &mut self_offset,
                        &mut result,
                        &mut last_digit_underflow,
                    );
                }
                result.set_positive_sign();
            }
        }

        result.normalize();

        result
    }
}

// Subtract of two passed digits.
fn subtract_digits(
    minuend_vec: &[i8],
    subtrahend_vec: &[i8],
    minuend_offset: &mut usize,
    subtrahend_offset: &mut usize,
    result: &mut ChonkerInt,
    last_digit_underflow: &mut i8,
) {
    // Calculate sum of digits.
    let mut difference = (*minuend_vec)[*minuend_offset]
        - (*subtrahend_vec)[*subtrahend_offset]
        - (*last_digit_underflow);

    // Nullify the underflow from previous operation after being used.
    *last_digit_underflow = 0;

    // Check for the difference being negative, if it is, set underflow.
    if difference.is_negative() {
        *last_digit_underflow = 1;
        difference += RADIX;
    }

    let _ = result.push(difference);
    *minuend_offset += 1;
    *subtrahend_offset += 1;
}

// Subtract of one passed digit and a result slot.
fn subtract_digit_and_underflow(
    one_vec: &[i8],
    one_offset: &mut usize,
    result: &mut ChonkerInt,
    last_digit_underflow: &mut i8,
) {
    // Calculate sum of digits.
    let mut difference = (*one_vec)[*one_offset] - (*last_digit_underflow);

    // Nullify the underflow from previous operation after being used.
    *last_digit_underflow = 0;

    // Check for the difference being negative, if it is, set underflow.
    if difference.is_negative() {
        *last_digit_underflow = 1;
        difference += RADIX;
    }

    let _ = result.push(difference);
    *one_offset += 1;
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::subtraction::{subtract_digit_and_underflow, subtract_digits};
    use crate::logic::bigint::ChonkerInt;

    // Test of BigInt's subtraction operation.
    #[test]
    fn test_bigint_subtraction() {
        // Positive BigInts.
        let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
        let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));

        // Negative BigInts.
        let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
        let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));

        // Empty/zero BigInt.
        let zero_bigint = ChonkerInt::from(String::from("0"));

        // Check subtraction from itself.
        let subtraction_from_itself = ChonkerInt::new();
        assert_eq!(
            &positive_smaller_bigint - &positive_smaller_bigint,
            subtraction_from_itself
        );

        // Check subtraction of positive BigInts. Try different positions.
        let positive_subtraction_smaller_from_bigger_result =
            ChonkerInt::from(String::from("76577"));
        assert_eq!(
            &positive_bigger_bigint - &positive_smaller_bigint,
            positive_subtraction_smaller_from_bigger_result
        );
        let positive_subtraction_bigger_from_smaller_result =
            ChonkerInt::from(String::from("-76577"));
        assert_eq!(
            &positive_smaller_bigint - &positive_bigger_bigint,
            positive_subtraction_bigger_from_smaller_result
        );

        // Check subtraction of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
        let from_bigger_positive_bigger_negative_subtraction_result =
            ChonkerInt::from(String::from("1330000"));
        assert_eq!(
            &positive_bigger_bigint - &negative_smaller_bigint,
            from_bigger_positive_bigger_negative_subtraction_result
        );
        let bigger_positive_from_bigger_negative_subtraction_result =
            ChonkerInt::from(String::from("-1330000"));
        assert_eq!(
            &negative_smaller_bigint - &positive_bigger_bigint,
            bigger_positive_from_bigger_negative_subtraction_result
        );

        // Check subtraction of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
        let from_smaller_positive_bigger_negative_subtraction_result =
            ChonkerInt::from(String::from("35768"));
        assert_eq!(
            &positive_smaller_bigint - &negative_bigger_bigint,
            from_smaller_positive_bigger_negative_subtraction_result
        );
        let smaller_positive_from_bigger_negative_subtraction_result =
            ChonkerInt::from(String::from("-35768"));
        assert_eq!(
            &negative_bigger_bigint - &positive_smaller_bigint,
            smaller_positive_from_bigger_negative_subtraction_result
        );

        // Check subtraction of negative BigInts. Try different positions.
        let smaller_from_bigger_negative_subtraction_result =
            ChonkerInt::from(String::from("1217655"));
        assert_eq!(
            &negative_bigger_bigint - &negative_smaller_bigint,
            smaller_from_bigger_negative_subtraction_result
        );
        let bigger_from_smaller_negative_subtraction_result =
            ChonkerInt::from(String::from("-1217655"));
        assert_eq!(
            &negative_smaller_bigint - &negative_bigger_bigint,
            bigger_from_smaller_negative_subtraction_result
        );

        // Check subtraction of positive BigInts with an empty/zero BigInt. Try different positions.
        let from_bigger_positive_empty_subtraction_result =
            ChonkerInt::from(String::from("100000"));
        assert_eq!(
            &positive_bigger_bigint - &zero_bigint,
            from_bigger_positive_empty_subtraction_result
        );
        let smaller_positive_from_empty_subtraction_result =
            ChonkerInt::from(String::from("-23423"));
        assert_eq!(
            &zero_bigint - &positive_smaller_bigint,
            smaller_positive_from_empty_subtraction_result
        );

        // Check subtraction of negative BigInts with an empty/zero BigInt. Try different positions.
        let from_bigger_negative_empty_subtraction_result =
            ChonkerInt::from(String::from("-12345"));
        assert_eq!(
            &negative_bigger_bigint - &zero_bigint,
            from_bigger_negative_empty_subtraction_result
        );
        let smaller_negative_from_empty_subtraction_result =
            ChonkerInt::from(String::from("1230000"));
        assert_eq!(
            &zero_bigint - &negative_smaller_bigint,
            smaller_negative_from_empty_subtraction_result
        );
    }

    // Test subtraction of two digits.
    #[test]
    fn test_digits_subtraction() {
        let one_vec1: Vec<i8> = vec![9];
        let one_vec2: Vec<i8> = vec![2];
        let mut one_offset1 = 0;
        let mut one_offset2 = 0;
        let other_vec1: Vec<i8> = vec![1];
        let other_vec2: Vec<i8> = vec![5];
        let mut other_offset1 = 0;
        let mut other_offset2 = 0;
        let mut result1 = ChonkerInt::new();
        let mut result2 = ChonkerInt::new();
        let mut last_digit_underflow1 = 0;
        let mut last_digit_underflow2 = 0;

        subtract_digits(
            &one_vec1,
            &other_vec1,
            &mut one_offset1,
            &mut other_offset1,
            &mut result1,
            &mut last_digit_underflow1,
        );
        subtract_digits(
            &one_vec2,
            &other_vec2,
            &mut one_offset2,
            &mut other_offset2,
            &mut result2,
            &mut last_digit_underflow2,
        );

        // First test of subtraction, 9-1
        assert_eq!((*result1.get_vec())[0], 8);
        assert_eq!(last_digit_underflow1, 0);
        assert_eq!(one_offset1, 1);
        assert_eq!(other_offset1, 1);

        // Second test of subtraction, 2-5
        assert_eq!((*result2.get_vec())[0], 7);
        assert_eq!(last_digit_underflow2, 1);
        assert_eq!(one_offset2, 1);
        assert_eq!(other_offset2, 1);
    }

    // Test subtraction of a digit with and underflow.
    #[test]
    fn test_digit_and_underflow_subtraction() {
        let one_vec1: Vec<i8> = vec![9];
        let one_vec2: Vec<i8> = vec![0];
        let mut one_offset1 = 0;
        let mut one_offset2 = 0;
        let mut result1 = ChonkerInt::new();
        let mut result2 = ChonkerInt::new();
        let mut last_digit_underflow1 = 0;
        let mut last_digit_underflow2 = 1;

        subtract_digit_and_underflow(
            &one_vec1,
            &mut one_offset1,
            &mut result1,
            &mut last_digit_underflow1,
        );
        subtract_digit_and_underflow(
            &one_vec2,
            &mut one_offset2,
            &mut result2,
            &mut last_digit_underflow2,
        );

        // First test of subtraction, 9 - 0
        assert_eq!((*result1.get_vec())[0], 9);
        assert_eq!(last_digit_underflow1, 0);
        assert_eq!(one_offset1, 1);

        // Second test of subtraction, 0 - 1
        assert_eq!((*result2.get_vec())[0], 9);
        assert_eq!(last_digit_underflow2, 1);
        assert_eq!(one_offset2, 1);
    }
}
