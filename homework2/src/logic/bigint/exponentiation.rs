// BigInt module regarding (modular) exponentiation of BigInts.

use crate::logic::bigint::ChonkerInt;

// Implement conversion methods for BigInt.
impl ChonkerInt {
    // Implement exponentiation operation.
    // It is accomplished with the use of exponentiation by squaring algorithm, in an iterative form.
    // More about the idea: https://en.wikipedia.org/wiki/Exponentiation_by_squaring
    pub fn pow(&self, power: &ChonkerInt) -> ChonkerInt {
        let mut base = (*self).clone();
        let mut power = (*power).clone();
        let zero_bigint = ChonkerInt::new();
        let big_one = ChonkerInt::from(1);
        let big_two = ChonkerInt::from(2);

        // If the base is zero, return zero.
        if *self == zero_bigint {
            return zero_bigint;
        }

        let mut result = ChonkerInt::from(1);

        // Check if the power is zero, one, positive or negative and take according action.
        if power == zero_bigint {
            return big_one;
        } else if power == big_one {
            return (*self).clone();
        } else if power > zero_bigint {
            while power > zero_bigint {
                // If the power is odd, split it in half and multiply base by itself.
                if &power % &big_two == big_one {
                    result = &result * &base;
                }

                base = &base * &base;
                power = &power / &big_two;
            }
        } else if power < zero_bigint {
            return zero_bigint;
        }

        result
    }

    // Implement modular exponentiation with Right-to-left binary which includes memory efficient method.
    pub fn modpow(&self, power: &ChonkerInt, modulus: &ChonkerInt) -> ChonkerInt {
        let mut base = (*self).clone();
        let mut power = (*power).clone();
        let modulus = (*modulus).clone();

        let zero_bigint = ChonkerInt::new();
        let big_one = ChonkerInt::from(1);
        let big_two = ChonkerInt::from(2);

        // If the base is zero, return zero.
        if *self == zero_bigint {
            return zero_bigint;
        }

        let mut result = ChonkerInt::from(1);
        base = &base % &modulus;

        // Check if the power is zero, one, positive or negative and take according action.
        if power == zero_bigint {
            return big_one;
        } else if power == big_one {
            return (*self).clone();
        } else if power > zero_bigint {
            loop {
                if (&power % &big_two) == big_one {
                    result = &result * &base;
                    result = &result % &modulus;
                }

                if power == big_one {
                    return result;
                }

                power = &power / &big_two;
                base = &base * &base;
                base = &base % &modulus;
            }
        } else if power < zero_bigint {
            return zero_bigint;
        }

        result
    }
}

// Test module.
#[cfg(test)]
mod tests {
    use crate::logic::bigint::ChonkerInt;

    // Test BigInt's power operation.
    #[test]
    fn test_bigint_pow_operation() {
        // Positive BigInts.
        let positive_bigint = ChonkerInt::from(String::from("13"));

        // Negative BigInts.
        let negative_bigint = ChonkerInt::from(String::from("-8"));

        // Very big BigInts. If you take more, result will be very long and
        // calculations will take a long time even with exponentiation by squaring.
        let positive_very_big_bigint = ChonkerInt::from(String::from("714"));
        let negative_very_big_bigint = ChonkerInt::from(String::from("-831"));

        // Empty/zero BigInt.
        let zero_bigint = ChonkerInt::from(String::from("0"));

        // Check exponentiation of positive BigInt.
        let positive_into_positive_exponentiation_result =
            ChonkerInt::from(String::from("302875106592253"));
        assert_eq!(
            positive_bigint.pow(&positive_bigint),
            positive_into_positive_exponentiation_result
        );

        // Check negative exponentiation of negative BigInts.
        let negative_into_negative_exponentiation_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            negative_bigint.pow(&negative_bigint),
            negative_into_negative_exponentiation_result
        );

