use num_bigint::BigUint;

pub trait FiniteFieldTrait {
    fn addition(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint;
    fn multiply(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint;
    fn addition_inverse(c: &BigUint, d: &BigUint) -> BigUint;
    fn multiply_inverse(c: &BigUint, d: &BigUint) -> BigUint;
}
