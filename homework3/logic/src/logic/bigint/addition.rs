// BigInt module regarding addition of BigInts.

use std::ops::{Add, Neg};

use crate::logic::bigint::{clip, overflow, BigIntSign, ChonkerInt};

// Implement addition "+" operator for the BigInt.
// Addition is done with school style long addition.
impl<'a, 'b> Add<&'b ChonkerInt> for &'a ChonkerInt {
    type Output = ChonkerInt;

    fn add(self, other: &'b ChonkerInt) -> Self::Output {
        // If self is empty/zero, return other BigInt.
        if self.sign == BigIntSign::Zero {
            return (*other).clone();
        }

        // If other is empty/zero, return self.
        if other.sign == BigIntSign::Zero {
            return (*self).clone();
        }

        // Check the signs of both operands, if they are not the same, determine an appropriate operation.
        // If the signs are the same and negative, negate them and recall addition operation,
        // then negate the retrieved result back to negative.
        if self.sign != other.sign {
            return if self.sign == BigIntSign::Positive {
                self - &(-other) // +self - (-other) = self - other
            } else {
                other - &(-self) // other - (-self) = other - self
            };
        } else if self.sign == BigIntSign::Negative {
            let sum_of_negated_bigints = &(-self) + &(-other);
            let sum_ref: &ChonkerInt = &sum_of_negated_bigints;
            let result: ChonkerInt = sum_ref.neg();
            return result; // (-self) + (-other) = -(self + other)
        }

        let mut result = ChonkerInt::new();

        let mut last_digit_overflow = 0;
        let mut self_offset = 0;
        let self_length = self.digits.len();
        let mut other_offset = 0;
        let other_length = other.digits.len();

        // Determine the longer/bigger BigInt and calculate addition.
        // If vector were of different lengths, finish operation on the digits of the longer vector/BigInt with a second loop.
        if *self >= *other {
            while other_offset < other_length {
                add_digits(
                    &self.digits,
                    &other.digits,
                    &mut self_offset,
                    &mut other_offset,
                    &mut result,
                    &mut last_digit_overflow,
                );
            }
            while self_offset < self_length {
                add_digit_and_overflow(
                    &self.digits,
                    &mut self_offset,
                    &mut result,
                    &mut last_digit_overflow,
                );
            }
        } else {
            while self_offset < self_length {
                add_digits(
                    &self.digits,
                    &other.digits,
                    &mut self_offset,
                    &mut other_offset,
                    &mut result,
                    &mut last_digit_overflow,
                );
            }
            while other_offset < other_length {
                add_digit_and_overflow(
                    &other.digits,
                    &mut other_offset,
                    &mut result,
                    &mut last_digit_overflow,
                );
            }
        }

        // Check for a possible remaining overflow.
        if last_digit_overflow > 0 {
            let _ = result.push(last_digit_overflow);
        }

        // Set a positive sing of the resulting BigInt.
        result.set_positive_sign();

        result
    }
}

// Addition of two passed digits.
fn add_digits(
    one_vec: &[i8],
    other_vec: &[i8],
    one_offset: &mut usize,
    other_offset: &mut usize,
    result: &mut ChonkerInt,
    last_digit_overflow: &mut i8,
) {
    // Calculate sum of digits.
    let mut sum = (*one_vec)[*one_offset] + (*other_vec)[*other_offset] + (*last_digit_overflow);

    // Check for the overflow.
    *last_digit_overflow = overflow(sum);
    sum = clip(sum);

    let _ = result.push(sum);
    *one_offset += 1;
    *other_offset += 1;
}

