fn main() {
    println!("{}", spiral(5)); // 101
    println!("{}", spiral(1001)); // 669171001
}

fn spiral(size: u64) -> u64 {
    let mut state = 1;
    let mut step = 0;
    let mut sum = 1;

    loop {
        step += 2;

        if step > size {
            break;
        }

        for _ in 0..4 {
            state += step;
            sum += state;
        }
    }

    sum
}
