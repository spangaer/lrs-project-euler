use std::usize;

fn main() {
    count_triangles(120);

    let res = (3..=1000_usize).max_by_key(|p| count_triangles(*p));

    println!("{:?}", res); // Some(840)
}

fn count_triangles(p: usize) -> usize {
    println!("for {}", p);

    let mut count = 0_usize;

    for a in 1..=(p / 3) {
        let leftover = p - a;
        let half_left = leftover >> 1;

        for b in a..=half_left {
            let c = p - a - b;

            if c * c == a * a + b * b {
                count += 1;

                println!(" ({},{},{})", a, b, c)
            }
        }
    }

    println!("count {}", count);

    count
}