// Addition of one passed digit and a result slot.
fn add_digit_and_overflow(
    one_vec: &[i8],
    one_offset: &mut usize,
    result: &mut ChonkerInt,
    last_digit_overflow: &mut i8,
) {
    // Calculate sum of digits.
    let mut sum = (*one_vec)[*one_offset] + (*last_digit_overflow);

    // Check for the overflow.
    *last_digit_overflow = overflow(sum);
    sum = clip(sum);

    let _ = result.push(sum);
    *one_offset += 1;
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::addition::{add_digit_and_overflow, add_digits};
    use crate::logic::bigint::ChonkerInt;

    // Test of BigInt's addition operation.
    #[test]
    fn test_bigint_addition() {
        // Positive BigInts.
        let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
        let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));

        // Negative BigInts.
        let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
        let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));

        // Empty/zero BigInt.
        let zero_bigint = ChonkerInt::from(String::from("0"));

        // Check addition of positive BigInts. Try different positions.
        let positive_addition_result = ChonkerInt::from(String::from("123423"));
        assert_eq!(
            &positive_bigger_bigint + &positive_smaller_bigint,
            positive_addition_result
        );
        assert_eq!(
            &positive_smaller_bigint + &positive_bigger_bigint,
            positive_addition_result
        );

        // Check addition of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
        let bigger_positive_smaller_negative_addition_result =
            ChonkerInt::from(String::from("-1130000"));
        assert_eq!(
            &positive_bigger_bigint + &negative_smaller_bigint,
            bigger_positive_smaller_negative_addition_result
        );
        assert_eq!(
            &negative_smaller_bigint + &positive_bigger_bigint,
            bigger_positive_smaller_negative_addition_result
        );

        // Check addition of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
        let smaller_positive_bigger_negative_addition_result =
            ChonkerInt::from(String::from("11078"));
        assert_eq!(
            &positive_smaller_bigint + &negative_bigger_bigint,
            smaller_positive_bigger_negative_addition_result
        );
        assert_eq!(
            &negative_bigger_bigint + &positive_smaller_bigint,
            smaller_positive_bigger_negative_addition_result
        );

        // Check addition of negative BigInts. Try different positions.
        let negative_addition_result = ChonkerInt::from(String::from("-1242345"));
        assert_eq!(
            &negative_bigger_bigint + &negative_smaller_bigint,
            negative_addition_result
        );
        assert_eq!(
            &negative_smaller_bigint + &negative_bigger_bigint,
            negative_addition_result
        );

        // Check addition of positive BigInts with an empty/zero BigInt. Try different positions.
        let bigger_positive_empty_addition_result = ChonkerInt::from(String::from("100000"));
        assert_eq!(
            &positive_bigger_bigint + &zero_bigint,
            bigger_positive_empty_addition_result
        );
        let smaller_positive_empty_addition_result = ChonkerInt::from(String::from("23423"));
        assert_eq!(
            &zero_bigint + &positive_smaller_bigint,
            smaller_positive_empty_addition_result
        );

        // Check addition of negative BigInts with an empty/zero BigInt. Try different positions.
        let bigger_negative_empty_addition_result = ChonkerInt::from(String::from("-12345"));
        assert_eq!(
            &negative_bigger_bigint + &zero_bigint,
            bigger_negative_empty_addition_result
        );
        let smaller_negative_empty_addition_result = ChonkerInt::from(String::from("-1230000"));
        assert_eq!(
            &zero_bigint + &negative_smaller_bigint,
            smaller_negative_empty_addition_result
        );
    }

    // Test addition of two digits.
    #[test]
    fn test_digits_addition() {
        let one_vec1: Vec<i8> = vec![1];
        let one_vec2: Vec<i8> = vec![2];
        let mut one_offset1 = 0;
        let mut one_offset2 = 0;
        let other_vec1: Vec<i8> = vec![9];
        let other_vec2: Vec<i8> = vec![5];
        let mut other_offset1 = 0;
        let mut other_offset2 = 0;
        let mut result1 = ChonkerInt::new();
        let mut result2 = ChonkerInt::new();
        let mut last_digit_overflow1 = 0;
        let mut last_digit_overflow2 = 0;

        add_digits(
            &one_vec1,
            &other_vec1,
            &mut one_offset1,
            &mut other_offset1,
            &mut result1,
            &mut last_digit_overflow1,
        );
        add_digits(
            &one_vec2,
            &other_vec2,
            &mut one_offset2,
            &mut other_offset2,
            &mut result2,
            &mut last_digit_overflow2,
        );

        // First test of addition, 1+9
        assert_eq!((*result1.get_vec())[0], 0);
        assert_eq!(last_digit_overflow1, 1);
        assert_eq!(one_offset1, 1);
        assert_eq!(other_offset1, 1);

        // Second test of addition, 0+5
        assert_eq!((*result2.get_vec())[0], 7);
        assert_eq!(last_digit_overflow2, 0);
        assert_eq!(one_offset2, 1);
        assert_eq!(other_offset2, 1);
    }

    // Test addition of a digit with an overflow.
    #[test]
    fn test_digit_and_overflow_addition() {
        let one_vec1: Vec<i8> = vec![9];
        let one_vec2: Vec<i8> = vec![0];
        let mut one_offset1 = 0;
        let mut one_offset2 = 0;
        let mut result1 = ChonkerInt::new();
        let mut result2 = ChonkerInt::new();
        let mut last_digit_overflow1 = 1;
        let mut last_digit_overflow2 = 0;

        add_digit_and_overflow(
            &one_vec1,
            &mut one_offset1,
            &mut result1,
            &mut last_digit_overflow1,
        );
        add_digit_and_overflow(
            &one_vec2,
            &mut one_offset2,
            &mut result2,
            &mut last_digit_overflow2,
        );

        // First test of addition, 9 + 1
        assert_eq!((*result1.get_vec())[0], 0);
        assert_eq!(last_digit_overflow1, 1);
        assert_eq!(one_offset1, 1);

        // Second test of addition, 0 + 0
        assert_eq!((*result2.get_vec())[0], 0);
        assert_eq!(last_digit_overflow2, 0);
        assert_eq!(one_offset2, 1);
    }
}
