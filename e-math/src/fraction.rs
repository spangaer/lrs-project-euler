use std::{
    fmt::{Debug, Display},
    ops::Mul,
};

use crate::primesg::{Pint, Primes};

#[derive(Eq, PartialEq)]
pub struct Fraction<I: Pint> {
    pub num: I,
    pub denom: I,
}

impl<I: Pint + Debug> Debug for Fraction<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}/{:?}", self.num, self.denom))
    }
}

impl<I: Pint + Display> Display for Fraction<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:}/{:}", self.num, self.denom))
    }
}

impl<I: Pint> Fraction<I> {
    pub fn new(num: I, denom: I) -> Self {
        Fraction { num, denom }
    }

    pub fn simplify(&self, primes: &mut Primes<I>) -> Self {
        let gcd = primes.gcd(&[self.num, self.denom]);

        Fraction {
            num: self.num / gcd,
            denom: self.denom / gcd,
        }
    }
}

impl<I: Pint> Mul for Fraction<I> {
    type Output = Fraction<I>;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction {
            num: self.num * rhs.num,
            denom: self.denom * rhs.denom,
        }
    }
}
