use std::str::FromStr;
use rangemap::{StepLite};
use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct StoreCoords(pub usize, pub usize, pub usize);

impl StoreCoords {
    pub fn previous(&self, store_max_idx: usize) -> Option<Self> {
        if self.2 != 0 {
            return Some((self.0, self.1, self.2-1).into());
        }
        if self.1 != 0 {
            return Some((self.0, self.1 - 1, store_max_idx - 1).into());
        }
        if self.0 != 0 {
            return Some((self.0 - 1, store_max_idx - 1, store_max_idx -1).into());
        }
        None
    }
    
    pub fn next(&self,store_max_idx: usize) -> Option<Self> {
        if self.2 != store_max_idx - 1 {
            return Some((self.0, self.1, self.2+1).into());
        }
        if self.1 != store_max_idx - 1 {
            return Some((self.0, self.1+1, 0).into());
        }
        if self.0 != store_max_idx - 1 {
            return Some((self.0+1, 0, 0).into());
        }
        None
    }
}
impl From<(usize, usize, usize)> for StoreCoords {
    fn from(value: (usize, usize, usize)) -> Self {
        StoreCoords(value.0, value.1, value.2)
    }
}

impl FromStr for StoreCoords {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.trim().split(' ');

        let row = crate::get_from_iterator_and_parse(&mut splits)?;
        let shelf = crate::get_from_iterator_and_parse(&mut splits)?;
        let zone = crate::get_from_iterator_and_parse(&mut splits)?;

        if splits.next().is_some() {
            return Err(());
        }

        Ok(StoreCoords(row, shelf, zone))
    }
}

impl From<&StoreCoords> for (usize, usize, usize) {
    fn from(value: &StoreCoords) -> Self {
        (value.0, value.1, value.2)
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct LimitStoreCoords {
    coords: StoreCoords,
    max_idx: usize,
}

impl LimitStoreCoords {
    pub fn from_with_max(coords: StoreCoords, max_idx: usize) -> Self {
        LimitStoreCoords {
            coords,
            max_idx
        }
    }
}

impl From<&LimitStoreCoords> for StoreCoords {
    fn from(value: &LimitStoreCoords) -> Self {
        value.coords.clone()
    }
}

impl StepLite for LimitStoreCoords {
    fn add_one(&self) -> Self {
        let coords = self.coords.next(self.max_idx).unwrap_or((self.max_idx,self.max_idx,self.max_idx).into());
        
        LimitStoreCoords {
            coords, 
            max_idx: self.max_idx
        }
    }

    fn sub_one(&self) -> Self {
        let coords = self.coords.previous(self.max_idx).unwrap_or((self.max_idx,self.max_idx,self.max_idx).into());

        LimitStoreCoords {
            coords,
            max_idx: self.max_idx
        }
    }
}