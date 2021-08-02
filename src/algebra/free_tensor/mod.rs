mod conversions;
mod implementation;

use std::borrow::{Borrow, BorrowMut};

use crate::coefficients::{CoefficientField, FromDegreeType};
use crate::basis::{Basis, TensorBasis, TensorKey};
use crate::vector::{Vector, KeyType, RationalType};


use super::Algebra;
use crate::DegreeType;
use crate::algebra::ScalarField;


pub trait FreeTensor<const NLETTERS: DegreeType> : Algebra<BasisType=TensorBasis<{ NLETTERS }>> {

    const MAX_DEGREE: DegreeType;

    fn exp(&self) -> Self
    {
        let tunit = Self::from_key(KeyType::<Self>::new());
        let mut result = Self::from_key(KeyType::<Self>::new());
        let borrowed_result = result.borrow_mut();

        for i in (1..Self::MAX_DEGREE).rev() {
            borrowed_result.mul_rat_rdiv(self, ScalarField::<Self>::from_degree(&i), Some(Self::MAX_DEGREE));
            borrowed_result.add_inplace(&tunit);
        }
        result
    }

    fn fmexp(&mut self, arg: impl Borrow<Self>) -> &mut Self
    {
        let old_self = self.to_owned();
        let mut x = arg.borrow().to_owned();

        let oself = old_self.borrow();

        if let Some(unit) = x.get_mut(KeyType::<Self>::new()) {
            *unit = Self::ScalarFieldType::ZERO.clone();
        }

        for i in (1..Self::MAX_DEGREE).rev() {
            self.mul_rat_rdiv(&x, ScalarField::<Self>::from_degree(&i), Some(Self::MAX_DEGREE -i + 1));
            self.add_inplace(oself);
        }

        self
    }

    fn log(&self) -> Self
    {
        let kunit = KeyType::<Self>::new();
        let tunit = Self::from_key(kunit.clone());
        let mut x = self.to_owned();

        let mut rv = Self::new();

        if let Some(unit) = x.get_mut(kunit) {
            *unit = Self::ScalarFieldType::ZERO.clone();
        }

        for i in (1..Self::MAX_DEGREE).rev() {
            if i % 2 == 0 {
                rv.sub_scalar_rdivide(&tunit, RationalType::<Self>::from_degree(&i));
            } else {
                rv.add_scalar_rdivide(&tunit, RationalType::<Self>::from_degree(&i));
            }
            rv.multiply(&x, Some(Self::MAX_DEGREE));
        }

        rv
    }

}