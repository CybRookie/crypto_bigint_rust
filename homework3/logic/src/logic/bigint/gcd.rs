// BigInt module regarding greatest common divisor of BigInts.

use crate::logic::bigint::ChonkerInt;

#[derive(Debug, PartialEq, Eq)]
pub struct EGCDResult {
    pub gcd: ChonkerInt,
    pub self_x: ChonkerInt,
    pub other_y: ChonkerInt,
}

// Implement gcd method for BigInt.
impl ChonkerInt {
    // A recursive function to find the greatest common divisor.
    pub fn gcd(&self, other: &ChonkerInt) -> ChonkerInt {
        let big_zero = ChonkerInt::new();

        // Check arguments for zeros.
        if *self == big_zero || self.digits.is_empty() {
            return (*other).clone();
        } else if *other == big_zero || other.digits.is_empty() {
            return (*self).clone();
        }

        // Ensure that the operands are positive, make values absolute.
        let mut first_operand = (*self).clone();
        first_operand.set_positive_sign();
        let mut second_operand = (*other).clone();
        second_operand.set_positive_sign();

        if first_operand < second_operand {
            second_operand.gcd(&first_operand)
        } else if (&first_operand % &second_operand) == big_zero {
            second_operand.clone()
        } else {
            second_operand.gcd(&(&first_operand % &second_operand))
        }
    }

