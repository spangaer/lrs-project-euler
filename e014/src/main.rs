fn main() {
    println!("test {}", collatz_step(13));
    println!("test {}", collatz_step(40));
    println!("test {}", collatz(13));

    let res = (1..1_000_000_u64)
        .map(|n| (n, collatz(n)))
        .max_by_key(|t| t.1)
        .unwrap();

    //(837799, 525)
    println!("{:?}", res);
}

fn collatz(n: u64) -> u64 {
    let mut n = n;
    let mut i = 1_u64;

    loop {
        if n <= 1 {
            break;
        } else {
            i += 1;
            n = collatz_step(n);
        }
    }

    i
}

fn collatz_step(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        n * 3 + 1
    }
}
