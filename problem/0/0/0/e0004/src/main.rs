fn main() {
    for i in [123321, 1234321, 12345321] {
        println!("{} {}", i, is_palindrome(i))
    }

    let top = 1000_isize;
    let mut max = 1_isize;

    for i in 1..top {
        for j in 1..=i {
            let n = (top - i) * (top - j);

            if n > max {
                if is_palindrome(n) {
                    max = n;
                }
            } else {
                break;
            }
        }
    }

    println!("{}", max); //906609
}

fn is_palindrome(n: isize) -> bool {
    let string = format!("{}", n);

    // let half = string.len() / 2; // given it's digits UTF8 is single byte
    let half = string.chars().count() / 2; // more formally correct
    let odd = !(half * 2 == string.len());

    let mut stack = Vec::<char>::new();

    string.chars().enumerate().all(|(i, c)| {
        if i < half {
            stack.push(c);
            true
        } else if i == half && odd {
            true // middle char
        } else {
            stack.pop().unwrap() == c
        }
    })
}
