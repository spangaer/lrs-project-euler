use std::sync::atomic::{fence, Ordering};
use std::time::Instant;

use e_math::primesg::Primes;
use humantime::format_duration;

const TOP: u64 = 28123;

fn main() {
    let start = Instant::now();
    fence(Ordering::AcqRel);

    let mut primes = Primes::<u64>::new();

    let abundant = |&n: &u64| {
        let divisors = primes.divisors(n);
        let sum = divisors[0..divisors.len() - 1].iter().sum::<u64>();
        sum > n
    };

    let abundants: Vec<u64> = (12..=TOP).filter(abundant).collect();

    println!(
        "{} {} {} {}",
        abundants[0],
        abundants[1],
        abundants.last().unwrap(),
        abundants.len()
    );

    let non_a_sum: Vec<u64> = (1..=TOP)
        .filter(|&n| {
            !abundants.iter().take_while(|&&a1| a1 < n).any(|&a1| {
                let a2 = n - a1;

                abundants.iter().take_while(|&&x| x <= a2).any(|&x| x == a2)
            })
        })
        .collect();

    println!(
        "{} {} {} {}",
        non_a_sum[0],
        non_a_sum[1],
        non_a_sum.last().unwrap(),
        non_a_sum.len()
    );

    println!("{}", non_a_sum.iter().sum::<u64>()); //4179871

    // release mode: 10s 626ms
    // debug mode: 7m 22s 270ms

    fence(Ordering::AcqRel); // prevent compiler tricks
    println!("{}", format_duration(start.elapsed()));
}
