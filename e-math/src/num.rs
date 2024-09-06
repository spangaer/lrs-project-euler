use std::{
    iter::{from_fn, Sum},
    num::IntErrorKind,
};

use num::ToPrimitive;
use num_integer::Integer;

pub fn fibonacci<I: Integer + Clone>() -> impl Iterator<Item = I> {
    let mut prev: I = I::zero();
    let mut last: I = I::one();

    std::iter::from_fn(move || {
        let next = prev.clone() + last.clone();
        prev = last.clone();
        last = next.clone();
        Some(next)
    })
}

pub fn digits<I: Integer + Clone + From<u8> + ToPrimitive + 'static>(
    i: &I,
) -> impl Iterator<Item = u8> {
    let zero = I::zero();
    let ten = I::zero() + 10.into();

    let mut n = i.clone();

    from_fn(move || {
        if n > zero {
            let (dev, rem) = n.div_rem(&ten);
            n = dev;
            rem.to_u8()
        } else {
            None
        }
    })
}

pub fn num_from_digits<I: Integer + Clone + From<u8>>(digits: &[u8]) -> Result<I, IntErrorKind> {
    let zero = I::zero();
    let ten = I::zero() + 10.into();

    let mut n = Ok(zero.clone());

    digits.iter().for_each(|&d| match n.clone() {
        Ok(nn) => {
            let di: I = d.into();
            let shift_add = ten.clone() * nn.clone() + di;
            // overflow detection
            if shift_add > nn {
                n = Ok(shift_add);
            } else {
                n = Err(IntErrorKind::PosOverflow);
            }
        }
        _ => (), // do nothing,
    });

    n
}

pub fn digit_sum<I: Integer + Clone + From<u8> + ToPrimitive + Sum + 'static>(i: &I) -> I {
    digits(i).map(|x| x.into()).sum()
}

pub fn digit_len<I: Integer + Clone + From<u8> + ToPrimitive + 'static>(i: &I) -> usize {
    digits(i).count()
}

pub fn faculty<I: Integer + Clone>(i: &I) -> I {
    let zero = I::zero();

    let mut prod = I::zero() + I::one();
    let mut n = i.clone();

    while n > zero {
        prod = prod * n.clone();
        n = n - I::one();
    }

    prod
}
