

pub type DegreeType = u32;
pub type DimensionType = usize;
pub type SignedDimensionType = isize;
pub type LetterType = u32;

pub(crate) mod implementation;

pub mod coefficients;
pub mod vector;
pub mod basis;
pub mod algebra;
pub mod free_tensor;
pub mod lie;




