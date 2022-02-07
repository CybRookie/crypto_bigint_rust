// BigInt module regarding modulus division of BigInts.
// Paper on Bernikel Zielger’s recursive division algorithm https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.565&rep=rep1&type=pdf

use std::ops::Rem;

use crate::logic::bigint::division::quotient_estimation_algorithm;
use crate::logic::bigint::{BigIntSign, ChonkerInt};

// Implement modulus "%" operator for the BigInt. The sign of the result follows the divisor.
// The implementation is a copy of the division implementation, except it returns the remainder instead of quotient,
// which is not even stored.
impl<'a, 'b> Rem<&'b ChonkerInt> for &'a ChonkerInt {
    type Output = ChonkerInt;

    fn rem(self, rhs: &'b ChonkerInt) -> Self::Output {
        // Check for division by zero, if the divisor is zero, panic.
        if *rhs == ChonkerInt::new() || rhs.digits.is_empty() {
            panic!("attempt to divide/take modulus by zero (ChonkerInt::rem())");
        }

        // Check if the zero is divided, if the dividend is zero, return zero.
        if *self == ChonkerInt::new() || self.digits.is_empty() {
            return ChonkerInt::new();
        }

        // Clone dividend and divisor, make them absolute for comparisons.
        let mut absolute_dividend = (*self).clone();
        absolute_dividend.set_positive_sign();
        let mut absolute_divisor = (*rhs).clone();
        absolute_divisor.set_positive_sign();

        // Compare the lengths/values of the dividend and divisor.
        // If self/dividend is smaller that the divisor, either the dividend or the sum of  the divisor with the dividend.
        if absolute_dividend < absolute_divisor {
            if (self.sign == BigIntSign::Negative && rhs.sign == BigIntSign::Positive)
                || (self.sign == BigIntSign::Positive && rhs.sign == BigIntSign::Negative)
            {
                return rhs + self;
            } else if self.sign == BigIntSign::Negative && rhs.sign == BigIntSign::Negative {
                return (*self).clone();
            }
            // Both dividend and divisor are positive.
            return (*self).clone();
        }

        let mut remainder;
        let mut cut_dividend = ChonkerInt::new();
        cut_dividend.set_positive_sign();

        // Compare lengths of the dividend and divisor and normalization of the most significant digits.
        // Length of the dividend should not be more than length of the divisor + 1.
        // If it is, cut the dividend.
        if self.digits.len() > rhs.digits.len()
        {
            // Cut the dividend to the smaller size of divisor's length with or without additional digit,
            // calculate the remainder that will be used as the dividend,
            // and then repeat the process by adding digits from the original dividend to the remainders one by one in the loop.

            let mut dividend_index = self.digits.len();
            // let mut difference = rhs.digits.len() + 1;
            let mut difference;

            // Check normalization of the divisor's most significant digit and compare it to the dividend's digit,
            // if divisor's digit is not normalized, and it is smaller than the dividend's one,
            // make the difference/required length for the cut dividend equal to the divisor's length;
            // otherwise add 1 to it.
            if (self.digits[self.digits.len() - 1] > rhs.digits[rhs.digits.len() - 1])
                || (absolute_dividend > absolute_divisor)
            {
                difference = rhs.digits.len();
            } else {
                difference = rhs.digits.len() + 1;
            }

            let cut_dividend_splice =
                &self.digits[(dividend_index - difference)..=(dividend_index - 1)];

            for digit in cut_dividend_splice.iter().rev() {
                cut_dividend.digits.insert(0, *digit);
            }

            let (_quotient_digit, remainder_digit) =
                quotient_estimation_algorithm(&cut_dividend, rhs);

            cut_dividend = remainder_digit;

            dividend_index -= difference;

            difference = 1;

            // Loop over the dividend's digits one by one and add them to the remainder from the previous iteration.
            while dividend_index > 0 {
                let cut_dividend_splice =
                    &self.digits[(dividend_index - difference)..=(dividend_index - 1)];

                // Add remaining digits from the dividend to the remainder from previous division operation.
                // Preserve little endian from the dividend with reverse of the iterator over the dividend.
                for digit in cut_dividend_splice.iter().rev() {
                    cut_dividend.digits.insert(0, *digit);
                }
                cut_dividend.set_positive_sign();
                cut_dividend.normalize();

                // Check if the dividend became shorter/smaller than the divisor, if so,
                // skip the iteration.
                if cut_dividend < absolute_divisor {
                    dividend_index -= difference;
                    continue;
                }

                let (_quotient_digit, remainder_digit) =
                    quotient_estimation_algorithm(&cut_dividend, rhs);

                // Use the remainder digit(-s) for the new partial dividend.
                // Reverse digits from little endian to big endian.
                cut_dividend = remainder_digit.clone();

                dividend_index -= difference;
            }

            // Clone the remainder.
            remainder = cut_dividend.clone();
        } else {
            // If lengths of dividend and divisor are equal.
            let (_quotient_digit, remainder_digit) = quotient_estimation_algorithm(self, rhs);
            // Clone the remainder.
            remainder = remainder_digit;
        }

        // This is an implementaion of the modulo operation, not the remainder,
        // thus the final sign follows the sign of the divisor.
        // Check for the sign of the dividend,
        // if it is negative and the remainder is not a zero,
        // make the remainder negative and add 1 divisor to it.
        if remainder != ChonkerInt::new() {
            if self.sign == BigIntSign::Negative && rhs.sign == BigIntSign::Positive {
                remainder.set_negative_sign();
                remainder = &remainder + rhs;
            } else if self.sign == BigIntSign::Positive && rhs.sign == BigIntSign::Negative {
                remainder = &remainder + rhs;
            } else if self.sign == BigIntSign::Negative && rhs.sign == BigIntSign::Negative {
                remainder.set_negative_sign();
            }
        }

        // Cut the leading zeros.
        remainder.normalize();
        remainder
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::bigint::ChonkerInt;

    // Test modulus division of two BigInts.
    #[test]
    fn test_bigint_modulus_division() {
        // Positive BigInts.
        let positive_bigger_bigint = ChonkerInt::from(String::from("100000"));
        let positive_smaller_bigint = ChonkerInt::from(String::from("23423"));

        // Negative BigInts.
        let negative_bigger_bigint = ChonkerInt::from(String::from("-12345"));
        let negative_smaller_bigint = ChonkerInt::from(String::from("-1230000"));

        // Very big BigInts.
        let positive_bigger_very_big_bigint = ChonkerInt::from(String::from("4379853178597859156740573149857154310578942357435678165781568134756871356187956143975358713583915634785431658143560178536107563147805634807561348506134"));
        let positive_smaller_very_big_bigint = ChonkerInt::from(String::from(
            "7142756019471983982475239851587182390573438756286598175918",
        ));
        let negative_bigger_very_big_bigint = ChonkerInt::from(String::from(
            "-3714856173245610358671095834519578134957135871390587314982",
        ));
        let negative_smaller_very_big_bigint = ChonkerInt::from(String::from("-7846518746531895729834723194263984236421304673218561384612384623198412894123506123859123452319048712958714309584104712340823408213842130948"));

        // Empty/zero BigInt.
        let zero_bigint = ChonkerInt::from(String::from("0"));

        // Check modulus division of positive BigInts. Try different positions.
        let positive_bigger_by_smaller_modulus_division_result =
            ChonkerInt::from(String::from("6308"));
        assert_eq!(
            &positive_bigger_bigint % &positive_smaller_bigint,
            positive_bigger_by_smaller_modulus_division_result
        );
        let positive_smaller_by_bigger_modulus_division_result =
            ChonkerInt::from(String::from("23423"));
        assert_eq!(
            &positive_smaller_bigint % &positive_bigger_bigint,
            positive_smaller_by_bigger_modulus_division_result
        );

        // Check modulus division of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
        let bigger_positive_by_smaller_negative_modulus_division_result =
            ChonkerInt::from(String::from("-1130000"));
        assert_eq!(
            &positive_bigger_bigint % &negative_smaller_bigint,
            bigger_positive_by_smaller_negative_modulus_division_result
        );
        let smaller_negative_by_bigger_positive_modulus_division_result =
            ChonkerInt::from(String::from("70000"));
        assert_eq!(
            &negative_smaller_bigint % &positive_bigger_bigint,
            smaller_negative_by_bigger_positive_modulus_division_result
        );

        // Check modulus division of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
        let smaller_positive_by_bigger_negative_modulus_division_result =
            ChonkerInt::from(String::from("-1267"));
        assert_eq!(
            &positive_smaller_bigint % &negative_bigger_bigint,
            smaller_positive_by_bigger_negative_modulus_division_result
        );
        let bigger_negative_by_smaller_positive_modulus_division_result =
            ChonkerInt::from(String::from("11078"));
        assert_eq!(
            &negative_bigger_bigint % &positive_smaller_bigint,
            bigger_negative_by_smaller_positive_modulus_division_result
        );

        // Check modulus division of negative BigInts. Try different positions.
        let negative_bigger_by_smaller_modulus_division_result =
            ChonkerInt::from(String::from("-12345"));
        assert_eq!(
            &negative_bigger_bigint % &negative_smaller_bigint,
            negative_bigger_by_smaller_modulus_division_result
        );
        let negative_smaller_by_bigger_modulus_division_result =
            ChonkerInt::from(String::from("-7845"));
        assert_eq!(
            &negative_smaller_bigint % &negative_bigger_bigint,
            negative_smaller_by_bigger_modulus_division_result
        );

        // Check modulus division of positive BigInts with an empty/zero BigInt. Try different positions.
        // let positive_bigger_by_empty_modulus_division_result = ChonkerInt::from(String::from("0"));
        // assert_eq!(&positive_bigger_bigint % &zero_bigint, positive_bigger_by_empty_division_result);

        let empty_by_positive_smaller_modulus_division_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            &zero_bigint % &positive_smaller_bigint,
            empty_by_positive_smaller_modulus_division_result
        );

        // Check modulus division of negative BigInts with an empty/zero BigInt. Try different positions.
        // let negative_bigger_by_empty_modulus_division_result = ChonkerInt::from(String::from("0"));
        // assert_eq!(&negative_bigger_bigint % &zero_bigint, negative_bigger_by_empty_division_result);

        let empty_by_negative_smaller_modulus_division_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            &zero_bigint % &negative_smaller_bigint,
            empty_by_negative_smaller_modulus_division_result
        );

