use ibapi;
use range_data_storage::range_data_storage::RangeDataStorage;
use std::collections::HashMap;
use ibapi::market_data::historical::BarSize;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum HashedBarSize {
    Sec,
    Sec5,
    Sec15,
    Sec30,
    Min,
    Min2,
    Min3,
    Min5,
    Min15,
    Min20,
    Min30,
    Hour,
    Hour2,
    Hour3,
    Hour4,
    Hour8,
    Day,
    Week,
    Month,
}

pub struct DataBroker<K, V> {
    timeframe_to_data_map: HashMap<BarSize, RangeDataStorage<K, V>>,
}

impl<K, V> DataBroker<K, V> {
    pub fn new() -> Self {
        Self {
            timeframe_to_data_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, bar_size: BarSize, key_start: K, key_end: K, value: V) {
        let range_data_storage = self.timeframe_to_data_map.entry(bar_size).or_insert_with(|| RangeDataStorage::new(None).unwrap());
        range_data_storage.insert(key_start, key_end, value);
    }

    pub fn get(&self, bar_size: BarSize, key: K) -> Option<&V> {
        self.timeframe_to_data_map.get(&bar_size).and_then(|range_data_storage| range_data_storage.get(key))
    }
}