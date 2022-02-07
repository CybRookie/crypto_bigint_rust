// BigInt module regarding multiplication of BigInts.

use std::ops::Mul;

use crate::logic::bigint::{clip, overflow, BigIntSign, ChonkerInt};

// Implement multiplication "*" operator for the BigInt.
// Multiplication is done with school style long multiplication.
// Could be sped up with an implementation of a faster algorithm, e.g. with Karatsuba Karatsuba algorithm.
// Considering that division and modulus operation depend on multiplication,
// and other operations depend on them, faster multiplicaiton could speed overall perfomance of the library.
impl<'a, 'b> Mul<&'b ChonkerInt> for &'a ChonkerInt {
    // impl Mul for ChonkerInt {
    type Output = ChonkerInt;
    // type Output = Self;

    fn mul(self, rhs: &'b ChonkerInt) -> Self::Output {
        // fn mul(self, rhs: Self) -> Self::Output {

        let positive_one = ChonkerInt::from(1);
        let negative_one = ChonkerInt::from(-1);

        // If self or rhs is empty/zero, return rhs empty/zero BigInt.
        // If self or rhs is 1 or -1, return opposite value with, possibly, an opposite sign.
        if self.sign == BigIntSign::Zero || rhs.sign == BigIntSign::Zero {
            return ChonkerInt::new();
        } else if *self == positive_one {
            // If self multiplier is a positive 1, return rhs multiplier clone.
            return (*rhs).clone();
        } else if *self == negative_one {
            // If self multiplier is a negative 1, return a negated rhs multiplier clone.
            let mut negated_rhs = (*rhs).clone();
            match negated_rhs.sign {
                BigIntSign::Positive => negated_rhs.sign = BigIntSign::Negative,
                BigIntSign::Negative => negated_rhs.sign = BigIntSign::Positive,
                BigIntSign::Zero => {
                    panic!("error in the bigint multiplication, skipped zero check")
                }
            }
            return negated_rhs;
        } else if *rhs == positive_one {
            // If rhs multiplier is a positive 1, return self multiplier clone.
            return (*self).clone();
        } else if *rhs == negative_one {
            // If rhs multiplier is a negative 1, return a negated self multiplier clone.
            let mut negated_self = (*self).clone();
            match negated_self.sign {
                BigIntSign::Positive => negated_self.sign = BigIntSign::Negative,
                BigIntSign::Negative => negated_self.sign = BigIntSign::Positive,
                BigIntSign::Zero => {
                    panic!("error in the bigint multiplication, skipped zero check")
                }
            }
            return negated_self;
        }

        let mut result = ChonkerInt::new();

        let mut last_digit_overflow = 0;
        let mut self_offset = 0;
        let self_length = self.digits.len();
        let mut rhs_offset = 0;
        let rhs_length = rhs.digits.len();
        let mut partial_product_bigint: ChonkerInt;
        let mut partial_product_digit: i8;

        // Calculate intermediate/partial products and add them together to get the final product.
        while rhs_offset < rhs_length {
            // Calculate a partial product. Iterate one digit of the rhs BigInt over all digits of the self BigInt
            // and shift the result with zeros.

            // Construct a temporary BigInt for the partial product.
            partial_product_bigint = ChonkerInt::new();
            partial_product_bigint.set_positive_sign();

            // println!("Constructed partial product bigint: {:?}", partial_product_bigint);

            // Shift partial product by a number of zeros, which aligns with the iteration number over rhs BigInt.
            for _x in 0..rhs_offset {
                let _ = partial_product_bigint.push(0);
            }

            // println!("Partial product bigint after shifting: {:?}", partial_product_bigint);

            while self_offset < self_length {
                partial_product_digit =
                    self.digits[self_offset] * rhs.digits[rhs_offset] + last_digit_overflow;

                // Check a partial product for overflow.
                last_digit_overflow = overflow(partial_product_digit);
                partial_product_digit = clip(partial_product_digit);

                let _ = partial_product_bigint.push(partial_product_digit);

                self_offset += 1;
            }

            // Reset index for the self BigInt, to iterate over it again during next iteration of the outer loop.
            self_offset = 0;

            // println!("Partial product bigint after product calculation: {:?}", partial_product_bigint);

            // If there is an overflow add as a last digit.
            // Considering that a number length after multiplication equals the sum of lengths of both operands,
            // and the target is multiplied only by one digit at a time, the result will be longer only by 1 digit.
            // Nullify overflow after its addition.
            if last_digit_overflow > 0 {
                let _ = partial_product_bigint.push(last_digit_overflow);
                last_digit_overflow = 0;
            }

            // println!("Partial product bigint after final overflow addition: {:?}", partial_product_bigint);

            // Add a partial product to the total final resulting product.
            result = &result + &partial_product_bigint;

            // println!("Product after addition of the partial product: {:?}\n\n", result);

            rhs_offset += 1;
        }

        // Check the signs of both operands, if they are not the same, the resulting sign is negative.
        // By default the sign is positive.
        // If the signs are the same and positive, the resulting sign is positive.
        // If the signs are the same and negative, the resulting sign is positive.
        if self.sign != rhs.sign {
            result.set_negative_sign();
        }

        result
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::ChonkerInt;

    // Test multiplication of two BigInts.
    #[test]
    fn test_bigint_multiplication() {
        // Positive BigInts.
        let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
        let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));

