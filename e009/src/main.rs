fn main() {
    let s = 1000_u64;

    for a in 1..=s - 2 {
        for b in a + 1..(s - a) / 2 {
            // only need to go halfway: b < c
            let c = s - a - b;

            if a * a + b * b == c * c {
                println!("{} {} {} {}", a, b, c, a * b * c);
                // 200 375 425 31875000
                return;
            }
        }
    }
}
