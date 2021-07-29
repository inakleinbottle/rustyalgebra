


use std::cmp;
use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;

use super::{Vector, KeyType, RationalType};
use crate::coefficients::{CoefficientField};
use crate::basis::{OrderedBasis};
use crate::DimensionType;



pub struct OwnedDenseVector<B: OrderedBasis, S: CoefficientField>(Vec<S>, PhantomData<B>);

impl<B, S> Clone for OwnedDenseVector<B, S>
    where B: OrderedBasis, S: CoefficientField
{
    fn clone(&self) -> Self
    {
        Self(self.0.clone(), PhantomData)
    }
}

impl<B, S> AsRef<OwnedDenseVector<B, S>> for OwnedDenseVector<B, S>
    where B: OrderedBasis, S: CoefficientField
{
    fn as_ref(&self) -> &OwnedDenseVector<B, S> {
        self
    }
}

impl<B, S> AsMut<OwnedDenseVector<B, S>> for OwnedDenseVector<B ,S>
    where B: OrderedBasis, S: CoefficientField
{
    fn as_mut(&mut self) -> &mut OwnedDenseVector<B, S> { self }
}


impl<B: OrderedBasis, S: CoefficientField> OwnedDenseVector<B, S> {

    pub fn new() -> OwnedDenseVector<B, S>
    {
        OwnedDenseVector(Vec::new(), PhantomData)
    }

    pub fn from(vec: Vec<S>) -> OwnedDenseVector<B, S>
    {
        OwnedDenseVector(vec, PhantomData)
    }

    pub fn from_dimension(size: DimensionType) -> OwnedDenseVector<B, S>
    {
        OwnedDenseVector(vec![S::ZERO; size], PhantomData)
    }

    pub fn resize(&mut self, size: DimensionType)
    {
        self.0.resize(size, S::ZERO);
    }

}


impl<B, S> Vector for OwnedDenseVector<B, S>
    where B: OrderedBasis,
          S: CoefficientField
{
    type BasisType = B;
    type ScalarFieldType = S;
    type OwnedVectorType = Self;

    fn new() -> Self::OwnedVectorType {
        Self::OwnedVectorType::new()
    }

    fn from_key(key: impl Into<KeyType<Self>>) -> Self::OwnedVectorType {
        Self::from_key_scalar(key, Self::ScalarFieldType::ONE)
    }

    fn from_key_scalar(key: impl Into<KeyType<Self>>, scalar: impl Into<Self::ScalarFieldType>) -> Self::OwnedVectorType {
        let k = key.into();
        let size = Self::BasisType::vector_dimension_for_key(&k);
        let mut new_vect = Self::from_dimension(size);
        unsafe {
            // We have just created the vector to have at least size+1 elements
            // So this is safe
            *new_vect.0.get_unchecked_mut(Self::BasisType::key_to_index(&k)) = scalar.into();
        }
        new_vect
    }

    fn from_iterator(iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>) -> Self::OwnedVectorType {
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
                Self::ScalarFieldType::add_inplace(result.0.get_unchecked_mut(i), &v);
            }
        }

        result
    }

    fn swap(&mut self, other: impl BorrowMut<Self>) {
        todo!()
    }

    fn to_owned(&self) -> Self::OwnedVectorType {
        Self::from(self.0.clone())
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn get(&self, key: impl AsRef<KeyType<Self>>) -> Option<&Self::ScalarFieldType> {
        self.0.get(B::key_to_index(key.as_ref()))
    }

    fn get_mut(&mut self, key: impl AsRef<KeyType<Self>>) -> &mut Self::ScalarFieldType {
        self.0.get_mut(B::key_to_index(key.as_ref()))
            .expect("Requested key is not present in this vector")
    }

    fn insert_single(&mut self, key: impl AsRef<KeyType<Self>>, value: impl Into<Self::ScalarFieldType>) {
        todo!()
    }

    fn insert(&mut self, iterator: impl IntoIterator<Item=(KeyType<Self>, Self::ScalarFieldType)>) {
        todo!()
    }


    fn erase(&mut self, key: impl AsRef<KeyType<Self>>) {
        todo!()
    }

    fn uminus_inplace(&mut self) -> &mut Self {
        for val in self.0.iter_mut() {
            *val = Self::ScalarFieldType::uminus(val);
        }
        self
    }

    fn add_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        let lhs_vec = other.borrow();

        if lhs_vec.0.len() > self.0.len() {
            panic!("The size of the right hand side is larger than the left hand side")
        }

        for (lhs,rhs) in self.0.iter_mut().zip(&lhs_vec.0) {
            Self::ScalarFieldType::add_inplace(lhs, rhs);
        }

        self
    }

    fn sub_inplace(&mut self, other: impl Borrow<Self>) -> &mut Self {
        let lhs_vec = other.borrow();

        if lhs_vec.0.len() > self.0.len() {
            panic!("The size of the right hand side is larger than the left hand side")
        }

        for (lhs,rhs) in self.0.iter_mut().zip(&lhs_vec.0) {
            Self::ScalarFieldType::sub_inplace(lhs, rhs);
        }

        self
    }

    fn scalar_lmultiply_inplace(&mut self, scalar: impl Into<Self::ScalarFieldType>) -> &mut Self {
        let val = scalar.into();

        for lhs in self.0.iter_mut() {
            Self::ScalarFieldType::mul_inplace(lhs, &val);
        }

        self
    }

    fn scalar_rdivide_inplace(&mut self, rational: impl Into<RationalType<Self>>) -> &mut Self {
        let val = rational.into();

        for lhs in self.0.iter_mut() {
            Self::ScalarFieldType::div_inplace(lhs, &val);
        }

        self
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

        fn compare(lhs: &Self::KeyType, rhs: &Self::KeyType) -> Ordering
        {
            cmp::Ord::cmp(lhs, rhs)
        }

        fn first_key() -> Self::KeyType {
            0
        }

        fn next_key(key: &Self::KeyType) -> Self::KeyType {
            key + 1
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


    type DenseVec = OwnedDenseVector<IntegerBasis, f32>;

    #[test]
    fn test_create_empty() {
        let vec = DenseVec::new();

        assert_eq!(vec.0, vec![]);
    }

    #[test]
    fn test_create_from_iterator() {
        let buffer: Vec<(u8, f32)> = vec![(0, 0.0), (1, 1.0), (2, 2.0), (3, 3.0)];

        let expected = DenseVec::from({
            let mut expect_buffer = vec![0.0f32, 1.0f32, 2.0f32, 3.0f32];
            expect_buffer.resize(u8::MAX as usize, 0.0);
            expect_buffer
        });

        let result = DenseVec::from_iterator(buffer);

        assert_eq!(result.0, expected.0);
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

        assert_eq!(result.0, expected.0);
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

        assert_eq!(result.0, expected.0);
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

        assert_eq!(result.0, expected.0);
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

        assert_eq!(result.0, expected.0);
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

        assert_eq!(result.0, expected.0);
    }



}