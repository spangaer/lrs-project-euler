use std::collections::HashSet;

use e_math::num::num_from_digits;
use e_tools::log_timings;
use itertools::Itertools; // permutations

fn main() {
    log_timings(|| {
        let len = 9_usize;
        let mut results = HashSet::new();

        (1_u8..=9).permutations(9).for_each(|p| {
            for i in 1..len - 2 {
                let a = num_from_digits::<u64>(&p[0..i]).unwrap();
                for j in i + 1..len - 1 {
                    let b = num_from_digits::<u64>(&p[i..j]).unwrap();
                    let c = num_from_digits::<u64>(&p[j..]).unwrap();

                    if a * b == c {
                        println!("{a} * {b} == {c}");
                        results.insert(c);
                    }
                }
            }
        });

        println!("sum: {}", results.iter().sum::<u64>())
        // 45228
        // release: 92ms 60us 66ns
        // debug: 4s 227ms 435us 454ns
    })
}
