use e_math::primes::Primes;

fn main() {
    let mut primes = Primes::<u64>::new();

    let res = primes
        .iterator()
        .take_while(|&p| p < 2_000_000_u64)
        .sum::<u64>();

    println!("{}", res); // 142913828922
}
