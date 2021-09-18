use std::borrow::Borrow;

use crate::vector::{Vector, KeyType, ScalarField, RationalType};
use crate::coefficients::CoefficientField;

pub trait VectorKeyExt : Vector {

    fn add_scalar_multiply(
        &mut self,
        key: impl Borrow<KeyType<Self>>,
        val: impl Into<ScalarField<Self>>
    ) -> &mut Self
    {
        if let Some(v) = self.get_mut(key.borrow()) {
            <ScalarField<Self> as CoefficientField>::add_inplace(v,  &val.into());
        } else {
            self.insert_single(key.borrow(), val.into());
        }
        self
    }

    fn sub_scalar_multiply(
        &mut self,
        key: impl Borrow<KeyType<Self>>,
        val: impl Into<ScalarField<Self>>
    ) -> &mut Self
    {
        if let Some(v) = self.get_mut(key.borrow()) {
            <ScalarField<Self> as CoefficientField>::sub_inplace(v, &val.into());
        } else {
            self.insert_single(key.borrow(), <ScalarField<Self> as CoefficientField>::uminus(&val.into()));
        }
        self
    }

    fn add_scalar_divide(
        &mut self,
        key: impl Borrow<KeyType<Self>>,
        val: impl Into<<ScalarField<Self> as CoefficientField>::RationalType>
    ) -> &mut Self
    {
        let sca = <ScalarField<Self> as CoefficientField>::div(&ScalarField::<Self>::ONE, &val.into());
        if let Some(v) = self.get_mut(key.borrow()) {
            <ScalarField<Self> as CoefficientField>::add_inplace(v, &sca);
        } else {
            self.insert_single(key.borrow().clone(), sca);
        }
        self
    }


    fn sub_scalar_divide(
        &mut self,
        key: impl Borrow<KeyType<Self>>,
        val: impl Into<<ScalarField<Self> as CoefficientField>::RationalType>
    ) -> &mut Self
    {
        let sca = <ScalarField<Self> as CoefficientField>::div(&ScalarField::<Self>::MONE, &val.into());
        if let Some(v) = self.get_mut(key.borrow()) {
            <ScalarField<Self> as CoefficientField>::sub_inplace(v, &sca);
        } else {
            self.insert_single(key.borrow().clone(), sca);
        }
        self
    }
}



impl<V: Vector> VectorKeyExt for V {}




pub trait CrossTypeVectorExt<V> : Vector
    where V: Vector<BasisType=Self::BasisType, ScalarFieldType=Self::ScalarFieldType>
{

    fn add_inplace(&mut self, other: &V) -> &mut Self;
    fn sub_inplace(&mut self, other: &V) -> &mut Self;

    fn add_scalar_mul(&mut self, other: &V, s: ScalarField<Self>) -> &mut Self;
    fn sub_scalar_mul(&mut self, other: &V, s: ScalarField<Self>) -> &mut Self;
    fn add_scalar_div(&mut self, other: &V, s: RationalType<Self>) -> &mut Self;
    fn sub_scalar_div(&mut self, other: &V, s: RationalType<Self>) -> &mut Self;

}


impl<U, V> CrossTypeVectorExt<V> for U
    where U: Vector,
          V: Vector<BasisType=U::BasisType, ScalarFieldType=U::ScalarFieldType>,
          for<'a> &'a V: IntoIterator<Item=(&'a KeyType<U>, &'a ScalarField<U>)>
{
    fn add_inplace(&mut self, other: &V) -> &mut Self {
        for (k, v) in other.into_iter() {
            if let Some(curr) = self.get_mut( k) {
                ScalarField::<U>::add_inplace(curr, v);
            } else {
                self.insert_single(k, v.clone());
            }
        }
        self
    }

    fn sub_inplace(&mut self, other: &V) -> &mut Self {
        for (k, v) in other.into_iter() {
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::sub_inplace(curr, v);
            } else {
                self.insert_single(k, ScalarField::<U>::uminus(v));
            }
        }
        self
    }

    fn add_scalar_mul(&mut self, other: &V, s: ScalarField<Self>) -> &mut Self {
        for (k, v) in other.into_iter() {
            let tmp = ScalarField::<U>::mul(&s, v);
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::add_inplace(curr, &tmp);
            } else {
                self.insert_single(k, tmp);
            }
        }
        self
    }

    fn sub_scalar_mul(&mut self, other: &V, s: ScalarField<Self>) -> &mut Self {
        for (k, v) in other.into_iter() {
            let tmp = ScalarField::<U>::mul(&s, v);
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::sub_inplace(curr, &tmp);
            } else {
                self.insert_single(k, ScalarField::<U>::uminus(&tmp));
            }
        }
        self
    }

    fn add_scalar_div(&mut self, other: &V, s: RationalType<Self>) -> &mut Self {
        for (k, v) in other.into_iter() {
            let tmp = ScalarField::<U>::div( v, &s);
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::add_inplace(curr, &tmp);
            } else {
                self.insert_single(k, tmp);
            }
        }
        self
    }

    fn sub_scalar_div(&mut self, other: &V, s: RationalType<Self>) -> &mut Self {
        for (k, v) in other.into_iter() {
            let tmp = ScalarField::<U>::div( v, &s);
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::sub_inplace(curr, &tmp);
            } else {
                self.insert_single(k, ScalarField::<U>::uminus(&tmp));
            }
        }
        self
    }
}
