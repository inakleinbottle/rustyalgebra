
use crate::DegreeType;
use crate::basis::BasisWithDegree;

use super::{Vector};

pub trait VectorWithDegree<'vec> : Vector<'vec>
    where <Self as Vector<'vec>>::BasisType: BasisWithDegree
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
