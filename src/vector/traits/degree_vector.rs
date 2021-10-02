
use crate::DegreeType;
use crate::basis::BasisWithDegree;

use super::{Vector, KeyType, ScalarField};

pub trait VectorWithDegree : Vector
    where <Self as Vector>::BasisType: BasisWithDegree
{

    fn degree(&self) -> DegreeType;

}



/*
impl<'a, V> VectorWithDegree<'a> for V
    where V: Vector<'a>,
          <V as Vector<'a>>::BasisType: BasisWithDegree
{
    fn degree(&self) -> DegreeType {
        match self.iter_item()
            .map(|(k, v)| <V as Vector<'a>>::BasisType::degree(&k))
            .max()
        {
            Some(val) => val,
            None => 0
        }
    }
}
*/
