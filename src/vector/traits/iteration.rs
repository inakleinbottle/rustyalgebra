
use std::borrow::Borrow;

use super::{Vector, KeyType, ScalarField};
use crate::DimensionType;


pub struct VectorIterItem<'a, K, S>(K, &'a S);

impl<'a, K, S> VectorIterItem<'a, K, S> {

    fn key(&self) -> &K { &self.0 }

    fn value(&self) -> &'a S { &self.1 }

}


impl<'a, K, S> From<(K, &'a S)> for VectorIterItem<'a, K, S>
{
    fn from(arg: (K, &'a S)) -> Self {
        Self(arg.0, arg.1)
    }
}