        // Check modulus division of very BigInts.
        let positive_very_big_bigger_by_negative_very_big_smaller_result = ChonkerInt::from(String::from("-4160032753209023619198251321622880242821224555894280046814442463878143415701119919859842686220966394065360364693594909227653167239533471886"));
        assert_eq!(
            &positive_bigger_very_big_bigint % &negative_smaller_very_big_bigint,
            positive_very_big_bigger_by_negative_very_big_smaller_result
        );
        let positive_very_big_smaller_by_negative_very_big_smaller_result = ChonkerInt::from(String::from("-7846518746531895729834723194263984236421304673218561384612384623198412894123506116716367432847064730483474457996922321767384651927243955030"));
        assert_eq!(
            &positive_smaller_very_big_bigint % &negative_smaller_very_big_bigint,
            positive_very_big_smaller_by_negative_very_big_smaller_result
        );
        let positive_very_big_bigger_by_negative_very_big_bigger_result = ChonkerInt::from(
            String::from("-2040600818200247843220407180540756771951755806741487576794"),
        );
        assert_eq!(
            &positive_bigger_very_big_bigint % &negative_bigger_very_big_bigint,
            positive_very_big_bigger_by_negative_very_big_bigger_result
        );
        let positive_very_big_smaller_by_negative_very_big_bigger_result = ChonkerInt::from(
            String::from("-286956327019236734866951817451973879340832986494576454046"),
        );
        assert_eq!(
            &positive_smaller_very_big_bigint % &negative_bigger_very_big_bigint,
            positive_very_big_smaller_by_negative_very_big_bigger_result
        );

