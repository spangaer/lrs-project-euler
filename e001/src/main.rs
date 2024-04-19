fn main() {
    let res: i64 = (1_i64..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("{}", res); // 233168
}