    // A recursive function to find the greatest common divisor.
    pub fn egcd(&self, other: &ChonkerInt) -> EGCDResult {
        let big_zero = ChonkerInt::new();
        let big_one = ChonkerInt::from(1);

        // Check arguments for zeros.
        if *self == big_zero {
            return EGCDResult {
                gcd: (*other).clone(),
                self_x: big_zero,
                other_y: big_one,
            };
        } else if *other == big_zero {
            return EGCDResult {
                gcd: (*self).clone(),
                self_x: big_zero,
                other_y: big_one,
            };
        }

        let mut first_operand = (*self).clone();
        first_operand.set_positive_sign();
        let mut second_operand = (*other).clone();
        second_operand.set_positive_sign();

        // Bézout coefficients x for self and y for other.
        let mut self_xs_old = big_one.clone();
        let mut self_xs = big_zero.clone();

        let mut other_yt_old = big_zero.clone();
        let mut other_yt = big_one.clone();

        // GCD / remainder.
        // Ensure that the operands are positive, make values absolute.
        let mut gcd_r_old = first_operand.clone();
        let mut gcd_r = second_operand.clone();

        let mut temp;
        let mut quotient;

        while gcd_r != big_zero {
            quotient = &gcd_r_old / &gcd_r;

            // Calculate remainders, last non-zero remainder is the GCD.
            temp = gcd_r.clone();
            gcd_r = &gcd_r_old - &(&quotient * &temp);
            gcd_r_old = temp.clone();

            // Calculate coefficient x/s of the self, penultimate is the x/s coefficient of the self.
            temp = self_xs.clone();
            self_xs = &self_xs_old - &(&quotient * &temp);
            self_xs_old = temp.clone();

            // Calculate coefficient y/t of the other, penultimate is the y/t coefficient of the other.
            temp = other_yt.clone();
            other_yt = &other_yt_old - &(&quotient * &temp);
            other_yt_old = temp.clone();
        }

        // Signs should be appropriately modified, unfinished section.
        // Change the signs of coefficient based on the signs of inputs.
        // if self.sign == BigIntSign::Negative && other.sign == BigIntSign::Negative {
        //     self_xs_old.set_negative_sign();
        //     other_yt_old.set_positive_sign();
        // } else if self.sign == BigIntSign::Positive && other.sign == BigIntSign::Negative {
        //     other_yt_old.set_positive_sign();
        // } else if self.sign == BigIntSign::Negative && other.sign == BigIntSign::Positive {
        //     self_xs_old.set_negative_sign();
        // } // If both are positive, x is positive, y is negative by default.

        EGCDResult {
            gcd: gcd_r_old,
            self_x: self_xs_old,
            other_y: other_yt_old,
        }
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::{BigIntSign, ChonkerInt};

    // Test the method computing the greatest common divisor between two BigInts.
    #[test]
    fn test_bigint_gcd() {
        let bigint1 = ChonkerInt::new_rand(&13, &BigIntSign::Positive);
        let bigint2 = ChonkerInt::new_rand(&10, &BigIntSign::Positive);
        let bigint3 = ChonkerInt::new_rand(&10, &BigIntSign::Negative);
        let bigint4 = ChonkerInt::from(4235);
        let bigint5 = ChonkerInt::from(43634615);
        let bigint_zero = ChonkerInt::new();

        println!("bigint1: {}", bigint1);
        println!("bigint1 len: {}", bigint1.digits.len());
        println!("bigint2: {}", bigint2);
        println!("bigint2 len: {}", bigint2.digits.len());
        println!("bigint3: {}", bigint3);
        println!("bigint3 len: {}", bigint3.digits.len());

        let result_positives_gcd = bigint1.gcd(&bigint2);
        let result_positive_and_negative_gcd = bigint1.gcd(&bigint3);
        let result_negative_and_positive_gcd = bigint3.gcd(&bigint2);
        let result_positive_and_zero_gcd = bigint_zero.gcd(&bigint1);
        let result_negative_and_zero_gcd = bigint_zero.gcd(&bigint3);
        let result_custom_gcd = ChonkerInt::from(5);

        println!(
            "Result GCD between positive values: {}",
            result_positives_gcd
        );
        println!(
            "Result GCD between a positive and a negative value: {}",
            result_positive_and_negative_gcd
        );
        println!(
            "Result GCD between a negative and a positive value: {}",
            result_negative_and_positive_gcd
        );
        println!(
            "Result GCD between a positive and a zero value: {}",
            result_positive_and_zero_gcd
        );
        println!(
            "Result GCD between a negative and a zero value: {}",
            result_negative_and_zero_gcd
        );

        assert_eq!(bigint4.gcd(&bigint5), result_custom_gcd);
    }

    // Test the method computing the greatest common divisor and
    // Bézout coefficients with extended Euclidena algorithm between two BigInts.
    #[test]
    fn test_bigint_egcd() {
        let bigint1 = ChonkerInt::new_rand(&13, &BigIntSign::Positive);
        let bigint2 = ChonkerInt::new_rand(&10, &BigIntSign::Positive);
        let bigint3 = ChonkerInt::new_rand(&10, &BigIntSign::Negative);
        let bigint4 = ChonkerInt::from(4235);
        let bigint5 = ChonkerInt::from(43634615);
        let custom_xs = ChonkerInt::from(30910);
        let custom_yt = ChonkerInt::from(-3);
        let bigint_zero = ChonkerInt::new();

        println!("bigint1: {}", bigint1);
        println!("bigint1 len: {}", bigint1.digits.len());
        println!("bigint2: {}", bigint2);
        println!("bigint2 len: {}", bigint2.digits.len());
        println!("bigint3: {}", bigint3);
        println!("bigint3 len: {}", bigint3.digits.len());

        let result_positives_gcd = bigint1.egcd(&bigint2);
        let result_positive_and_negative_gcd = bigint1.egcd(&bigint3);
        let result_negative_and_positive_gcd = bigint3.egcd(&bigint2);
        let result_positive_and_zero_gcd = bigint_zero.egcd(&bigint1);
        let result_negative_and_zero_gcd = bigint_zero.egcd(&bigint3);
        let result_custom_gcd = ChonkerInt::from(5);
        let result_custom_gcd_calculated = bigint4.egcd(&bigint5);

        println!(
            "Result GCD between positive values: {:?}\n",
            result_positives_gcd
        );
        println!(
            "Result GCD between a positive and a negative value: {:?}\n",
            result_positive_and_negative_gcd
        );
        println!(
            "Result GCD between a negative and a positive value: {:?}\n",
            result_negative_and_positive_gcd
        );
        println!(
            "Result GCD between a positive and a zero value: {:?}\n",
            result_positive_and_zero_gcd
        );
        println!(
            "Result GCD between a negative and a zero value: {:?}\n",
            result_negative_and_zero_gcd
        );
        println!(
            "Result GCD between a custom values: {:?}\n",
            result_custom_gcd_calculated
        );

        assert_eq!(result_custom_gcd_calculated.gcd, result_custom_gcd);
        assert_eq!(result_custom_gcd_calculated.self_x, custom_xs);
        assert_eq!(result_custom_gcd_calculated.other_y, custom_yt);
    }
}
