fn main() {
    let mut primes = e_math::generate::Primes::<u64>::new();

    let start: u64 = 600_851_475_143;

    let res = primes.factorize(start);

    println!("{:?}", res); //6857
}
