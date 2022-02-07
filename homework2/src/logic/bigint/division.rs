// BigInt module regarding division of BigInts.
// Paper on Bernikel Zielger’s recursive division algorithm https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.565&rep=rep1&type=pdf

use std::ops::Div;

use crate::logic::bigint::{BigIntSign, ChonkerInt, RADIX};

// Implement division "/" operator for the BigInt
// Division and remainder calculation were achieved with Quotient Estimation Algorithm,
// based on on Lemma 2 from Bernikel Zielger’s recursive division algorithm paper.
// Algorithm says that if A (the dividend) is at most l digits longer than B (the divisor) and B is normalized
// (more than half the value if all digits were max. value, e.g. 444 is not normalized because it is less than 999/2).
// A is also less than RADIX*B.
// Quotient has to fit into one digit under the RADIX. In decimal system, allowed values would be in the range of 0-9.
// Then you can estimate the quotient to be the division of A be B,
// where we exclude the lowest k digits of both numbers (where k is B’s length minus l).
// This estimate is at most two more than the actual quotient.

// Example: In the decimal system: 378,546÷78,356.
//
//     378,546 is (atmost) 1 digit longer than 78,356 (l = 1).
//     Hence, k = 5 – 1=4, where 5 = length(78,356) and 1= l.
//     78,356 is normalized — it’s greater than half of 99999.
//     Clearly, 378,546 is less than 10 times 78,356 (and 10 is our radix); hence, this is a valid use case for us.
//
// Hence, our quotient estimate will be division of 37/7 (excluding lower k digits) which is 5. Now,
// 5 is at most 2 more than the actual quotient — hence, q can be 3, 4, or 5.
//
//     78,356*5 = 391,780 (wrong)
//     78,356*4 = 313,424 (right, where remainder r is 65122)
//     don’t check for 3 now — we found q already.
//
// Normalization of B is ensured with fractional equivalency.
//
// We need to multiply A and B by a number such that B becomes normalized, if not already.
// That number is equal Math.floor(radix / (B._digits[0] + 1)) in code.
// This ensures that the first digit of B is as high as it can get without increasing the number of digits.

impl<'a, 'b> Div<&'b ChonkerInt> for &'a ChonkerInt {
    type Output = ChonkerInt;

