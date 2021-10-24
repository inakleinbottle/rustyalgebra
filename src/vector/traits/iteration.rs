#![feature(generic_associated_types)]

use std::iter::Iterator;
use std::ops::{Deref,DerefMut};


use crate::basis::Basis;
use crate::coefficients::CoefficientField;
use crate::vector::Vector;


pub trait VectorIteratorItem<K, S>
{
    type KeyItem: Deref<Target=K>;
    type ValueItem: Deref<Target=S>;

    fn key(&self) -> Self::KeyItem;
    fn value(&self) -> Self::ValueItem;
}


pub trait VectorIteratorMutItem<K, S>
{
    type KeyItem: Deref<Target=K>;
    type ValueItem: DerefMut<Target=S>;

    fn key(&self) -> Self::KeyItem;
    fn value(&self) -> Self::ValueItem;
}


pub trait VectorIterator<B, S> : Iterator
    where B: Basis, S: CoefficientField
{}


pub trait IntoVecIter<K, S> : IntoIterator
    where <Self as IntoIterator>::Item: VectorIteratorItem<K, S>
{}


pub trait IntoVecMutIter<K, S> : IntoIterator
    where <Self as IntoIterator>::Item: VectorIteratorMutItem<K, S>
{}

type Item<I> = <I as IntoIterator>::Item;

pub trait IntoVectorIterator<K, S>
{}

impl<'a, T: 'a, K, S> IntoVectorIterator<K, S> for T
    where &'a T: IntoIterator,
          //&'_ mut T: IntoIterator,
          Item<&'a T>: VectorIteratorItem<K, S>//,
          //Item<&'_ mut T>: VectorIteratorMutItem<K, S>
{}