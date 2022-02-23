#![feature(generic_associated_types)]

use std::iter::Iterator;
use std::ops::{Deref,DerefMut};


use crate::basis::Basis;
use crate::coefficients::CoefficientField;
use crate::vector::Vector;


pub trait VectorIteratorItem<'vec, K, S>
{
    type KeyItem: 'vec + Deref<Target=K>;
    type ValueItem: 'vec + Deref<Target=S>;

    fn key(&self) -> Self::KeyItem;
    fn value(&self) -> Self::ValueItem;
}


pub trait VectorIteratorMutItem<'vec, K, S>
{
    type KeyItem: 'vec + Deref<Target=K>;
    type ValueItem: 'vec + DerefMut<Target=S>;

    fn key(&self) -> Self::KeyItem;
    fn value(&self) -> Self::ValueItem;
}



type Item<I> = <I as IntoIterator>::Item;

pub trait IntoVectorIterator<'vec, K, S>
    where Self: 'vec,
          &'vec Self: IntoIterator,
          &'vec mut Self: IntoIterator,
          Item<&'vec Self>: VectorIteratorItem<'vec, K, S>,
          Item<&'vec mut Self>: VectorIteratorMutItem<'vec, K, S>
{}

impl<'vec, T: 'vec, K, S> IntoVectorIterator<'vec, K, S> for T
    where &'vec T: IntoIterator,
          &'vec mut T: IntoIterator,
          Item<&'vec T>: VectorIteratorItem<'vec, K, S>,
          Item<&'vec mut T>: VectorIteratorMutItem<'vec, K, S>
{}