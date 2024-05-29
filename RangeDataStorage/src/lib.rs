use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::ops::AddAssign;
use time;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
struct RangeDataStorage <K, V, E> where K: Eq + Hash + AddAssign, V: Clone, E: Eq + Hash + AddAssign{
    data: HashMap<K, V>,
}

impl<K, V, E> RangeDataStorage<K, V, E> where
    K: Eq + Hash + AddAssign {
    pub fn new<K, V>() -> RangeDataStorage<K, V, E>{
        RangeDataStorage{
            data: HashMap::new(),
        }
    }

    /// Stores an element into the internal hash map, whilst concatenating the
    /// end_of_range element to the value; this end_of_range element
    pub fn store(&mut self, mut key: K, val: V, end_of_range: E) {
        self.data.insert(key, (val, end_of_range));
    }

    pub fn get_range(&mut self, key: K) {
        // key is a key into the data hashmap, we get elements by incrementing it by 1
        // until we get either a non-existent element or the end_of_range element.
        // we should probably panic if we get a None.
    }
}
