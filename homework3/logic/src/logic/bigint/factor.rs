// BigInt module regarding factorisation of BigInts.

use crate::logic::bigint::{BigIntSign, ChonkerInt};

// Implement methods factoring the BigInt, time complexity is O(sqrt(n)).
impl ChonkerInt {
    // Generate a vector of all factors for the target BigInt.
    pub fn factor(&self) -> Vec<ChonkerInt> {
        let mut absolute_target = (*self).clone();
        absolute_target.set_positive_sign();
        let big_zero = ChonkerInt::new();
        let big_one = ChonkerInt::from(1);
        let big_negative_one = ChonkerInt::from(-1);
        let big_two = ChonkerInt::from(2);

        // Check for zero and one targets.
        if (*self) == big_zero {
            return vec![];
        } else if (*self) == big_one {
            return vec![big_one];
        } else if (*self) == big_negative_one {
            return vec![big_negative_one];
        }

        // Check if the target is a prime number.
        if self.is_prime_probabilistic(Some(2)) {
            return vec![];
        }

        let mut factor_list: Vec<ChonkerInt> = vec![];
        let mut factor_candidate;
        let mut factor_other;
        let iterator;

        factor_list.push(big_one);
        // Check if the target is even. And set appropriate values for variables used later.
        if (self % &big_two) == big_zero {
            factor_list.push(big_two.clone());
            factor_list.push(self / &big_two);
            factor_list.push((*self).clone());

            factor_candidate = ChonkerInt::from(3);
            iterator = ChonkerInt::from(1);
        } else {
            factor_list.push((*self).clone());

            factor_candidate = ChonkerInt::from(3);
            iterator = ChonkerInt::from(2);
        }

        // Loop from 1 or 3 to sqrt(n).
        while (factor_candidate.pow(&big_two)) <= absolute_target {
            if (self % &factor_candidate) == big_zero {
                factor_list.push(factor_candidate.clone());

                // Add another factor of the pair,
                // consider cases, when both factors are the same.
                factor_other = self / &factor_candidate;
                if factor_other != factor_candidate {
                    factor_list.push(factor_other.clone());
                }
            }

            factor_candidate = &factor_candidate + &iterator;
        }

        // Check if the target was negative, double the amount of factors,
        // create negated copies and append them to the main list.
        if (*self.get_sign()) == BigIntSign::Negative {
            let mut negated_factor_list: Vec<ChonkerInt> =
                factor_list.clone().iter().map(|factor| -factor).collect();
            factor_list.append(&mut negated_factor_list);
        }

        // Sort the vector of factors. Worst case O(n * log(n))
        factor_list.sort();
        factor_list
    }

    // Generate a vector of all prime factors for the target BigInt.
    pub fn prime_factor(&self) -> Vec<ChonkerInt> {
        // Check if the target is negative, if so - return an empty vector.
        if self.sign == BigIntSign::Negative {
            return vec![];
        }

        let mut target = (*self).clone();
        let big_zero = ChonkerInt::new();
        let big_one = ChonkerInt::from(1);
        let big_negative_one = ChonkerInt::from(-1);
        let big_two = ChonkerInt::from(2);

        // Check for zero and one targets.
        if (*self) == big_zero {
            return vec![];
        } else if (*self) == big_one {
            return vec![big_one];
        } else if (*self) == big_negative_one {
            return vec![big_negative_one];
        }

        // Check if the target is a prime number.
        if self.is_prime_probabilistic(Some(2)) {
            return vec![];
        }

        let mut factor_list: Vec<ChonkerInt> = vec![];
        let mut factor_candidate = ChonkerInt::from(3);

        // Add factors of 2 until the target becomes odd.
        while (&target % &big_two) == big_zero {
            factor_list.push(big_two.clone());
            target = &target / &big_two;
        }

        // Loop 3 to sqrt(n).
        while (factor_candidate.pow(&big_two)) <= target {
            while (&target % &factor_candidate) == big_zero {
                factor_list.push(factor_candidate.clone());
                target = &target / &factor_candidate;
            }

            factor_candidate = &factor_candidate + &big_two;
        }

        // This condition is to handle the case when remaining number/factor
        // is a prime number greater than 2
        if target > big_two {
            factor_list.push(target);
        }

        factor_list
    }

