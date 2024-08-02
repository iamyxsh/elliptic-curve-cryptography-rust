use num_bigint::BigUint;

#[derive(Debug, PartialEq, Clone)]
pub enum Point {
    Cordinates(BigUint, BigUint),
    Identity,
}