        let negative_very_big_bigger_by_positive_very_big_smaller_result = ChonkerInt::from(
            String::from("3427899846226373623804144017067604255616302884896010860936"),
        );
        assert_eq!(
            &negative_bigger_very_big_bigint % &positive_smaller_very_big_bigint,
            negative_very_big_bigger_by_positive_very_big_smaller_result
        );
        let negative_very_big_smaller_by_positive_very_big_smaller_result = ChonkerInt::from(
            String::from("4316350245186309090610264867148023910380230603937925784652"),
        );
        assert_eq!(
            &negative_smaller_very_big_bigint % &positive_smaller_very_big_bigint,
            negative_very_big_smaller_by_positive_very_big_smaller_result
        );
        let negative_very_big_bigger_by_positive_very_big_bigger_result = ChonkerInt::from(String::from("4379853178597859156740573149857154310578942357435678165781568134756871356187956143975358713580200778612186047784889082701587985012848498936170761191152"));
        assert_eq!(
            &negative_bigger_very_big_bigint % &positive_bigger_very_big_bigint,
            negative_very_big_bigger_by_positive_very_big_bigger_result
        );
        let negative_very_big_smaller_by_positive_very_big_bigger_result = ChonkerInt::from(String::from("4379853178597859156740573149857154310578942357435678165781568134756871356187956143975358713580200778612186047784889082701587985012848498936170761191152"));
        assert_eq!(
            &negative_bigger_very_big_bigint % &positive_bigger_very_big_bigint,
            negative_very_big_smaller_by_positive_very_big_bigger_result
        );

