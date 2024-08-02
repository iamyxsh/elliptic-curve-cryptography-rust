use num_bigint::BigUint;
use thiserror::Error;

pub trait FiniteFieldTrait {
    fn addition(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint;
    fn multiply(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint;
    fn addition_inverse(c: &BigUint, d: &BigUint) -> Result<BigUint, FieldError>;
    fn substract(c: &BigUint, d: &BigUint, p: &BigUint) -> Result<BigUint, FieldError>;
    fn multiply_inverse(c: &BigUint, p: &BigUint) -> Result<BigUint, FieldError>;
    fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> Result<BigUint, FieldError>;
}

#[derive(Debug, Error)]
pub enum FieldError {
    #[error("underflow in subtraction: {0} < {1}")]
    Underflow(BigUint, BigUint),

    #[error("arg is not a prime")]
    NotPrime(BigUint),
}
