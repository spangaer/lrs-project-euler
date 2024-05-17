use e_math::primes::Primes;

fn main() {
    let mut primes = Primes::<usize>::new();

    let res = primes.nth(10001);

    println!("{}", res); // 104743
}
