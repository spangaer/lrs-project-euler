use std::{
    cmp::{max, Ord},
    iter::Iterator,
    ops::{Add, Div, Mul, RangeInclusive, Rem},
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
    thread::{self, JoinHandle},
};

use once_cell::sync::Lazy;

/* Primes */

pub static PRIMES_16: [u32; 6] = [2, 3, 5, 7, 11, 13];

const BLOCK: u32 = 256 * 256;
const BATCH_BLOCKS: usize = 2;
const BACKLOG: usize = 16;

pub static PRIMES_256: Lazy<Vec<u32>> = Lazy::new(|| {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&PRIMES_16);
    buffer.append(&mut Primes::<u32>::sieve(&PRIMES_16, 17_u32..=256));
    buffer
});

pub static PRIMES_64K: Lazy<Vec<u32>> = Lazy::new(|| {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&PRIMES_256);
    buffer.append(&mut Primes::<u32>::sieve(&PRIMES_256, 257_u32..=BLOCK));
    buffer
});

enum PrimeCommand<T> {
    Stop,
    /// primes, batch
    Job(Arc<Vec<T>>, RangeInclusive<T>),
}

struct PrimeResult<T> {
    new_primes: Vec<T>,
    #[allow(dead_code)]
    /// using this to bring the arc back to the admin thread
    /// make all access to atomic and allocations happen from admin thread
    prev_primes: Arc<Vec<T>>,
}

pub struct Primes<T> {
    /// current set of primes
    primes: Arc<Vec<T>>,
    /// batch index of completed batches
    complete: usize,
    /// batch index of highest running batch
    running: usize,
    /// sieving workers
    workers: Vec<(
        JoinHandle<()>,
        SyncSender<PrimeCommand<T>>,
        Receiver<PrimeResult<T>>,
    )>,
}

impl<T> Drop for Primes<T> {
    fn drop(&mut self) {
        self.workers.iter().for_each(|(_, sender, _)| {
            let _ = sender.send(PrimeCommand::Stop); // let all workers go
        })
    }
}

impl<
        T: Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Rem<Output = T>
            + From<u32>
            + TryFrom<usize>
            + Ord
            + Send
            + Sync
            + 'static,
    > Primes<T>
