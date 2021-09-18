
use std::borrow::Borrow;

use super::{CoefficientBase, Commutative, CoefficientField, FromDegreeType};
use crate::DegreeType;

impl CoefficientBase for f32 {}
impl CoefficientBase for f64 {}

impl Commutative for f32 {}
impl Commutative for f64 {}


impl FromDegreeType for f32 {
    fn from_degree(deg: &DegreeType) -> Self {
        *deg as Self
    }
}

impl FromDegreeType for f64 {
    fn from_degree(deg: &DegreeType) -> Self {
        *deg as Self
    }
}

impl CoefficientField for f32
{
    type RationalType = f32;
    const ZERO: f32 = 0.0f32;
    const ONE: f32 = 1.0f32;
    const MONE: f32 = -1.0f32;

    #[inline(always)]
    fn uminus(&self) -> Self {
        -(*self)
    }
    #[inline(always)]
    fn inv(arg: impl Borrow<Self::RationalType>) -> Self
    {
        Self::ONE / arg.borrow()
    }

    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        self + other
    }
    #[inline(always)]
    fn sub(&self, other: &Self) -> Self {
        self - other
    }
    #[inline(always)]
    fn mul(&self, other: &Self) -> Self {
        self * other
    }
    #[inline(always)]
    fn div(&self, other: &Self::RationalType) -> Self {
        self / other
    }
    #[inline(always)]
    fn add_inplace(&mut self, other: &Self) -> &mut Self {
        *self += other;
        self
    }
    #[inline(always)]
    fn sub_inplace(&mut self, other: &Self) -> &mut Self {
        *self -= other;
        self
    }
    #[inline(always)]
    fn mul_inplace(&mut self, other: &Self) -> &mut Self {
        *self *= other;
        self
    }
    #[inline(always)]
    fn div_inplace(&mut self, other: &Self::RationalType) -> &mut Self {
        *self /= other;
        self
    }
}


impl CoefficientField for f64
{
    type RationalType = f64;

    const ZERO: f64 = 0.0f64;
    const ONE: f64 = 1.0f64;
    const MONE: f64 = -1.0f64;
    #[inline(always)]
    fn uminus(&self) -> Self {
        -(*self)
    }
    #[inline(always)]
    fn inv(arg: impl Borrow<Self::RationalType>) -> Self
    {
        Self::ONE / arg.borrow()
    }

    #[inline(always)]
    fn add(&self, other: &Self) -> Self {
        self + other
    }
    #[inline(always)]
    fn sub(&self, other: &Self) -> Self {
        self - other
    }
    #[inline(always)]
    fn mul(&self, other: &Self) -> Self {
        self * other
    }
    #[inline(always)]
    fn div(&self, other: &Self::RationalType) -> Self {
        self / other
    }
    #[inline(always)]
    fn add_inplace(&mut self, other: &Self) -> &mut Self {
        *self += other;
        self
    }
    #[inline(always)]
    fn sub_inplace(&mut self, other: &Self) -> &mut Self {
        *self -= other;
        self
    }
    #[inline(always)]
    fn mul_inplace(&mut self, other: &Self) -> &mut Self {
        *self *= other;
        self
    }
    #[inline(always)]
    fn div_inplace(&mut self, other: &Self::RationalType) -> &mut Self {
        *self /= other;
        self
    }
}
