use e_math::num::{digits, faculty};
use e_tools::log_timings;
use std::iter;

fn main() {
    log_timings(|| {
        let chunk = faculty(&9_u64);

        println!("{chunk}");

        // see problem 30

        fn sum_of_digit_fact(i: u64) -> (u64, usize) {
            let collected: Vec<_> = digits(&i).map(|d| faculty(&(d as u64))).collect();

            (collected.iter().sum(), collected.len())
        }

        let equal_sum_of_digit_fact_series = {
            let mut i = 3;
            iter::from_fn(move || {
                i += 1;
                let (sum, len) = sum_of_digit_fact(i);

                // when the digit-len * 9! drops below 'i', we can't reach i anymore so we can stop
                // add + 1 just for certainty of not cutting off too soon (digit increase verge)
                if i <= (len as u64 + 1) * chunk {
                    Some((i, sum))
                } else {
                    None
                }
            })
            .filter_map(|(i, s)| if i == s { Some(i) } else { None })
        };

        println!();
        let res: u64 = equal_sum_of_digit_fact_series
            .map(|i| {
                println!("{i}");
                i
            })
            .sum();

        println!();
        println!("{res}"); // 40730
        println!();
        // release: 452ms 880us 425ns
        // debug: 3s 552ms 220us 215ns
    });
}
