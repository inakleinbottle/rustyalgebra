
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use crate::vector::traits::IntoVectorIterator;
use crate::vector::*;



impl<'vec, U, V> Vector<'vec> for V
    where V: 'vec + Deref<Target=U> + DerefMut<Target=U> + From<U> + PartialEq,
          //  + IntoVectorIterator<'vec, U::KeyType, U::ScalarType>,
          U: Vector<'vec>
{
    type BasisType = U::BasisType;
    type KeyType = U::KeyType;
    type ScalarType = U::ScalarType;
    type RationalType = U::RationalType; 

    fn new() -> Self
    {
        Self::from(U::new())
    }

    fn from_key(key: impl Into<Self::KeyType>) -> Self {
        Self::from(U::from_key(key))
    }

    fn from_key_scalar(key: impl Into<Self::KeyType>, scalar: impl Into<Self::ScalarType>) -> Self {
        Self::from(U::from_key_scalar(key, scalar))
    }

    fn from_iterator(iterator: impl IntoIterator<Item=(Self::KeyType, Self::ScalarType)>) -> Self {
        Self::from(U::from_iterator(iterator))
    }

    fn swap(&mut self, mut other: impl BorrowMut<Self>) {
        self.deref_mut().swap(other.borrow_mut().deref_mut());
    }

    fn to_owned(&self) -> Self {
        self.deref().to_owned().into()
    }

    fn clear(&mut self) {
        self.deref_mut().clear();
    }

    fn get(&self, key: impl Borrow<Self::KeyType>) -> Option<&Self::ScalarType> {
        self.borrow().get(key)
    }

    fn get_mut(&mut self, key: impl Borrow<Self::KeyType>) -> Option<&mut Self::ScalarType> {
        self.borrow_mut().get_mut(key)
    }

    fn insert_single(&mut self, key: &Self::KeyType, value: impl Into<Self::ScalarType>) {
        self.deref_mut().insert_single(key, value);
    }

    fn insert(&mut self, iterator: impl IntoIterator<Item=(Self::KeyType, Self::ScalarType)>) {
        self.deref_mut().insert(iterator)
    }

    fn erase(&mut self, key: impl Borrow<Self::KeyType>) {
        self.deref_mut().erase(key);
    }

    // The underlying type might have optimised versions of these functions, so we should
    // pass through to those
    fn uminus(&self) -> Self {
        self.deref().uminus().into()
    }

    fn add(&self, other: impl Borrow<Self>) -> Self {
        self.deref().add(other.borrow().deref()).into()
    }

    fn sub(&self, other: impl Borrow<Self>) -> Self {
        self.deref().sub(other.borrow().deref()).into()
    }

    fn scalar_rmultiply(&self, scalar: impl Into<Self::ScalarType>) -> Self {
        self.deref().scalar_rmultiply(scalar).into()
    }

    fn scalar_lmultiply(&self, scalar: impl Into<Self::ScalarType>) -> Self {
        self.deref().scalar_lmultiply(scalar).into()
    }

    fn scalar_rdivide(&self, rational: impl Into<Self::RationalType>) -> Self {
        self.deref().scalar_rdivide(rational).into()
    }

    fn scalar_ldivide(&self, rational: impl Into<Self::RationalType>) -> Self {
        self.deref().scalar_ldivide(rational).into()
    }

    fn uminus_inplace(&mut self) -> &mut Self {
        self.deref_mut().uminus_inplace();
        self
    }

    fn add_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        self.deref_mut().add_inplace(other.borrow().deref());
        self
    }

    fn sub_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        self.deref_mut().sub_inplace(other.borrow().deref());
        self
    }

    fn scalar_lmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarType>) -> &mut Self {
        self.deref_mut().scalar_lmultiply_inplace(scalar);
        self
    }

    fn scalar_rmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarType>) -> &mut Self {
        self.deref_mut().scalar_rmultiply_inplace(scalar);
        self
    }

    fn scalar_rdivide_inplace(&mut self, rational: impl Into<Self::RationalType>) -> &mut Self {
        self.deref_mut().scalar_rdivide_inplace(rational);
        self
    }

    fn scalar_ldivide_inplace(&mut self, rational: impl Into<Self::RationalType>) -> &mut Self {
        self.deref_mut().scalar_ldivide_inplace(rational);
        self
    }

    fn add_scalar_rmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarType>) -> &mut Self {
        self.deref_mut().add_scalar_rmultiply(other.borrow().deref(), scalar);
        self
    }

    fn add_scalar_lmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarType>) -> &mut Self {
        self.deref_mut().add_scalar_lmultiply(other.borrow().deref(), scalar);
        self
    }

    fn sub_scalar_rmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarType>) -> &mut Self {
        self.deref_mut().sub_scalar_rmultiply(other.borrow().deref(), scalar);
        self
    }

    fn sub_scalar_lmultiply(&mut self, other: impl Borrow<Self>, scalar: impl Into<Self::ScalarType>) -> &mut Self {
        self.deref_mut().sub_scalar_lmultiply(other.borrow().deref(), scalar);
        self
    }

    fn add_scalar_rdivide(&mut self, other: impl Borrow<Self>, rational: impl Into<Self::RationalType>) -> &mut Self {
        self.deref_mut().add_scalar_rdivide(other.borrow().deref(), rational);
        self
    }

    fn add_scalar_ldivide(&mut self, other: impl Borrow<Self>, rational: impl Into<Self::RationalType>) -> &mut Self {
        self.deref_mut().add_scalar_ldivide(other.borrow().deref(), rational);
        self
    }

    fn sub_scalar_rdivide(&mut self, other: impl Borrow<Self>, rational: impl Into<Self::RationalType>) -> &mut Self {
        self.deref_mut().sub_scalar_rdivide(other.borrow().deref(), rational);
        self
    }

    fn sub_scalar_ldivide(&mut self, other: impl Borrow<Self>, rational: impl Into<Self::RationalType>) -> &mut Self {
        self.deref_mut().sub_scalar_ldivide(other.borrow().deref(), rational);
        self
    }
}

