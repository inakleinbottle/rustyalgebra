

use std::fmt::{self, Display, Formatter};
use std::cmp;
use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use std::ops::{Range, Index};
use std::mem;
use std::slice::{SliceIndex, Iter};



use crate::vector::{Vector, KeyType, RationalType};
use crate::coefficients::{CoefficientField};
use crate::basis::{OrderedBasis, OrderedBasisWithDegree};
use crate::{DimensionType, DegreeType};
use crate::vector::{VectorWithDegree, ScalarField, DenseVector};





#[derive(Debug, PartialEq)]
enum SimpleDenseVectorData<'a, S: CoefficientField>
{
    Owned(Vec<S>),
    Borrowed(&'a [S]),
    BorrowedMut(&'a mut [S])
}
use SimpleDenseVectorData::*;
use std::iter::Zip;


#[derive(Debug)]
pub struct SimpleDenseVector<'a, B: OrderedBasis, S: CoefficientField>(
    SimpleDenseVectorData<'a, S>, PhantomData<B>
);



impl<'a, B, S> Clone for SimpleDenseVector<'a, B, S>
    where B: OrderedBasis, S: CoefficientField
{
    fn clone(&self) -> Self
    {
        Self(match &self.0 {
            Owned(v) => Owned(v.clone()),
            Borrowed(v) => Owned(v.to_vec()),
            BorrowedMut(v) => Owned(v.to_vec())
        }, PhantomData)
    }
}

impl<'a, B: OrderedBasis, S: CoefficientField> SimpleDenseVector<'a, B, S> {

    pub fn new() -> SimpleDenseVector<'a, B, S>
    {
        SimpleDenseVector(Owned(Vec::new()), PhantomData)
    }

    pub fn from(vec: Vec<S>) -> SimpleDenseVector<'a, B, S>
    {
        SimpleDenseVector(Owned(vec), PhantomData)
    }

    pub fn from_dimension(size: DimensionType) -> SimpleDenseVector<'a, B, S>
    {
        SimpleDenseVector(Owned(vec![S::ZERO; size]), PhantomData)
    }



    fn to_owned_with_size(&mut self, resize: Option<DimensionType>)
    {
        let sz = resize.unwrap_or(self.size());
        let mut new_vec = Vec::with_capacity(sz);

        match self.0 {
            Borrowed(ref v) => new_vec.extend_from_slice(v),
            BorrowedMut(ref v) => new_vec.extend_from_slice(v),
            Owned(_) => unreachable!()
        }

        if let Some(dim) = resize {
            new_vec.resize(dim, S::ZERO);
        }


        self.0 = Owned(new_vec)
        //let old = mem::replace(&mut self.0, Owned(new_vec));
    }

    pub fn resize(&mut self, size: DimensionType)
    {
        match &mut self.0 {
            Owned(v) => v.resize(size, S::ZERO),
            BorrowedMut(_) | Borrowed(_) => Self::to_owned_with_size(self, Some(size))
        }
    }

    pub fn size(&self) -> DimensionType
    {
        match &self.0 {
            Owned(v) => v.len(),
            BorrowedMut(v) => v.len(),
            Borrowed(v) => v.len()
        }
    }

}

impl<'a, B: OrderedBasis, S: CoefficientField> PartialEq for SimpleDenseVector<'a, B, S> {
    fn eq(&self, other: &Self) -> bool {
        let a = match &self.0 {
            Owned(ref v) => v.as_slice(),
            BorrowedMut(v) => v,
            Borrowed(v ) => v
        };

        let b = match &other.0 {
            Owned(ref v) => v.as_slice(),
            BorrowedMut(v) => v,
            Borrowed(v ) => v
        };

        a == b
    }
}


impl<'a, B: OrderedBasis, S: CoefficientField> SimpleDenseVector<'a, B, S> {

