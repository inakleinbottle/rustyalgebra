use std::cmp::PartialEq;
use std::borrow::Borrow;

use crate::DegreeType;

/// Composite trait describing the basic properties that all coefficient-like types should satisfy.
///
/// At the moment, the only restriction is that the `Clone` trait is implemented.
pub trait CoefficientBase
    : Clone + PartialEq
{}


pub trait Commutative
{}

pub trait FromDegreeType {

    fn from_degree(deg: &DegreeType) -> Self;

}

pub trait CoefficientField
    : CoefficientBase + Commutative
    + From<i8> + From<i16> + FromDegreeType
{
    type RationalType: From<i8> + From<i16> + FromDegreeType;
    const ZERO: Self;
    const ONE: Self;
    const MONE: Self;

    fn from_rational<I: Into<Self>>(num: I, denom: impl Into<Self::RationalType>) -> Self
    {
        num.into().div(&denom.into())
    }

    fn uminus(&self) -> Self;
    fn inv(arg: impl Borrow<Self::RationalType>) -> Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self::RationalType) -> Self;

    fn add_inplace(&mut self, other: &Self) -> &mut Self;
    fn sub_inplace(&mut self, other: &Self) -> &mut Self;
    fn mul_inplace(&mut self, other: &Self) -> &mut Self;
    fn div_inplace(&mut self, other: &Self::RationalType) -> &mut Self;

}


mod floating_point_fields;