//! Traits defining vector-like behaviour and implementations.
//!
//! Vectors, or similar constructions, form the backbone of all structures described by this library.
//! The `Vector` trait describes the basic behaviours that a representation of a vector should
//! satisfy. These behaviours include simple construction methods, vector arithmetic, and iteration
//! over pairs consisting of one element from the basis - the *key* - and one element from the
//! scalar field - the *value*. In this way, a basic vector type should behave somewhat like a
//! `HashMap`, associating basis elements with a coefficient takent from the coordinate field.
//!
//! You might notice that the `Vector` trait defines its own arithmetic operations rather than
//! requiring that the type implements the arithmetic traits provided by the language. There are two
//! reasons for this. The first reason is simplicity. The built-in arithmetic traits are have to be
//! implemented for either the type itself or for a reference to the type, which in practice means
//! juggling a number of lifetime parameters. Defining our own methods allow us to take a a type
//! implementing `Borrow<Self>` as the second argument (where appropriate), which takes care of
//! numerous combinations simultaneously. The second reason is to avoid ambiguity. It is possible
//! that we may wish to implement several different vector structures on a data container, and
//! implementing the built-in traits would make this tricky.
//!
//! The `Vector` trait expresses it's connection to the basis type and scalar coefficient field by
//! means of the associated types `BasisType` and `ScalarFieldType`. To make accessing the numerous
//! coincidental types, we define three helper alias types `KeyType`, `ScalarField`, and
//! `RationalType`, which take the vector type as a generic and correctly disambiguate associated
//! types.
//!
//! Notice that the `Vector` trait does not require a basis to be ordered. This is a useful position
//! that yields better utility overall, even if most bases that we consider are ordered.
//!
//! Basis types that have a degree should implement the `VectorWithDegree` trait, which adds the a
//! method for computing the maximum degree represented by a vector. The default implementation
//! simply computes the maximum degree over the iterable of key-value pairs (provided by the
//! `Vector` trait). You may wish to implement an optimised implementation.

use std::borrow::{Borrow, BorrowMut};
use std::iter::IntoIterator;
use std::slice::SliceIndex;


pub use wrapper::VectorWrapper;

use crate::{DegreeType, DimensionType, LetterType};
use crate::basis::{Basis, BasisWithDegree, OrderedBasis};
use crate::coefficients::CoefficientField;

pub type KeyType<V> = <<V as Vector>::BasisType as Basis>::KeyType;
pub type ScalarField<V> = <V as Vector>::ScalarFieldType;
pub type RationalType<V> = <<V as Vector>::ScalarFieldType as CoefficientField>::RationalType;

pub trait Vector : Sized + PartialEq
{
    type BasisType: Basis;
    type ScalarFieldType: CoefficientField;


    //type IteratorType: for <'a> Iterator<Item=(KeyType<Self>, &'a ScalarField<Self>)>;

    // Creation methods
    fn new() -> Self;
    fn from_key(key: impl Into<KeyType<Self>>) -> Self;
    fn from_key_scalar(key: impl Into<KeyType<Self>>, scalar: impl Into<Self::ScalarFieldType>) -> Self;
    fn from_iterator(iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>) -> Self;

    fn swap(&mut self, other: impl BorrowMut<Self>);

    // To owned type method
    fn to_owned(&self) -> Self;

    // Global modification of vector
    fn clear(&mut self);

    // Element access methods
    fn get(&self, key: impl AsRef<<<Self as Vector>::BasisType as Basis>::KeyType>) -> Option<&Self::ScalarFieldType>;
    fn get_mut(&mut self, key: impl AsRef<KeyType<Self>>) -> Option<&mut Self::ScalarFieldType>;

    fn insert_single(&mut self, key: impl Into<KeyType<Self>>, value: impl Into<Self::ScalarFieldType>);
    fn insert(&mut self, iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>);
    fn erase(&mut self, key: impl AsRef<KeyType<Self>>);

    // Iterator access
    //fn iter_pairs(&self) -> Self::IteratorType;


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

