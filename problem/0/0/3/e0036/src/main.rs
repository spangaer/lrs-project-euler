use e_math::num;

fn main() {
    let res = (1..1_000_000_usize)
        .filter(|x| is_digit_palindrome(x))
        .filter(|x| is_binary_palindrome(x))
        .sum::<usize>();

    println!("{}", res) //872187
}

fn is_digit_palindrome(x: &usize) -> bool {
    let digits = num::digits(x).collect::<Vec<_>>();

    (0..digits.len() / 2).all(|i| digits[i] == digits[digits.len() - 1 - i])
}

fn is_binary_palindrome(x: &usize) -> bool {
    let bits = num::bits(x).collect::<Vec<_>>();

    (0..bits.len() / 2).all(|i| bits[i] == bits[bits.len() - 1 - i])
}
