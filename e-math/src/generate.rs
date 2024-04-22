use std::{iter::Iterator, ops::RangeInclusive, u128};

use once_cell::sync::Lazy;

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

/* Prime */
pub fn sieve(cap: u128, primes: &[u128], range: RangeInclusive<u128>) -> Vec<u128> {
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

pub const PRIMES_16: [u128; 6] = [2, 3, 5, 7, 11, 13];

static BATCH_64_K: u128 = 256 * 256;

pub static PRIMES_256: Lazy<Vec<u128>> = Lazy::new(|| {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&PRIMES_16);
    buffer.append(&mut sieve(16, &PRIMES_16, 17_u128..=256));
    buffer
});

pub static PRIMES_64K: Lazy<Vec<u128>> = Lazy::new(|| {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&PRIMES_256);
    buffer.append(&mut sieve(256, &PRIMES_256, 257_u128..=BATCH_64_K));
    buffer
});

pub struct Primes {
    primes: Vec<u128>,
    // batch_running: Vec<(u128, usize)>,
    // workers
}

impl Primes {
    pub fn new() -> Self {
        Primes {
            primes: PRIMES_256.clone(),
            // batch_running: vec![],
        }
    }

    pub fn get(self: &mut Self, index: usize) -> u128 {
        // FIXME will run out of indices
        self.primes[index]
    }

    pub fn iterator(self: &mut Self) -> PrimeIter {
        PrimeIter {
            primes: self,
            index: 0,
        }
    }
}

pub struct PrimeIter<'a> {
    primes: &'a mut Primes,
    index: usize,
}

impl Iterator for PrimeIter<'_> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        Some(self.primes.get(self.index))
    }
}
