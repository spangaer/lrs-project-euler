use e_math::num;

fn main() {
    println!("{:?}", pan_concat_product(192));
    println!("{:?}", pan_concat_product(9));

    println!();

    // stop at 9999, which *2 will produce 5 digits,
    // everything higher will produce more then 9 digits in total
    let res = (2_u64..=(9999_u64))
        .map(|x| pan_concat_product(x))
        .flatten()
        .map(|tup| {
            println!("{:?}", tup);
            tup
        })
        .max_by_key(|tup| tup.2);

    println!();
    println!("{:?}", res); // Some((9327, 2, 932718654))
}

/// returns x, n and pan digital if they exist for x
fn pan_concat_product(x: u64) -> Option<(u64, u64, u64)> {
    let mut n = 1_u64;

    let mut digits = [0_u8; 9];
    let mut i = 0_usize;

    while i < digits.len() {
        let mut mul_digits = num::digits(&(n * x)).collect::<Vec<_>>();
        mul_digits.reverse();

        mul_digits.iter().for_each(|d| {
            if i < digits.len() {
                if !digits.contains(d) {
                    digits[i] = *d;
                } else {
                    i = 10; // we hade prior digit or 0, abort
                }
            }
            i += 1; // if we have too many digits, move the index past 9
        });

        if i < digits.len() {
            n += 1;
        }
    }

    if n > 1 && i == 9 {
        Some((x, n, num::num_from_digits::<u64>(&digits).unwrap()))
    } else {
        None
    }
}
