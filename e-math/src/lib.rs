pub mod primes;

pub fn fibonacci() -> impl Iterator<Item = u128> {
    let mut prev: u128 = 0;
    let mut last: u128 = 1;

    std::iter::from_fn(move || {
        let next = prev + last;
        prev = last;
        last = next;
        Some(next)
    })
}
