pub mod tensor_key;
pub mod key_iterator;

use std::mem::size_of;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::iter::{Iterator};
use std::ops::Range;

use crate::basis::{Basis, BasisWithDegree, OrderedBasis, OrderedBasisWithDegree};
use crate::{DimensionType, DegreeType, LetterType};


pub use tensor_key::{TensorKey, TensorKeyIterator};
pub use key_iterator::TensorBasisIterator;

use crate::implementation::{const_logn, make_mask};

#[derive(Debug)]
pub struct TensorBasis<const NLETTERS: DegreeType>;

impl<const NLETTERS: DegreeType> TensorBasis<NLETTERS> {

    pub const fn max_degree() -> DegreeType
    {
        TensorKey::<NLETTERS>::max_depth()
    }

    pub(crate) const fn start_of_degree(deg: DegreeType) -> DimensionType
    {
        (((NLETTERS as DimensionType).pow(deg) - 1) / (NLETTERS as DimensionType - 1)) as DimensionType
    }

    pub(crate) const fn degree_range(deg: DegreeType) -> Range<DimensionType>
    {
        Self::start_of_degree(deg)..Self::start_of_degree(deg+1)
    }

    const fn index_to_degree_impl(index: &DimensionType, deg: DegreeType) -> DegreeType
    {
        if *index >= Self::start_of_degree(deg) && *index < Self::start_of_degree(deg+1) {
            deg
        } else {
            Self::index_to_degree_impl(index, deg+1)
        }
    }

    const fn index_to_degree(index: &DimensionType) -> DegreeType
    {
        if *index == 0 {
            0
        } else if 1 <= *index && *index <= NLETTERS as DimensionType {
            1
        } else if Self::start_of_degree(2) <= *index && *index < Self::start_of_degree(3) {
            2
        } else {
            Self::index_to_degree_impl(index, 3)
        }
    }

}


impl<const NLETTERS: DegreeType> Basis for TensorBasis<NLETTERS> {
    type KeyType = TensorKey<NLETTERS>;
}

impl<const NLETTERS: DegreeType> OrderedBasis for TensorBasis<NLETTERS> {
    type KeyIterator = TensorBasisIterator<NLETTERS>;

    fn compare(lhs: &Self::KeyType, rhs: &Self::KeyType) -> Ordering {
        Ord::cmp(&lhs.data(), &rhs.data())
    }

    fn iter_keys() -> Self::KeyIterator {
        Self::KeyIterator::new()
    }

    fn key_to_index(key: &Self::KeyType) -> DimensionType {
        let mut result = 0;

        let mut i = key.size();
        let size = key.size();
        while i >= 1 {
            i -= 1;
            result *= NLETTERS as DimensionType;
            result += key.get_letter( i) as DimensionType;
        }

        result
    }

    fn index_to_key(index: DimensionType) -> Self::KeyType
    {
        if index == 0 {
            return Self::KeyType::new();
        } else if index <= NLETTERS as DimensionType {
            return Self::KeyType::from_letter(index as LetterType);
        }


        let FACTOR: DimensionType = NLETTERS as usize;

        let mut pos = index;
        let mut result = Self::KeyType::new();

        while pos > 0 {
            pos -= 1;
            result.push_front_raw((pos % FACTOR) as LetterType);
            pos /= FACTOR;
        }

        result
    }

    fn vector_dimension_for_key(key: &Self::KeyType) -> DimensionType {
        Self::start_of_degree(Self::degree(key) + 1)
    }

    fn vector_dimension_for_index(index: impl Into<DimensionType>) -> DimensionType {
        Self::start_of_degree(Self::index_to_degree(&index.into()) + 1)
    }
}

impl<const NLETTERS: DegreeType> BasisWithDegree for TensorBasis<NLETTERS> {
    fn degree(key: &Self::KeyType) -> DegreeType {
        key.size()
    }
}