        // Check mixed exponentiation.
        let positive_into_negative_exponentiation_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            positive_bigint.pow(&negative_bigint),
            positive_into_negative_exponentiation_result
        );
        let negative_into_positive_exponentiation_result =
            ChonkerInt::from(String::from("-549755813888"));
        assert_eq!(
            negative_bigint.pow(&positive_bigint),
            negative_into_positive_exponentiation_result
        );

        // Check zero exponentiation.
        let positive_into_zero_exponentiation_result = ChonkerInt::from(String::from("1"));
        assert_eq!(
            positive_bigint.pow(&zero_bigint),
            positive_into_zero_exponentiation_result
        );
        let negative_into_zero_exponentiation_result = ChonkerInt::from(String::from("1"));
        assert_eq!(
            negative_bigint.pow(&zero_bigint),
            negative_into_zero_exponentiation_result
        );

        // Check exponentiation of zero BigInt.
        let zero_exponentiation_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            zero_bigint.pow(&positive_bigint),
            zero_exponentiation_result
        );
        assert_eq!(
            zero_bigint.pow(&negative_bigint),
            zero_exponentiation_result
        );

        // Check very big BigInt exponentiation.
        let exponentiation_by_big_positive_result = ChonkerInt::from(String::from("2267532639227975293484649359822622477267689237579697617906192701053898488585679615971899020889703769424853593769941894099705006188147791222757056131368726677466232928799681558371072954085332934432778999746503487442415093710752191893125909910080172104071186196202318808905629441412179120109245122253741155406625147838162586256820577038206529709540777905603717397005471625810253776105763148136621464827535515291812707058866519618646290545419919167807400141040627830812565548535385001713064127210523318363986953541795681288429435802412989826355065278574119348352219099460207465524775197105990445932679942909728731805572486726083245008659975610583638158014896578631266675555540161864816514746194270973162359461556173014105269004989142077146806003668299107697188717385561871063129881589233956646361289"));
        assert_eq!(
            positive_bigint.pow(&positive_very_big_bigint),
            exponentiation_by_big_positive_result
        );
        assert_eq!(
            positive_very_big_bigint.pow(&negative_very_big_bigint),
            zero_bigint
        );
        let big_negative_exponentiation_by_big_positive_result = ChonkerInt::from(String::from("393668786826267553077076474140299370880918568633563724842666026887776380400692507101958767814164298006252398130266405011634320074245315113033300793421272121694777985497897167256840060270989737120689785782511428409896396048195430068938559250888431303594273773458951383706995409676453086456521994456295614684097413271167286341695248290067965403698108928941471664208182091242033334793930243716022436130436364911194491778238188273770470802023623079742059009766543879957296830437165202578267998722610234076548932922293704169105785971273074051301109900762041708423339290740902499110645226372903295714474590337870063400159212122982099331841277519357737923455372909102252600266400418575947091730422665647428116914539158466916493885084085612950657386804563036551513189590436669087936432673802292033752746111866688630990125757428824046174459252601860997315161352243407845749473077838394751025949100852484995533549035660758414837703125558423697084106243864495746715761436733376868148766657885493971607216332569267922294404814980854030044970158555648288715014317812616390609047193586114161512702965856387218837858166052100553898789139804660797970747108950883914128113314021672344327825955132390747948842969862107736457509838732942623385512250976704094261050980772256073544253081541031345079031032364601997324753773880108846682540347154574904882318554809375741408296121927147437160030649395305425180669121870859453237154694425736638073587779704911653563977141631560940520786057038882281455819435104785227726034385354135947100172064790387713080298235953549473155071842855112924615851579027373517736475699137515166693637491228088704452538669947537597739225539365158053926355837721785585790095248003005310035036534010451112841933386560090762859855053853912069020851284005299437700326199339888487499713149192634147472202811307482794085250467256565249587688716142208770285689499321039876392577357106678155028658660245414611378631571628503673365981639037164007646301143945969330141469137765067899977984941827961563471054094969640836506834793455965606891806616235299067618810916324150922380957399772315521"));
        assert_eq!(
            negative_very_big_bigint.pow(&positive_very_big_bigint),
            big_negative_exponentiation_by_big_positive_result
        );
        assert_eq!(
            negative_very_big_bigint.pow(&negative_very_big_bigint),
            zero_bigint
        );
    }

    // Test BigInt's modular exponentiation operation.
    #[test]
    fn test_bigint_modpow_operation() {
        // Positive BigInts.
        let positive_bigint = ChonkerInt::from(String::from("13786234"));
        let bigint_modulus = ChonkerInt::from(String::from("45"));
        let bigint_negative_modulus = ChonkerInt::from(String::from("-45"));

        // Negative BigInts.
        let negative_bigint = ChonkerInt::from(String::from("-8"));

        // Empty/zero BigInt.
        let zero_bigint = ChonkerInt::from(String::from("0"));

        // Check exponentiation of positive BigInt.
        let positive_into_positive_exponentiation_result = ChonkerInt::from(String::from("16"));
        assert_eq!(
            positive_bigint.modpow(&positive_bigint, &bigint_modulus),
            positive_into_positive_exponentiation_result
        );

        // Check exponentiation of positive BigInt with negative BigInt.
        let positive_into_negative_exponentiation_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            positive_bigint.modpow(&negative_bigint, &bigint_modulus),
            positive_into_negative_exponentiation_result
        );

        // Check exponentiation of positive BigInt with zero BigInt.
        let positive_into_zero_exponentiation_result = ChonkerInt::from(String::from("1"));
        assert_eq!(
            positive_bigint.modpow(&zero_bigint, &bigint_modulus),
            positive_into_zero_exponentiation_result
        );

        // Check exponentiation of negative BigInt with positive BigInt.
        let negative_into_positive_exponentiation_result = ChonkerInt::from(String::from("19"));
        assert_eq!(
            negative_bigint.modpow(&positive_bigint, &bigint_modulus),
            negative_into_positive_exponentiation_result
        );

        // Check exponentiation of negative BigInt with negative BigInt.
        let negative_into_negative_exponentiation_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            negative_bigint.modpow(&negative_bigint, &bigint_modulus),
            negative_into_negative_exponentiation_result
        );

        // Check exponentiation of negative BigInt with zero BigInt.
        let negative_into_zero_exponentiation_result = ChonkerInt::from(String::from("1"));
        assert_eq!(
            negative_bigint.modpow(&zero_bigint, &bigint_modulus),
            negative_into_zero_exponentiation_result
        );

        // Repeat the same tests with negative modulus.

        // Check exponentiation of positive BigInt.
        // let positive_into_positive_exponentiation_result = ChonkerInt::from(String::from("16"));
        // assert_eq!(
        //     positive_bigint.modpow(&positive_bigint, &bigint_negative_modulus),
        //     positive_into_positive_exponentiation_result
        // );

        // Check exponentiation of positive BigInt with negative BigInt.
        let positive_into_negative_exponentiation_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            positive_bigint.modpow(&negative_bigint, &bigint_negative_modulus),
            positive_into_negative_exponentiation_result
        );

        // Check exponentiation of positive BigInt with zero BigInt.
        let positive_into_zero_exponentiation_result = ChonkerInt::from(String::from("1"));
        assert_eq!(
            positive_bigint.modpow(&zero_bigint, &bigint_negative_modulus),
            positive_into_zero_exponentiation_result
        );

        // Check exponentiation of negative BigInt with positive BigInt.
        // let negative_into_positive_exponentiation_result = ChonkerInt::from(String::from("19"));
        // assert_eq!(
        //     negative_bigint.modpow(&positive_bigint, &bigint_negative_modulus),
        //     negative_into_positive_exponentiation_result
        // );

        // Check exponentiation of negative BigInt with negative BigInt.
        let negative_into_negative_exponentiation_result = ChonkerInt::from(String::from("0"));
        assert_eq!(
            negative_bigint.modpow(&negative_bigint, &bigint_negative_modulus),
            negative_into_negative_exponentiation_result
        );

        // Check exponentiation of negative BigInt with zero BigInt.
        let negative_into_zero_exponentiation_result = ChonkerInt::from(String::from("1"));
        assert_eq!(
            negative_bigint.modpow(&zero_bigint, &bigint_negative_modulus),
            negative_into_zero_exponentiation_result
        );
    }
}
