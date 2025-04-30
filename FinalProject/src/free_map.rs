use std::ops::{RangeInclusive};
use rangemap::RangeInclusiveSet;
use serde_derive::{Deserialize, Serialize};
use crate::coords::{LimitStoreCoords, StoreCoords};

#[derive(Serialize, Deserialize)]
pub struct FreeMap {
    store_max_idx: usize,
    map: RangeInclusiveSet<LimitStoreCoords>,
}

impl FreeMap {
    pub fn new(store_max_idx: usize) -> FreeMap {
        let mut map = RangeInclusiveSet::new();
        map.insert(
            LimitStoreCoords::from_with_max((0,0,0).into(),store_max_idx)
                ..=
                LimitStoreCoords::from_with_max((store_max_idx-1,store_max_idx-1,store_max_idx-1).into(),store_max_idx)
        );
        
        FreeMap {
            store_max_idx,
            map,
        }
    }

    pub fn occupy_single(&mut self, place: StoreCoords) -> bool {
        let place = LimitStoreCoords::from_with_max(place,self.store_max_idx);
        if !self.map.contains(&place) {
            return false;
        }

        self.map.remove(place.clone()..=place);
        true
    }
    
    pub fn occupy_range(&mut self, range: RangeInclusive<StoreCoords>) -> bool
    {
        let range = 
            LimitStoreCoords::from_with_max(range.start().clone(), self.store_max_idx)
            ..=
            LimitStoreCoords::from_with_max(range.end().clone(), self.store_max_idx);
        
        if !self.map.overlaps(&range) {
            return false;
        }
        
        self.map.remove(range);
        true
    }
    
    pub fn free_single(&mut self, place: StoreCoords) -> bool {
        let place = LimitStoreCoords::from_with_max(place,self.store_max_idx);
        if self.map.contains(&place) {
            return false;
        }
        
        self.map.insert(place.clone()..=place);
        true
    }
    
    pub fn free_range(&mut self, range: RangeInclusive<StoreCoords>) -> bool {
        let range =
            LimitStoreCoords::from_with_max(range.start().clone(), self.store_max_idx)
                ..=
                LimitStoreCoords::from_with_max(range.end().clone(), self.store_max_idx);
        if self.map.overlaps(&range) {
            return false;
        }
        
        self.map.insert(range);
        true
    }
    
    pub fn iter(&self) -> impl Iterator<Item=RangeInclusive<StoreCoords>> {
        self.map.iter().map(|i| i.start().into()..=i.end().into())
    }
    
    #[allow(unused)]
    pub fn iter_from(&self, place: StoreCoords) -> impl Iterator<Item=RangeInclusive<StoreCoords>> {
        let place = LimitStoreCoords::from_with_max(place,self.store_max_idx);
        let max = LimitStoreCoords::from_with_max((self.store_max_idx-1,self.store_max_idx-1,self.store_max_idx-1).into(),self.store_max_idx);
        self.map.overlapping(place..=max).map(|i| i.start().into()..=i.end().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_persist() {
        let mut map = FreeMap::new(10);
        assert_eq!(map.occupy_single((0,0,0).into()), true);
        assert_eq!(map.occupy_single((0,0,0).into()), false);
    }
    
    #[test]
    fn test_persist_range() {
        let mut map = FreeMap::new(10);
        assert_eq!(map.occupy_range((1,0,0).into()..=(3,0,0).into()), true);
        assert_eq!(map.occupy_range((1,0,0).into()..=(2,0,0).into()), false);
    }
    
    #[test]
    fn test_iter_single() {
        let mut map = FreeMap::new(10);
        assert_eq!(map.occupy_single((1,0,0).into()), true);
        
        let mut iter = map.iter();
        
        assert_eq!(iter.next(), Some((0,0,0).into()..=(0,9,9).into()));
        assert_eq!(iter.next(), Some((1,0,1).into()..=(9,9,9).into()));
        assert_eq!(iter.next(), None);
    }
    
    #[test]
    fn test_iter_single_max() {
        let mut map = FreeMap::new(10);
        map.occupy_single((0,0,9).into());
        
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((0,0,0).into()..=(0,0,8).into()));
        assert_eq!(iter.next(), Some((0,1,0).into()..=(9,9,9).into()));
        assert_eq!(iter.next(), None);
    }
    
    #[test]
    fn test_iter_range_max() {
        let mut map = FreeMap::new(10);
        map.occupy_range((0,0,5).into()..=(0,0,9).into());
        
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((0,0,0).into()..=(0,0,4).into()));
        assert_eq!(iter.next(), Some((0,1,0).into()..=(9,9,9).into()));
        assert_eq!(iter.next(), None)
    }
    
    #[test]
    fn test_iter_range() {
        let mut map = FreeMap::new(10);
        assert_eq!(map.occupy_range((1,0,0).into()..=(1,9,9).into()), true);

        let mut iter = map.iter();

        assert_eq!(iter.next(), Some((0,0,0).into()..=(0,9,9).into()));
        assert_eq!(iter.next(), Some((2,0,0).into()..=(9,9,9).into()));
        assert_eq!(iter.next(), None);
    }
    
    #[test]
    fn test_free_single() {
        let mut map = FreeMap::new(10);
        assert_eq!(map.occupy_single((1,0,0).into()), true);
        assert_eq!(map.free_single((1,0,0).into()), true);
    }
}
