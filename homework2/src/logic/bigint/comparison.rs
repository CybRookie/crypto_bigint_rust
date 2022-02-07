// BigInt module regarding comparison and total order of BigInts.

use std::cmp::Ordering;

use crate::logic::bigint::{BigIntSign, ChonkerInt};

// Implement total ordering for the BigInt.
impl Ord for ChonkerInt {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare signs, if they are equal, proceed with length and content checks.
        // If both are zero/empty, return equal ordering.
        if (self.digits.is_empty() && other.digits.is_empty())
            || (self.sign == BigIntSign::Zero && other.sign == BigIntSign::Zero)
        {
            return Ordering::Equal;
        }

        if self.sign == other.sign {
            // If the lengths are equal compare separate digits in the vector.
            // Reminder: digits are stored in little endian format.
            // If the self length is less that other's return Ordering variant Less, Greater otherwise.
            // Consider the signs: if the signs are positive, longer length - greater, bigger separate digit - greater;
            // if the signs are negative, shorter length - greater, smaller separate digit - greater.
            let ordering_determination = |sign: &BigIntSign| {
                match self.digits.len().cmp(&other.digits.len()) {
                    Ordering::Less => {
                        if *sign == BigIntSign::Positive {
                            Ordering::Less
                        } else {
                            // Negative case.
                            Ordering::Greater
                        }
                    }
                    Ordering::Equal => {
                        let mut self_char_index = self.digits.len() - 1;
                        let mut other_char_index = other.digits.len() - 1;

                        loop {
                            match self.digits[self_char_index].cmp(&other.digits[other_char_index])
                            {
                                Ordering::Less => {
                                    return if *sign == BigIntSign::Positive {
                                        Ordering::Less
                                    } else {
                                        // Negative case.
                                        Ordering::Greater
                                    };
                                }
                                Ordering::Equal => {}
                                Ordering::Greater => {
                                    return if *sign == BigIntSign::Positive {
                                        Ordering::Greater
                                    } else {
                                        // Negative case.
                                        Ordering::Less
                                    };
                                }
                            }

                            // Check for index being zero, if allowed,
                            // to proceed an attempt at subtraction against unsigned zero would happen.
                            if self_char_index == 0 {
                                break;
                            }

                            self_char_index -= 1;
                            other_char_index -= 1;
                        }

                        Ordering::Equal
                    }
                    Ordering::Greater => {
                        if *sign == BigIntSign::Positive {
                            Ordering::Greater
                        } else {
                            // Negative case.
                            Ordering::Less
                        }
                    }
                }
            };

            return match self.sign {
                BigIntSign::Positive => ordering_determination(&self.sign),
                BigIntSign::Zero => Ordering::Equal,
                BigIntSign::Negative => ordering_determination(&self.sign),
            };
        } else if self.sign == BigIntSign::Negative
            && (other.sign == BigIntSign::Positive || other.sign == BigIntSign::Zero)
        {
            return Ordering::Less;
        } else if self.sign == BigIntSign::Positive
            && (other.sign == BigIntSign::Negative || other.sign == BigIntSign::Zero)
        {
            return Ordering::Greater;
        } else if self.sign == BigIntSign::Zero && other.sign == BigIntSign::Positive {
            return Ordering::Less;
        } else if self.sign == BigIntSign::Zero && other.sign == BigIntSign::Negative {
            return Ordering::Greater;
        }

        // If the signs, lengths and digits were equal, return Ordering variant of Equal.
        Ordering::Equal
    }
}

// Implement comparison operators "<", "<=", ">", ">=" for the BigInt,
// based on the implementation of the total ordering.
impl PartialOrd for ChonkerInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::ChonkerInt;

    // Test comparisons of BigInts.
    #[test]
    fn test_bigint_comparison() {
        let positive_bigint = ChonkerInt::from(String::from("123"));
        let positive_bigint_clone = ChonkerInt::from(String::from("123"));
        let negative_bigint = ChonkerInt::from(String::from("-123"));
        let zero_bigint = ChonkerInt::new();

        assert!(positive_bigint > negative_bigint);
        assert!(positive_bigint > zero_bigint);
        assert!(negative_bigint < positive_bigint);
        assert!(negative_bigint < zero_bigint);
        assert!(zero_bigint < positive_bigint);
        assert!(zero_bigint > negative_bigint);
        assert!(positive_bigint >= positive_bigint_clone);
        assert!(positive_bigint <= positive_bigint_clone);
    }

    // Test total ordering of BigInts.
    #[test]
    fn test_bigint_total_ordering() {
        let mut bigint_target_list = vec![
            ChonkerInt::from(18653),
            ChonkerInt::from(48467647),
            ChonkerInt::from(0),
            ChonkerInt::from(0),
            ChonkerInt::from(23),
            ChonkerInt::from(3418022758u32),
            ChonkerInt::from(4214578),
            ChonkerInt::from(1622),
            ChonkerInt::from(1),
            ChonkerInt::from(96935294),
            ChonkerInt::from(2107289),
            ChonkerInt::from(811),
            ChonkerInt::from(2),
            ChonkerInt::from(37306),
            ChonkerInt::from(1709011379),
            ChonkerInt::from(46),
            ChonkerInt::from(78614523434u64),
            ChonkerInt::from(39307261717u64),
            ChonkerInt::from(-124),
            ChonkerInt::from(-1),
            ChonkerInt::from(-124),
            ChonkerInt::from(-10004),
        ];

        let bigint_comparison_list = vec![
            ChonkerInt::from(-10004),
            ChonkerInt::from(-124),
            ChonkerInt::from(-124),
            ChonkerInt::from(-1),
            ChonkerInt::from(0),
            ChonkerInt::from(0),
            ChonkerInt::from(1),
            ChonkerInt::from(2),
            ChonkerInt::from(23),
            ChonkerInt::from(46),
            ChonkerInt::from(811),
            ChonkerInt::from(1622),
            ChonkerInt::from(18653),
            ChonkerInt::from(37306),
            ChonkerInt::from(2107289),
            ChonkerInt::from(4214578),
            ChonkerInt::from(48467647),
            ChonkerInt::from(96935294),
            ChonkerInt::from(1709011379),
            ChonkerInt::from(3418022758u32),
            ChonkerInt::from(39307261717u64),
            ChonkerInt::from(78614523434u64),
        ];

        bigint_target_list.sort();

        assert_eq!(bigint_target_list, bigint_comparison_list)
    }
}
