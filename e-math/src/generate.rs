use std::iter::Iterator;

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
