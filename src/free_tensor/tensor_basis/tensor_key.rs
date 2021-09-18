

use std::fmt::{self, Debug, Display, Formatter};

use crate::{DegreeType, DimensionType, LetterType};
use crate::implementation::{const_logn, make_mask};


/// Compact representation of a tensor word.
///
/// A tensor key is a compact representation of a single tensor word, consisting of a finite
/// sequence of letters taken from the alphabet. In some respects, a tensor key is similar to
/// a string, except that the alphabet over which the tensor word is defined can be arbitrary.
/// To simplify matters, we restrict our attention to alphabets consisting of the numbers 1 to N
/// (inclusive), where N is a positive integer which we shall call the _width_ of the alphabet.
///
/// A naive approach for defining a tensor word would be store a sequence of vectors in a
/// `Vec<LetterType>`, but in practice this is wasteful and not ideal for cache locality in cases
/// where this matters. Since a typical alphabet will have relatively few letters, typically not
/// more than 1024, we can form a compact representation of a tensor word by packing the binary
/// representation of each letter into the bits of a `u64`. Even in the case when the alphabet has
/// width 10 (1024 letters), we can still pack approximately 6 letters into a 64 bit integer type.
/// The only minor bump is that we must subtract 1 from the letter so the stored word consists of
/// letters from the modified alphabet 0 to N-1.
#[derive(PartialEq, Clone)]
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

    pub(super) const fn bits_per_letter() -> u32
    {
        const_logn::<2>(NLetters) + 1
    }

    pub(super) const fn size_bits() -> u32
    {
        const_logn::<2>(((64 - 1) as u32) / Self::bits_per_letter())
    }

    pub(super) const fn letter_mask() -> u64
    {
        make_mask(Self::bits_per_letter(), 0)
    }

    pub(super) const fn size_mask() -> u64
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

    pub(super) fn update_size(&mut self, new_size: DegreeType)
    {
        self.size = new_size;
    }

    pub(super) fn set_letter(&mut self, letter: DegreeType, new_val: LetterType)
    {
        if letter >= self.size() {
            panic!("Index {} exceeds length of word", letter);
        }
        let mask = !make_mask(Self::bits_per_letter(), letter);
        let mut data = self.data();
        data &= mask;
        data += (new_val as u64 - 1) << Self::bits_per_letter()*letter;
        self.data = data;
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
            data += (*letter - 1) as u64;
            //result.push_front(*letter);
        }
        Self {
            size: letters.len() as u32,
            data
        }
    }

    pub(super) const fn get_letter_unadjusted(&self, letter: DegreeType) -> LetterType
    {
        //debug_assert!(letter <= self.size());
        let pos = letter;
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
            result.push(self.get_letter( i));
        }
        debug_assert_eq!(result.len(), self.size() as DimensionType);
        result
    }

    pub(super) fn push_front_raw(&mut self, letter: LetterType) -> &mut Self
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


impl<const NLETTERS: DegreeType> AsRef<TensorKey<NLETTERS>> for TensorKey<NLETTERS>
{
    fn as_ref(&self) -> &TensorKey<NLETTERS> {
        self
    }
}



pub struct TensorKeyIterator<'a, const NLETTERS: DegreeType>
{
    index: DegreeType,
    key: &'a TensorKey<NLETTERS>
}

impl<'a, const NLETTERS: DegreeType> TensorKeyIterator<'a, NLETTERS> {

    pub(super) fn new(key: &'a TensorKey<NLETTERS>) -> Self
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
            Some(self.key.get_letter(self.key.size() - self.index ))
        }
    }
}




#[cfg(test)]
mod tests {

    use super::*;

    type Key = TensorKey<3>;

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
    fn test_iterator_3_letters() {
        let key = Key::from_letters(&[1, 2, 3]);
        let mut itr = TensorKeyIterator::<3>::new(&key);

        assert_eq!(itr.next(), Some(1));
        assert_eq!(itr.next(), Some(2));
        assert_eq!(itr.next(), Some(3));
        assert_eq!(itr.next(), None);
    }


}