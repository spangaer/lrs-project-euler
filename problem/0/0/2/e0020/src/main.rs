use std::{thread, time::Duration};

use e_math::num::*;
use num_bigint::BigUint;

fn main() {
    let num = faculty(&BigUint::from(100_u64));

    println!("num {}", num);
    thread::sleep(Duration::from_millis(100));

    let sum = digit_sum(&num);

    println!("res {}", sum); // 648
    thread::sleep(Duration::from_millis(100));
}