    fn div(self, rhs: &'b ChonkerInt) -> Self::Output {
        // Check for division by zero, if the divisor is zero, panic.
        if *rhs == ChonkerInt::new() || rhs.digits.is_empty() {
            panic!("cannot divide by zero (ChonkerInt::div())");
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
        // If self/dividend is smaller that the divisor, return empty/zero BigInt as a quotient/result.
        // If self/dividend is equal to the divisor, return 1 or -1 as a BigInt.
        // If self/dividend is bigger that the divisor, proceed with calculations.
        if absolute_dividend < absolute_divisor {
            return ChonkerInt::new();
        } else if *self == *rhs && *self == absolute_divisor {
            // If self/dividend and rhs/divisor are positive.
            return ChonkerInt::from(1);
        } else if *self != *rhs && *self == absolute_divisor {
            // If self/dividend is positive, and rhs/divisor is negative.
            return ChonkerInt::from(-1);
        } else if *self == *rhs && *self != absolute_divisor {
            // If self/dividend and rhs/divisor are negative.
            return ChonkerInt::from(1);
        } else if *self != *rhs && absolute_dividend == *rhs {
            // If self/dividend is negative, and rhs/divisor is positive.
            return ChonkerInt::from(-1);
        }

        let mut quotient = ChonkerInt::new();

        let mut cut_dividend = ChonkerInt::new();
        cut_dividend.set_positive_sign();

        // Compare lengths of the dividend and divisor and normalization of the most significant digits.
        // Length of the dividend should not be more than length of the divisor + 1.
        // If it is, cut the dividend.
        if self.digits.len() > rhs.digits.len()
        {
            // Cut the dividend to the smaller size of divisor's length with or without additional digit,
            // calculate the quotient digit and the remainder that will be used as the dividend,
            // and then repeat the process by adding digits from the original dividend to the remainders one by one in the loop.

            let mut dividend_index = self.digits.len();
            // let mut difference = rhs.digits.len() + 1;
            let mut difference;

            // Check normalization of the divisor's most significant digit and compare it to the divident's digit,
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

            let (mut quotient_digit, remainder_digit) =
                quotient_estimation_algorithm(&cut_dividend, rhs);

            if quotient_digit.digits.len() > 1 {
                quotient_digit.digits.reverse();
            }

            quotient.push_vec(&quotient_digit.digits);

            cut_dividend = remainder_digit;

            dividend_index -= difference;

            difference = 1;

            // Loop over the dividend's digits one by one and add them to the remainder from the previous iteration.
            while dividend_index > 0 {
                let cut_dividend_splice =
                    &self.digits[(dividend_index - difference)..=(dividend_index - 1)];

                // Add remaining digits form the dividend to the remainder from previous division operation.
                // Preserve little endian from the dividend with reverse of the iterator over the dividend.
                for digit in cut_dividend_splice.iter().rev() {
                    cut_dividend.digits.insert(0, *digit);
                }
                cut_dividend.set_positive_sign();
                cut_dividend.normalize();

                // Check if the dividend became shorter/smaller than the divisor, if so,
                // add a quotient digit of zero and proceed to the next iteration.
                if cut_dividend < absolute_divisor {
                    let _ = quotient.push(0);
                    dividend_index -= difference;
                    continue;
                }

                let (mut quotient_digit, remainder_digit) =
                    quotient_estimation_algorithm(&cut_dividend, rhs);

                // If quotient has several digit, then it stored in the little endian by default.
                // But separate quotient digits are stored in the big endian format.
                // As the result, if the partial quotient has several digits by itself,
                // it has to be reversed.
                if quotient_digit.digits.len() > 1 {
                    quotient_digit.digits.reverse();
                }

                // Save the quotient digit.
                quotient.push_vec(&quotient_digit.digits);

                // Use the remainder digit(-s) for the new partial dividend.
                // Reverse digits from little endian to big endian.
                cut_dividend = remainder_digit.clone();

                dividend_index -= difference;
            }

            // Digits of the quotient were stored in big endian during calculation, reverse the vector of digits.
            quotient.digits.reverse();
        } else {
            // If lengths of dividend and divisor are equal.
            let (quotient_digit, _remainder_digit) = quotient_estimation_algorithm(self, rhs);
            // Save the quotient digit.
            quotient.push_vec(&quotient_digit.digits);
        }

        // Determine the sign of the quotient.
        // Check the signs of both operands, if they are not the same, the resulting sign is negative.
        // By default the sign is zero, check if it should be change wiht the digits vector's length.
        // If the signs are the same and positive, the resulting sign is positive.
        // If the signs are the same and negative, the resulting sign is positive.
        if !quotient.digits.is_empty() {
            if self.sign != rhs.sign {
                quotient.set_negative_sign();
            } else {
                quotient.set_positive_sign();
            }
        }

        // Cut the leading zeros.
        quotient.normalize();
        quotient
    }
}

// Calculate division or modulus depending on the mode.
// Dividend should be bigger than the divisor, thus dividend should be longer or equal in length to the divisor.
// The function returns separate digits of the quotient or the remainder,
// in form of a tuple: (quotient, remainder)
pub fn quotient_estimation_algorithm(
    dividend: &ChonkerInt,
    divisor: &ChonkerInt,
) -> (ChonkerInt, ChonkerInt) {
    // Make dividends and divisors absolute, positive.
    let mut dividend_original = (*dividend).clone();
    dividend_original.set_positive_sign();
    let mut dividend = (*dividend).clone();
    dividend.set_positive_sign();
    let mut divisor_original = (*divisor).clone();
    divisor_original.set_positive_sign();
    let mut divisor = (*divisor).clone();
    divisor.set_positive_sign();

    // Normalize divisor and calculate the coefficient for the fractional equivalency.
    let coefficient = RADIX / (divisor.digits[divisor.digits.len() - 1] + 1);

    // Check if the calculated equivalency bigger than 1, if it is,
    // use it to increase dividend and divisor.
    if coefficient > 1 {
        let bigint_coefficient = ChonkerInt::from(coefficient as i128);
        dividend = &dividend * &bigint_coefficient;
        divisor = &divisor * &bigint_coefficient;
    }

    let mut quotient;
    let remainder;

    // Calculate quotient estimate. If dividend's length is equal or longer by 1 than the divisor's.
    if (dividend.digits.len()) == (divisor.digits.len() + 1) {
        quotient = ChonkerInt::from(
            ((dividend.digits[dividend.digits.len() - 1] * RADIX
                + dividend.digits[dividend.digits.len() - 2])
                / divisor.digits[divisor.digits.len() - 1]) as i128,
        );
    } else if dividend.digits.len() == divisor.digits.len() {
        quotient = ChonkerInt::from(
            (dividend.digits[dividend.digits.len() - 1] / divisor.digits[divisor.digits.len() - 1])
                as i128,
        );
    } else {
        panic!("dividend has to be equal in length or longer by 1 than the divisor (division_or_modulus_calculation)")
    }

    // Calculate a product between calculated quotient modified divisor.
    let mut check_quotient_product = &quotient * &divisor_original;

    // Create an empty/zero BigInt for comparison with the estimated quotient,
    // if they are equal, create a BigInt, containing zero and return as the quotient.
    let empty_bigint = ChonkerInt::new();

    // Check the quotient estimate, if it does not suit, reduce it by 1 up to 3 times.
    // If it does suit, exit the loop.
    for delta in 0..=3 {
        check_quotient_product.set_positive_sign();
        if quotient == empty_bigint {
            break;
        }
        // if check_quotient_product <= dividend  {
        if check_quotient_product <= dividend_original {
            break;
        }

        quotient = &quotient - &ChonkerInt::from(1);
        check_quotient_product = &check_quotient_product - &divisor_original;

        // There is an error in the algorithm, if after three reductions in the quotient,
        // quotient is still unsuitable and we reach this point.
        if delta == 3 {
            panic!("error with the division algorithm (division_or_modulus_calculation)")
        }
    }

    remainder = &dividend_original - &(&quotient * &divisor_original);

    if quotient == empty_bigint {
        quotient.digits.push(0);
    }
    (quotient, remainder)
}

// Bruteforce method of calculating the division, the bigger the quotient the more time it takes to compute.
// Time requirement increases exponentially, to compute very big quotients.
// While recursive method provides immediate answer, bruteforce could not calculate the result for several minutes.
fn bruteforce_division(dividend: &ChonkerInt, divisor: &ChonkerInt) -> (ChonkerInt, ChonkerInt) {
    // Make dividends and divisors absolute, positive.
    let mut dividend_original = (*dividend).clone();
    dividend_original.set_positive_sign();
    let mut dividend = (*dividend).clone();
    dividend.set_positive_sign();
    let mut divisor_original = (*divisor).clone();
    divisor_original.set_positive_sign();
    let mut divisor = (*divisor).clone();
    divisor.set_positive_sign();

    let zero_bigint = ChonkerInt::new();
    let mut quotient = ChonkerInt::new();

    while dividend >= zero_bigint {
        dividend = &dividend - &divisor;
        quotient = &quotient + &ChonkerInt::from(String::from("1"));
        println!("{:?}", dividend);
        println!("{:?}", quotient);
    }

    if dividend.sign == BigIntSign::Negative {
        quotient = &quotient - &ChonkerInt::from(String::from("1"));
    }

    let remainder = &dividend_original - &(&divisor_original * &quotient);

    (quotient, remainder)
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::ChonkerInt;

    // Test division of two BigInts.
    #[test]
    fn test_bigint_division() {
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

        // Check division of positive BigInts. Try different positions.
        let positive_bigger_by_smaller_division_result = ChonkerInt::from(String::from("4"));
        assert_eq!(
            &positive_bigger_bigint / &positive_smaller_bigint,
            positive_bigger_by_smaller_division_result
        );
        let positive_smaller_by_bigger_division_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            &positive_smaller_bigint / &positive_bigger_bigint,
            positive_smaller_by_bigger_division_result
        );

        // Check division of a bigger positive BigInt with a smaller negative BigInt. Try different positions.
        let bigger_positive_by_smaller_negative_division_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &positive_bigger_bigint / &negative_smaller_bigint,
            bigger_positive_by_smaller_negative_division_result
        );
        let smaller_negative_by_bigger_positive_division_result =
            ChonkerInt::from(String::from("-12"));
        assert_eq!(
            &negative_smaller_bigint / &positive_bigger_bigint,
            smaller_negative_by_bigger_positive_division_result
        );

