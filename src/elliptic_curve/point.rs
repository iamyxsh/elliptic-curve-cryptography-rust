use num_bigint::BigUint;

#[derive(Debug, PartialEq)]
pub enum Point {
    Cordinates(BigUint, BigUint),
    Identity,
}
