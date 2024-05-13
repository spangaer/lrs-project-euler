use std::iter;

use e_math::generate::Primes;

fn main() {
    let mut i = 0_u64;

    let mut primes = Primes::<u64>::new();

    let mut factors = |x: u64| {
        // for each prime factor, we can have a power of 0 to n
        // for all powers 0 we get factor 1
        // for all powers n we get x
        // any combination of powers in between gives another divisor
        // so the possible combinations is product of (n_i + 1)
        // +1 for the zero option

        primes
            .factorize(x)
            .iter()
            .map(|fact| fact.1 + 1)
            .product::<u32>()
    };

    let triangle = iter::from_fn(|| {
        i += 1;
        Some(i)
    })
    .map(|x| (1..=x).sum::<u64>());

    // triangle
    //     .take_while(|&t| t < 60)
    //     .for_each(|t| println!("{}:  {:?}", t, primes.factorize(t)));

    let res = triangle
        .skip_while(|&t| {
            let f = factors(t);
            println!("{}: {}", t, f);
            f <= 500
        })
        .next()
        .unwrap();

    println!("{}", res); // 76576500: 576
}
