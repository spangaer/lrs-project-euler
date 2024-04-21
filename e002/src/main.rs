fn main() {
    const UPPER_BOUND: u64 = 4_000_000;

    let res: u64 = e_math::generate::fibonacci()
        .take_while(|x| *x <= UPPER_BOUND)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("{}", res); //4613732
}