where
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn zero() -> T {
        0.into()
    }
    fn one() -> T {
        1.into()
    }

    pub fn log(x: T, y: T) -> u32 {
        let mut res: u32 = 1;
        let mut mult = y;
        while mult <= x {
            mult = mult * y;
            res = res + 1;
        }
        // when we break out of the loop we're one to far
        res - 1
    }

    fn range_iter(range: RangeInclusive<T>) -> impl Iterator<Item = T> {
        let mut n = *range.start();

        std::iter::from_fn(move || {
            if range.contains(&n) {
                let temp = n;
                n = n + Self::one();
                Some(temp)
            } else {
                None
            }
        })
    }

    pub fn sieve(primes: &[T], range: RangeInclusive<T>) -> Vec<T> {
        Self::range_iter(range)
            .filter(|n| {
                !primes
                    .iter()
                    .take_while(|p| **p * **p <= *n)
                    .any(|p| (*n) % *p == Self::zero())
            })
            .collect()
    }

    pub fn new() -> Self {
        // initialize workers
        let worker_count = max(std::thread::available_parallelism().unwrap().get() - 1, 1);
        let mut workers: Vec<(
            JoinHandle<()>,
            SyncSender<PrimeCommand<T>>,
            Receiver<PrimeResult<T>>,
        )> = Vec::with_capacity(worker_count);

        for i in 0..worker_count {
            let (command_sender, command_receiver) = sync_channel::<PrimeCommand<T>>(BACKLOG);
            let (result_sender, result_receiver) = sync_channel::<PrimeResult<T>>(BACKLOG);

            let t = thread::spawn(move || {
                while let Ok(PrimeCommand::Job(primes, range)) = command_receiver.recv() {
                    let new_primes = Self::sieve(primes.as_ref(), range);

                    let _ = result_sender.send(PrimeResult {
                        new_primes: new_primes,
                        prev_primes: primes,
                    });
                }
                eprintln!("end worker {}", i);
            });

            workers.push((t, command_sender, result_receiver));
        }

        let start_primes = PRIMES_64K.iter().map(|&p| p.into()).collect();

        Primes {
            primes: Arc::new(start_primes),
            complete: 1,
            running: 1,
            workers: workers,
        }
    }

    fn grow(&mut self) {
        self.add_work(); // ensure there will be something to receive
        self.receive_work(); // receive
        self.add_work(); // ensure there will be something to pick up immediately next time
    }

    fn add_work(&mut self) {
        // giver every worker BACKLOG of work to do
        while self.running - self.complete <= self.workers.len() * BACKLOG {
            let next_batch = self.running + BATCH_BLOCKS;
            let range = (self.running * BLOCK as usize + 1).try_into().unwrap()
                ..=(next_batch * BLOCK as usize).try_into().unwrap();
            let job = PrimeCommand::Job(self.primes.clone(), range);
            let _ = self.workers[next_batch % self.workers.len()].1.send(job);
            self.running = next_batch;
        }
    }

    fn receive_work(&mut self) {
        let mut buffer = vec![];
        let mut receive_counter = self.complete + BATCH_BLOCKS;

        // single sync receive
        buffer.push(
            self.workers[receive_counter % self.workers.len()]
                .2
                .recv()
                .unwrap()
                .new_primes,
        ); //will drop first  prev_primes arc

        // try to receive as much as possible in one go
        while let Ok(res) = self.workers[(receive_counter + BATCH_BLOCKS) % self.workers.len()]
            .2
            .try_recv()
        {
            receive_counter += BATCH_BLOCKS;
            buffer.push(res.new_primes);
        }

        let total_primes = self.primes.len() + buffer.iter().map(|ps| ps.len()).sum::<usize>();

        let mut final_primes: Vec<T> = Vec::with_capacity(total_primes);

        final_primes.extend_from_slice(self.primes.as_slice());

        buffer
            .iter()
            .for_each(|ps| final_primes.extend_from_slice(ps));

        self.primes = Arc::new(final_primes);
        self.complete = receive_counter;
        // will drop the other previous prime arcs
    }

    pub fn nth(&mut self, n: usize) -> T {
        while self.primes.len() < n {
            self.grow();
        }

        self.primes[n - 1]
    }

    pub fn iterator(&mut self) -> PrimeIter<T> {
        PrimeIter {
            primes: self,
            index: 0,
        }
    }

    pub fn factorize(&mut self, n: T) -> Vec<(T, u32)> {
        // this solution could cut short when state becomes smaller then p^2
        self.factorize_with(n, false)
            .iter()
            .filter(|(_, e)| *e != 0)
            .map(|t| t.clone())
            .collect()
    }

    pub fn factorize_with_zeros(&mut self, n: T) -> Vec<(T, u32)> {
        self.factorize_with(n, true)
    }

    fn factorize_with(&mut self, n: T, zeros: bool) -> Vec<(T, u32)> {
        self.iterator()
            .scan(n, |state, prime| match state {
                state if *state == Self::one() => None,
                state if *state == prime => {
                    let temp = *state;
                    *state = Self::one();
                    Some((temp, 1))
                } // current state is a prime
                state if *state < (prime * prime) => {
                    if zeros {
                        Some((prime, 0))
                    } else {
                        let temp = *state;
                        *state = Self::one();
                        Some((temp, 1))
                    }
                }
                state => {
                    let mut pow = 0;

                    while *state % prime == Self::zero() {
                        *state = *state / prime;
                        pow += 1;
                    }

                    Some((prime, pow))
                }
            })
            .collect()
    }

    pub fn is_prime(&mut self, x: T) -> bool {
        self.iterator()
            .take_while(|&p| p * p <= x)
            .all(|p| x % p != Self::zero())
    }

    fn pow(t: T, exp: u32) -> T {
        (0..exp).fold(Self::one(), |acc, _| acc * t)
    }

    /// lcm is max power of each prime factor
    pub fn lcm(&mut self, numbers: &[T]) -> T {
        let factors = numbers
            .iter()
            .map(|&n| self.factorize_with_zeros(n))
            .collect::<Vec<_>>();

        let max_len = factors.iter().map(|fr| fr.len()).max().unwrap();

        let mut lcm_factors = vec![];

        for i in 0..max_len {
            factors
                .iter()
                .map(|fr| fr.get(i))
                .fold::<Option<&(T, u32)>, _>(None, |current, other| {
                    match (current, other) {
                        // if only one of either defined that wins
                        // if both defined the larges wins
                        (x, None) => x,
                        (None, y) => y,
                        (ox @ Some((_, x)), Some((_, y))) if x >= y => ox, // avoid new object
                        (_, y) => y,
                    }
                })
                .iter()
                .for_each(|&factor| lcm_factors.push(factor));
        }

        lcm_factors
            .iter()
            .fold(Self::one(), |current, (prime, pow)| {
                current * Self::pow(*prime, *pow)
            })
    }
}

pub struct PrimeIter<'a, T> {
    primes: &'a mut Primes<T>,
    index: usize,
}

impl<
        T: Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Rem<Output = T>
            + From<u32>
            + TryFrom<usize>
            + Ord
            + Send
            + Sync
            + 'static,
    > Iterator for PrimeIter<'_, T>
where
    <T as TryFrom<usize>>::Error: std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        Some(self.primes.nth(self.index))
    }
}
