use num_bigint::BigUint;

fn main() {
    // u128 max: 340_282_366_920_938_463_463_374_607_431_768_211_455
    // need bigints

    let mut all = (2..=100_u32)
        .flat_map(|a| (2..=100).map(move |b| BigUint::from(a).pow(b)))
        .collect::<Vec<_>>();

    all.sort();
    all.dedup();

    println!("{}", all.len()); // 9183
}
