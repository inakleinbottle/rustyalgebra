

use std::fmt::{self, Debug, Display, Formatter};

use crate::implementation::{make_mask, const_logn};
use crate::{DegreeType, LetterType, DimensionType};

use super::DataType;
use super::{get_hall_set, hall_set::{HallSet, HallSetError}};

const KEY_BITS: DegreeType = 18;
const PARENT_BITS: DegreeType = 18;
const SIZE_BITS: DegreeType = 10;

const LPARENT_SHIFT: DegreeType = KEY_BITS;
const RPARENT_SHIFT: DegreeType = KEY_BITS + PARENT_BITS;
const SIZE_SHIFT: DegreeType = KEY_BITS + 2*PARENT_BITS;




pub struct HallKey<const NLETTERS: DegreeType>(DataType);



impl<const NLETTERS: DegreeType> HallKey<NLETTERS> {

    const fn get_key(&self) -> LetterType
    {
        (self.0 & make_mask(KEY_BITS, 0)) as LetterType
    }

    pub const fn get_lparent(&self) -> LetterType
    {
        ((self.0 & make_mask(PARENT_BITS, LPARENT_SHIFT)) >> LPARENT_SHIFT) as LetterType
    }

    pub const fn get_rparent(&self) -> LetterType
    {
        ((self.0 & make_mask(PARENT_BITS, RPARENT_SHIFT)) >> RPARENT_SHIFT) as LetterType
    }

    pub const fn size(&self) -> DegreeType
    {
        ((self.0 & make_mask(SIZE_BITS, SIZE_SHIFT)) >> SIZE_SHIFT) as DegreeType
    }

    const fn make_key_data(key: DataType, lparent: DataType, rparent: DataType, size: DegreeType) -> DataType
    {
        let msize = (size as DataType) << SIZE_SHIFT;
        let mlparent = lparent << LPARENT_SHIFT;
        let mrparent = rparent << RPARENT_SHIFT;
        key + mlparent + mrparent + msize
    }


    pub const fn new() -> Self
    {
        HallKey(0)
    }

    pub fn from_letter(letter: LetterType) -> Self
    {
        debug_assert!(1 <= letter && letter <= NLETTERS);
        Self(Self::make_key_data(letter as DataType, 0, letter as DataType, 1))
    }

    pub const fn is_valid(&self) -> bool
    {
        self.0 != 0
    }

    pub const fn is_letter(&self) -> bool
    {
        self.size() == 1
    }

    pub const fn to_index(&self) -> DimensionType
    {
        self.get_key() as DimensionType - 1
    }

    pub fn lparent(&self) -> Self
    {
        let mut hs = get_hall_set::<NLETTERS>(None);
        let (key, lparent, rparent, sz) = hs.rparent_details(self.get_rparent() as DataType)
            .expect("encountered a problem getting parent data");
        Self(Self::make_key_data(key, lparent, rparent, sz))
    }

    pub fn parent(&self) -> Self
    {
        let mut hs = get_hall_set::<NLETTERS>(None);
        let (key, lparent, rparent, sz) = hs.rparent_details(self.get_rparent() as DataType)
            .expect("encountered a problem getting parent data");

        Self(Self::make_key_data(key, lparent, rparent, sz))
    }

}


impl<const NLETTERS: DegreeType> Debug for HallKey<NLETTERS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "HallKey(key={}, lparent={}, rparent={}, size={})",
               self.get_key(), self.get_lparent(), self.get_rparent(), self.size())
    }
}


impl<const NLETTERS: DegreeType> Display for HallKey<NLETTERS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_letter() {
            write!(f, "{}", self.get_key())
        } else {
            write!(f, "[{}, {}]", self.get_lparent(), self.get_rparent())
        }
    }
}