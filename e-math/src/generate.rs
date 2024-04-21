use std::{iter::Iterator, ops::RangeInclusive, u64};

use once_cell::sync::Lazy;

pub fn fibonacci() -> impl Iterator<Item = u64> {
    let mut prev: u64 = 0;
    let mut last: u64 = 1;

    std::iter::from_fn(move || {
        let next = prev + last;
        prev = last;
        last = next;
        Some(next)
    })
}

/* Prime */
pub fn sieve(cap: u64, primes: &[u64], range: RangeInclusive<u64>) -> Vec<u64> {
    range
        .filter(|n| {
            primes
                .iter()
                .take_while(|p| **p <= cap)
                .find(|p| n % *p == 0)
                .is_none()
        })
        .collect()
}

pub const PRIMES_16: [u64; 6] = [2, 3, 5, 7, 11, 13];

static BATCH_64_K: u64 = 256 * 256;

pub static PRIMES_256: Lazy<Vec<u64>> = Lazy::new(|| {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&PRIMES_16);
    buffer.append(&mut sieve(16, &PRIMES_16, 17_u64..=256));
    buffer
});

pub static PRIMES_64K: Lazy<Vec<u64>> = Lazy::new(|| {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&PRIMES_256);
    buffer.append(&mut sieve(256, &PRIMES_256, 257_u64..=BATCH_64_K));
    buffer
});
