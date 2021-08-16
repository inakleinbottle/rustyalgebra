

use std::collections::HashMap;
use std::marker::PhantomData;


use crate::basis::Basis;
use crate::coefficients::CoefficientField;
use crate::vector::{Vector, KeyType, RationalType};
use std::borrow::{BorrowMut, Borrow};
use std::hash::Hash;


#[derive(Debug)]
pub struct SimpleSparseVector<B, S, K>(HashMap<K, S>, PhantomData<B>)
    where B: Basis<KeyType=K>,
          K: Hash + Eq + Clone,
          S: CoefficientField;


impl<B, S, K> PartialEq for SimpleSparseVector<B, S, K>
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


impl<B, S, K> Vector for SimpleSparseVector<B, S, K>
    where B: Basis<KeyType=K>,
          K: Hash + Eq + Clone,
          S: CoefficientField
{
    type BasisType = B;
    type ScalarFieldType = S;

    fn new() -> Self {
        Self(HashMap::new(), PhantomData)
    }

    fn from_key(key: impl Into<KeyType<Self>>) -> Self {
        Self::from_key_scalar(key, S::ONE)
    }

    fn from_key_scalar(key: impl Into<KeyType<Self>>, scalar: impl Into<Self::ScalarFieldType>) -> Self {
        let mut inner = HashMap::new();
        inner.insert(key.into(), scalar.into());
        Self(inner, PhantomData)
    }

    fn from_iterator(iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>) -> Self {
        todo!()
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

    fn get(&self, key: impl AsRef<<<Self as Vector>::BasisType as Basis>::KeyType>) -> Option<&Self::ScalarFieldType> {
        self.0.get(key.as_ref())
    }

    fn get_mut(&mut self, key: impl AsRef<KeyType<Self>>) -> Option<&mut Self::ScalarFieldType> {
        self.0.get_mut(key.as_ref())
    }

    fn insert_single(&mut self, key: impl Into<KeyType<Self>>, value: impl Into<Self::ScalarFieldType>) {
        self.0.insert(key.into(), value.into());
    }

    fn insert(&mut self, iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>) {
        todo!()
    }

    fn erase(&mut self, key: impl AsRef<KeyType<Self>>) {
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

    fn scalar_lmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self {
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

    fn scalar_rdivide_inplace(&mut self, rational: impl Into<RationalType<Self>>) -> &mut Self {
        let r = rational.into();

        self.0.iter_mut().for_each(move |(_, v)| {
            S::div_inplace(v, &r);
        });

        self
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    use crate::free_tensor::{TensorBasis, TensorKey};

    type TKey = TensorKey<3>;
    type TBasis = TensorBasis<3>;

    type Vect = SimpleSparseVector<TBasis, f64, TKey>;


    #[test]
    fn test_add_inplace() {



    }

}