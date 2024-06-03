use std::ops::{Add, Mul, Sub};

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

pub fn faculty<I: Integer + Clone + Mul<I> + Add<u32, Output = I> + Sub<u32, Output = I>>(
    i: &I,
) -> I {
    let zero = I::zero();

    let mut prod = I::zero() + 1;
    let mut n = i.clone();

    while n > zero {
        prod = prod * n.clone();
        n = n - 1;
    }

    prod
}
