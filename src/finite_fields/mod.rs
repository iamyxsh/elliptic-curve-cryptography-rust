pub mod finite_field_trait;

use crate::finite_fields::finite_field_trait::FiniteFieldTrait;
use num_bigint::BigUint;

pub struct FiniteField {}

impl FiniteFieldTrait for FiniteField {
    fn addition(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        (c + d).modpow(&BigUint::from(1u32), p)
    }

    fn multiply(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        todo!()
    }

    fn addition_inverse(c: &BigUint, d: &BigUint) -> BigUint {
        todo!()
    }

    fn multiply_inverse(c: &BigUint, d: &BigUint) -> BigUint {
        todo!()
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
}
