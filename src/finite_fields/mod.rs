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

    fn substract(c: &BigUint, d: &BigUint, p: &BigUint) -> Result<BigUint, FieldError> {
        let res = match FiniteField::addition_inverse(d, p) {
            Ok(r) => r,
            Err(err) => return Err(err),
        };
        Ok(FiniteField::addition(c, &res, p))
    }

    fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> Result<BigUint, FieldError> {
        let res = match FiniteField::multiply_inverse(d, p) {
            Ok(r) => r,
            Err(err) => return Err(err),
        };
        Ok(FiniteField::multiply(c, &res, p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mod_add(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
        (a + b) % m
    }

    fn mod_mul(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
        (a * b) % m
    }

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

    #[test]
    fn test_substract() {
        let c = BigUint::from(5u32);
        let d = BigUint::from(10u32);
        let p = BigUint::from(7u32);

        let res = FiniteField::substract(&c, &d, &p).unwrap();
        let expected = mod_add(&c, &(d.clone() - p.clone()), &p);
        assert_eq!(res, expected);
        assert_eq!(res, BigUint::from(1u32));
    }

    #[test]
    fn test_divide() {
        let c = BigUint::from(4u32);
        let d = BigUint::from(3u32);
        let p = BigUint::from(7u32);

        let result = FiniteField::divide(&c, &d, &p).unwrap();
        assert_eq!(result, BigUint::from(6u32));

        let check = mod_mul(&result, &d, &p);
        assert_eq!(check, c % &p);
    }
}