    pub(crate) fn as_slice(&self) -> &[S]
    {
        match &self.0 {
            Owned(v) => v,
            Borrowed(v) => *v,
            BorrowedMut(v) => *v
        }
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [S]
    {

        if let Borrowed(v) = self.0 {
            self.0 = Owned(v.to_vec());
        }

        match &mut self.0 {
            Owned(v) => v,
            BorrowedMut(v) => *v,
            Borrowed(_) => unreachable!()
        }
    }

}


impl<'a, B, S> Vector for SimpleDenseVector<'a, B, S>
    where B: OrderedBasis,
          S: CoefficientField
{
    type BasisType = B;
    type ScalarFieldType = S;
    //type IteratorType = Zip<B::KeyIterator, Iter<'a, S>>;

    fn new() -> Self {
        Self::new()
    }

    fn from_key(key: impl Into<KeyType<Self>>) -> Self {
        Self::from_key_scalar(key, Self::ScalarFieldType::ONE)
    }

    fn from_key_scalar(key: impl Into<KeyType<Self>>, scalar: impl Into<Self::ScalarFieldType>) -> Self {
        let k = key.into();
        let size = Self::BasisType::vector_dimension_for_key(&k);
        let mut new_vect = Self::from_dimension(size);
        unsafe {
            // We have just created the vector to have at least size+1 elements
            // So this is safe
            *new_vect.as_mut_slice()
                .get_unchecked_mut(Self::BasisType::key_to_index(&k)) = scalar.into();
        }
        new_vect
    }

    fn from_iterator(iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>) -> Self {
        let mut vec: Vec<(DimensionType, Self::ScalarFieldType)> = iterator.into_iter()
            .map(|(k, s)| { (Self::BasisType::key_to_index(&k), s) })
            .collect();

        if vec.is_empty() {
            return Self::new();
        }

        vec.sort_by(|(i1, _v1), (i2, _v2)| { cmp::Ord::cmp(i1, i2) });
        let dimension = Self::BasisType::vector_dimension_for_index(vec.last().unwrap().0);

        let mut result = Self::from_dimension(dimension);

        // We have just created a vector large enough to accomodate all these indices.
        unsafe {
            for (i, v) in vec.into_iter() {
                Self::ScalarFieldType::add_inplace(result.as_mut_slice().get_unchecked_mut(i), &v);
            }
        }

        result
    }

    fn swap(&mut self, other: impl BorrowMut<Self>) {
        todo!()
    }

    fn to_owned(&self) -> Self {
        Self::from(Vec::from(self.as_slice()))
    }

    fn clear(&mut self) {
        match &mut self.0 {
            Owned(v) => v.clear(),
            BorrowedMut(v) => {v.fill(S::ZERO)},
            Borrowed(_) => self.0 = Owned(Vec::new())
        };
    }

    fn get(&self, key: impl AsRef<KeyType<Self>>) -> Option<&Self::ScalarFieldType> {
        self.as_slice().get(B::key_to_index(key.as_ref()))
    }

    fn get_mut(&mut self, key: impl AsRef<KeyType<Self>>) -> Option<&mut Self::ScalarFieldType> {
        self.as_mut_slice().get_mut(B::key_to_index(key.as_ref()))
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
    fn iter_pairs(&self) -> Self::IteratorType
    {
        let a = match &self.0 {
            Owned(ref v) => v.as_slice(),
            Borrowed(v) => v,
            BorrowedMut(v) => v
        };

        <Self as Vector>::BasisType::iter_keys().zip()
    }
*/

    fn uminus_inplace(&mut self) -> &mut Self {
        for val in self.as_mut_slice() {
            *val = Self::ScalarFieldType::uminus(val);
        }
        self
    }

    fn add_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        let lhs_vec = other.borrow();

        if let Borrowed(v) = self.0 {
            self.0 = Owned(v.to_vec());
        }

        if lhs_vec.size() > self.size() {
            self.resize(lhs_vec.size());
        }

        for (lhs, rhs) in self.as_mut_slice().iter_mut().zip(lhs_vec.as_slice()) {
            Self::ScalarFieldType::add_inplace(lhs, rhs);
        }

        self
    }

    fn sub_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        let lhs_vec = other.borrow();

        if lhs_vec.size() > self.size() {
            self.resize(lhs_vec.size());
        }

        for (lhs, rhs) in self.as_mut_slice().iter_mut().zip(lhs_vec.as_slice()) {
            Self::ScalarFieldType::sub_inplace(lhs, rhs);
        }

        self
    }

    fn scalar_lmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self {
        let val = scalar.into();

        if let Borrowed(v) = self.0 {
            self.0 = Owned(v.to_vec());
        }

        for lhs in self.as_mut_slice() {
            Self::ScalarFieldType::mul_inplace(lhs, &val);
        }

        self
    }

