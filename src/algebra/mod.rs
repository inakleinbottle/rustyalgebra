use std::borrow::{Borrow, BorrowMut};




use crate::coefficients::CoefficientField;
use crate::DegreeType;
use crate::vector::Vector;





pub trait Algebra<'vec> : Vector<'vec> {

    fn multiply_and_add_into_impl(
        &mut self,
        lhs: impl Borrow<Self>,
        rhs: impl Borrow<Self>,
        func: impl FnMut(&Self::ScalarType) -> Self::ScalarType,
        to_degree: Option<DegreeType>
    );

    fn multiply_into_impl(
        &mut self,
        rhs: impl Borrow<Self>,
        func: impl FnMut(&Self::ScalarType) -> Self::ScalarType,
        to_degree: Option<DegreeType>
    );

    fn multiply(
        &self,
        rhs: impl Borrow<Self>,
        to_degree: Option<DegreeType>
    ) -> Self
    {
        let mut result = Self::new();
        result.borrow_mut().multiply_and_add_into_impl(self, rhs.borrow(), |v| { v.clone() }, to_degree);
        result
    }

    fn multiply_inplace(
        &mut self,
        rhs: impl Borrow<Self>,
        to_degree: Option<DegreeType>
    ) -> &mut Self
    {
        self.multiply_into_impl(rhs.borrow(), |v| { v.clone() }, to_degree);
        self
    }

    fn add_mul(
        &mut self,
        lhs: impl Borrow<Self>,
        rhs: impl Borrow<Self>,
        to_degree: Option<DegreeType>
    ) -> &mut Self
    {
        self.multiply_and_add_into_impl(
            lhs.borrow(),
            rhs.borrow(),
            |v| { v.clone() },
            to_degree
        );
        self
    }

    fn sub_mul(
        &mut self,
        lhs: impl Borrow<Self>,
        rhs: impl Borrow<Self>,
        to_degree: Option<DegreeType>
    ) -> &mut Self
    {
        self.multiply_and_add_into_impl(
            lhs.borrow(),
            rhs.borrow(),
            <Self::ScalarType as CoefficientField>::uminus,
            to_degree);
        self
    }

    fn commutator(
        &self,
        rhs: impl Borrow<Self>,
        to_degree: Option<DegreeType>
    ) -> Self
    {
        let mut result = Self::new();
        let rhs_borrowed = rhs.borrow();
        result.borrow_mut()
            .add_mul(self, rhs_borrowed, to_degree.clone())
            .sub_mul(rhs_borrowed, self, to_degree);
        result
    }

    fn mul_scal_lprod(
        &mut self,
        rhs: impl Borrow<Self>,
        scalar: impl Into<Self::ScalarType>,
        to_degree: Option<DegreeType>
    ) -> &mut Self
    {
        let sca = scalar.into();
        self.multiply_into_impl(
            rhs.borrow(),
            move |v| { <Self::ScalarType as CoefficientField>::mul(&sca, v) },
            to_degree
        );
        self
    }

    fn mul_scal_rprod(
        &mut self,
        rhs: impl Borrow<Self>,
        scalar: impl Into<Self::ScalarType>,
        to_degree: Option<DegreeType>
    ) -> &mut Self
    {
        let sca = scalar.into();
        self.multiply_into_impl(
            rhs.borrow(),
            move |v| { <Self::ScalarType as CoefficientField>::mul(v, &sca) },
            to_degree
        );
        self
    }

    fn mul_rat_ldiv(
        &mut self,
        rhs: impl Borrow<Self>,
        scalar: impl Into<<Self::ScalarType as CoefficientField>::RationalType>,
        to_degree: Option<DegreeType>
    ) -> &mut Self
    {
        let sca = scalar.into();
        self.multiply_into_impl(
            rhs.borrow(),
            move |v| { <Self::ScalarType as CoefficientField>::div(v, &sca) },
            to_degree
        );
        self
    }

    fn mul_rat_rdiv(
        &mut self,
        rhs: impl Borrow<Self>,
        scalar: impl Into<Self::ScalarType>,
        to_degree: Option<DegreeType>
    ) -> &mut Self
    {
        let sca = scalar.into();
        self.multiply_into_impl(
            rhs.borrow(),
            move |v| { <Self::ScalarType as CoefficientField>::mul(v, &sca) },
            to_degree
        );
        self
    }

}
