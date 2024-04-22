use std::{thread::sleep, time::Duration};

fn main() {
    const UPPER_BOUND: u128 = 4_000_000;

    let res: u128 = e_math::generate::fibonacci()
        .take_while(|x| *x <= UPPER_BOUND)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("{}", res); //4613732

    println!("{:#?}", e_math::generate::PRIMES_64K.clone());
    println!();
    println!("{}", e_math::generate::PRIMES_64K.len());
    println!("{:?}", e_math::generate::PRIMES_64K.get(5000 - 1));

    let mut primes = e_math::generate::Primes::new();
    let iter = primes.iterator();

    iter.take(10).for_each(|p| println!("{}", p));

    sleep(Duration::from_secs(1));
}
