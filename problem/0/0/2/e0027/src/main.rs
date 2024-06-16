use std::iter;

use e_math::primesg::Primes;

fn main() {
    let mut primes = Primes::<u64>::new();

    println!("{}", quad_prime_len(&mut primes, 1, 41));
    println!("{}", quad_prime_len(&mut primes, -79, 1601));

    let (a, b, len) = (-999..=999_i64)
        .flat_map(|a| (-1000..=1000_i64).map(move |b| (a, b)))
        .map(|(a, b)| (a, b, quad_prime_len(&mut primes, a, b)))
        .max_by_key(|&(_, _, len)| len)
        .unwrap();

    // -61  971  71  -59231
    println!("{}  {}  {}  {}", a, b, len, a * b);
}

fn quad_prime_len(primes: &mut Primes<u64>, a: i64, b: i64) -> usize {
    let q = |n| n * n + a * n + b;

    iter::successors(Some(0_i64), |&i| Some(i + 1))
        .map(q)
        .take_while(|&p| p > 1 && primes.is_prime(p as u64))
        .count()
}
