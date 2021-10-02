#![feature(generic_associated_types)]


use std::iter::Iterator;
use crate::basis::Basis;
use crate::coefficients::CoefficientField;
use crate::vector::Vector;


pub trait VectorIteratorItem<'a, K, S>
{
    fn key(&self) -> &K;
    fn value(&self) -> &S;
}


pub trait VectorIterator<B, S> : Iterator
    where B: Basis, S: CoefficientField
{

}
/*
pub struct VectorIterItem<'a, K, S>(K, &'a S);

impl<'a, K, S> VectorIterItem<'a, K, S> {

    fn key(&self) -> &'a K { &self.0 }

    fn value(&self) -> &'a S { &self.1 }

}


impl<'a, K, S> From<(K, &'a S)> for VectorIterItem<'a, K, S>
{
    fn from(arg: (K, &'a S)) -> Self {
        Self(arg.0, arg.1)
    }
}


pub trait VectorIter<'a, K: 'a , S: 'a>
{
    type IteratorType: Iterator<Item=VectorIterItem<'a, K, S>>;

    fn iter_items(&self) -> Self::IteratorType;

}
*/
