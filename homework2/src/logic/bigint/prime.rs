// BigInt module regarding prime BigInts.

// Import required randomisation items.
use rand::seq::SliceRandom;
use rand::Rng;

use crate::logic::bigint::{BigIntSign, ChonkerInt};

// Implement BigInt methods for random prime generation and primality testing.
impl ChonkerInt {
    // Initialize a randomly filled prime BigInt.
    // Test for primality is based on the Miller-Rabin probabilistic test. 10 trials are done.
    pub fn new_prime(length: &u64) -> ChonkerInt {
        if *length == 0 {
            panic!("requested length for random bigint generation is 0, nothing to generate");
        }

        let mut rng = rand::thread_rng();
        let mut bigint = ChonkerInt::new();
        bigint.set_positive_sign();
        let main_length = *length - 2;
        let mut digit: i8;
        let least_significant_candidates: Vec<i8> = vec![1, 3, 5, 7, 9];

        // If the length of the requested prime is 1, generate the prime separately.
        if *length == 1 {
            let one_digit_prime_candidates: Vec<i8> = vec![2, 3, 5, 7];
            digit = *(one_digit_prime_candidates.choose(&mut rng).unwrap());
            let _ = bigint.push(digit);

            return bigint;
        }

        loop {
            // Ensure that the produced BigInt is odd, by limiting the least significant values to odd ones:
            // 1, 3, 5, 7, 9.
            digit = *(least_significant_candidates.choose(&mut rng).unwrap());
            let _ = bigint.push(digit);

            // Fill the empty BigInt with the requested amount of random digits in the range of 0-9.
            for _iteration in 0..main_length {
                digit = rng.gen_range(0..=9);
                let _ = bigint.push(digit);
            }
            // Ensure that the leading/last digit is not zero. Generate it separately.
            digit = rng.gen_range(1..=9);
            let _ = bigint.push(digit);

            if bigint.is_prime_probabilistic(Some(5)) {
                break;
            } else {
                bigint = ChonkerInt::new();
                bigint.set_positive_sign();
            }
        }

        bigint
    }

    // Generate a coprime to the number.
    pub fn new_coprime(&self) -> ChonkerInt {
        let big_zero = ChonkerInt::new();

        // Check the target for being a zero.
        if *self == big_zero {
            panic!(
                "a target number for coprime generation cannot be zero (ChonkerInt::new_coprime)"
            )
        }

        let mut candidate;

        // Generate a random candidate with length from the range 1 - self,
        // and check it is a comprime number to the target.
        loop {
            candidate = ChonkerInt::new_rand_range_len(
                &1,
                &(self.digits.len() as u64),
                &BigIntSign::Positive,
            );
            if candidate.is_coprime(self) {
                break;
            }
        }

        candidate
    }

