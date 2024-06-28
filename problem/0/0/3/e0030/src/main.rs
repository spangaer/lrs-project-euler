use std::iter;

use e_math::num::digits;

fn main() {
    for p in 2..=6 {
        println!();
        let res: u64 = equal_sum_of_digit_powers_series(p)
            .map(|i| {
                println!("{i}");
                i
            })
            .sum();

        println!();
        println!("{p}   {res}");
        println!();
    }

    // 3   1301
    // 4   19316
    // 5   443839
    // 6   548834
}

fn sum_of_digit_powers(i: u64, pow: u32) -> (u64, usize) {
    let collected: Vec<_> = digits(&i).map(|d| (d as u64).pow(pow)).collect();

    (collected.iter().sum(), collected.len())
}

fn equal_sum_of_digit_powers_series(pow: u32) -> impl Iterator<Item = u64> {
    let mut i = 1;
    iter::from_fn(move || {
        i += 1;
        let (sum, len) = sum_of_digit_powers(i, pow);

        // when the digit-len * 9^pow drops below 'i', we can't reach i anymore so we can stop
        // add + 1 just for certainty of not cutting off too soon (digit increase verge)
        if i <= (len as u64 + 1) * 9_u64.pow(pow) {
            Some((i, sum))
        } else {
            None
        }
    })
    .filter_map(|(i, s)| if i == s { Some(i) } else { None })
}
