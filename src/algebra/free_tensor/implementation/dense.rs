

use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::cmp;


use crate::algebra::{Algebra, ScalarField};
use crate::algebra::free_tensor::FreeTensor;
use crate::coefficients::CoefficientField;
use crate::basis::Basis;
use crate::basis::tensor_basis::{TensorBasis, TensorKey, TensorKeyIterator};
use crate::vector::{Vector, VectorWithDegree};
use crate::vector::dense_vector::SimpleDenseVector;
use crate::DegreeType;



pub struct DenseTensor<
    'a,
    S: CoefficientField,
    const NLETTERS: DegreeType
>(SimpleDenseVector<'a, TensorBasis<NLETTERS>, S>);

type Key<V> = <<V as Vector>::BasisType as Basis>::KeyType;
type BasisT<V> = <V as Vector>::BasisType;


impl<'a, S, const NLETTERS: DegreeType> Deref for DenseTensor<'a, S, NLETTERS>
    where S: CoefficientField
{
    type Target = SimpleDenseVector<'a, TensorBasis<NLETTERS>, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<'a, S, const NLETTERS: DegreeType> DerefMut for DenseTensor<'a, S, NLETTERS>
    where S: CoefficientField
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl<'a, S, const NLETTERS: DegreeType> Into<SimpleDenseVector<'a, TensorBasis<NLETTERS>, S>>
    for DenseTensor<'a, S, NLETTERS>
        where S: CoefficientField
{
    fn into(self) -> SimpleDenseVector<'a, TensorBasis<NLETTERS>, S> {
        self.0
    }
}

impl<'a, S, const NLETTERS: DegreeType> From<SimpleDenseVector<'a, TensorBasis<NLETTERS>, S>>
    for DenseTensor<'a, S, NLETTERS>
        where S: CoefficientField
{
    fn from(arg: SimpleDenseVector<'a, TensorBasis<NLETTERS>, S>) -> Self {
        Self(arg)
    }
}


impl<'a, S, const NLETTERS: DegreeType> DenseTensor<'a, S, NLETTERS>
    where S: CoefficientField
{

    unsafe fn compute_multiply_and_add_into(
        out: &mut [S],
        lhs: &[S],
        rhs: &[S],
        func: &mut impl FnMut(&S) -> S
    )
    {
        let mut out_ptr = out.as_mut_ptr();

        for lhs_v in lhs {
            for rhs_v in rhs {
                S::add_inplace(&mut *out_ptr, &func(&S::mul(lhs_v, rhs_v)));
                out_ptr = out_ptr.add(1);
            }
        }
    }

    unsafe fn compute_and_add_assign_different_range(
        out: &mut [S],
        lhs: &[S],
        rhs: &[S],
        func: &mut impl FnMut(&S) -> S
    )
    {
        let mut out_ptr = out.as_mut_ptr();

        for lhs_v in lhs {
            for rhs_v in rhs {
                *out_ptr = func(&S::mul(lhs_v, rhs_v));
                out_ptr = out_ptr.add(1);
            }
        }
    }

    fn compute_and_assign_same_range(
        out: &mut [S],
        rhs: &[S],
        func: &mut impl FnMut(&S) -> S
    )
    {
        for out_ptr in out {
            for rhs_v in rhs {
                *out_ptr = func(&S::mul(out_ptr, rhs_v));
            }
        }
    }

}


impl<'a, S, const NLETTERS: DegreeType> Algebra for DenseTensor<'a, S, NLETTERS>
    where S: CoefficientField
{
    fn multiply_and_add_into_impl(
        &mut self,
        lhs: impl Borrow<Self>,
        rhs: impl Borrow<Self>,
        mut func: impl FnMut(&<Self as Vector>::ScalarFieldType) -> ScalarField<Self>,
        to_degree: Option<DegreeType>
    )
    {
        let rhs_r = rhs.borrow();
        let lhs_r = lhs.borrow();

        let lhs_deg = lhs_r.degree();
        let rhs_deg = rhs_r.degree();

        let max_deg = cmp::min(
            to_degree.expect("Max degree should be set for tensor types"),
            lhs_deg + rhs_deg
        );

        if self.degree() < max_deg {
            self.resize(TensorBasis::<NLETTERS>::start_of_degree(max_deg+1));
        }

        unsafe {
            for out_deg in (0..=max_deg).rev() {
                // Notice that out_deg >= rhs_deg, out_deg - rhs_deg >= 0
                //cmp::max(0, (out_deg - rhs_deg));
                let lhs_deg_min = {
                    if out_deg > rhs_deg {
                        out_deg - rhs_deg
                    } else {
                        0
                    }
                };
                let lhs_deg_max = cmp::min(out_deg, lhs_deg);

                for lhs_d in (lhs_deg_min..=lhs_deg_max).rev() {
                    let rhs_d = out_deg - lhs_d;

                    Self::compute_multiply_and_add_into(
                        self.as_mut_slice().get_unchecked_mut(TensorBasis::<NLETTERS>::degree_range(out_deg)),
                        lhs_r.as_slice().get_unchecked(TensorBasis::<NLETTERS>::degree_range(lhs_d)),
                        rhs_r.as_slice().get_unchecked(TensorBasis::<NLETTERS>::degree_range(rhs_d)),
                        &mut func
                    );

                }
            }
        }
    }

    fn multiply_into_impl(
        &mut self,
        rhs: impl Borrow<Self>,
        mut func: impl FnMut(&<Self as Vector>::ScalarFieldType) -> ScalarField<Self>,
        to_degree: Option<DegreeType>
    )
    {

        let rhs_r = rhs.borrow();

        let lhs_deg = self.degree();
        let rhs_deg = rhs_r.degree();

        let max_deg = cmp::min(
            to_degree.expect("Max degree should be set for tensor types"),
            lhs_deg + rhs_deg
        );

        if self.degree() < max_deg {
            self.resize(TensorBasis::<NLETTERS>::start_of_degree(max_deg+1));
        }

        let (offset, assign) = match rhs_r.as_slice().get(0) {
            Some(val) if *val == S::ZERO => (1, true),
            Some(val) if *val == S::ONE => (1, false),
            _ => (0, true)
        };

        for out_deg in (1..=max_deg).rev() {

            let lhs_deg_min = cmp::max(0, out_deg - rhs_deg);

            let lhs_deg_max = cmp::min(out_deg - offset, lhs_deg);

            let mut reduce = 0;

            if assign {
                reduce = 1;
                unsafe {


                    let rhs_deg_range = TensorBasis::<NLETTERS>::degree_range(out_deg - lhs_deg_max);
                    let out_deg_range = TensorBasis::<NLETTERS>::degree_range(out_deg);

                    if offset == 1 {
                        let lhs_deg_range = TensorBasis::<NLETTERS>::degree_range(lhs_deg_max);
                        let lhs_as_slice = self.as_mut_slice();
                        let (a, b) = lhs_as_slice.split_at_mut(
                            TensorBasis::<NLETTERS>::start_of_degree(lhs_deg_max));


                        Self::compute_and_add_assign_different_range(
                            b.get_unchecked_mut(out_deg_range),
                            a.get_unchecked(lhs_deg_range),
                            rhs_r.as_slice().get_unchecked(rhs_deg_range),
                            &mut func
                        );
                    } else if offset == 0 {

                        Self::compute_and_assign_same_range(
                            self.as_mut_slice().get_unchecked_mut(out_deg_range),
                            rhs_r.as_slice().get_unchecked(rhs_deg_range),
                            &mut func
                        );
                    }
                }
            }

            unsafe {
                for lhs_d in (lhs_deg_min..=(lhs_deg_max-reduce)).rev() {
                    let rhs_d = out_deg - lhs_d;

                    let rhs_deg_range = TensorBasis::<NLETTERS>::degree_range(out_deg - lhs_d);
                    let out_deg_range = TensorBasis::<NLETTERS>::degree_range(rhs_d);
                    let lhs_deg_range = TensorBasis::<NLETTERS>::degree_range(lhs_d);
                    let lhs_as_slice = self.as_mut_slice();
                    let (a, b) = lhs_as_slice.split_at_mut(
                        TensorBasis::<NLETTERS>::start_of_degree(lhs_deg_max));

                    Self::compute_multiply_and_add_into(
                        b.get_unchecked_mut(out_deg_range),
                        a.get_unchecked(lhs_deg_range),
                        rhs_r.as_slice().get_unchecked(rhs_deg_range),
                        &mut func
                    );
                }
            }

        }
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    type TensorT<'a> = DenseTensor<'a, f64, 3>;
    type BasisT = TensorBasis<3>;
    type Key = TensorKey<3>;

    #[test]
    fn test_tensor_multiplication_two_letters() {

        let lhs = TensorT::from_key(Key::from_letter(1));
        let rhs = TensorT::from_key(Key::from_letter(2));

        let result = lhs.multiply(rhs, Some(2));

        let expected = TensorT::from_key(Key::from_letters(&[1, 2]));

        assert_eq!(result.as_slice(), expected.as_slice());

    }




}