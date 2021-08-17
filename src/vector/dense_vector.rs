
use std::slice::SliceIndex;

use super::Vector;
use crate::DimensionType;


/// Trait describing a dense vector type.
///
/// A dense vector should provide slice-like views into the underlying data
/// in the vector. As such, the data should be stored contiguously in memory.
pub trait DenseVector : Vector
{

    fn as_slice(&self) -> &[Self::ScalarFieldType];

    fn as_mut_slice(&mut self) -> &mut [Self::ScalarFieldType];

    fn get_slice<I>(&self, index: I) -> Option<&I::Output>
        where I: SliceIndex<[Self::ScalarFieldType]>
    {
        self.as_slice().get(index)
    }

    fn get_mut_slice<I>(&mut self, index: I) -> Option<&mut I::Output>
        where I: SliceIndex<[Self::ScalarFieldType]>
    {
        self.as_mut_slice().get_mut(index)
    }

}



pub trait ResizeableDenseVector : DenseVector
{

    fn resize(&mut self, new_dim: DimensionType);

}