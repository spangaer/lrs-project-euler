use e_math::num::digit_sum;
use num_bigint::BigUint;

fn main() {
    let num = BigUint::from(2_u64).pow(1000);

    println!("num {}", num);

    let sum = digit_sum(&num);

    println!("res {}", sum); // 1366
}
