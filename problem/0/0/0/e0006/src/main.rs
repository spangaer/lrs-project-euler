fn main() {
    let n = 100_u128; // better safe then sorry

    let res = u128::pow((1..=n).sum(), 2) - (1..=n).map(|x| u128::pow(x, 2)).sum::<u128>();

    println!("{}", res); // 25164150
}
