use e_math::primesg::Primes;

fn main() {
    let mut primes = Primes::<u64>::new();

    let res = primes.nth(10001);

    println!("{}", res); // 104743
}