    fn scalar_rdivide_inplace(&mut self, rational: impl Into<RationalType<Self>>) -> &mut Self {
        let val = rational.into();

        for lhs in self.as_mut_slice() {
            Self::ScalarFieldType::div_inplace(lhs, &val);
        }

        self
    }
}


impl<'a, B, S> DenseVector for SimpleDenseVector<'a, B, S>
    where B: OrderedBasis,
          S: CoefficientField
{
    fn as_slice(&self) -> &[Self::ScalarFieldType] {
        match &self.0 {
            Owned(v) => v,
            Borrowed(v) => *v,
            BorrowedMut(v) => *v
        }
    }

    fn as_mut_slice(&mut self) -> &mut [Self::ScalarFieldType] {
        if let Borrowed(v) = self.0 {
            self.0 = Owned(v.to_vec());
        }

        match &mut self.0 {
            Owned(v) => v,
            BorrowedMut(v) => *v,
            Borrowed(_) => unreachable!()
        }
    }
}


impl<'a, B, S> VectorWithDegree<B> for SimpleDenseVector<'a, B, S>
    where B: OrderedBasisWithDegree,
          S: CoefficientField
{
    fn degree(&self) -> DegreeType {
        let size = self.size();
        match self.size() {
            i if i == 0 => 0,
            i => <B as OrderedBasisWithDegree>::index_to_degree(i - 1)
        }

    }

}

impl<'a, B, S, K> Display for SimpleDenseVector<'a, B, S>
    where B: OrderedBasis<KeyType = K>,
          S: CoefficientField + Display,
          K: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (k, v) in B::iter_keys().zip(self.as_slice())  {
            write!(f, "{}{}", v, k)?;
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    use crate::basis::Basis;
    use std::cmp::Ordering;

    struct IntegerBasis;

    impl Basis for IntegerBasis
    {
        type KeyType = u8;
    }

    impl OrderedBasis for IntegerBasis
    {

        type KeyIterator = Range<Self::KeyType>;

        fn compare(lhs: &Self::KeyType, rhs: &Self::KeyType) -> Ordering
        {
            cmp::Ord::cmp(lhs, rhs)
        }

        fn iter_keys() -> Self::KeyIterator {
            todo!()
        }

        fn key_to_index(key: &Self::KeyType) -> DimensionType {
            *key as DimensionType
        }

        fn index_to_key(index: DimensionType) -> Self::KeyType {
            if index > Self::KeyType::MAX as DimensionType {
                panic!("{} is not a valid key", index);
            }
            index as Self::KeyType
        }

        fn vector_dimension_for_key(key: &Self::KeyType) -> DimensionType {
            Self::KeyType::MAX as DimensionType
        }

        fn vector_dimension_for_index(index: impl Into<DimensionType>) -> DimensionType {
            Self::KeyType::MAX as DimensionType
        }
    }


    type DenseVec<'a> = SimpleDenseVector<'a, IntegerBasis, f32>;



    #[test]
    fn test_create_from_iterator() {
        let buffer: Vec<(u8, f32)> = vec![(0, 0.0), (1, 1.0), (2, 2.0), (3, 3.0)];

        let expected = DenseVec::from({
            let mut expect_buffer = vec![0.0f32, 1.0f32, 2.0f32, 3.0f32];
            expect_buffer.resize(u8::MAX as usize, 0.0);
            expect_buffer
        });

        let result = DenseVec::from_iterator(buffer);

        assert_eq!(result.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_create_from_iterator_repeated_element() {
        let buffer: Vec<(u8, f32)> = vec![(0, 0.0), (1, 1.0), (1, 1.0), (2, 2.0), (3, 3.0)];

        let expected = DenseVec::from({
            let mut expect_buffer = vec![0.0f32, 2.0f32, 2.0f32, 3.0f32];
            expect_buffer.resize(u8::MAX as usize, 0.0);
            expect_buffer
        });

        let result = DenseVec::from_iterator(buffer);

        assert_eq!(result.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_create_from_iterator_unordered() {
        let mut buffer: Vec<(u8, f32)> = vec![(3, 3.0), (1, 1.0), (0, 0.0),(2, 2.0)];

        let expected = DenseVec::from({
            let mut expect_buffer = vec![0.0f32, 1.0f32, 2.0f32, 3.0f32];
            expect_buffer.resize(u8::MAX as usize, 0.0);
            expect_buffer
        });

        let result = DenseVec::from_iterator(buffer);

        assert_eq!(result.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_add_vectors() {
        let v1 = DenseVec::from_key(0u8);
        let v2 = DenseVec::from_key(1u8);

        let expected = DenseVec::from({
            let mut v = vec![0.0f32; u8::MAX as usize];
            v[0] = 1.0f32;
            v[1] = 1.0f32;
            v
        });

        let result = v1.add(v2);

        assert_eq!(result.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_sub_vectors() {
        let v1 = DenseVec::from_key(0u8);
        let v2 = DenseVec::from_key(1u8);

        let expected = DenseVec::from({
            let mut v = vec![0.0f32; u8::MAX as usize];
            v[0] = 1.0f32;
            v[1] = -1.0f32;
            v
        });

        let result = v1.sub(v2);

        assert_eq!(result.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_scalar_multiply() {
        let v1 = DenseVec::from({
            let mut vec: Vec<f32> = Vec::new();
            let mut val = 0.0f32;
            vec.fill_with(move || { val += 1.0; return val.clone() });
            vec
        });

        let expected = DenseVec::from({
            let mut vec: Vec<f32> =  Vec::new();
            let mut val = 0.0f32;
            vec.fill_with(move || { val += 2.0; return val.clone() });
            vec
        });

        let result = v1.scalar_lmultiply(2.0f32);

        assert_eq!(result.as_slice(), expected.as_slice());
    }



}