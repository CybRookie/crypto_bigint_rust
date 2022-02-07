// BigInt module regarding randomisation of BigInts.

// Import required randomisation items.
use rand::Rng;

use crate::logic::bigint::{BigIntSign, ChonkerInt};

// Implement randomisation methods for BigInt.
impl ChonkerInt {
    // Initialize a randomly filled BigInt.
    pub fn new_rand(length: &u64, sign: &BigIntSign) -> ChonkerInt {
        if *length == 0 {
            panic!("requested length for random bigint generation is 0, nothing to generate");
        }

        let mut rng = rand::thread_rng();
        let mut bigint = ChonkerInt::new();

        // Assign the requested sign.
        match *sign {
            BigIntSign::Positive => bigint.set_positive_sign(),
            BigIntSign::Negative => bigint.set_negative_sign(),
            BigIntSign::Zero => panic!("zeros are not randomly generated"),
        }

        let main_length = *length - 1;
        let mut digit: i8;

        // Fill the empty BigInt with the requested amount of random digits in the range of 0-9.
        for _iteration in 0..main_length {
            digit = rng.gen_range(0..=9);
            let _ = bigint.push(digit);
        }

        // Ensure that the leading/last digit is not zero. Generate it separately.
        digit = rng.gen_range(1..=9);
        let _ = bigint.push(digit);

        bigint
    }

    // Initialize a randomly filled BigInt from the provided range of lengths.
    pub fn new_rand_range_len(start: &u64, end: &u64, sign: &BigIntSign) -> ChonkerInt {
        if *start == 0 || *end == 0 {
            panic!("start or end length boundary for the random BigInt generation is zero, nothing to generate (ChonkerInt::new_rand_range_len)");
        }

        if *start > *end {
            panic!("provided incorrect boundaries for the random BigInt generation, starting boundary must be lower or equal to the ending one (ChonkerInt::new_rand_range_len)");
        }

        let mut rng = rand::thread_rng();
        let mut bigint = ChonkerInt::new();

        // Assign requested sign.
        match *sign {
            BigIntSign::Positive => bigint.set_positive_sign(),
            BigIntSign::Negative => bigint.set_negative_sign(),
            BigIntSign::Zero => panic!("zeros are not randomly generated"),
        }

        // Randomly generate the length of the BigInt from the provided range.
        let main_length = (rng.gen_range((*start)..=(*end))) - 1;

        let mut digit: i8;

        // Fill the empty BigInt with the requested amount of random digits in the range of 0-9.
        for _iteration in 0..main_length {
            digit = rng.gen_range(0..=9);
            let _ = bigint.push(digit);
        }

        // Ensure that the leading/last digit is not zero. Generate it separately.
        digit = rng.gen_range(1..=9);
        let _ = bigint.push(digit);

        bigint
    }