        // Check division of a smaller positive BigInt with a bigger negative BigInt. Try different positions.
        let smaller_positive_by_bigger_negative_division_result =
            ChonkerInt::from(String::from("-1"));
        assert_eq!(
            &positive_smaller_bigint / &negative_bigger_bigint,
            smaller_positive_by_bigger_negative_division_result
        );
        let bigger_negative_by_smaller_positive_division_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_bigger_bigint / &positive_smaller_bigint,
            bigger_negative_by_smaller_positive_division_result
        );

        // Check division of negative BigInts. Try different positions.
        let negative_bigger_by_smaller_division_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_bigger_bigint / &negative_smaller_bigint,
            negative_bigger_by_smaller_division_result
        );
        let negative_smaller_by_bigger_division_result = ChonkerInt::from(String::from("99"));
        assert_eq!(
            &negative_smaller_bigint / &negative_bigger_bigint,
            negative_smaller_by_bigger_division_result
        );

        // Check division of positive BigInts with an empty/zero BigInt. Try different positions.
        // let positive_bigger_by_empty_division_result = ChonkerInt::from(String::from("0"));
        // assert_eq!(&positive_bigger_bigint / &zero_bigint, positive_bigger_by_empty_division_result);

        let empty_by_positive_smaller_division_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            &zero_bigint / &positive_smaller_bigint,
            empty_by_positive_smaller_division_result
        );

        // Check division of negative BigInts with an empty/zero BigInt. Try different positions.
        // let negative_bigger_by_empty_division_result = ChonkerInt::from(String::from("0"));
        // assert_eq!(&negative_bigger_bigint / &zero_bigint, negative_bigger_by_empty_division_result);

        let empty_by_negative_smaller_division_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            &zero_bigint / &negative_smaller_bigint,
            empty_by_negative_smaller_division_result
        );

        // Check division of very BigInts.
        let positive_very_big_bigger_by_negative_very_big_smaller_result =
            ChonkerInt::from(String::from("-558190621864"));
        assert_eq!(
            &positive_bigger_very_big_bigint / &negative_smaller_very_big_bigint,
            positive_very_big_bigger_by_negative_very_big_smaller_result
        );
        let positive_very_big_smaller_by_negative_very_big_smaller_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &positive_smaller_very_big_bigint / &negative_smaller_very_big_bigint,
            positive_very_big_smaller_by_negative_very_big_smaller_result
        );
        let positive_very_big_bigger_by_negative_very_big_bigger_result = ChonkerInt::from(String::from("-1179010161992692051385290940889169636086403689958542693814405492871089212617077532843158926503"));
        assert_eq!(
            &positive_bigger_very_big_bigint / &negative_bigger_very_big_bigint,
            positive_very_big_bigger_by_negative_very_big_bigger_result
        );
        let positive_very_big_smaller_by_negative_very_big_bigger_result =
            ChonkerInt::from(String::from("-1"));
        assert_eq!(
            &positive_smaller_very_big_bigint / &negative_bigger_very_big_bigint,
            positive_very_big_smaller_by_negative_very_big_bigger_result
        );

        let negative_very_big_bigger_by_positive_very_big_smaller_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_bigger_very_big_bigint / &positive_smaller_very_big_bigint,
            negative_very_big_bigger_by_positive_very_big_smaller_result
        );
        let negative_very_big_smaller_by_positive_very_big_smaller_result = ChonkerInt::from(String::from("-1098528176678773945133770019243427742596676897941676790359156611511232741298674199"));
        assert_eq!(
            &negative_smaller_very_big_bigint / &positive_smaller_very_big_bigint,
            negative_very_big_smaller_by_positive_very_big_smaller_result
        );
        let negative_very_big_bigger_by_positive_very_big_bigger_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_bigger_very_big_bigint / &positive_bigger_very_big_bigint,
            negative_very_big_bigger_by_positive_very_big_bigger_result
        );
        let negative_very_big_smaller_by_positive_very_big_bigger_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_bigger_very_big_bigint / &positive_bigger_very_big_bigint,
            negative_very_big_smaller_by_positive_very_big_bigger_result
        );
        //971101161992692513852994889169636864368995854269381445492871892126177753284315892653
        // Check division of very big positive BigInts.
        let positive_very_big_bigger_by_positive_very_big_bigger_result =
            ChonkerInt::from(String::from("1"));
        assert_eq!(
            &positive_bigger_very_big_bigint / &positive_bigger_very_big_bigint,
            positive_very_big_bigger_by_positive_very_big_bigger_result
        );
        let positive_very_big_bigger_by_positive_very_big_smaller_result = ChonkerInt::from(String::from("613188126075967005949151232496730640186830021148675211669412879242084924584975233197896711416"));
        assert_eq!(
            &positive_bigger_very_big_bigint / &positive_smaller_very_big_bigint,
            positive_very_big_bigger_by_positive_very_big_smaller_result
        );
        let positive_very_big_smaller_by_positive_very_big_smaller_result =
            ChonkerInt::from(String::from("1"));
        assert_eq!(
            &positive_smaller_very_big_bigint / &positive_smaller_very_big_bigint,
            positive_very_big_smaller_by_positive_very_big_smaller_result
        );
        let positive_very_big_smaller_by_positive_very_big_bigger_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &positive_smaller_very_big_bigint / &positive_bigger_very_big_bigint,
            positive_very_big_smaller_by_positive_very_big_bigger_result
        );

        // Check division of very big negative BigInts.
        let negative_very_big_bigger_by_negative_very_big_bigger_result =
            ChonkerInt::from(String::from("1"));
        assert_eq!(
            &negative_bigger_very_big_bigint / &negative_bigger_very_big_bigint,
            negative_very_big_bigger_by_negative_very_big_bigger_result
        );
        let negative_very_big_bigger_by_negative_very_big_smaller_result =
            ChonkerInt::from(String::from("0"));
        assert_eq!(
            &negative_bigger_very_big_bigint / &negative_smaller_very_big_bigint,
            negative_very_big_bigger_by_negative_very_big_smaller_result
        );
        let negative_very_big_smaller_by_negative_very_big_smaller_result =
            ChonkerInt::from(String::from("1"));
        assert_eq!(
            &negative_smaller_very_big_bigint / &negative_smaller_very_big_bigint,
            negative_very_big_smaller_by_negative_very_big_smaller_result
        );
        let negative_very_big_smaller_by_negative_very_big_bigger_result = ChonkerInt::from(String::from("2112199875473649343041361462004034608444019134994613761100327207438381586024335294"));
        assert_eq!(
            &negative_smaller_very_big_bigint / &negative_bigger_very_big_bigint,
            negative_very_big_smaller_by_negative_very_big_bigger_result
        );

        //                                                             [3, 5, 6, 2, 9, 8, 5, 1, 3, 4, 8, 2, 3, 5, 7, 7, 7, 1, 6, 2, 1, 2, 9, 8, 1, 7, 8, 2, 9, 4, 5, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 4, 6, 8, 6, 3, 6, 9, 6, 1, 9, 8, 8, 4, 9, 9, 2, 5, 8, 3, 1, 5, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 9, 7, 1, 1],
        //                               [3, 0, 5, 6, 2, 9, 8, 5, 1, 3, 4, 8, 2, 3, 5, 7, 7, 0, 7, 1, 6, 2, 1, 2, 9, 8, 0, 1, 7, 8, 2, 9, ! 4, 5, 0, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 0, 4, 6, 8, 0, 6, 3, 6, 9, 6, 1, 9, 8, 8, 0, 4, 9, 0, 9, 2, 5, 8, 3, 1, 5, 0, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 0, 9, 7, 1, 1]
        // [1, 9, 4, 5, 9, 3, 8, 8, 2, 4, 2, 2, 7, 6, 6, 1, 6, 7, 2, 5, 3, 5, 0, 9, 3, 8, 4, 3, 3, 5, 6, 8, 2, 3, 1, 3, 9, 7, 0, 1, 7, 8, ! 4, 5, 0, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 0, 4, 6, 8, 0, 6, 3, 6, 9, 6, 1, 9, 8, 8, 0, 4, 9, 0, 9, 2, 5, 8, 3, 1, 5, 0, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 0, 9, 7, 1, 1]
        // [1, 9, 4, 5, 9, 3, 8, 8, 2, 4, 2, 2, 7, 6, 6, 1, 6, 7, 2, 5, 3, 5, 0, 9, 3, 8, 4, 3, 3, 5, 6, 8, 2, 3, 1, 3, 9, 7, 0, 1, 7, 8, ! 4, 5, 0, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 0, 4, 6, 8, 0, 6, 3, 6, 9, 6, 1, 9, 8, 8, 0, 4, 9, 0, 9, 2, 5, 8, 3, 1, 5, 0, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 0, 9, 7, 1, 1]
        // [1, 9, 4, 5, 9, 3, 8, 8, 2, 4, 2, 2, 7, 6, 6, 1, 6, 7, 2, 5, 3, 5, 0, 9, 3, 8, 4, 3, 3, 5, 6, 8, 2, 3, 1, 3, 9, 7, 0, 1, 7, 8, ! 4, 5, 0, 4, 4, 1, 8, 3, 9, 6, 2, 4, 5, 8, 5, 9, 9, 8, 6, 3, 0, 4, 6, 8, 0, 6, 3, 6, 9, 6, 1, 9, 8, 8, 0, 4, 9, 0, 9, 2, 5, 8, 3, 1, 5, 0, 2, 9, 6, 2, 9, 9, 1, 6, 1, 0, 1, 0, 9, 7, 1, 1]
        // 19459388242276616725350938433568231397017845044183962458599863046806369619880490925831502962991610109711
    }

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
