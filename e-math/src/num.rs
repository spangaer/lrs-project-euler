use std::ops::Add;

use num_integer::Integer;

pub fn digit_sum<I: Integer + Clone + Add<I> + Add<u32, Output = I>>(i: &I) -> I {
    let zero = I::zero();
    let ten = I::zero() + 10;

    let mut sum = I::zero();

    let mut n = i.clone();

    while n > zero {
        let (dev, rem) = n.div_rem(&ten);
        sum = sum + rem;
        n = dev;
    }

    sum
}
