mod tensor_key;

use std::mem::size_of;
use std::cmp::Ordering;
use std::iter::{Iterator};
use std::fmt::{self, Debug, Display, Formatter};

use super::{Basis, BasisWithDegree, OrderedBasis};
use crate::{DimensionType, DegreeType, LetterType};

pub const fn const_logn<const N: u32>(n: u32) -> u32
{
    if n < 2 {
        return 0;
    } else {
        return const_logn::<N>(n / 2) + 1;
    }
}

const fn make_mask(n_bits: u32, shift: u32) -> u64
{
    ((1u64 << n_bits) - 1) << shift
}

#[derive(PartialEq)]
pub struct TensorKey<const NLetters: u32>
{
    size: DegreeType,
    data: u64
}


impl<const NLETTERS: u32> Debug for TensorKey<NLETTERS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for i in 0..self.size() {
            if i > 0 {
                write!(f, ", {}", self.get_letter(i))?;
            } else {
                write!(f, "{}", self.get_letter(i))?;
            }
        }
        write!(f, ")")
    }
}

impl<const NLETTERS: u32> Display for TensorKey<NLETTERS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // In this case, the displayed key should be the same as the debug output
        Display::fmt(self, f)
    }
}


impl<const NLetters: u32> TensorKey<NLetters> {

    const fn bits_per_letter() -> u32
    {
        const_logn::<2>(NLetters) + 1
    }

    const fn size_bits() -> u32
    {
        const_logn::<2>(((64 - 1) as u32) / Self::bits_per_letter())
    }

    const fn letter_mask() -> u64
    {
        make_mask(Self::bits_per_letter(), 0)
    }

    const fn size_mask() -> u64
    {
        make_mask(Self::size_bits(), 64 - Self::size_bits())
    }

    pub const fn max_depth() -> DegreeType
    {
        (64 - Self::size_bits()) / Self::bits_per_letter()
    }

    pub const fn new() -> Self
    {
        Self { size: 0, data: 0 }
    }

    pub const fn from_letter(letter: LetterType) -> Self
    {
        Self {
            size: 1,
            data: letter as u64 - 1
        }
    }

    pub const fn size(&self) -> DegreeType
    {
        self.size
    }

    pub const fn data(&self) -> u64
    {
        self.data
    }

    fn update_size(&mut self, new_size: DegreeType)
    {
        self.size = new_size;
    }

    pub fn from_letters(letters: &[LetterType]) -> Self
    {
        if letters.len() > (Self::max_depth() as usize) {
            panic!("Number of letters exceeds maximum depth");
        }
        let mut data = 0;

        for letter in letters {
            debug_assert!(*letter >= 1 && *letter <= NLetters);
            data <<= Self::bits_per_letter();
            data += ((*letter - 1) as u64);
            //result.push_front(*letter);
        }
        Self {
            size: letters.len() as u32,
            data
        }
    }

    const fn get_letter_unadjusted(&self, letter: DegreeType) -> LetterType
    {
        //debug_assert!(letter <= self.size());
        let pos = self.size() - 1 - letter;
        let mask: u64 = make_mask(
            Self::bits_per_letter(),
            pos * Self::bits_per_letter()
        );
        let first_masked = self.data & mask;
        let second_mask= first_masked >> pos*Self::bits_per_letter();
        second_mask as LetterType
    }

    pub const fn get_letter(&self, letter: DegreeType) -> LetterType
    {
        self.get_letter_unadjusted(letter) + 1
    }

    pub const fn concatenate(&self, other: &Self) -> Self
    {
        Self {
            size: self.size() + other.size(),
            data: (self.data() << (other.size() * Self::bits_per_letter())) + other.data()
        }
    }

    pub fn to_letters(&self) -> Vec<LetterType>
    {
        let mut result = Vec::with_capacity(self.size() as usize);

        for i in 0..(self.size()) {
            result.push(self.get_letter(self.size() - 1 - i));
        }
        debug_assert_eq!(result.len(), self.size() as DimensionType);
        result
    }

    fn push_front_raw(&mut self, letter: LetterType) -> &mut Self
    {
        let mut data = self.data();
        data += ((letter as u64) & Self::letter_mask()) << (self.size()*Self::bits_per_letter());
        self.data = data;
        self.update_size(self.size() + 1);
        self
    }

    pub fn push_front(&mut self, letter: LetterType) -> &mut Self
    {
        debug_assert!(self.size() < Self::max_depth());
        debug_assert!(letter >= 1 && letter <= NLetters,
                      "Expected letter between 1 and {}, got {}", NLetters, letter);
        self.push_front_raw(letter - 1)
    }

}


pub struct TensorKeyIterator<'a, const NLETTERS: DegreeType>
{
    index: DegreeType,
    key: &'a TensorKey<NLETTERS>
}

impl<'a, const NLETTERS: DegreeType> TensorKeyIterator<'a, NLETTERS> {

