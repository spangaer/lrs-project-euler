use std::iter::from_fn;

fn main() {
    for i in 2..=50 {
        print!("{}  0.", i);
        spit_digits(i).for_each(|s| {
            match s {
                Spit::Digit(d) => print!("{}", d),
                Spit::Period(p) => print!("-{}", p),
            };
        });
        println!("");
    }

    let res = (2..1000)
        .map(|d| {
            let p = match spit_digits(d).last() {
                Some(Spit::Period(p)) => p,
                _ => 0,
            };
            (d, p)
        })
        .max_by_key(|&(_, p)| p);

    println!("{:?}", res) // Some((983, 982)) // (d,p)
}

fn spit_digits(d: u64) -> impl Iterator<Item = Spit> {
    let mut tail = 1_u64;
    let mut i = 0_usize;
    let mut tails: Vec<(usize, u64)> = vec![];

    from_fn(move || {
        if tail == 0 {
            None
        } else {
            match tails.iter().find(|(_, t)| *t == tail) {
                Some((ii, _)) => {
                    tail = 0;
                    Some(Spit::Period(i - ii))
                }
                None => {
                    tails.push((i, tail)); // allow period detection

                    // decimal shift
                    tail *= 10;
                    i += 1;

                    // compute next digit
                    let digit = (tail / d) as u8;
                    tail = tail % d; // retain tail
                    Some(Spit::Digit(digit))
                }
            }
        }
    })
}

enum Spit {
    Digit(u8),
    Period(usize),
}
