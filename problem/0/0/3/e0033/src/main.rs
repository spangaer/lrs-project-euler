use e_math::{fraction::Fraction, num::num_from_digits, primesg::Primes};

use itertools::Itertools;

fn main() {
    let mut _primes = Primes::<u64>::new();
    let primes = &mut _primes;

    println!("gcd {}", primes.gcd(&[55, 99]));

    let frac_in = Fraction::<u64>::new(55, 99);

    println!("{} -> {}", frac_in, frac_in.simplify(primes));

    let res = (1_u8..=9)
        .permutations(2)
        .flat_map(|num_d: Vec<u8>| {
            let num = num_from_digits::<u64>(&num_d).unwrap();
            (1_u8..=9)
                .permutations(2)
                .map(move |denom_d| (num_d.clone(), num, denom_d))
        })
        .filter_map(
            |(num_d, num, denom_d)| match denom_d.iter().find(|d| num_d.contains(d)) {
                None => None,
                Some(common_d) => {
                    let denom = num_from_digits::<u64>(&denom_d).unwrap();

                    if num >= denom {
                        return None;
                    }

                    let out = Fraction::new(num.clone(), denom);

                    let test = {
                        let num_d = num_d
                            .iter()
                            .filter(|&d| d != common_d)
                            .cloned()
                            .collect::<Vec<_>>();
                        let num = num_from_digits::<u64>(&num_d).unwrap();

                        let denom_d = denom_d
                            .iter()
                            .filter(|&d| d != common_d)
                            .cloned()
                            .collect::<Vec<_>>();

                        let denom = num_from_digits::<u64>(&denom_d).unwrap();

                        Fraction::new(num, denom)
                    };

                    let simple = out.simplify(primes);

                    if simple == test.simplify(primes) {
                        Some((out, test, simple))
                    } else {
                        None
                    }
                }
            },
        )
        .map(|tup| {
            println!("{tup:?}");
            tup.0
        })
        .fold(Fraction::new(1_u64, 1), |acc, next| acc * next)
        .simplify(primes);

    println!("{res}"); // 1/100
}