    fn new(key: &'a TensorKey<NLETTERS>) -> Self
    {
        Self { index: 0, key: key }
    }
}


impl<'a, const NLETTERS: DegreeType> Iterator for TensorKeyIterator<'a, NLETTERS> {
    type Item = LetterType;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index == self.key.size() {
            None
        } else {
            self.index += 1;
            Some(self.key.get_letter(self.index - 1))
        }
    }
}


pub struct TensorBasis<const NLETTERS: DegreeType>;

impl<const NLETTERS: DegreeType> TensorBasis<NLETTERS> {

    const fn start_of_degree(deg: DegreeType) -> DimensionType
    {
        (((NLETTERS as DimensionType).pow(deg) - 1) / (NLETTERS as DimensionType - 1)) as DimensionType
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
    fn compare(lhs: &Self::KeyType, rhs: &Self::KeyType) -> Ordering {
        Ord::cmp(&lhs.data(), &rhs.data())
    }

    fn first_key() -> Self::KeyType {
        <Self as Basis>::KeyType::new()
    }

    fn next_key(key: &Self::KeyType) -> Self::KeyType {
        todo!()
    }

    fn key_to_index(key: &Self::KeyType) -> DimensionType {
        let mut result = 0;

        let mut i = key.size();
        let size = key.size();
        while i >= 1 {
            i -= 1;
            result *= NLETTERS as DimensionType;
            result += key.get_letter(size - 1 - i) as DimensionType;
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

        let degree = Self::index_to_degree(&index);

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
        todo!()
    }

    fn vector_dimension_for_index(index: impl Into<DimensionType>) -> DimensionType {
        todo!()
    }
}

impl<const NLETTERS: DegreeType> BasisWithDegree for TensorBasis<NLETTERS> {
    fn degree(key: &Self::KeyType) -> DegreeType {
        key.size()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    type Key = TensorKey<3>;
    type Basis = TensorBasis<3>;

    #[test]
    fn test_make_mask_3_0()
    {
        assert_eq!(make_mask(3, 0), 0b111);
    }

    #[test]
    fn test_make_mask_3_5()
    {
        assert_eq!(make_mask(3, 5), 0b11100000);
    }

    #[test]
    fn test_make_mask_12_52()
    {
        assert_eq!(make_mask(12, 52), 0xFFF0000000000000);
    }

    #[test]
    fn test_size_bits_common()
    {
        assert_eq!(TensorKey::<2>::size_bits(), 4);
        assert_eq!(TensorKey::<5>::size_bits(), 4);
        assert_eq!(TensorKey::<9>::size_bits(), 3);
        assert_eq!(TensorKey::<17>::size_bits(), 3);
        assert_eq!(TensorKey::<33>::size_bits(), 3);
    }

    #[test]
    fn test_size_bits()
    {
        assert_eq!(Key::size_bits(), 4);
    }

    #[test]
    fn test_size_mask()
    {
        let mask = Key::size_mask();

        assert_eq!(mask, 0xF000000000000000)
    }

    #[test]
    fn test_letter_mask()
    {
        assert_eq!(Key::letter_mask(), 0b11);
    }

    #[test]
    fn test_bits_per_letter_correct()
    {
        assert_eq!(Key::bits_per_letter(), 2);
    }

    #[test]
    fn test_from_letter_each_letter()
    {
        assert_eq!(Key::from_letter(1).data(), 0u64);
        assert_eq!(Key::from_letter(2).data(), 1u64);
        assert_eq!(Key::from_letter(3).data(), 2u64);
    }

    #[test]
    fn test_from_array()
    {
        let array: [LetterType; 3] = [1, 2, 1];

        let key = Key::from_letters(&array);

        assert_eq!(key.data(), 0b000100)
    }

    #[test]
    fn test_concatenate()
    {
        let key1 = Key::from_letter(3);
        let key2 = Key::from_letter(1);

        let result = key1.concatenate(&key2);

        assert_eq!(result.data(), (2u64 << 2) + 0u64)
    }

    #[test]
    fn test_to_letters()
    {
        let key = Key::from_letters(&[1, 2, 1, 2, 3]);
        assert_eq!(key.size(), 5);
        assert_eq!(key.to_letters(), vec![3,2,1,2,1]);
    }

    #[test]
    fn test_equal_empty_key() {
        let key1 = Key::new();
        let key2 = Key::new();

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_not_equal_empty_vs_1_key() {
        let key1 = Key::new();
        let key2 = Key::from_letter(1);

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_not_equal_1_key_vs_2_key() {
        let key1 = Key::from_letter(1);
        let key2 = Key::from_letter(2);

        assert_ne!(key1, key2);
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
    fn test_iterator_3_letters() {
        let key = Key::from_letters(&[1, 2, 3]);
        let mut itr = TensorKeyIterator::<3>::new(&key);

        assert_eq!(itr.next(), Some(1));
        assert_eq!(itr.next(), Some(2));
        assert_eq!(itr.next(), Some(3));
        assert_eq!(itr.next(), None);
    }

}