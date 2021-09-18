
use crate::DegreeType;
use crate::basis::BasisWithDegree;

use super::{Vector, KeyType, ScalarField};

pub trait VectorWithDegree : Vector
    where <Self as Vector>::BasisType: BasisWithDegree
{

    fn degree(&self) -> DegreeType;

}




impl<V> VectorWithDegree for V
    where V: Vector,
          for<'a> &'a V: IntoIterator<Item=(KeyType<V>, &'a ScalarField<V>)>,
          <V as Vector>::BasisType: BasisWithDegree
{
    fn degree(&self) -> DegreeType {
        match self.into_iter()
            .map(|(k, v)| <V as Vector>::BasisType::degree(&k))
            .max()
        {
            Some(val) => val,
            None => 0
        }
    }
}