pub mod elliptic_curve_trait;
pub mod point;

use thiserror::Error;

use num_bigint::BigUint;

use crate::{
    elliptic_curve::{
        elliptic_curve_trait::{CurveError, ElliplticCurveTrait},
        point::Point,
    },
    finite_fields::{
        finite_field_trait::{FieldError, FiniteFieldTrait},
        FiniteField,
    },
};

pub struct ElliplticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl ElliplticCurve {
    pub fn new(a: BigUint, b: BigUint, p: BigUint) -> Self {
        Self { a, b, p }
    }

    fn compute_x3_y3(
        &self,
        x1: &BigUint,
        y1: &BigUint,
        x2: &BigUint,
        s: &BigUint,
    ) -> (BigUint, BigUint) {
        let s2 = s.modpow(&BigUint::from(2u32), &self.p);
        let x3 = FiniteField::substract(&s2, x1, &self.p).unwrap();
        let x3 = FiniteField::substract(&x3, x2, &self.p).unwrap();

        let y3 = FiniteField::substract(x1, &x3, &self.p).unwrap();
        let y3 = FiniteField::multiply(s, &y3, &self.p);
        let y3 = FiniteField::substract(&y3, y1, &self.p).unwrap();

        (x3, y3)
    }
}

impl ElliplticCurveTrait for ElliplticCurve {
    fn addition<'a>(&'a self, c: &'a Point, d: &'a Point) -> Result<Point, CurveError> {
        if !self.is_on_curve(c) {
            return Err(CurveError::NotOnCurve(c));
        };

        if !self.is_on_curve(d) {
            return Err(CurveError::NotOnCurve(d));
        };

        if *c == *d {
            return Err(CurveError::SameArgs(c, d));
        }

        match (c, d) {
            (Point::Cordinates(x1, y1), Point::Cordinates(x2, y2)) => {
                let y1plusy2 = FiniteField::addition(y1, y2, &self.p);

                if x1 == x2 && y1plusy2 == BigUint::from(0u32) {
                    return Ok(Point::Identity);
                }

                let numerator = FiniteField::substract(y2, y1, &self.p).unwrap();
                let denominator = FiniteField::substract(x2, x1, &self.p).unwrap();

                let s = FiniteField::divide(&numerator, &denominator, &self.p).unwrap();

                let (x3, y3) = self.compute_x3_y3(x1, y1, x2, &s);

                Ok(Point::Cordinates(x3, y3))
            }
            (_, Point::Identity) => Ok(c.clone()),
            (Point::Identity, _) => Ok(d.clone()),
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
                let ax = FiniteField::multiply(&self.a, x, &self.p);
                let x3plusax = FiniteField::addition(&x3, &ax, &self.p);

                y2 == FiniteField::addition(&x3plusax, &self.b, &self.p)
            }
            Point::Identity => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bu32(x: u32) -> BigUint {
        BigUint::from(x)
    }

    fn make_test_curve() -> ElliplticCurve {
        ElliplticCurve::new(bu32(2), bu32(2), bu32(17))
    }

    #[test]
    fn test_point_in_curve() {
        let ec = make_test_curve();

        let p1 = Point::Cordinates(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Cordinates(BigUint::from(5u32), BigUint::from(1u32));
        let p3 = Point::Cordinates(BigUint::from(10u32), BigUint::from(6u32));

        assert!(ec.is_on_curve(&p1));
        assert!(ec.is_on_curve(&p2));
        assert!(ec.is_on_curve(&p3));

        let p4 = Point::Cordinates(BigUint::from(4u32), BigUint::from(1u32));
        let p5 = Point::Cordinates(BigUint::from(1u32), BigUint::from(1u32));
        let p6 = Point::Cordinates(BigUint::from(0u32), BigUint::from(1u32));

        assert!(!ec.is_on_curve(&p4));
        assert!(!ec.is_on_curve(&p5));
        assert!(!ec.is_on_curve(&p6));
    }

    #[test]
    fn test_point_addition() {
        let ec = make_test_curve();

        let p1 = Point::Cordinates(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Cordinates(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Cordinates(BigUint::from(10u32), BigUint::from(6u32));

        let res = ec.addition(&p1, &p2).unwrap();
        assert_eq!(res, pr);
    }
}
