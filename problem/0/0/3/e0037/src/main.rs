use e_math::{num, primesg::Primes};

fn main() {
    let mut primes = Primes::<u64>::new();
    let mut collector: Vec<u64> = Vec::new();

    let mut numbers = 10u64..;

    while collector.len() < 11 {
        let n = numbers.next().unwrap();
        if test_truncate(&mut primes, n) {
            println!("{}", n);
            collector.push(n);
        }
    }

    println!("{}", collector.iter().sum::<u64>()); //748317
}

fn test_truncate(primes: &mut Primes<u64>, n: u64) -> bool {
    let digits = num::digits(&n).collect::<Vec<_>>();

    let mut is_prime = true;

    let mut chipper = digits.clone();
    chipper.reverse();

    while is_prime && chipper.len() > 0 {
        let un_digit = num::num_from_digits::<u64>(&chipper).unwrap();
        is_prime &= primes.is_prime(un_digit);
        chipper.remove(0);
    }

    let mut chipper = digits.clone();
    chipper.reverse();

    while is_prime && chipper.len() > 0 {
        let un_digit = num::num_from_digits::<u64>(&chipper).unwrap();
        is_prime &= primes.is_prime(un_digit);
        chipper.pop();
    }

    is_prime
}
