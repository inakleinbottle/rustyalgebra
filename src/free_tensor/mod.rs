
mod conversions;
mod implementation;
mod tensor_basis;

use std::borrow::{Borrow, BorrowMut};


use crate::DegreeType;
use crate::algebra::Algebra;
use crate::coefficients::{CoefficientField, FromDegreeType};

use crate::vector::{Vector};



pub use tensor_basis::{TensorKey, TensorBasis, TensorKeyIterator};
pub use implementation::DenseTensor;


pub trait FreeTensor<'vec, S: CoefficientField, const NLETTERS: DegreeType>
    : Algebra<'vec, BasisType=TensorBasis<NLETTERS>, KeyType=TensorKey<NLETTERS>, ScalarType=S, RationalType=S::RationalType> {

    const MAX_DEGREE: DegreeType;

    fn exp(&self) -> Self
    {
        let tunit = Self::from_key(Self::KeyType::new());
        let mut result = Self::from_key(Self::KeyType::new());
        let borrowed_result = result.borrow_mut();

        for i in (1..Self::MAX_DEGREE).rev() {
            borrowed_result.mul_rat_rdiv(self, Self::ScalarType::from_degree(&i), Some(Self::MAX_DEGREE));
            borrowed_result.add_inplace(&tunit);
        }
        result
    }

    fn fmexp(&mut self, arg: impl Borrow<Self>) -> &mut Self
    {
        let old_self = self.to_owned();
        let mut x = arg.borrow().to_owned();

        let oself = old_self.borrow();

        if let Some(unit) = x.get_mut(Self::KeyType::new()) {
            *unit = Self::ScalarType::ZERO.clone();
        }

        for i in (1..Self::MAX_DEGREE).rev() {
            self.mul_rat_rdiv(&x, Self::ScalarType::from_degree(&i), Some(Self::MAX_DEGREE -i + 1));
            self.add_inplace(oself);
        }

        self
    }

    fn log(&self) -> Self
    {
        let kunit = Self::KeyType::new();
        let tunit = Self::from_key(kunit.clone());
        let mut x = self.to_owned();

        let mut rv = Self::new();

        if let Some(unit) = x.get_mut(kunit) {
            *unit = Self::ScalarType::ZERO.clone();
        }

        for i in (1..Self::MAX_DEGREE).rev() {
            if i % 2 == 0 {
                rv.sub_scalar_rdivide(&tunit, Self::RationalType::from_degree(&i));
            } else {
                rv.add_scalar_rdivide(&tunit, Self::RationalType::from_degree(&i));
            }
            rv.multiply(&x, Some(Self::MAX_DEGREE));
        }

        rv
    }

}