        // Negative BigInts.
        let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
        let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));

        // Empty/zero BigInt.
        let zero_bigint = ChonkerInt::from(String::from("0"));

        // Check multiplication of positive BigInts. Try different positions.
        let positive_multiplication_result = ChonkerInt::from(String::from("2342300000"));
        assert_eq!(
            &positive_bigger_bigint * &positive_smaller_bigint,
            positive_multiplication_result
        );
        assert_eq!(
            &positive_smaller_bigint * &positive_bigger_bigint,
            positive_multiplication_result
        );

        // Check multiplication of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
        let bigger_positive_smaller_negative_multiplication_result =
            ChonkerInt::from(String::from("-123000000000"));
        assert_eq!(
            &positive_bigger_bigint * &negative_smaller_bigint,
            bigger_positive_smaller_negative_multiplication_result
        );
        assert_eq!(
            &negative_smaller_bigint * &positive_bigger_bigint,
            bigger_positive_smaller_negative_multiplication_result
        );

        // Check multiplication of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
        let smaller_positive_bigger_negative_multiplication_result =
            ChonkerInt::from(String::from("-289156935"));
        assert_eq!(
            &positive_smaller_bigint * &negative_bigger_bigint,
            smaller_positive_bigger_negative_multiplication_result
        );
        assert_eq!(
            &negative_bigger_bigint * &positive_smaller_bigint,
            smaller_positive_bigger_negative_multiplication_result
        );

        // Check multiplication of negative BigInts. Try different positions.
        let negative_multiplication_result = ChonkerInt::from(String::from("15184350000"));
        assert_eq!(
            &negative_bigger_bigint * &negative_smaller_bigint,
            negative_multiplication_result
        );
        assert_eq!(
            &negative_smaller_bigint * &negative_bigger_bigint,
            negative_multiplication_result
        );

        // Check multiplication of positive BigInts with an empty/zero BigInt. Try different positions.
        let bigger_and_smaller_positive_empty_multiplication_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &positive_bigger_bigint * &zero_bigint,
            bigger_and_smaller_positive_empty_multiplication_result
        );
        assert_eq!(
            &zero_bigint * &positive_smaller_bigint,
            bigger_and_smaller_positive_empty_multiplication_result
        );

        // Check multiplication of negative BigInts with an empty/zero BigInt. Try different positions.
        let bigger_and_smaller_negative_empty_multiplication_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_bigger_bigint * &zero_bigint,
            bigger_and_smaller_negative_empty_multiplication_result
        );
        assert_eq!(
            &zero_bigint * &negative_smaller_bigint,
            bigger_and_smaller_negative_empty_multiplication_result
        );
    }
}
