use e_math::num::{digit_len, fibonacci};
use e_tools::log_timings;
use num_bigint::BigUint;

fn main() {
    //NOTE i = 0 == F(2)
    fibonacci::<u32>()
        .take(10)
        .enumerate()
        .for_each(|(i, f)| println!("{}  {}  {}", i + 2, f, digit_len(&f)));

    let res = log_timings(|| {
        fibonacci::<BigUint>()
            .position(|x| digit_len(&x) >= 1000)
            .map(|i| i + 2)
    });

    // release: 1s 285ms
    // debug: 3s 264ms

    println!("{:?}", res); // Some(4782)
}
