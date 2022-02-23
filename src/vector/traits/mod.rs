//! This module contains various traits for vectors.
//!
//!
//!

pub use vector::{Vector};
pub use dense_vector::{DenseVector, ResizeableDenseVector};
pub use sparse_vector::SparseVector;
pub use extension::VectorKeyExt;
pub use degree_vector::VectorWithDegree;
//pub use iteration::{VectorIterItem, VectorIter};

pub use iteration::{VectorIteratorItem, VectorIteratorMutItem, IntoVectorIterator};

mod as_vec;
mod dense_vector;
mod sparse_vector;
mod vector;
mod extension;
mod degree_vector;
mod iteration;


