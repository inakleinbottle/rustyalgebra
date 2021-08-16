
use std::convert::{AsRef, AsMut, Into, From};
use crate::DegreeType;
use crate::vector::Vector;


use super::{TensorBasis, FreeTensor};


// This is a little hack I took from the rust-lang forum to emulate a where clause for
// const generics.
// See https://internals.rust-lang.org/t/const-generics-where-restrictions/12742/6
struct If<const B: bool>;

trait True {}

impl True for If<true> {}



