
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock, Mutex};


use lazy_static::lazy_static;

use crate::{DegreeType, LetterType, DimensionType, SignedDimensionType};
use super::DataType;

use super::degree_range_map::{DegreeRangeMap, DegreeRangeMapError};
use std::fmt::Formatter;


lazy_static! {
    static ref HALL_SET_CACHE: RwLock<HashMap<DegreeType, HallSetObject>> =
        RwLock::new(HashMap::new());
}

#[derive(Debug)]
#[non_exhaustive]
pub enum HallSetError {
    HallSetTooSmall,
    HallSetReadError,
    HallSetWriteError,
    OtherError(Box<dyn Error>)
}

use HallSetError::*;

impl fmt::Display for HallSetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use HallSetError::*;
        match self {
            HallSetTooSmall => write!(f, "the hall set is not sufficiently large"),
            OtherError(ref inner) => write!(f, "another error occurred: {}", inner),
            _ => write!(f, "an unknown error occured")
        }
    }
}

impl Error for HallSetError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            HallSetError::OtherError(ref inner) => Some(inner.as_ref()),
            _ => None
        }
    }
}

impl From<DegreeRangeMapError> for HallSetError {
    fn from(_: DegreeRangeMapError) -> Self {
        HallSetTooSmall
    }
}


pub type ParentInfo = (DataType, DataType, DataType, DegreeType);



pub trait HallSet {

    fn is_letter(&self, key: DataType) -> Result<bool, HallSetError>;
    fn lparent_details(&self, key: DataType) -> Result<ParentInfo, HallSetError>;
    fn rparent_details(&self, key: DataType) -> Result<ParentInfo, HallSetError>;
    fn key_details(&self, key: DataType) -> Result<ParentInfo, HallSetError>;
    fn grow_up(&mut self, new_degree: DegreeType) -> Result<(), HallSetError>;
    fn degree_range(&self, deg: DegreeType) -> Result<(DimensionType, DimensionType), HallSetError>;

}

#[derive(Clone)]
pub struct HallSetObject(Arc<RwLock<dyn HallSet + Send + Sync>>);

impl HallSetObject {

    fn new(obj: impl HallSet + Send + Sync + 'static) -> Self
    {
        Self(Arc::new(RwLock::new(obj)))
    }
}

impl HallSet for HallSetObject {

    fn is_letter(&self, key: DataType) -> Result<bool, HallSetError> {
        let inner = self.0.read()
            .map_err(|_| HallSetError::HallSetReadError)?;
        inner.is_letter(key)
    }

    fn lparent_details(&self, key: DataType) -> Result<ParentInfo, HallSetError> {
        let inner = self.0.read()
            .map_err(|_| HallSetError::HallSetReadError)?;
        inner.lparent_details(key)
    }

    fn rparent_details(&self, key: DataType) -> Result<ParentInfo, HallSetError> {
        let inner = self.0.read()
            .map_err(|_| HallSetError::HallSetReadError)?;
        inner.rparent_details(key)
    }

    fn key_details(&self, key: DataType) -> Result<ParentInfo, HallSetError> {
        let inner = self.0.read()
            .map_err(|_| HallSetError::HallSetReadError)?;
        inner.key_details(key)
    }

    fn grow_up(&mut self, new_degree: DegreeType) -> Result<(), HallSetError> {
        let mut inner = self.0.write()
            .map_err(|_| HallSetError::HallSetWriteError)?;
        inner.grow_up(new_degree)
    }

    fn degree_range(&self, deg: DegreeType) -> Result<(DimensionType, DimensionType), HallSetError> {
        let inner = self.0.read()
            .map_err(|_| HallSetError::HallSetReadError)?;
        inner.degree_range(deg)
    }
}



const fn starting_size_for_width(letters: DegreeType) -> DegreeType
{
    match letters {
        2 => 30,
        3..=5 => 20,
        6..=10 => 8,
        11..=20 => 3,
        _ => 2
    }
}



struct HallSetInner<const NLETTERS: DegreeType> {
    current_degree: DegreeType,
    set: Vec<(LetterType, LetterType)>,
    reverse: HashMap<(LetterType, LetterType), DataType>,
    degrees: DegreeRangeMap<NLETTERS>
}

impl<const NLETTERS: DegreeType> HallSetInner<NLETTERS> {

