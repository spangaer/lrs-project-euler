use num_bigint::BigUint;
use num_integer::Integer;

fn main() {
    let mut num = BigUint::from(2_u64).pow(1000);
    let ten = BigUint::from(10_u64);

    println!("num {}", num);

    let mut sum = BigUint::ZERO;

    while num > BigUint::ZERO {
        let (dev, rem) = num.div_rem(&ten);
        sum += rem;
        num = dev;
    }

    println!("res {}", sum); // 1366
}
