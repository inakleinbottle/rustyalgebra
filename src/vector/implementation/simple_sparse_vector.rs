

use std::collections::{HashMap, hash_map::{Iter as HashMapIter, IterMut as HashMapIterMut}};
use std::marker::PhantomData;
use std::iter::IntoIterator;

use crate::basis::{Basis};
use crate::coefficients::CoefficientField;
use crate::vector::{Vector, VectorIteratorItem, VectorIteratorMutItem, IntoVecMutIter};
use std::borrow::{BorrowMut, Borrow};
use std::hash::Hash;



#[derive(Debug)]
pub struct SimpleSparseVector<'a, B, S, K>(HashMap<K, S>, PhantomData<&'a B>)
    where B: Basis<KeyType=K>,
          K: Hash + Eq + Clone,
          S: CoefficientField;


impl<'a, B, S, K> PartialEq for SimpleSparseVector<'a, B, S, K>
    where B: Basis<KeyType=K>,
          K: Hash + Eq + Clone,
          S: CoefficientField
{
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for (k, v) in self.0.iter() {
            match other.0.get(&k) {
                Some(v1) if *v1 != *v => return false,
                Some(_) => continue,
                None => return false
            }
        }
        true
    }
}



impl<'a, K: 'a, S: 'a> VectorIteratorItem<K, S> for (&'a K, &'a S)
{
    type KeyItem = &'a K;
    type ValueItem = &'a S;

    fn key(&self) -> Self::KeyItem {
        &*self.0
    }

    fn value(&self) -> Self::ValueItem {
        &*self.1
    }
}



impl<'a, B, S, K> IntoIterator for &'a SimpleSparseVector<'a, B, S, K>
    where B: Basis<KeyType=K>,
          K: 'a + Hash + Eq + Clone,
          S: 'a + CoefficientField
{
    type Item = (&'a K, &'a S);
    type IntoIter = HashMapIter<'a, K, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}




impl<'vec, 'a: 'vec, B, S, K> Vector<'vec> for SimpleSparseVector<'a, B, S, K>
    where B: 'static + Basis<KeyType=K>,
          K: 'static + Hash + Eq + Clone,
          S: 'static + CoefficientField
{
    type BasisType = B;
    type KeyType = B::KeyType;
    type ScalarType = S;
    type RationalType = S::RationalType;

    fn new() -> Self {
        Self(HashMap::new(), PhantomData)
    }

    fn from_key(key: impl Into<Self::KeyType>) -> Self {
        Self::from_key_scalar(key, S::ONE)
    }

    fn from_key_scalar(key: impl Into<Self::KeyType>, scalar: impl Into<Self::ScalarType>) -> Self {
        let mut inner = HashMap::new();
        inner.insert(key.into(), scalar.into());
        Self(inner, PhantomData)
    }

    fn from_iterator(iterator: impl IntoIterator<Item=(Self::KeyType, Self::ScalarType)>) -> Self {
        let mut inner = HashMap::<K, S>::new();

        for (key, val) in iterator.into_iter() {
            inner.insert(key, val);
        }
        SimpleSparseVector::<B, S, K>(inner, PhantomData)
    }

    fn swap(&mut self, other: impl BorrowMut<Self>) {
        todo!()
    }

    fn to_owned(&self) -> Self {
        todo!()
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn get(&self, key: impl Borrow<Self::KeyType>) -> Option<&Self::ScalarType> {
        self.0.get(key.borrow())
    }

    fn get_mut(&mut self, key: impl Borrow<Self::KeyType>) -> Option<&mut Self::ScalarType> {
        self.0.get_mut(key.borrow())
    }

    fn insert_single(&mut self, key: &Self::KeyType, value: impl Into<Self::ScalarType>) {
        self.0.insert(key.clone(), value.into());
    }

    fn insert(&mut self, iterator: impl IntoIterator<Item=(Self::KeyType, Self::ScalarType)>) {
        todo!()
    }

    fn erase(&mut self, key: impl Borrow<Self::KeyType>) {
        todo!()
    }

    fn uminus_inplace(&mut self) -> &mut Self {
        self.0.iter_mut().for_each(|(k, v)| { *v = S::uminus(v); });
        self
    }

    fn add_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        for (k, v) in other.borrow().0.iter() {
            match self.0.get_mut(k) {
                Some(i) => {
                    let r = S::add_inplace(i, v);
                    if *r == S::ZERO {
                        self.0.remove(k);
                    }
                },
                None => { self.0.insert(k.clone(), v.clone()); }
            };
        }
        self
    }

    fn sub_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        for (k, v) in other.borrow().0.iter() {
            match self.0.get_mut(k) {
                Some(i) => {
                    let r = S::sub_inplace(i, v);
                    if *r == S::ZERO {
                        self.0.remove(k);
                    }
                },
                None => { self.0.insert(k.clone(), S::uminus(v)); }
            };
        }
        self
    }

    fn scalar_lmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarType>) -> &mut Self {
        let s = scalar.into();
        if s == S::ZERO {
            self.0.clear();
            return self;
        }

        self.0.iter_mut().for_each(move |(_, v)| {
            S::mul_inplace(v, &s);
        });

        self
    }

    fn scalar_rdivide_inplace(&mut self, rational: impl Into<Self::RationalType>) -> &mut Self {
        let r = rational.into();

        self.0.iter_mut().for_each(move |(_, v)| {
            S::div_inplace(v, &r);
        });

        self
    }
}


#[cfg(test)]
mod tests {
/*
    use super::*;
    use crate::free_tensor::{TensorBasis, TensorKey};

    type TKey = TensorKey<3>;
    type TBasis = TensorBasis<3>;

    type Vect<'a> = SimpleSparseVector<'a, TBasis, f64, TKey>;


    #[test]
    fn test_add_inplace() {



    }



    #[test]
    fn test_iterator()
    {
        let vec = Vect::from_iterator(vec![
            (TKey::new(), 1.0),
            (TKey::from_letter(1), 2.0),
            (TKey::from_letter(2), 3.0)
        ]);

        let mut itr = vec.into_iter();

        let itm = itr.next().unwrap();
        assert_eq!(itm.key(), TKey::new());
        assert_eq!(itm.value(), 1.0);

        let itm = itr.next().unwrap();
        assert_eq!(itm.key(), TKey::from_letter(1));
        assert_eq!(itm.value(), 2.0);

        let itm = itr.next().unwrap();
        assert_eq!(itm.key(), TKey::from_letter(2));
        assert_eq!(itm.value(), 3.0);

        assert_eq!(itr.next(), None);

    }
*/

}