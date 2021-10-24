
use std::borrow::{Borrow, BorrowMut};



use crate::basis::{Basis};
use crate::coefficients::CoefficientField;
//use super::iteration::VectorIterItem;
use super::IntoVectorIterator;
//use super::VectorIter;



pub trait Vector<'vec> : 'vec + Sized + PartialEq /* + VectorIter<Self::KeyType, Self::ScalarFieldType> */
{
    type BasisType: 'static + Basis;
    type KeyType: 'static;
    type ScalarType: 'static +  CoefficientField;
    type RationalType: 'static;

    // Creation methods
    fn new() -> Self;
    fn from_key(key: impl Into<Self::KeyType>) -> Self;
    fn from_key_scalar(key: impl Into<Self::KeyType>, scalar: impl Into<Self::ScalarType>) -> Self;
    fn from_iterator(iterator: impl IntoIterator<Item=(Self::KeyType, Self::ScalarType)>) -> Self;

    fn swap(&mut self, other: impl BorrowMut<Self>);

    // To owned type method
    fn to_owned(&self) -> Self;

    // Global modification of vector
    fn clear(&mut self);

    // Element access methods
    fn get(&self, key: impl Borrow<Self::KeyType>) -> Option<&Self::ScalarType>;
    fn get_mut(&mut self, key: impl Borrow<Self::KeyType>) -> Option<&mut Self::ScalarType>;

    fn insert_single(&mut self, key: &Self::KeyType, value: impl Into<Self::ScalarType>);
    fn insert(&mut self, iterator: impl IntoIterator<Item=(Self::KeyType, Self::ScalarType)>);
    fn erase(&mut self, key: impl Borrow<Self::KeyType>);

    // Binary operations returning an owned vector
    fn uminus(&self) -> Self
    {
        let mut result = self.to_owned();
        result.borrow_mut().uminus_inplace();
        result
    }

    fn add(&self, other: impl Borrow<Self>) -> Self
    {
        let mut result = self.to_owned();
        result.borrow_mut().add_inplace(other.borrow());
        result
    }

    fn sub(&self, other: impl Borrow<Self>) -> Self
    {
        let mut result = self.to_owned();
        result.borrow_mut().sub_inplace(other.borrow());
        result
    }

    fn scalar_rmultiply(&self, scalar: impl Into<Self::ScalarType>) -> Self
    {
        self.scalar_lmultiply(scalar)
    }
    fn scalar_lmultiply(&self, scalar: impl Into<Self::ScalarType>) -> Self
    {
        let mut result = self.to_owned();
        result.borrow_mut().scalar_lmultiply_inplace(scalar);
        result
    }

    fn scalar_rdivide(&self, rational: impl Into<Self::RationalType>) -> Self
    {
        let mut result = self.to_owned();
        result.borrow_mut().scalar_rdivide_inplace(rational.into());
        result
    }

    fn scalar_ldivide(&self, rational: impl Into<Self::RationalType>) -> Self
    {
        self.scalar_rdivide(rational)
    }

    // Inplace binary operations acting on self.
    fn uminus_inplace(&mut self) -> &mut Self;

    fn add_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self;
    fn sub_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self;

    fn scalar_lmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarType>) -> &mut Self;
    fn scalar_rmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarType>) -> &mut Self
    {
        self.scalar_lmultiply_inplace(scalar)
    }

    fn scalar_rdivide_inplace(&mut self, rational: impl Into<Self::RationalType>) -> &mut Self;
    fn scalar_ldivide_inplace(&mut self, rational: impl Into<Self::RationalType>) -> &mut Self
    {
        self.borrow_mut().scalar_rdivide_inplace(rational)
    }


    // Some utility functions that can be optimised for specific cases but should always be implemented
    /// Efficient implementation of self += (other * s)
    fn add_scalar_rmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarType>) -> &mut Self
    {
        self.add_inplace(other.borrow().scalar_rmultiply(scalar))
    }

    /// Efficient implementation of self += (other * s)
    fn add_scalar_lmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarType>) -> &mut Self
    {
        self.add_inplace(other.borrow().scalar_lmultiply(scalar))
    }

    /// Efficient implementation of self -= (other * s)
    fn sub_scalar_rmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarType>) -> &mut Self
    {
        self.sub_inplace(other.borrow().scalar_rmultiply(scalar))
    }

    /// Efficient implementation of self -= (other * s)
    fn sub_scalar_lmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarType>) -> &mut Self
    {
        self.sub_inplace(other.borrow().scalar_lmultiply(scalar))
    }

    /// Efficient implementation of self += (other / s)
    fn add_scalar_rdivide(&mut self, other: impl Borrow<Self>, rational: impl Into<Self::RationalType>) -> &mut Self
    {
        self.add_inplace(other.borrow().scalar_rdivide(rational))
    }

    /// Efficient implementation of self += ((1/s)*other)
    fn add_scalar_ldivide(&mut self, other: impl Borrow<Self>, rational: impl Into<Self::RationalType>) -> &mut Self
    {
        self.add_inplace(other.borrow().scalar_ldivide(rational))
    }

    /// Efficient implementation of self -= (other / s)
    fn sub_scalar_rdivide(&mut self, other: impl Borrow<Self>, rational: impl Into<Self::RationalType>) -> &mut Self
    {
        self.sub_inplace(other.borrow().scalar_rdivide(rational))
    }

    /// Efficient implementation of self -= ((1/s)*other)
    fn sub_scalar_ldivide(&mut self, other: impl Borrow<Self>, rational: impl Into<Self::RationalType>) -> &mut Self
    {
        self.sub_inplace(other.borrow().scalar_ldivide(rational))
    }

}



