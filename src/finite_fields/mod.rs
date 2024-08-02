pub mod finite_field_trait;

use crate::finite_fields::finite_field_trait::{FieldError, FiniteFieldTrait};
use num_bigint::BigUint;
use num_traits::ops::checked::CheckedSub;

pub struct FiniteField {}

impl FiniteFieldTrait for FiniteField {
    fn addition(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        (c + d).modpow(&BigUint::from(1u32), p)
    }

    fn multiply(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        (c * d).modpow(&BigUint::from(1u32), p)
    }

    fn addition_inverse(c: &BigUint, d: &BigUint) -> Result<BigUint, FieldError> {
        c.checked_sub(d)
            .ok_or_else(|| FieldError::Underflow(c.clone(), d.clone()))
    }

    fn multiply_inverse(c: &BigUint, p: &BigUint) -> Result<BigUint, FieldError> {
        // TODO - check if p is prime
        Ok(c.modpow(&(p - BigUint::from(2u32)), p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let res = FiniteField::addition(&c, &d, &p);
        assert_eq!(res, BigUint::from(3u32));
    }

    #[test]
    fn test_multiplication() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(31u32);

        let res = FiniteField::multiply(&c, &d, &p);
        assert_eq!(res, BigUint::from(9u32));
    }

    #[test]
    fn test_addition_inverse() {
        let c = BigUint::from(10u32);
        let d = BigUint::from(5u32);

        let res = FiniteField::addition_inverse(&c, &d).unwrap();

        assert_eq!(res, BigUint::from(5u32));
    }

    #[test]
    fn test_addition_inverse_negative() {
        let c = BigUint::from(5u32);
        let d = BigUint::from(10u32);

        let err = FiniteField::addition_inverse(&c, &d).unwrap_err();
        match err {
            FieldError::Underflow(a, b) => {
                assert_eq!(a, c);
                assert_eq!(b, d);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_multiply_inverse() {
        let c = BigUint::from(3u32);
        let p = BigUint::from(7u32);

        let res = FiniteField::multiply_inverse(&c, &p).unwrap();
        assert_eq!(res, BigUint::from(5u32));
    }
}
