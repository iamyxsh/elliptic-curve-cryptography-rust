pub mod elliptic_curve_trait;
pub mod point;

use num_bigint::BigUint;

use crate::elliptic_curve::{
    elliptic_curve_trait::{CurveError, ElliplticCurveTrait},
    point::Point,
};

pub struct ElliplticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl ElliplticCurveTrait for ElliplticCurve {
    fn addition(&self, c: &Point, d: &Point) -> Result<BigUint, CurveError> {
        if !self.is_on_curve(c) {
            Err(CurveError::NotOnCurve(c))
        };

        if !self.is_on_curve(d) {
            Err(CurveError::NotOnCurve(d))
        };

        if *c == *d {
            Err(CurveError::SameArgs(c, d))
        }

        match (c, d) {
            (Point::Cordinates(big_uint, big_uint1), Point::Cordinates(x, y)) => {
                todo!()
            }
            (Point::Cordinates(big_uint, big_uint1), Point::Identity) => todo!(),
            (Point::Identity, Point::Cordinates(big_uint, big_uint1)) => todo!(),
            (Point::Identity, Point::Identity) => todo!(),
        }

    }

    fn double(c: &BigUint) -> point::Point {
        todo!()
    }

    fn scalar_multiple(c: &BigUint, d: &BigUint) -> point::Point {
        todo!()
    }

    fn is_on_curve(&self, c: &Point) -> bool {
        match c {
            Point::Cordinates(x, y) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = &self.a * x;

                y2 == x3 + ax + &self.b
            }
            Point::Identity => true,
        }
    }
}