    fn new() -> Self {
        Self {
            current_degree:0,
            set: vec![(0, 0)],
            reverse: HashMap::new(),
            degrees: DegreeRangeMap::new(starting_size_for_width(NLETTERS))
        }
    }


    pub fn get_parents(&self, key: DataType) -> Option<&(LetterType, LetterType)>
    {
        self.set.get(key as usize)
    }

    pub fn degree(&self, key: DataType) -> Option<DegreeType>
    {
        self.degrees.find_deg(key).ok()
    }

    pub fn curr_degree(&self) -> DegreeType
    {
        self.current_degree
    }

    pub fn grow_up(&mut self, new_degree: DegreeType)
    {
        // Reserve for efficiency

        while self.current_degree < new_degree {
            self.current_degree += 1;
/*
            for e in 1..(self.current_degree / 2) {
                let (i_lower, i_upper) = self.degree_ranges[e];
                let (j_lower, j_upper) = self.degree_ranges[self.current_degree - e];

                for i in i_lower..i_upper {
                    for j in cmp::max(j_lower, i+1)..j_upper {
                        if self.set[j].0 <= i {
                            self.set.push((i, j));
                            self.degrees.push(self.current_degree);
                            self.reverse[(i, j)] = self.set.len() - 1;
                        }
                    }
                }
            }
*/


        }
    }

}

impl<const NLETTERS: DegreeType> HallSet for HallSetInner<NLETTERS> {
    fn is_letter(&self, key: DataType) -> Result<bool, HallSetError> {
        Ok(1 <= key && key <= NLETTERS as DataType)
    }

    fn lparent_details(&self, key: DataType) -> Result<ParentInfo, HallSetError> {
        if let Some((parent, _)) = self.get_parents(key) {
            let p = *parent as DataType;
            let (l, r) = self.get_parents(p).unwrap();
            Ok((p, *l as DataType, *r as DataType, self.degree(p).unwrap()))
        } else {
            Err(HallSetError::HallSetTooSmall)
        }

    }

    fn rparent_details(&self, key: DataType) -> Result<ParentInfo, HallSetError> {
        if let Some((_, parent)) = self.get_parents(key) {
            let p = *parent as DataType;
            let (l, r) = self.get_parents(p).unwrap();
            Ok((p, *l as DataType, *r as DataType, self.degree(p).unwrap()))
        } else {
            Err(HallSetError::HallSetTooSmall)
        }
    }

    fn key_details(&self, key: DataType) -> Result<ParentInfo, HallSetError> {
        if let Some((l, r)) = self.get_parents(key) {
            Ok((key, *l as DataType, *r as DataType, self.degree(key).unwrap()))
        } else {
            Err(HallSetError::HallSetTooSmall)
        }
    }

    fn grow_up(&mut self, new_degree: DegreeType) -> Result<(), HallSetError> {
        todo!()
    }

    fn degree_range(&self, deg: DegreeType) -> Result<(DimensionType, DimensionType), HallSetError> {
        Ok((self.degrees.get(deg)?, self.degrees.get(deg+1)?))
    }
}




pub fn get_hall_set<const NLETTERS: DegreeType>(pre_grow: Option<DegreeType>) -> HallSetObject
{
    if let Some(val) = HALL_SET_CACHE.read()
        .expect("Error retrieving from hall set cache")
        .get(&NLETTERS) {
        val.clone()
    } else {
        let mut inner = HallSetInner::<NLETTERS>::new();

        if let Some(deg) = pre_grow {
            inner.grow_up(deg);
        } else {
            inner.grow_up(1);
        }

        let new_hs = HallSetObject::new(inner);
        HALL_SET_CACHE.write().expect("Could not get write access to hall set cache")
            .insert(NLETTERS, new_hs.clone());
        new_hs
    }

}




#[cfg(test)]
mod tests {

    use super::*;






}