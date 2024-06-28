use e_math::num::digits;

fn main() {
    (11..=100000)
        .map(|i| (i, sum_of_digit_powers(i, 4)))
        .filter(|(i, s)| i == s)
        .for_each(|(i, s)| println!("{i} {s}"));

    println!();

    let res: u64 = (11..=1_000_000)
        .map(|i| (i, sum_of_digit_powers(i, 5)))
        .filter(|(i, s)| i == s)
        .map(|(i, _)| {
            println!("{i}");
            i
        })
        .sum();

    println!();
    println!("{res}") // 443839
}

fn sum_of_digit_powers(i: u64, pow: u32) -> u64 {
    digits(&i).map(|d| (d as u64).pow(pow)).sum()
}