    // Generate a primitive root to the modulo of prime n
    pub fn new_primitive_root(&self) -> ChonkerInt {
        let big_one = ChonkerInt::from(1);
        let big_two = ChonkerInt::from(2);

        // Check if the target is a prime number.
        if !self.is_prime_probabilistic(Some(2)) {
            panic!("a prime number is required for primitive root generation (ChonkerInt::new_primitive_root)");
        }

        // Find value of Euler Totient function of n. Since n is a prime number, the value of Euler
        // Totient function is n-1 as there are n-1 relatively prime numbers.
        let target_one = self - &big_one;

        // Calculate factors of the prime-1 and determine exponents for the testing
        // from (prime-1 / factor) calculation.
        let mut prime_factor_list: Vec<ChonkerInt> = target_one.prime_factor();
        prime_factor_list.dedup();
        let mut candidate;

        // Pick a random number from the suitable range, check if it is a primitive root.
        'outer: loop {
            candidate =
                ChonkerInt::new_rand_range_value(&big_two, &target_one, &BigIntSign::Positive);

            // Check x^(p-1) = 1 (modulo p), if the result does not equal 1, restart the search.
            if candidate.modpow(&target_one, self) != big_one {
                continue;
            }

            // Check x^((p-1)/f) (modulo p), if any factor results in 1, restart the search.
            for exponent in prime_factor_list.iter() {
                if candidate.modpow(&(&target_one / exponent), self) == big_one {
                    continue 'outer;
                }
            }

            return candidate;
        }
    }

    // Check if this BigInt is a prime number, works only with the natural numbers.
    // Returns true, when the number is a prime one, false otherwise.
    // Time complexity is O(sqrt(N)), check of the even numbers is skipped.
    pub fn is_prime(&self) -> bool {
        // Return false if the BigInt is negative, zero or one.
        if (*self == ChonkerInt::from(1))
            || (*self == ChonkerInt::new() || self.sign == BigIntSign::Zero)
            || (self.sign == BigIntSign::Negative)
        {
            return false;
        }

        let mut factor = ChonkerInt::from(5);
        let big_zero = ChonkerInt::new();
        let big_two = ChonkerInt::from(2);
        let big_three = ChonkerInt::from(3);
        let big_six = ChonkerInt::from(6);

        // Check if the target number is 2 or 3, which are primes.
        if (*self == big_two) || (*self == big_three) {
            return true;
        }

        // Check if the target is even, divisible by even numbers,
        // or if it is divisible by 3.
        if (self % &big_two == big_zero) || (self % &big_three == big_zero) {
            return false;
        }

        // Loop will cover odd values from 3 to sqrt(self)
        // Equality is allowed for the cases, such as 4 * 4 = 16,
        // when the target number may have a doubled factor.
        while (&factor * &factor) <= (*self) {
            // Check if the factor divides self without leaving a remainder.
            if (self % &factor == big_zero) || (self % &(&factor + &big_two) == big_zero) {
                // This means that self has an odd factor in between 3 and sqrt(self),
                // so it is not a prime number.
                return false;
            }

            factor = &factor + &big_six;
        }

        true
    }

    // Miller - Rabin primality test. Bottle-necked by the exponentiation when big primes are checked.
    // Running complexity is O(k log3n).
    // More information: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
    pub fn is_prime_probabilistic(&self, number_of_trials: Option<u64>) -> bool {
        // A number of tests to run.
        let number_of_trials = number_of_trials.unwrap_or(40);
        let target_original = (*self).clone();

        // Return false if the BigInt is negative, zero or one.
        if (*self == ChonkerInt::from(1))
            || (*self == ChonkerInt::new() || self.sign == BigIntSign::Zero)
            || (self.sign == BigIntSign::Negative)
        {
            return false;
        }

        let big_zero = ChonkerInt::new();
        let big_one = ChonkerInt::from(1);
        let big_two = ChonkerInt::from(2);
        let big_three = ChonkerInt::from(3);

        // Check if the target number is 2 or 3, which are primes.
        if (*self == big_two) || (*self == big_three) {
            return true;
        }

        // Check if the target is even, divisible by even numbers,
        // or if it is divisible by 3.
        if (self % &big_two == big_zero) || (self % &big_three == big_zero) {
            return false;
        }

        // 2^s * d + 1 = n , d - odd; d = (n - 1) / 2^s
        let target_one = &target_original - &big_one;
        let mut d = target_one.clone();
        let mut s = ChonkerInt::new();

        while &d % &big_two == big_one {
            d = &d / &big_two;
            s = &s + &big_one;
        }

        let mut base;
        let mut trial_result;

        // Testing loop/witness loop.
        'outer: for _iteration in 0..number_of_trials {
            // Generate a random base, a possible witness or a liar, from the range 2 - (self - 2)
            base = ChonkerInt::new_rand_range_value(
                &big_two,
                &(&target_original - &big_two),
                &BigIntSign::Positive,
            );

            trial_result = base.modpow(&d, &target_original);

            // Check the trial result, if it is equals 1 or (self - 1), proceed to the next trials,
            // otherwise continue the current one.
            if (trial_result == big_one) || (trial_result == target_one) {
                continue;
            }

            let mut s_clone = s.clone();

            // Increase the base, take a power of 2 of the base, while decreasing the s exponent by one
            // and take modulus of the original target.
            // If the calculation result equals (self - 1), proceed to the next trial,
            // otherwise the target is a composite number.
            while s_clone > big_zero {
                trial_result = trial_result.modpow(&big_two, &target_original);

                if trial_result == target_one {
                    continue 'outer;
                }

                s_clone = &s_clone - &big_one;
            }

            return false;
        }

        true
    }

    // Check if the target is a coprime BigInt to another target BigInt.
    pub fn is_coprime(&self, other: &ChonkerInt) -> bool {
        if self.gcd(other) != ChonkerInt::from(1) {
            return false;
        }

        true
    }

    // Check if this BigInt is a primitive root, works only with the prime numbers.
    // Returns true, when the number is a primitive root, false otherwise.
    // Time complexity is O(sqrt(N)), check of the even numbers is skipped.
    pub fn is_primitive_root(&self, prime: &ChonkerInt) -> bool {
        let big_one = ChonkerInt::from(1);

        // Check if the primitive root is negative or zero.
        if self.sign == BigIntSign::Negative || self.sign == BigIntSign::Zero {
            panic!("a positive primitive root is required for the primitive root validity check (ChonkerInt::is_primitive_root)");
        }

        // Check if the target is a prime number.
        if !prime.is_prime_probabilistic(Some(2)) {
            panic!("a prime number is required for the primitive root validity check (ChonkerInt::is_primitive_root)");
        }

        // Find value of Euler Totient function of n. Since n is a prime number, the value of Euler
        // Totient function is n-1 as there are n-1 relatively prime numbers.
        let target_one = prime - &big_one;

        // Calculate factors of the prime-1 and determine exponents for the testing
        // from (prime-1 / factor) calculation.
        let mut prime_factor_list: Vec<ChonkerInt> = target_one.prime_factor();
        prime_factor_list.dedup();

        // Check x^(p-1) = 1 (modulo p), if the result does not equal 1, return false.
        if self.modpow(&target_one, prime) != big_one {
            return false;
        }

        // Check x^((p-1)/f) (modulo p), if any factor results in 1, return false.
        for exponent in prime_factor_list.iter() {
            if self.modpow(&(&target_one / exponent), prime) == big_one {
                return false;
            }
        }

        true
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::{BigIntSign, ChonkerInt};

    // Test creation/construction of a random prime BigInt.
    #[test]
    fn test_random_prime_bigint_construction() {
        let requested_length: u64 = 12; // Reasonable limit, considering O(n) time complexity. Length: max 11-12.
                                        // let requested_zero_length: u64 = 0;

        let random_prime_bigint = ChonkerInt::new_prime(&requested_length);

        // let random_zero_bigint = ChonkerInt::new_rand(&requested_zero_length);

        assert_eq!(random_prime_bigint.sign, BigIntSign::Positive);
        assert_eq!(random_prime_bigint.digits.len(), requested_length as usize);
        assert!(random_prime_bigint.is_prime());
    }

    // Test creation/construction of a random coprime to the target BigInt.
    #[test]
    fn test_random_coprime_bigint_construction() {
        let requested_length: u64 = 12; // Reasonable limit, considering O(n) time complexity. Length: max 11-12.
        let random_prime_bigint = ChonkerInt::new_prime(&requested_length);
        let random_bigint = ChonkerInt::new_rand(&requested_length, &BigIntSign::Positive);
        // let zero_bigint = ChonkerInt::new();
        let negative_bigint = ChonkerInt::from(-10);

        let coprime_for_prime = random_prime_bigint.new_coprime();
        let coprime_for_random_bigint = random_bigint.new_coprime();
        // let coprime_for_zero = zero_bigint.new_coprime(); // Panics, as it should.
        let coprime_for_negative_bigint = negative_bigint.new_coprime();

        assert!(coprime_for_prime.is_coprime(&random_prime_bigint));
        assert!(coprime_for_random_bigint.is_coprime(&random_bigint));
        // assert!(coprime_for_zero.is_coprime(&zero_bigint));
        assert!(coprime_for_negative_bigint.is_coprime(&negative_bigint));
    }

    // Test creation/construction of a random primitive root to a prime BigInt.
    #[test]
    fn test_random_primitive_root_bigint_construction() {
        let bigint_prime = ChonkerInt::from(13);
        let primitive_root = bigint_prime.new_primitive_root();

        let primitive_root_comparison1 = ChonkerInt::from(2);
        let primitive_root_comparison2 = ChonkerInt::from(6);
        let primitive_root_comparison3 = ChonkerInt::from(7);
        let primitive_root_comparison4 = ChonkerInt::from(11);

        if primitive_root != primitive_root_comparison1
            && primitive_root != primitive_root_comparison2
            && primitive_root != primitive_root_comparison3
            && primitive_root != primitive_root_comparison4
        {
            panic!("did not calculate correct primitive roots");
        }

        let bigint_prime = ChonkerInt::new_prime(&10);
        let primitive_root = bigint_prime.new_primitive_root();

        println!("Random target prime: {}", bigint_prime);
        println!("Generated primitive root: {}", primitive_root);
    }

    // Test the methods checking the BigInt, if it is a prime number. Iterative and probabilistic approaches.
    #[test]
    fn test_bigint_is_prime() {
        let negative_bigint = ChonkerInt::from(-123534);
        let big_one = ChonkerInt::from(1);
        let big_zero = ChonkerInt::new();

        let bigint_not_prime1 = ChonkerInt::from(String::from(
            "4231689648728034761024109348723094713208529386505712",
        ));
        let bigint_not_prime2 = ChonkerInt::from(String::from("9231689641731777"));
        let bigint_not_prime3 = ChonkerInt::from(String::from("4920945105274017443"));
        let bigint_not_prime4 = ChonkerInt::from(String::from("1963760928849712729"));
        let bigint_not_prime5 = ChonkerInt::from(String::from(
            "91913571835595342720975337196553217435917295531",
        ));
        let bigint_not_prime6 = ChonkerInt::from(String::from("612805246882657062501053758885060201204312902577192906873202382957257140215187148278954238693446490196431299436910286231"));
        let small_bigint_not_prime = ChonkerInt::from(String::from("4230"));

        let bigint_prime1 = ChonkerInt::from(String::from("57885161"));
        let bigint_prime2 = ChonkerInt::from(String::from("7434295549380978012839955681932280624399018262337538072234694442121788312959451806126604174504220901"));
        let bigint_prime3 = ChonkerInt::from(String::from(
            "14043145053387290701740553217226309216528545163443",
        ));
        // let bigint_prime4 = ChonkerInt::from(String::from("87989249816948596463490931421050640230915364594808832915245649794766910416355601009344627352778928041250895432095896869913026898577325779505914260099267273891723698030806841435326256829587373811552723"));
        // let bigint_prime5 = ChonkerInt::from(String::from("562457128101735933004861281229980320017117423199759147390620125871795298609636011392770132344949088969751961333591438819671223807833285269283606992239"));
        let bigint_prime6 = ChonkerInt::from(String::from("1000000000000066600000000000001"));
        let bigint_prime7 = ChonkerInt::from(String::from("299572883"));
        let bigint_prime8 = ChonkerInt::from(String::from("1894964749"));
        let bigint_prime9 = ChonkerInt::from(String::from("69954509893")); // Length 11.
        let bigint_prime10 = ChonkerInt::from(String::from("855111008179")); // Length 12.
        let bigint_prime11 = ChonkerInt::from(String::from("78139"));
        let small_bigint_prime = ChonkerInt::from(String::from("7"));

        // Iterative approach, with O(sqrt(n) runtime complexity. Guaranteed outcome,
        // but takes significantly more time with big inputs.
        assert!(!negative_bigint.is_prime());
        assert!(!big_one.is_prime());
        assert!(!big_zero.is_prime());
        assert!(!bigint_not_prime1.is_prime());
        assert!(!bigint_not_prime2.is_prime());
        assert!(!small_bigint_not_prime.is_prime());

        assert!(bigint_prime1.is_prime());
        // assert!(bigint_prime2.is_prime());
        // assert!(bigint_prime3.is_prime());
        // assert!(bigint_prime4.is_prime());
        // assert!(bigint_prime5.is_prime());
        // assert!(bigint_prime6.is_prime());
        assert!(bigint_prime7.is_prime());
        // assert!(bigint_prime8.is_prime());
        // assert!(bigint_prime9.is_prime());
        // assert!(bigint_prime10.is_prime());
        assert!(small_bigint_prime.is_prime());

        // Probabilistic approach, with O(k log3n) runtime complexity.
        // If a composite value is detected - it is 100% composite,
        // if a prime value is assumed - it is not 100% guranteed.
        // Significantly faster, speed also depends on the amount of trials,
        // there are 40 by default.
        assert!(!negative_bigint.is_prime_probabilistic(None));
        assert!(!big_one.is_prime_probabilistic(None));
        assert!(!big_zero.is_prime_probabilistic(None));
        assert!(!bigint_not_prime1.is_prime_probabilistic(None));
        assert!(!bigint_not_prime2.is_prime_probabilistic(None));
        assert!(!bigint_not_prime3.is_prime_probabilistic(None));
        assert!(!bigint_not_prime4.is_prime_probabilistic(None));
        assert!(!bigint_not_prime5.is_prime_probabilistic(Some(2)));
        assert!(!bigint_not_prime6.is_prime_probabilistic(Some(1)));
        assert!(!small_bigint_not_prime.is_prime_probabilistic(None));

        assert!(bigint_prime1.is_prime_probabilistic(None));
        assert!(bigint_prime2.is_prime_probabilistic(Some(2)));
        assert!(bigint_prime3.is_prime_probabilistic(Some(2)));
        // assert!(bigint_prime4.is_prime_probabilistic(Some(2)));
        // assert!(bigint_prime5.is_prime_probabilistic(Some(2)));
        assert!(bigint_prime6.is_prime_probabilistic(Some(2)));
        assert!(bigint_prime7.is_prime_probabilistic(None));
        assert!(bigint_prime8.is_prime_probabilistic(None));
        assert!(bigint_prime9.is_prime_probabilistic(None));
        assert!(bigint_prime10.is_prime_probabilistic(None));
        assert!(bigint_prime11.is_prime_probabilistic(None));
        assert!(small_bigint_prime.is_prime_probabilistic(Some(2)));
    }

    // Test the method checking the BigInt, if it is a primitive root of a prime number.
    #[test]
    fn test_bigint_is_primitive_root() {
        let bigint_prime = ChonkerInt::from(13);
        let bigint_primitive_root1 = ChonkerInt::from(7);
        let bigint_primitive_root2 = ChonkerInt::from(2);
        let bigint_not_primitive_root1 = ChonkerInt::from(10);
        let bigint_not_primitive_root2 = ChonkerInt::from(12);

        let random_bigint_prime = ChonkerInt::new_prime(&10);
        let random_bigint_primitive_root = random_bigint_prime.new_primitive_root();

        assert!(bigint_primitive_root1.is_primitive_root(&bigint_prime));
        assert!(bigint_primitive_root2.is_primitive_root(&bigint_prime));
        assert!(!bigint_not_primitive_root1.is_primitive_root(&bigint_prime));
        assert!(!bigint_not_primitive_root2.is_primitive_root(&bigint_prime));
        assert!(random_bigint_primitive_root.is_primitive_root(&random_bigint_prime));
    }

    // Test the method checking the BigInt, if it is a coprime to another BigInt.
    #[test]
    fn test_bigint_is_coprime() {
        let bigint_prime = ChonkerInt::from(13);
        let bigint_coprime1 = ChonkerInt::from(7);
        let bigint_coprime2 = ChonkerInt::from(17);
        let bigint_negative_coprime3 = ChonkerInt::from(-17);
        let bigint_not_coprime1 = ChonkerInt::from(169);
        let bigint_not_coprime2 = ChonkerInt::from(0);
        let bigint_negative_not_coprime3 = ChonkerInt::from(-169);

        let bigint_not_prime = ChonkerInt::from(25684);
        let bigint_coprime3 = ChonkerInt::from(25673);
        let bigint_coprime4 = ChonkerInt::from(25683);
        let bigint_negative_coprime5 = ChonkerInt::from(-25683);
        let bigint_not_coprime3 = ChonkerInt::from(4);
        let bigint_not_coprime4 = ChonkerInt::from(42);
        let bigint_negative_not_coprime5 = ChonkerInt::from(-42);

        assert!(bigint_prime.is_coprime(&bigint_coprime1));
        assert!(bigint_prime.is_coprime(&bigint_coprime2));
        assert!(bigint_prime.is_coprime(&bigint_negative_coprime3));
        assert!(!bigint_prime.is_coprime(&bigint_not_coprime1));
        assert!(!bigint_prime.is_coprime(&bigint_not_coprime2));
        assert!(!bigint_prime.is_coprime(&bigint_negative_not_coprime3));

        assert!(bigint_not_prime.is_coprime(&bigint_coprime3));
        assert!(bigint_not_prime.is_coprime(&bigint_coprime4));
        assert!(bigint_not_prime.is_coprime(&bigint_negative_coprime5));
        assert!(!bigint_not_prime.is_coprime(&bigint_not_coprime3));
        assert!(!bigint_not_prime.is_coprime(&bigint_not_coprime4));
        assert!(!bigint_not_prime.is_coprime(&bigint_negative_not_coprime5));
    }
}
