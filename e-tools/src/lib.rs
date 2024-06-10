use std::{
    sync::atomic::{fence, Ordering},
    time::Instant,
};

use humantime::format_duration;

pub mod efile;

pub fn log_timings<T, F: FnOnce() -> T>(f: F) -> T {
    let start = Instant::now();
    fence(Ordering::AcqRel);

    let res = f();

    fence(Ordering::AcqRel); // prevent compiler tricks
    println!("time: {}", format_duration(start.elapsed()));

    res
}