impl<const NLETTERS: DegreeType> OrderedBasisWithDegree for TensorBasis<NLETTERS> {
    fn index_to_degree(index: DimensionType) -> DegreeType {
        Self::index_to_degree(&index)
    }
    fn start_of_degree(deg: DegreeType) -> DimensionType {
        Self::start_of_degree(deg)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    type Key = TensorKey<3>;
    type Basis = TensorBasis<3>;




    #[test]
    fn test_key_from_index_empty_key() {
        assert_eq!(
            Basis::index_to_key(0),
            Key::new()
        );
    }

    #[test]
    fn test_key_from_index_letters() {
        for i in 1..=3 {
            assert_eq!(
                Basis::index_to_key(i),
                Key::from_letter(i as LetterType)
            )
        }
    }

    #[test]
    fn test_key_from_index_depth_2() {
        assert_eq!(
            Basis::index_to_key(4),
            Key::from_letters(&[1, 1])
        );
        assert_eq!(
            Basis::index_to_key(5),
            Key::from_letters(&[1, 2])
        );
        assert_eq!(
            Basis::index_to_key(6),
            Key::from_letters(&[1, 3])
        );
        assert_eq!(
            Basis::index_to_key(7),
            Key::from_letters(&[2, 1])
        );
        assert_eq!(
            Basis::index_to_key(8),
            Key::from_letters(&[2, 2])
        );
        assert_eq!(
            Basis::index_to_key(9),
            Key::from_letters(&[2, 3])
        );
        assert_eq!(
            Basis::index_to_key(10),
            Key::from_letters(&[3, 1])
        );
        assert_eq!(
            Basis::index_to_key(11),
            Key::from_letters(&[3, 2])
        );
        assert_eq!(
            Basis::index_to_key(12),
            Key::from_letters(&[3, 3])
        );

    }

    #[test]
    fn test_compare_key_ordering_smaller_key() {
        let key1 = Key::from_letters(&[1, 2, 3]);
        let key2 = Key::from_letters(&[1, 2]);

        assert_eq!(Basis::compare(&key1, &key2), Ordering::Greater);
        assert_eq!(Basis::compare(&key2, &key1), Ordering::Less);
    }

    #[test]
    fn test_compare_equal_same_length() {
        let key1 = Key::from_letters(&[1, 2, 3]);
        let key2 = Key::from_letters(&[2, 1, 3]);

        assert_eq!(Basis::compare(&key1, &key2), Ordering::Less);
        assert_eq!(Basis::compare(&key2, &key1), Ordering::Greater);
    }

    #[test]
    fn test_equal_keys_equal() {
        let key1 = Key::from_letters(&[1, 2, 3]);
        let key2 = Key::from_letters(&[1, 2, 3]);

        assert_eq!(Basis::compare(&key1, &key2), Ordering::Equal);
    }


    #[test]
    fn test_key_to_index_empty_key() {
        let key = Key::new();

        assert_eq!(Basis::key_to_index(&key), 0);
    }

    #[test]
    fn test_key_to_index_letters() {
        let key1 = Key::from_letter(1);
        let key2 = Key::from_letter(2);
        let key3 = Key::from_letter(3);

        assert_eq!(Basis::key_to_index(&key1), 1);
        assert_eq!(Basis::key_to_index(&key2), 2);
        assert_eq!(Basis::key_to_index(&key3), 3);
    }

    #[test]
    fn test_key_to_index_depth_2() {
        let key4 = Key::from_letters(&[1, 1]);
        let key5 = Key::from_letters(&[1, 2]);
        let key6 = Key::from_letters(&[1, 3]);
        let key7 = Key::from_letters(&[2, 1]);
        let key8 = Key::from_letters(&[2, 2]);
        let key9 = Key::from_letters(&[2, 3]);
        let key10 = Key::from_letters(&[3, 1]);
        let key11 = Key::from_letters(&[3, 2]);
        let key12 = Key::from_letters(&[3, 3]);

        assert_eq!(Basis::key_to_index(&key4), 4);
        assert_eq!(Basis::key_to_index(&key5), 5);
        assert_eq!(Basis::key_to_index(&key6), 6);
        assert_eq!(Basis::key_to_index(&key7), 7);
        assert_eq!(Basis::key_to_index(&key8), 8);
        assert_eq!(Basis::key_to_index(&key9), 9);
        assert_eq!(Basis::key_to_index(&key10), 10);
        assert_eq!(Basis::key_to_index(&key11), 11);
        assert_eq!(Basis::key_to_index(&key12), 12);
    }

    #[test]
    fn test_key_to_index_depth_3() {
        let key13 = Key::from_letters(&[1, 1, 1]);
        let key14 = Key::from_letters(&[1, 1, 2]);
        let key15 = Key::from_letters(&[1, 1, 3]);
        let key16 = Key::from_letters(&[1, 2, 1]);
        let key17 = Key::from_letters(&[1, 2, 2]);
        let key18 = Key::from_letters(&[1, 2, 3]);
        let key19 = Key::from_letters(&[1, 3, 1]);
        let key20 = Key::from_letters(&[1, 3, 2]);
        let key21 = Key::from_letters(&[1, 3, 3]);
        let key22 = Key::from_letters(&[2, 1, 1]);
        let key23 = Key::from_letters(&[2, 1, 2]);
        let key24 = Key::from_letters(&[2, 1, 3]);
        let key25 = Key::from_letters(&[2, 2, 1]);
        let key26 = Key::from_letters(&[2, 2, 2]);
        let key27 = Key::from_letters(&[2, 2, 3]);
        let key28 = Key::from_letters(&[2, 3, 1]);
        let key29 = Key::from_letters(&[2, 3, 2]);
        let key30 = Key::from_letters(&[2, 3, 3]);
        let key31 = Key::from_letters(&[3, 1, 1]);
        let key32 = Key::from_letters(&[3, 1, 2]);
        let key33 = Key::from_letters(&[3, 1, 3]);
        let key34 = Key::from_letters(&[3, 2, 1]);
        let key35 = Key::from_letters(&[3, 2, 2]);
        let key36 = Key::from_letters(&[3, 2, 3]);
        let key37 = Key::from_letters(&[3, 3, 1]);
        let key38 = Key::from_letters(&[3, 3, 2]);
        let key39 = Key::from_letters(&[3, 3, 3]);



        assert_eq!(Basis::key_to_index(&key13), 13);
        assert_eq!(Basis::key_to_index(&key14), 14);
        assert_eq!(Basis::key_to_index(&key15), 15);
        assert_eq!(Basis::key_to_index(&key16), 16);
        assert_eq!(Basis::key_to_index(&key17), 17);
        assert_eq!(Basis::key_to_index(&key18), 18);
        assert_eq!(Basis::key_to_index(&key19), 19);
        assert_eq!(Basis::key_to_index(&key20), 20);
        assert_eq!(Basis::key_to_index(&key21), 21);
        assert_eq!(Basis::key_to_index(&key22), 22);
        assert_eq!(Basis::key_to_index(&key23), 23);
        assert_eq!(Basis::key_to_index(&key24), 24);
        assert_eq!(Basis::key_to_index(&key25), 25);
        assert_eq!(Basis::key_to_index(&key26), 26);
        assert_eq!(Basis::key_to_index(&key27), 27);
        assert_eq!(Basis::key_to_index(&key28), 28);
        assert_eq!(Basis::key_to_index(&key29), 29);
        assert_eq!(Basis::key_to_index(&key30), 30);
        assert_eq!(Basis::key_to_index(&key31), 31);
        assert_eq!(Basis::key_to_index(&key32), 32);
        assert_eq!(Basis::key_to_index(&key33), 33);
        assert_eq!(Basis::key_to_index(&key34), 34);
        assert_eq!(Basis::key_to_index(&key35), 35);
        assert_eq!(Basis::key_to_index(&key36), 36);
        assert_eq!(Basis::key_to_index(&key37), 37);
        assert_eq!(Basis::key_to_index(&key38), 38);
        assert_eq!(Basis::key_to_index(&key39), 39);
    }
}