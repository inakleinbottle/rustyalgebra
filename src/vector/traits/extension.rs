use std::borrow::Borrow;

use crate::vector::{Vector};
use crate::coefficients::CoefficientField;

pub trait VectorKeyExt<'vec> : Vector<'vec> {

    fn add_scalar_multiply(
        &mut self,
        key: impl Borrow<Self::KeyType>,
        val: impl Into<Self::ScalarType>
    ) -> &mut Self
    {
        if let Some(v) = self.get_mut(key.borrow()) {
            <Self::ScalarType as CoefficientField>::add_inplace(v,  &val.into());
        } else {
            self.insert_single(key.borrow(), val.into());
        }
        self
    }

    fn sub_scalar_multiply(
        &mut self,
        key: impl Borrow<Self::KeyType>,
        val: impl Into<Self::ScalarType>
    ) -> &mut Self
    {
        if let Some(v) = self.get_mut(key.borrow()) {
            <Self::ScalarType as CoefficientField>::sub_inplace(v, &val.into());
        } else {
            self.insert_single(key.borrow(), <Self::ScalarType as CoefficientField>::uminus(&val.into()));
        }
        self
    }

    fn add_scalar_divide(
        &mut self,
        key: impl Borrow<Self::KeyType>,
        val: impl Into<<Self::ScalarType as CoefficientField>::RationalType>
    ) -> &mut Self
    {
        let sca = <Self::ScalarType as CoefficientField>::div(&Self::ScalarType::ONE, &val.into());
        if let Some(v) = self.get_mut(key.borrow()) {
            <Self::ScalarType as CoefficientField>::add_inplace(v, &sca);
        } else {
            self.insert_single(key.borrow().clone(), sca);
        }
        self
    }


    fn sub_scalar_divide(
        &mut self,
        key: impl Borrow<Self::KeyType>,
        val: impl Into<<Self::ScalarType as CoefficientField>::RationalType>
    ) -> &mut Self
    {
        let sca = <Self::ScalarType as CoefficientField>::div(&Self::ScalarType::MONE, &val.into());
        if let Some(v) = self.get_mut(key.borrow()) {
            <Self::ScalarType as CoefficientField>::sub_inplace(v, &sca);
        } else {
            self.insert_single(key.borrow().clone(), sca);
        }
        self
    }
}



impl<'vec, V: Vector<'vec>> VectorKeyExt<'vec> for V {}




pub trait CrossTypeVectorExt<'vec1, 'vec2, V> : Vector<'vec1>
    where V: Vector<'vec2, BasisType=Self::BasisType, ScalarType=Self::ScalarType>
{

    fn add_inplace(&mut self, other: &V) -> &mut Self;
    fn sub_inplace(&mut self, other: &V) -> &mut Self;

    fn add_scalar_mul(&mut self, other: &V, s: Self::ScalarType) -> &mut Self;
    fn sub_scalar_mul(&mut self, other: &V, s: Self::ScalarType) -> &mut Self;
    fn add_scalar_div(&mut self, other: &V, s: Self::RationalType) -> &mut Self;
    fn sub_scalar_div(&mut self, other: &V, s: Self::RationalType) -> &mut Self;

}

/*
impl<U, V> CrossTypeVectorExt<V> for U
    where U: Vector,
          V: Vector<BasisType=U::BasisType, ScalarFieldType=U::ScalarFieldType>,
{
    fn add_inplace(&mut self, other: &V) -> &mut Self {
        for (k, v) in other.iter_item() {
            if let Some(curr) = self.get_mut( k) {
                ScalarField::<U>::add_inplace(curr, v);
            } else {
                self.insert_single(k, v.clone());
            }
        }
        self
    }

    fn sub_inplace(&mut self, other: &V) -> &mut Self {
        for (k, v) in other.iter_item() {
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::sub_inplace(curr, v);
            } else {
                self.insert_single(k, ScalarField::<U>::uminus(v));
            }
        }
        self
    }

    fn add_scalar_mul(&mut self, other: &V, s: Self::ScalarType) -> &mut Self {
        for (k, v) in other.iter_item() {
            let tmp = ScalarField::<U>::mul(&s, v);
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::add_inplace(curr, &tmp);
            } else {
                self.insert_single(k, tmp);
            }
        }
        self
    }

    fn sub_scalar_mul(&mut self, other: &V, s: Self::ScalarType) -> &mut Self {
        for (k, v) in other.iter_item() {
            let tmp = ScalarField::<U>::mul(&s, v);
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::sub_inplace(curr, &tmp);
            } else {
                self.insert_single(k, ScalarField::<U>::uminus(&tmp));
            }
        }
        self
    }

    fn add_scalar_div(&mut self, other: &V, s: Self::RationalType) -> &mut Self {
        for (k, v) in other.iter_item() {
            let tmp = ScalarField::<U>::div( v, &s);
            if let Some(curr) = self.get_mut(k) {
                ScalarField::<U>::add_inplace(curr, &tmp);
            } else {
                self.insert_single(k, tmp);
            }
        }
        self
    }

    fn sub_scalar_div(&mut self, other: &V, s: Self::RationalType) -> &mut Self {
        for (k, v) in other.iter_item() {
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
*/