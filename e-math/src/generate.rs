use std::{
    cmp::max,
    iter::Iterator,
    ops::RangeInclusive,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
    thread::{self, JoinHandle},
    u128,
};

use paste::paste;

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
macro_rules! sieve_for_ux {
    ($T:ty) => {
        paste! {
            named_sieve_for_ux!([<sieve_ $T>],$T);
        }
    };
}
// 2 stage macro allows composing the name, with a macro, but doesn't brake formatting
macro_rules! named_sieve_for_ux {
    ($name:ident,$T:ty) => {
        pub fn $name(cap: $T, primes: &[$T], range: RangeInclusive<$T>) -> Vec<$T> {
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
    };
}

sieve_for_ux!(usize);
sieve_for_ux!(u32);
sieve_for_ux!(u64);
sieve_for_ux!(u128);

pub const PRIMES_16: [usize; 6] = [2, 3, 5, 7, 11, 13];

static BATCH_64_K: usize = 256 * 256;
static BACKLOG: usize = 4;

pub static PRIMES_256: Lazy<Vec<usize>> = Lazy::new(|| {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&PRIMES_16);
    buffer.append(&mut sieve_usize(16, &PRIMES_16, 17_usize..=256));
    buffer
});

pub static PRIMES_64K: Lazy<Vec<usize>> = Lazy::new(|| {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(&PRIMES_256);
    buffer.append(&mut sieve_usize(256, &PRIMES_256, 257_usize..=BATCH_64_K));
    buffer
});

enum PrimeCommand<T> {
    Stop,
    /// primes, divider cap, batch
    Job(Arc<Vec<T>>, T, RangeInclusive<T>),
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

pub struct PrimeIter<'a, T> {
    primes: &'a mut Primes<T>,
    index: usize,
}

macro_rules! impl_primes {
    ($T:ty) => {
        impl Primes<$T> {
            pub fn new() -> Self {
                // initialize workers
                let worker_count = max(std::thread::available_parallelism().unwrap().get() - 1, 1);
                let mut workers: Vec<(
                    JoinHandle<()>,
                    SyncSender<PrimeCommand<$T>>,
                    Receiver<PrimeResult<$T>>,
                )> = Vec::with_capacity(worker_count);

                for i in 0..worker_count {
                    let (command_sender, command_receiver) =
                        sync_channel::<PrimeCommand<$T>>(BACKLOG);
                    let (result_sender, result_receiver) = sync_channel::<PrimeResult<$T>>(BACKLOG);

                    let t = thread::spawn(move || {
                        while let Ok(PrimeCommand::Job(primes, cap, range)) =
                            command_receiver.recv()
                        {
                            let new_primes = paste! {[<sieve_ $T>](cap, primes.as_ref(), range)};

                            let _ = result_sender.send(PrimeResult {
                                new_primes: new_primes,
                                prev_primes: primes,
                            });
                        }
                        eprintln!("end worker {}", i);
                    });

                    workers.push((t, command_sender, result_receiver));
                }

                Primes {
                    primes: Arc::new(PRIMES_64K.clone()),
                    complete: 1,
                    running: 1,
                    workers: workers,
                }
            }

            pub fn nth(self: &mut Self, n: usize) -> $T {
                while self.primes.len() < n {
                    self.grow();
                }

                self.primes[n - 1]
            }

            pub fn iterator(self: &mut Self) -> PrimeIter<$T> {
                PrimeIter {
                    primes: self,
                    index: 0,
                }
            }

            fn grow(self: &mut Self) {
                self.add_work(); // ensure there will be something to receive
                self.receive_work(); // receive
                self.add_work(); // ensure there will be something to pick up immediately next time
            }

            fn add_work(self: &mut Self) {
                // giver every worker BACKLOG of work to do
                while self.running - self.complete <= self.workers.len() * BACKLOG {
                    let next_batch = self.running + 1;
                    let cap = (next_batch as f64).sqrt().ceil() as $T * 256;
                    let range =
                        (self.running as $T * BATCH_64_K + 1)..=(next_batch as $T * BATCH_64_K);
                    let job = PrimeCommand::Job(self.primes.clone(), cap, range);
                    let _ = self.workers[next_batch % self.workers.len()].1.send(job);
                    self.running = next_batch;
                }
            }

            fn receive_work(self: &mut Self) {
                let mut buffer = vec![];
                let mut receive_counter = self.complete + 1;

                // single sync receive
                buffer.push(
                    self.workers[receive_counter % self.workers.len()]
                        .2
                        .recv()
                        .unwrap()
                        .new_primes,
                ); //will drop first  prev_primes arc

                // try to receive as much as possible in one go
                while let Ok(res) = self.workers[(receive_counter + 1) % self.workers.len()]
                    .2
                    .try_recv()
                {
                    receive_counter += 1;
                    buffer.push(res.new_primes);
                }

                let total_primes =
                    self.primes.len() + buffer.iter().map(|ps| ps.len()).sum::<usize>();

                let mut final_primes: Vec<$T> = Vec::with_capacity(total_primes);

                final_primes.extend_from_slice(self.primes.as_slice());

                buffer
                    .iter()
                    .for_each(|ps| final_primes.extend_from_slice(ps));

                self.primes = Arc::new(final_primes);
                self.complete = receive_counter;
                // will drop the other previous prime arcs
            }
        }

        impl Iterator for PrimeIter<'_, $T> {
            type Item = $T;

            fn next(&mut self) -> Option<Self::Item> {
                self.index += 1;
                Some(self.primes.nth(self.index))
            }
        }
    };
}

impl_primes!(usize);
