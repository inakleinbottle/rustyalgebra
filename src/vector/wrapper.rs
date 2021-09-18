
use std::ops;
use std::borrow::{Borrow, BorrowMut};

use crate::vector::{Vector, KeyType};
use crate::coefficients::CoefficientField;



#[derive(Debug, PartialEq)]
pub struct VectorWrapper<V>(V)
    where V: 'static + Vector;


impl<V> VectorWrapper<V>
    where V: 'static + Vector
{

    pub fn new() -> Self
    {
        Self(V::new())
    }

    fn from_inner(inner: V) -> Self
    {
        Self(inner)
    }

}


impl<T, V> ops::Add<T> for VectorWrapper<V>
    where V: 'static + Vector,
          T: Borrow<VectorWrapper<V>>
{
    type Output = VectorWrapper<V>;

    fn add(self, rhs: T) -> Self::Output {
        Self(self.0.add(&rhs.borrow().0))
    }
}

impl<T, V> ops::Add<T> for &VectorWrapper<V>
    where V: 'static + Vector,
          T: Borrow<VectorWrapper<V>>
{
    type Output = VectorWrapper<V>;

    fn add(self, rhs: T) -> Self::Output {
        VectorWrapper(self.0.add(&rhs.borrow().0))
    }
}

impl<T, V> ops::Sub<T> for VectorWrapper<V>
    where V: 'static + Vector,
          T: Borrow<VectorWrapper<V>>
{
    type Output = VectorWrapper<V>;

    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0.sub(&rhs.borrow().0))
    }
}

impl<T, V> ops::Sub<T> for &VectorWrapper<V>
    where V: 'static + Vector,
          T: Borrow<VectorWrapper<V>>
{
    type Output = VectorWrapper<V>;

    fn sub(self, rhs: T) -> Self::Output {
        VectorWrapper(self.0.sub(&rhs.borrow().0))
    }
}

impl<V, S> ops::Mul<S> for VectorWrapper<V>
    where V: 'static + Vector,
          S: Into<V::ScalarFieldType>
{
    type Output = VectorWrapper<V>;

    fn mul(self, rhs: S) -> Self::Output {
        Self(self.0.scalar_rmultiply(rhs))
    }
}

impl<V, S> ops::Mul<S> for &VectorWrapper<V>
    where V: 'static + Vector,
          S: Into<V::ScalarFieldType>
{
    type Output = VectorWrapper<V>;

    fn mul(self, rhs: S) -> Self::Output {
        VectorWrapper(self.0.scalar_rmultiply(rhs))
    }
}



impl<V, I> From<I> for VectorWrapper<V>
    where V: 'static + Vector,
          I: IntoIterator<Item=(KeyType<V>, <V as Vector>::ScalarFieldType)>
{
    fn from(iterable: I) -> Self {
        Self(V::from_iterator(iterable))
    }
}




















#[cfg(test)]
mod tests {

    use super::*;

    use crate::free_tensor::{TensorBasis, TensorKey};
    use crate::vector::SimpleDenseVector;

    type TKey = TensorKey<2>;
    type Vect<'a> = VectorWrapper<SimpleDenseVector<'a, TensorBasis<2>, f64>>;


    #[test]
    fn test_add_vector_wrapper() {
        let tmp1 = vec![
            (TKey::new(), 1.0),
            (TKey::from_letter(1), 2.0),
            (TKey::from_letter(2), 3.0)
        ];
        let tmp2 = vec![
            (TKey::new(), -1.0),
            (TKey::from_letter(1), -1.0),
            (TKey::from_letter(2), -1.0)
        ];

        let vec1 = Vect::from(tmp1);
        let vec2 = Vect::from(tmp2);

        let expected = Vect::from(vec![
            (TKey::new(), 0.0),
            (TKey::from_letter(1), 1.0),
            (TKey::from_letter(2), 2.0)
        ]);

        assert_eq!(vec1 + vec2, expected);


    }



}