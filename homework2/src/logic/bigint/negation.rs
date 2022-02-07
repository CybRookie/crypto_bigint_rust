// BigInt module regarding negation of a BigInt.

use std::ops::Neg;

use crate::logic::bigint::{BigIntSign, ChonkerInt};

// Implement negation "-" operator for the BigInt.
// impl Neg for ChonkerInt {
impl Neg for &ChonkerInt {
    // type Output = Self;
    type Output = ChonkerInt;

    fn neg(self) -> Self::Output {
        match self.sign {
            BigIntSign::Negative => ChonkerInt {
                digits: self.digits.clone(),
                sign: BigIntSign::Positive,
            },
            BigIntSign::Positive => ChonkerInt {
                digits: self.digits.clone(),
                sign: BigIntSign::Negative,
            },
            BigIntSign::Zero => ChonkerInt::new(),
        }
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::negation::Neg;
    use crate::logic::bigint::ChonkerInt;

    // Test change of BigInt's sign, when it is empty.
    #[test]
    fn test_empty_bigint_negation() {
        let negated_empty_bigint = ChonkerInt::new().neg();

        assert_eq!(ChonkerInt::new(), negated_empty_bigint);
    }

    // Test change of BigInt's sign, when it is negative.
    #[test]
    fn test_negative_bigint_negation() {
        let negated_negative_bigint = ChonkerInt::from(String::from("-123")).neg();
        let comparison_positive_bigint = ChonkerInt::from(String::from("123"));

        assert_eq!(comparison_positive_bigint, negated_negative_bigint);
    }

    // Test change of BigInt's sign, when it is positive.
    #[test]
    fn test_positive_bigint_negation() {
        let negated_positive_bigint = ChonkerInt::from(String::from("123")).neg();
        let comparison_negative_bigint = ChonkerInt::from(String::from("-123"));

        assert_eq!(comparison_negative_bigint, negated_positive_bigint);
    }
}
