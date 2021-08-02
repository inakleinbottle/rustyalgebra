
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

    fn uminus(&self) -> Self {
        -(*self)
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn mul(&self, other: &Self) -> Self {
        self * other
    }

    fn div(&self, other: &Self::RationalType) -> Self {
        self / other
    }

    fn add_inplace(&mut self, other: &Self) -> &mut Self {
        *self += other;
        self
    }

    fn sub_inplace(&mut self, other: &Self) -> &mut Self {
        *self -= other;
        self
    }

    fn mul_inplace(&mut self, other: &Self) -> &mut Self {
        *self *= other;
        self
    }

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

    fn uminus(&self) -> Self {
        -(*self)
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn mul(&self, other: &Self) -> Self {
        self * other
    }

    fn div(&self, other: &Self::RationalType) -> Self {
        self / other
    }

    fn add_inplace(&mut self, other: &Self) -> &mut Self {
        *self += other;
        self
    }

    fn sub_inplace(&mut self, other: &Self) -> &mut Self {
        *self -= other;
        self
    }

    fn mul_inplace(&mut self, other: &Self) -> &mut Self {
        *self *= other;
        self
    }

    fn div_inplace(&mut self, other: &Self::RationalType) -> &mut Self {
        *self /= other;
        self
    }
}
