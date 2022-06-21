use monoid_derive::*;

use crate::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Last<T>(pub Option<T>);

impl<T> Default for Last<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> From<T> for Last<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T> From<Option<T>> for Last<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T> From<Last<T>> for Option<T> {
    fn from(value: Last<T>) -> Self {
        value.0
    }
}

impl<T> Semigroup for Last<T> {
    fn combine(self, rhs: Self) -> Self {
        Self(rhs.0.or(self.0))
    }
}

#[derive(Debug, Semigroup, Monoid, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum<T>(pub T);

impl<T: PartialEq> PartialEq<T> for Sum<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Sum<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T> From<T> for Sum<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Semigroup)]
pub struct Product<T>(pub T);

impl<T: Semigroup + num_traits::Num> Monoid for Product<T> {
    fn empty() -> Self {
        Self(num_traits::identities::One::one())
    }
}

impl<T> From<T> for Product<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Monoid for T
where
    T: Semigroup + Default,
{
    fn empty() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_with_into() {
        let x = Last::empty()
            .combine(53.into())
            .combine(None.into())
            .combine(42.into());

        assert_eq!(x.0, Some(42));
    }

    #[test]
    fn sum_test() {
        let nums = vec![10, 24, 3, 7, 42];
        let sum = nums
            .into_iter()
            .fold(Sum::empty(), |acc, x| acc.combine(Sum::from(x)));

        assert_eq!(sum, 86);
    }

    #[test]
    fn option_sum() {
        let sum = None
            .combine(Some(Sum::from(10)))
            .combine(None)
            .combine(Some(Sum::from(5)))
            .combine(Some(Sum::from(7)))
            .combine(None)
            .combine(Some(Sum::from(42)))
            .combine(None);

        assert_eq!(sum.unwrap(), 64);
    }

    #[test]
    fn option_combine_macro() {
        let sum: Option<Sum<i32>> = crate::combine!(
            None =>
            Sum::from(10),
            None,
            Sum::from(5),
            Sum::from(7),
            None,
            Sum::from(42),
            None,
        );

        assert_eq!(sum.unwrap(), 64);
    }

    #[test]
    fn combine_macro() {
        let x = crate::combine! {
            Last::from(53) => None, 42, {let b = None; b},
        };

        assert_eq!(x.0, Some(42));
    }

    #[test]
    fn last_to_option_conversion() {
        let last = Last::from(42);
        let res: Option<i32> = last.into();

        assert_eq!(res, Some(42));
    }
}