        // Check modulus division of very big positive BigInts.
        let positive_very_big_bigger_by_positive_very_big_bigger_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &positive_bigger_very_big_bigint % &positive_bigger_very_big_bigint,
            positive_very_big_bigger_by_positive_very_big_bigger_result
        );
        let positive_very_big_bigger_by_positive_very_big_smaller_result = ChonkerInt::from(
            String::from("5918573268533236639722045796608111873199720905564901626246"),
        );
        assert_eq!(
            &positive_bigger_very_big_bigint % &positive_smaller_very_big_bigint,
            positive_very_big_bigger_by_positive_very_big_smaller_result
        );
        let positive_very_big_smaller_by_positive_very_big_smaller_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &positive_smaller_very_big_bigint % &positive_smaller_very_big_bigint,
            positive_very_big_smaller_by_positive_very_big_smaller_result
        );
        let positive_very_big_smaller_by_positive_very_big_bigger_result = ChonkerInt::from(
            String::from("7142756019471983982475239851587182390573438756286598175918"),
        );
        assert_eq!(
            &positive_smaller_very_big_bigint % &positive_bigger_very_big_bigint,
            positive_very_big_smaller_by_positive_very_big_bigger_result
        );

        // Check modulus division of very big negative BigInts.
        let negative_very_big_bigger_by_negative_very_big_bigger_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_bigger_very_big_bigint % &negative_bigger_very_big_bigint,
            negative_very_big_bigger_by_negative_very_big_bigger_result
        );
        let negative_very_big_bigger_by_negative_very_big_smaller_result = ChonkerInt::from(
            String::from("-3714856173245610358671095834519578134957135871390587314982"),
        );
        assert_eq!(
            &negative_bigger_very_big_bigint % &negative_smaller_very_big_bigint,
            negative_very_big_bigger_by_negative_very_big_smaller_result
        );
        let negative_very_big_smaller_by_negative_very_big_smaller_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_smaller_very_big_bigint % &negative_smaller_very_big_bigint,
            negative_very_big_smaller_by_negative_very_big_smaller_result
        );
        let negative_very_big_smaller_by_negative_very_big_bigger_result = ChonkerInt::from(
            String::from("-1368537949400640214032806262892480125625296555619084556240"),
        );
        assert_eq!(
            &negative_smaller_very_big_bigint % &negative_bigger_very_big_bigint,
            negative_very_big_smaller_by_negative_very_big_bigger_result
        );
    }
}
