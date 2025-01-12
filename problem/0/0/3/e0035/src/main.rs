use std::vec;

use e_math::num::{digits, num_from_digits};
use e_math::primesg::Primes;

fn main() {
    let mut primes = Primes::<u32>::new();

    let prilion = primes
        .iterator()
        .take_while(|p| *p < 1_000_000_u32)
        .collect::<Vec<_>>();

    let circulars = prilion
        .iter()
        .filter(|p| {
            let mut pigits = digits(*p).collect::<Vec<_>>();
            pigits.reverse(); // iterates most significant digit first

            let len = pigits.len();
            let mut buffer = vec![0_u8; len];

            (0..len).all(|start| {
                (0..len).for_each(|select| {
                    buffer[select] = pigits[(select + start) % len];
                });

                let perm_number = num_from_digits::<u32>(&buffer).unwrap();

                primes.is_prime(perm_number)
            })
        })
        .collect::<Vec<_>>();

    println!("circulars {:?}", circulars);
    println!("count {}", circulars.len()); // 55
}
