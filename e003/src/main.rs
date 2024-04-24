fn main() {
    let mut primes = e_math::generate::Primes::<u64>::new();

    let start: u64 = 600_851_475_143;

    let iter = primes.iterator();

    let res = iter
        .scan(start, |current, prime| match current {
            current if *current == prime => None, // current state is a prime
            current if *current % prime == 0 => {
                // current state was prime dividable
                let new = *current / prime;
                *current = new; // write back internal state
                eprintln!("{}", new);
                Some(new)
            }
            current => Some(*current), // just repeat
        })
        .last() //when iterator ends result will have been in last element
        .unwrap();

    println!("{}", res); //6857
}
