

use std::ops::{Deref, DerefMut};

use super::*;



impl<U, V> Vector for V
    where V: Deref<Target=U> + DerefMut<Target=U> + From<U> + PartialEq,
          U: Vector
{
    type BasisType = U::BasisType;
    type ScalarFieldType = U::ScalarFieldType;
    //type IteratorType = U::IteratorType;

    fn new() -> Self
    {
        Self::from(U::new())
    }

    fn from_key(key: impl Into<KeyType<Self>>) -> Self {
        Self::from(U::from_key(key))
    }

    fn from_key_scalar(key: impl Into<KeyType<Self>>, scalar: impl Into<Self::ScalarFieldType>) -> Self {
        Self::from(U::from_key_scalar(key, scalar))
    }

    fn from_iterator(iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>) -> Self {
        Self::from(U::from_iterator(iterator))
    }

    fn swap(&mut self, other: impl BorrowMut<Self>) {
        todo!()
    }

    fn to_owned(&self) -> Self {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn get(&self, key: impl AsRef<<<Self as Vector>::BasisType as Basis>::KeyType>) -> Option<&Self::ScalarFieldType> {
        todo!()
    }

    fn get_mut(&mut self, key: impl AsRef<KeyType<Self>>) -> Option<&mut Self::ScalarFieldType> {
        todo!()
    }

    fn insert_single(&mut self, key: impl Into<KeyType<Self>>, value: impl Into<Self::ScalarFieldType>) {
        todo!()
    }

    fn insert(&mut self, iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>) {
        todo!()
    }

    fn erase(&mut self, key: impl AsRef<KeyType<Self>>) {
        todo!()
    }
/*
    fn iter_pairs(&self) -> Self::IteratorType {
        self.borrow().iter_pairs()
    }
*/
    fn uminus_inplace(&mut self) -> &mut Self {
        todo!()
    }

    fn add_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        todo!()
    }

    fn sub_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        todo!()
    }

    fn scalar_lmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self {
        todo!()
    }

    fn scalar_rdivide_inplace(&mut self, rational: impl Into<RationalType<Self>>) -> &mut Self {
        todo!()
    }
}

