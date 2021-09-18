
use std::error::Error;
use std::fmt;

use crate::{DegreeType, DimensionType, SignedDimensionType};
use super::DataType;

use std::fmt::Formatter;



mod compute {
    use crate::{DimensionType, SignedDimensionType};

    const fn is_squarefree_impl(n: DimensionType, base: DimensionType) -> bool
    {
        (n % (base * base)) != 0 && (base * base >= n || is_squarefree_impl(n, base+1))
    }

    const fn is_squarefree(n: DimensionType) -> bool
    {
        is_squarefree_impl(n, 2)
    }

    const fn mobius_impl(n: DimensionType, divisor: DimensionType) -> SignedDimensionType
    {
        if divisor == n {
            -1
        } else if n % divisor == 0 {
            mobius(n) * mobius(n / divisor)
        } else {
            mobius_impl(n, divisor+1)
        }
    }

    const fn mobius(n: DimensionType) -> SignedDimensionType
    {
        if n == 1 {
            1
        } else if !is_squarefree(n) {
            0
        } else {
            mobius_impl(n, 2)
        }
    }

    const fn hall_set_level_term(
        width: DimensionType,
        level: DimensionType,
        divisor: DimensionType
    ) -> SignedDimensionType
    {
        mobius(divisor) *
            (width as SignedDimensionType).pow((level/divisor) as u32) /
            (level as SignedDimensionType)
    }

    const fn hall_set_level_size_impl(
        width: DimensionType,
        level:DimensionType,
        divisor: DimensionType
    ) -> SignedDimensionType
    {
        let a;
        let b;

        if level % divisor == 0 {
            a = hall_set_level_term(width, level, divisor);
        } else {
            a = 0;
        }

        if divisor < level {
            b = hall_set_level_size_impl(width, level, divisor+1);
        } else {
            b = 0;
        }

        a + b
    }

    const fn hall_set_level_size(width: DimensionType, depth: DimensionType) -> DimensionType
    {
        hall_set_level_size_impl(width, depth, 1) as DimensionType
    }

    const fn hall_set_size_impl(
        width: DimensionType,
        depth: DimensionType,
        level: DimensionType
    ) -> DimensionType
    {
        let a = hall_set_level_size(width, level);

        let b;
        if level < depth {
            b = hall_set_size_impl(width, depth, level + 1);
        } else {
            b = 0;
        }

        a + b as DimensionType
    }

    pub const fn hall_set_size(width: DimensionType, depth: DimensionType) -> DimensionType
    {
        if depth == 0 {
            0
        } else {
            hall_set_size_impl(width, depth, 1)
        }
    }

}

use compute::hall_set_size;




#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DegreeRangeMapError {
    MapIsEmpty,
    DegreeOutOfBounds
}
use DegreeRangeMapError::*;


impl fmt::Display for DegreeRangeMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MapIsEmpty => write!(f, "degree range map is empty"),
            DegreeOutOfBounds => write!(f, "requested degree is out of bounds"),
            _ => write!(f, "unknown degree range map error")
        }
    }
}

impl Error for DegreeRangeMapError {}


pub struct DegreeRangeMap<const NLETTERS: DegreeType>(Vec<DimensionType>);

impl<const NLETTERS: DegreeType> DegreeRangeMap<NLETTERS> {

    pub fn new(degree: DegreeType) -> Self
    {
        let mut inner = Vec::with_capacity(degree as usize);
        for i in 0..(degree as DimensionType) {
            inner.push(hall_set_size(NLETTERS as DimensionType, i));
        }
        Self(inner)
    }

    pub fn grow(&mut self, requested: DegreeType) -> Result<(), DegreeRangeMapError>
    {
        let len = self.0.len();
        if (requested as usize) < len {
            return Ok(());
        }

        let ureq = requested as usize;

        self.0.reserve(ureq + 1 - len);

        for d in len..=ureq {
            self.0.push(hall_set_size(NLETTERS as DimensionType, d));
        }

        Ok(())
    }

    pub fn get(&self, deg: DegreeType) -> Result<DimensionType, DegreeRangeMapError>
    {
        if let Some(v) = self.0.get(deg as usize) {
            Ok(*v)
        } else {
            Err(DegreeOutOfBounds)
        }
    }

    pub fn find_deg(&self, key: DataType) -> Result<DegreeType, DegreeRangeMapError>
        {
            if self.0.is_empty() {
                return Err(MapIsEmpty);
            }

            let dkey = key as DimensionType;

            if let Some(&last) = self.0.last() {
                if last < dkey {
                    return Err(DegreeOutOfBounds);
                }
            } else {
                return Err(DegreeOutOfBounds);
            }

            Ok(self.0.partition_point(move |&v| v <= dkey) as DegreeType)
        }

}






#[cfg(test)]
mod tests {
    use super::*;

    type Map = DegreeRangeMap<5>;

    #[test]
    fn test_degree_range_depth_0() {
        let m = Map::new(6);

        assert_eq!(m.get(0).unwrap(), 0);
    }

    #[test]
    fn test_degree_range_depth_1() {
        let m = Map::new(6);

        assert_eq!(m.get(1).unwrap(), 5);
    }

    #[test]
    fn test_degree_range_depth_2() {
        let m = Map::new(6);

        assert_eq!(m.get(2).unwrap(), 15);
    }

    #[test]
    fn test_degree_range_depth_3() {
        let m = Map::new(6);

        assert_eq!(m.get(3).unwrap(), 55);
    }

    #[test]
    fn test_degree_range_depth_4() {
        let m = Map::new(6);

        assert_eq!(m.get(4).unwrap(), 205);
    }

    #[test]
    fn test_degree_range_depth_5() {
        let m = Map::new(6);

        assert_eq!(m.get(5).unwrap(), 829);
    }



}