    fn scalar_rmultiply(&self, scalar: impl Into<Self::ScalarFieldType>) -> Self
    {
        self.scalar_lmultiply(scalar)
    }
    fn scalar_lmultiply(&self, scalar: impl Into<Self::ScalarFieldType>) -> Self
    {
        let mut result = self.to_owned();
        result.borrow_mut().scalar_lmultiply_inplace(scalar);
        result
    }

    fn scalar_rdivide(&self, rational: impl Into<RationalType<Self>>) -> Self
    {
        let mut result = self.to_owned();
        result.borrow_mut().scalar_rdivide_inplace(rational.into());
        result
    }

    fn scalar_ldivide(&self, rational: impl Into<RationalType<Self>>) -> Self
    {
        self.scalar_rdivide(rational)
    }

    // Inplace binary operations acting on self.
    fn uminus_inplace(&mut self) -> &mut Self;

    fn add_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self;
    fn sub_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self;

    fn scalar_lmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self;
    fn scalar_rmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self
    {
        self.scalar_lmultiply_inplace(scalar)
    }

    fn scalar_rdivide_inplace(&mut self, rational: impl Into<RationalType<Self>>) -> &mut Self;
    fn scalar_ldivide_inplace(&mut self, rational: impl Into<RationalType<Self>>) -> &mut Self
    {
        self.borrow_mut().scalar_rdivide_inplace(rational)
    }


    // Some utility functions that can be optimised for specific cases but should always be implemented
    /// Efficient implementation of self += (other * s)
    fn add_scalar_rmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self
    {
        self.add_inplace(other.borrow().scalar_rmultiply(scalar))
    }

    /// Efficient implementation of self += (other * s)
    fn add_scalar_lmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self
    {
        self.add_inplace(other.borrow().scalar_lmultiply(scalar))
    }

    /// Efficient implementation of self -= (other * s)
    fn sub_scalar_rmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self
    {
        self.sub_inplace(other.borrow().scalar_rmultiply(scalar))
    }

    /// Efficient implementation of self -= (other * s)
    fn sub_scalar_lmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self
    {
        self.sub_inplace(other.borrow().scalar_lmultiply(scalar))
    }

    /// Efficient implementation of self += (other / s)
    fn add_scalar_rdivide(&mut self, other: impl Borrow<Self>, rational: impl Into<RationalType<Self>>) -> &mut Self
    {
        self.add_inplace(other.borrow().scalar_rdivide(rational))
    }

    /// Efficient implementation of self += ((1/s)*other)
    fn add_scalar_ldivide(&mut self, other: impl Borrow<Self>, rational: impl Into<RationalType<Self>>) -> &mut Self
    {
        self.add_inplace(other.borrow().scalar_ldivide(rational))
    }

    /// Efficient implementation of self -= (other / s)
    fn sub_scalar_rdivide(&mut self, other: impl Borrow<Self>, rational: impl Into<RationalType<Self>>) -> &mut Self
    {
        self.sub_inplace(other.borrow().scalar_rdivide(rational))
    }

    /// Efficient implementation of self -= ((1/s)*other)
    fn sub_scalar_ldivide(&mut self, other: impl Borrow<Self>, rational: impl Into<RationalType<Self>>) -> &mut Self
    {
        self.sub_inplace(other.borrow().scalar_ldivide(rational))
    }

}



pub trait VectorWithDegree : Vector
    where <Self as Vector>::BasisType: BasisWithDegree
{

    fn degree(&self) -> DegreeType;

}

impl<V> VectorWithDegree for V
    where V: Vector,
          for<'a> &'a V: IntoIterator<Item=(KeyType<V>, &'a ScalarField<V>)>,
          <V as Vector>::BasisType: BasisWithDegree
{
    fn degree(&self) -> DegreeType {
        match self.into_iter()
            .map(|(k, v)| <V as Vector>::BasisType::degree(&k))
            .max()
        {
            Some(val) => val,
            None => 0
        }
    }
}



mod deref_impl;
mod wrapper;
mod implementation;
mod dense_vector;
mod sparse_vector;

pub use dense_vector::{DenseVector, ResizeableDenseVector};
pub use sparse_vector::SparseVector;
pub use implementation::SimpleDenseVector;