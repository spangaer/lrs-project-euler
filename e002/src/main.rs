fn main() {
    const UPPER_BOUND: i64 = 4_000_000;

    let generator = {
        let mut prev: i64 = 0;
        let mut last: i64 = 1;

        std::iter::from_fn(move || {
            let next = prev + last;
            prev = last;
            last = next;
            Some(next)
        })
    };

    let res: i64 = generator
        .take_while(|x| *x <= UPPER_BOUND)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("{}", res); //4613732
}
