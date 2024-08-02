use std::error::Error;

use num_bigint::BigUint;
use thiserror::Error as thiserror;

use crate::{elliptic_curve::point::Point, finite_fields::finite_field_trait::FieldError};

pub trait ElliplticCurveTrait {
    fn addition<'a>(&'a self, c: &'a Point, d: &'a Point) -> Result<Point, impl Error>;
    fn double(c: &BigUint) -> Point;
    fn scalar_multiple(c: &BigUint, d: &BigUint) -> Point;
    fn is_on_curve(&self, c: &Point) -> bool;
}

#[derive(Debug, thiserror)]
pub enum CurveError<'a> {
    #[error("not on curve")]
    NotOnCurve(&'a Point),

    #[error("args should not be same")]
    SameArgs(&'a Point, &'a Point),

    #[error("finite filed error")]
    FiniteField(FieldError),

    #[error("both fields identity")]
    Identity,
}
