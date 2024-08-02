use num_bigint::BigUint;
use thiserror::Error;

use crate::elliptic_curve::point::Point;

pub trait ElliplticCurveTrait {
    fn addition(&self, c: &Point, d: &Point) -> Result<BigUint, CurveError>;
    fn double(c: &BigUint) -> Point;
    fn scalar_multiple(c: &BigUint, d: &BigUint) -> Point;
    fn is_on_curve(&self, c: &Point) -> bool;
}

#[derive(Debug, Error)]
pub enum CurveError {
    #[error("not on curve")]
    NotOnCurve(Point),

    #[error("args should not be same")]
    SameArgs(Point, Point),
}