    // Generate a vector of all factors for the target BigInt.
    pub fn factor_rsa_modulus(&self, iteration_start_point: &ChonkerInt) -> Vec<ChonkerInt> {
        let mut absolute_target = (*self).clone();
        absolute_target.set_positive_sign();
        let mut factor_candidate = (*iteration_start_point).clone();

        let big_zero = ChonkerInt::new();
        let big_one = ChonkerInt::from(1);
        let big_two = ChonkerInt::from(2);

        // Check for the provided starting point for factor candidate calculation, for zero or being negative.
        if factor_candidate == big_zero || iteration_start_point.sign == BigIntSign::Negative {
            panic!("the provided starting point for factor candidate calculation, for the factoring of the RSA modulus is incorrect. The candidate should be a positive number. (factor_rsa_modulus)");
        }

        // Check for zero, one and two targets.
        // Check if the target is a prime number or negative.
        if ((*self) == big_zero)
            || ((*self) == big_one)
            || ((*self) == big_two)
            || self.is_prime_probabilistic(Some(2))
        {
            panic!("the provided target for factorisation, for the factoring of the RSA modulus is incorrect. The target should be a positive composite number. (factor_rsa_modulus)");
        }

        let mut factor_list: Vec<ChonkerInt> = vec![];
        let mut factor_other;

        // Check if the target is even, if it is,
        // check if the second is a prime number as well, if both are prime,
        // return teh vector with the pair of values.
        if (self % &big_two) == big_zero {
            let second_factor = self / &big_two;

            if second_factor.is_prime_probabilistic(Some(2)) {
                factor_list.push(big_two.clone());
                factor_list.push(second_factor);

                // Sort the vector of factors. Worst case O(n * log(n))
                factor_list.sort();
                return factor_list;
            }
        }

        // Check if the starting point is even, if it is, make it odd.
        if (iteration_start_point % &big_two) == big_zero {
            factor_candidate = &factor_candidate + &big_one;
        }

        // Loop from the requested start to the sqrt(n).
        while (factor_candidate.pow(&big_two)) <= absolute_target {
            // Check if the candidate factor is a prime value, if it is not,
            // continue to the next iteration.
            if !factor_candidate.is_prime_probabilistic(Some(1)) {
                factor_candidate = &factor_candidate + &big_two;
                continue;
            }

            if (self % &factor_candidate) == big_zero {
                factor_list.push(factor_candidate.clone());

                // Add another factor of the pair.
                factor_other = self / &factor_candidate;

                // Check if the calculated factor is composite, if so, panic.
                if !factor_other.is_prime_probabilistic(Some(2)) {
                    panic!("the generated factor of the target is a composite number, thus the received RSA modulus was incorrect. Correct RSA modulus is a produce of two prime numbers. (factor_rsa_modulus)");
                }

                factor_list.push(factor_other.clone());
                break;
            }

            factor_candidate = &factor_candidate + &big_two;
        }

        // Sort the vector of factors. Worst case O(n * log(n))
        factor_list.sort();
        factor_list
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::ChonkerInt;

    // Test a factorisation of a composite BigInt.
    #[test]
    fn test_bigint_factorisation() {
        let bigint_candidate1 = ChonkerInt::from(String::from("14531"));
        let bigint_candidate2 = ChonkerInt::from(String::from("78614524")); // Length 8, quite fast.
        let bigint_candidate3 = ChonkerInt::from(String::from("7935467891")); // Length 10, takes some time, a reasonable limit.
                                                                              // let bigint_candidate4 = ChonkerInt::from(String::from("7935467894234"));
        let bigint_candidate5 = ChonkerInt::from(String::from("-14531"));

        let bigint_zero = ChonkerInt::new();
        let bigint_one = ChonkerInt::from(1);
        let bigint_prime = ChonkerInt::new_prime(&10);

        // Calculate vectors with factors of the target BigInts.
        let bigint_candidate1_factors = bigint_candidate1.factor();
        let bigint_candidate2_factors = bigint_candidate2.factor();
        let bigint_candidate3_factors = bigint_candidate3.factor();
        let bigint_candidate5_factors = bigint_candidate5.factor();

        let bigint_zero_factors = bigint_zero.factor();
        let bigint_one_factors = bigint_one.factor();
        let bigint_prime_factors = bigint_prime.factor();

        assert_eq!(
            bigint_candidate1_factors,
            vec![
                ChonkerInt::from(1),
                ChonkerInt::from(11),
                ChonkerInt::from(1321),
                ChonkerInt::from(14531),
            ]
        );
        assert_eq!(
            bigint_candidate2_factors,
            vec![
                ChonkerInt::from(1),
                ChonkerInt::from(2),
                ChonkerInt::from(4),
                ChonkerInt::from(127),
                ChonkerInt::from(254),
                ChonkerInt::from(508),
                ChonkerInt::from(154753),
                ChonkerInt::from(309506),
                ChonkerInt::from(619012),
                ChonkerInt::from(19653631),
                ChonkerInt::from(39307262),
                ChonkerInt::from(78614524),
            ]
        );
        assert_eq!(
            bigint_candidate3_factors,
            vec![
                ChonkerInt::from(1),
                ChonkerInt::from(13),
                ChonkerInt::from(2311),
                ChonkerInt::from(30043),
                ChonkerInt::from(264137),
                ChonkerInt::from(3433781),
                ChonkerInt::from(610420607),
                ChonkerInt::from(7935467891u64),
            ]
        );
        assert_eq!(
            bigint_candidate5_factors,
            vec![
                ChonkerInt::from(-14531),
                ChonkerInt::from(-1321),
                ChonkerInt::from(-11),
                ChonkerInt::from(-1),
                ChonkerInt::from(1),
                ChonkerInt::from(11),
                ChonkerInt::from(1321),
                ChonkerInt::from(14531),
            ]
        );

        assert!(bigint_zero_factors.is_empty());
        assert_eq!(bigint_one_factors, vec![ChonkerInt::from(1)]);
        assert!(bigint_prime_factors.is_empty());
    }

    // Test a prime factorisation of a composite BigInt.
    #[test]
    fn test_bigint_prime_factorisation() {
        let bigint_candidate1 = ChonkerInt::from(String::from("14531"));
        let bigint_candidate2 = ChonkerInt::from(String::from("78614524")); // Length 8, quite fast.
        let bigint_candidate3 = ChonkerInt::from(String::from("7935467891")); // Length 10, takes some time, a reasonable limit.
                                                                              // let bigint_candidate4 = ChonkerInt::from(String::from("7935467894234"));
        let bigint_candidate5 = ChonkerInt::from(String::from("-14531"));

        let bigint_zero = ChonkerInt::new();
        let bigint_one = ChonkerInt::from(1);
        let bigint_prime = ChonkerInt::new_prime(&10);

        // Calculate vectors with factors of the target BigInts.
        let bigint_candidate1_factors = bigint_candidate1.prime_factor();
        let bigint_candidate2_factors = bigint_candidate2.prime_factor();
        let bigint_candidate3_factors = bigint_candidate3.prime_factor();
        let bigint_candidate5_factors = bigint_candidate5.prime_factor();

        let bigint_zero_factors = bigint_zero.prime_factor();
        let bigint_one_factors = bigint_one.prime_factor();
        let bigint_prime_factors = bigint_prime.prime_factor();

        assert_eq!(
            bigint_candidate1_factors,
            vec![ChonkerInt::from(11), ChonkerInt::from(1321),]
        );
        assert_eq!(
            bigint_candidate2_factors,
            vec![
                ChonkerInt::from(2),
                ChonkerInt::from(2),
                ChonkerInt::from(127),
                ChonkerInt::from(154753),
            ]
        );
        assert_eq!(
            bigint_candidate3_factors,
            vec![
                ChonkerInt::from(13),
                ChonkerInt::from(2311),
                ChonkerInt::from(264137),
            ]
        );
        assert_eq!(bigint_candidate5_factors, vec![]);

        assert!(bigint_zero_factors.is_empty());
        assert_eq!(bigint_one_factors, vec![ChonkerInt::from(1)]);
        assert!(bigint_prime_factors.is_empty());
    }

    // Test a factorisation of an RSA modulus.
    #[test]
    fn test_bigint_rsa_modulus_factorisation() {
        // Took 0.2 s, length 5, primes: 47, 643
        // Randomly generated RSA public/private key modulus n: 30221
        // Randomly generated RSA public key exponent e: 3589
        // Randomly generated RSA private key exponent d: 2485

        // Took ~14 s, length 8, primes: 1613, 36037
        // Randomly generated RSA public/private key modulus n: 58127681
        // Randomly generated RSA public key exponent e: 3373
        // Randomly generated RSA private key exponent d: 23904229

        // Took ~10 s, length 10, primes: 19441, 95027
        // Randomly generated RSA public/private key modulus n: 1847419907
        // Randomly generated RSA public key exponent e: 13807417
        // Randomly generated RSA private key exponent d: 811358953

        // Took ~1 min, length 11, primes: 19441, 95027
        // Randomly generated RSA public/private key modulus n: 84524006669
        // Randomly generated RSA public key exponent e: 7
        // Randomly generated RSA private key exponent d: 36224324419

        // Randomly generated RSA public/private key modulus n: 3934282079717923720147924446353532421128856963
        // Randomly generated RSA public key exponent e: 4106747
        // Randomly generated RSA private key exponent d: 1831756323838058424537676474418754902473565363

        let target_modulus1 = ChonkerInt::from(30221);
        let target_modulus2 = ChonkerInt::from(58127681);
        let target_modulus3 = ChonkerInt::from(1847419907);
        // let target_modulus4 = ChonkerInt::from(84524006669u64);
        // let target_modulus5 = ChonkerInt::from(String::from("3934282079717923720147924446353532421128856963"));

        // Panics with zero and negative values.
        // let iteration_start_point1 = ChonkerInt::new();
        // let iteration_start_point2 = ChonkerInt::from(-1);

        let iteration_start_point = ChonkerInt::from(1);

        let factor_list1 = target_modulus1.factor_rsa_modulus(&iteration_start_point);
        let factor_list2 = target_modulus2.factor_rsa_modulus(&iteration_start_point);
        let factor_list3 = target_modulus3.factor_rsa_modulus(&iteration_start_point);
        // let factor_list4 = target_modulus4.factor_rsa_modulus(&iteration_start_point);

        assert_eq!(
            factor_list1,
            vec![ChonkerInt::from(47), ChonkerInt::from(643),]
        );
        assert_eq!(
            factor_list2,
            vec![ChonkerInt::from(1613), ChonkerInt::from(36037),]
        );
        assert_eq!(
            factor_list3,
            vec![ChonkerInt::from(19441), ChonkerInt::from(95027),]
        );
        // assert_eq!(
        //     factor_list3,
        //     vec![
        //         ChonkerInt::from(270163),
        //         ChonkerInt::from(312863),
        //     ]
        // );
    }
}
