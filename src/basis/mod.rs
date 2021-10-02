
use std::cmp::{PartialEq, Ordering};
use std::convert::Into;
use std::fmt::Display;

use crate::{DegreeType, DimensionType};

pub trait Basis
{
    type KeyType: PartialEq + Display;
}


pub trait OrderedBasis : Basis
{
    type KeyIterator: Iterator<Item = Self::KeyType>;

    fn compare(lhs: &Self::KeyType, rhs: &Self::KeyType) -> Ordering;

    fn iter_keys() -> Self::KeyIterator;

    fn key_to_index(key: &Self::KeyType) -> DimensionType;

    fn index_to_key(index: DimensionType) -> Self::KeyType;

    fn vector_dimension_for_key(key: &Self::KeyType) -> DimensionType;

    fn vector_dimension_for_index(index: impl Into<DimensionType>) -> DimensionType;

}


pub trait BasisWithDegree : Basis
{
    fn degree(key: &Self::KeyType) -> DegreeType;
}


pub trait OrderedBasisWithDegree : OrderedBasis + BasisWithDegree
{
    fn index_to_degree(index: DimensionType) -> DegreeType;

    fn start_of_degree(deg: DegreeType) -> DimensionType;
}
