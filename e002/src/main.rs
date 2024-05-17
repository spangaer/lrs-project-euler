use std::{
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    const UPPER_BOUND: u128 = 4_000_000;

    let res: u128 = e_math::fibonacci()
        .take_while(|x| *x <= UPPER_BOUND)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("{}", res); //4613732

    {
        let start = Instant::now();

        // println!("{:#?}", e_math::generate::PRIMES_64K.clone());
        // println!();
        // println!("{}", e_math::generate::PRIMES_64K.len());
        // println!("{:?}", e_math::generate::PRIMES_64K.get(5000 - 1));

        let mut primes = e_math::primes::Primes::<u64>::new();
        // let iter = primes.iterator();

        // iter.take(10).for_each(|p| println!("{}", p));

        // cargo run --release
        // 25000000th prime is 472882027
        // duration: 61233
        // let n: usize = 25_000_000;

        let n: usize = 1_000_000;
        println!("{}th prime is {}", n, primes.nth(n));

        let end = Instant::now();

        println!("duration: {}", (end - start).as_millis());
    }

    sleep(Duration::from_secs(1));
}