    // Initialize a randomly filled BigInt from the provided range of lengths, boundaries included.
    pub fn new_rand_range_value(
        start: &ChonkerInt,
        end: &ChonkerInt,
        sign: &BigIntSign,
    ) -> ChonkerInt {
        let big_zero = ChonkerInt::new();

        // Check if either of boundaries is zero.
        if *start == big_zero || *end == big_zero {
            panic!("start or end length boundary for the random BigInt generation is zero, nothing to generate (ChonkerInt::new_rand_range_value)");
        }

        // Check if either of boundaries is negative.
        if *start < big_zero || *end < big_zero {
            panic!("start or end length boundary for the random BigInt generation is negative, nothing to generate (ChonkerInt::new_rand_range_value)");
        }

        // Check if starting boundary is bigger than the ending boundary.
        if *start >= *end {
            panic!("provided incorrect boundaries for the random BigInt generation, starting boundary must be lower and not equal to the ending one (ChonkerInt::new_rand_range_value)");
        }

        // Check requested sign.
        if (*sign) == BigIntSign::Zero {
            panic!("zeros are not randomly generated");
        }

        let mut bigint = ChonkerInt::new();
        bigint.set_positive_sign();

        // Randomly generate the BigInt from the provided range of values.
        loop {
            bigint = ChonkerInt::new_rand_range_len(
                &(start.digits.len() as u64),
                &(end.digits.len() as u64),
                &BigIntSign::Positive,
            );

            // Check if the generated value is in between the requested boundaries.
            if (bigint >= (*start)) && (bigint <= (*end)) {
                // Assign requested sign.
                match *sign {
                    BigIntSign::Positive => bigint.set_positive_sign(),
                    BigIntSign::Negative => bigint.set_negative_sign(),
                    _ => (),
                }

                return bigint;
            } else {
                bigint = ChonkerInt::new();
                bigint.set_positive_sign();
            }
        }
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::{BigIntSign, ChonkerInt};

    // Test creation/construction of a filled BigInt with random digits.
    #[test]
    fn test_random_bigint_construction() {
        let requested_length: u64 = 418256; // Reasonable limit, considering O(n) time complexity. Length: 6-7.
                                            // let requested_zero_length: u64 = 0;
        let requested_positive_sign = BigIntSign::Positive;
        let requested_negative_sign = BigIntSign::Negative;
        // let requested_zero_sign = BigIntSign::Zero;

        let random_positive_bigint =
            ChonkerInt::new_rand(&requested_length, &requested_positive_sign);
        let random_negative_bigint =
            ChonkerInt::new_rand(&requested_length, &requested_negative_sign);

        // let random_zero_bigint1 = ChonkerInt::new_rand(&requested_zero_length, &requested_negative_sign);
        // let random_zero_bigint1 = ChonkerInt::new_rand(&requested_length, &requested_zero_sign);

        assert_eq!(random_positive_bigint.sign, requested_positive_sign);
        assert_eq!(
            random_positive_bigint.digits.len(),
            requested_length as usize
        );
        assert_eq!(random_negative_bigint.sign, requested_negative_sign);
        assert_eq!(
            random_negative_bigint.digits.len(),
            requested_length as usize
        );
    }

    // Test creation/construction of random BigInt from the provided range of lengths.
    #[test]
    fn test_random_bigint_range_length_construction() {
        let requested_length_start: u64 = 1; // Reasonable limit, considering O(n) time complexity. Length: max 11-12.
        let requested_length_end: u64 = 12;
        let requested_positive_sign = BigIntSign::Positive;
        let requested_negative_sign = BigIntSign::Negative;
        // let requested_zero_sign = BigIntSign::Zero;

        let random_positive_bigint = ChonkerInt::new_rand_range_len(
            &requested_length_start,
            &requested_length_end,
            &requested_positive_sign,
        );
        let random_negative_bigint = ChonkerInt::new_rand_range_len(
            &requested_length_start,
            &requested_length_end,
            &requested_negative_sign,
        );

        // let random_zero_bigint1 = ChonkerInt::new_rand_range_len(&requested_length_start, &requested_length_end, &requested_negative_sign);
        // let random_zero_bigint1 = ChonkerInt::new_rand_range_len(&requested_length_start, &requested_length_end, &requested_zero_sign);

        assert_eq!(random_positive_bigint.sign, requested_positive_sign);
        assert!((1..=12).contains(&(random_positive_bigint.digits.len())));
        assert_eq!(random_negative_bigint.sign, requested_negative_sign);
        assert!((1..=12).contains(&(random_negative_bigint.digits.len())));
    }

    // Test creation/construction of random BigInt from the provided range of values.
    #[test]
    fn test_random_bigint_range_value_construction() {
        let requested_value_start = ChonkerInt::from(1);
        let requested_value_end = ChonkerInt::from(1234567);
        let requested_positive_sign = BigIntSign::Positive;
        let requested_negative_sign = BigIntSign::Negative;
        // let requested_zero_sign = BigIntSign::Zero;

        let random_positive_bigint = ChonkerInt::new_rand_range_value(
            &requested_value_start,
            &requested_value_end,
            &requested_positive_sign,
        );
        let mut random_negative_bigint = ChonkerInt::new_rand_range_value(
            &requested_value_start,
            &requested_value_end,
            &requested_negative_sign,
        );

        // let random_zero_bigint1 = ChonkerInt::new_rand_range_value(&requested_value_start, &requested_value_end, &requested_negative_sign);
        // let random_zero_bigint1 = ChonkerInt::new_rand_range_value(&requested_value_start, &requested_value_end, &requested_zero_sign);

        assert_eq!(random_positive_bigint.sign, requested_positive_sign);
        assert!(
            random_positive_bigint >= requested_value_start
                && random_positive_bigint <= requested_value_end
        );
        assert_eq!(random_negative_bigint.sign, requested_negative_sign);
        random_negative_bigint.set_positive_sign();
        assert!(
            random_positive_bigint >= requested_value_start
                && random_positive_bigint <= requested_value_end
        );
    }
}
