//! Traits defining vector-like behaviour and implementations.
//!
//! Vectors, or similar constructions, form the backbone of all structures described by this library.
//! The `Vector` trait describes the basic behaviours that a representation of a vector should
//! satisfy. These behaviours include simple construction methods, vector arithmetic, and iteration
//! over pairs consisting of one element from the basis - the *key* - and one element from the
//! scalar field - the *value*. In this way, a basic vector type should behave somewhat like a
//! `HashMap`, associating basis elements with a coefficient taken from the coordinate field.
//!
//! You might notice that the `Vector` trait defines its own arithmetic operations rather than
//! requiring that the type implements the arithmetic traits provided by the language. There are two
//! reasons for this. The first reason is simplicity. The built-in arithmetic traits are have to be
//! implemented for either the type itself or for a reference to the type, which in practice means
//! juggling a number of lifetime parameters. Defining our own methods allow us to take a a type
//! implementing `Borrow<Self>` as the second argument (where appropriate), which takes care of
//! numerous combinations simultaneously. The second reason is to avoid ambiguity. It is possible
//! that we may wish to implement several different vector structures on a data container, and
//! implementing the built-in traits would make this tricky.
//!
//! The `Vector` trait expresses it's connection to the basis type and scalar coefficient field by
//! means of the associated types `BasisType` and `ScalarFieldType`. To make accessing the numerous
//! coincidental types, we define three helper alias types `KeyType`, `ScalarField`, and
//! `RationalType`, which take the vector type as a generic and correctly disambiguate associated
//! types.
//!
//! Notice that the `Vector` trait does not require a basis to be ordered. This is a useful position
//! that yields better utility overall, even if most bases that we consider are ordered.
//!
//! Basis types that have a degree should implement the `VectorWithDegree` trait, which adds the a
//! method for computing the maximum degree represented by a vector. The default implementation
//! simply computes the maximum degree over the iterable of key-value pairs (provided by the
//! `Vector` trait). You may wish to implement an optimised implementation.


use std::iter::IntoIterator;


pub use implementation::SimpleDenseVector;
pub use traits::*;
//pub use wrapper::VectorWrapper;





pub mod implementation;
pub mod traits;
//pub mod wrapper;




