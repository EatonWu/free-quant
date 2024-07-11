use ibapi;
use range_data_storage::range_data_storage::RangeDataStorage;
use std::collections::HashMap;
use anyhow::{bail, Error};
use std::path::Path;
use ibapi::market_data::historical::BarSize;
use rangemap::StepLite;
use serde::de::DeserializeOwned;
use serde::Serialize;
use ibapi_handler::IbapiHandler;

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

impl HashedBarSize {
    pub fn to_bar_size(&self) -> BarSize {
        match self {
            HashedBarSize::Sec => BarSize::Sec,
            HashedBarSize::Sec5 => BarSize::Sec5,
            HashedBarSize::Sec15 => BarSize::Sec15,
            HashedBarSize::Sec30 => BarSize::Sec30,
            HashedBarSize::Min => BarSize::Min,
            HashedBarSize::Min2 => BarSize::Min2,
            HashedBarSize::Min3 => BarSize::Min3,
            HashedBarSize::Min5 => BarSize::Min5,
            HashedBarSize::Min15 => BarSize::Min15,
            HashedBarSize::Min20 => BarSize::Min20,
            HashedBarSize::Min30 => BarSize::Min30,
            HashedBarSize::Hour => BarSize::Hour,
            HashedBarSize::Hour2 => BarSize::Hour2,
            HashedBarSize::Hour3 => BarSize::Hour3,
            HashedBarSize::Hour4 => BarSize::Hour4,
            HashedBarSize::Hour8 => BarSize::Hour8,
            HashedBarSize::Day => BarSize::Day,
            HashedBarSize::Week => BarSize::Week,
            HashedBarSize::Month => BarSize::Month,
        }
    }

    pub fn from_bar_size(bar_size: BarSize) -> Self {
        match bar_size {
            BarSize::Sec => HashedBarSize::Sec,
            BarSize::Sec5 => HashedBarSize::Sec5,
            BarSize::Sec15 => HashedBarSize::Sec15,
            BarSize::Sec30 => HashedBarSize::Sec30,
            BarSize::Min => HashedBarSize::Min,
            BarSize::Min2 => HashedBarSize::Min2,
            BarSize::Min3 => HashedBarSize::Min3,
            BarSize::Min5 => HashedBarSize::Min5,
            BarSize::Min15 => HashedBarSize::Min15,
            BarSize::Min20 => HashedBarSize::Min20,
            BarSize::Min30 => HashedBarSize::Min30,
            BarSize::Hour => HashedBarSize::Hour,
            BarSize::Hour2 => HashedBarSize::Hour2,
            BarSize::Hour3 => HashedBarSize::Hour3,
            BarSize::Hour4 => HashedBarSize::Hour4,
            BarSize::Hour8 => HashedBarSize::Hour8,
            BarSize::Day => HashedBarSize::Day,
            BarSize::Week => HashedBarSize::Week,
            BarSize::Month => HashedBarSize::Month,
        }
    }

    pub fn to_location(&self) -> String {
        match self {
            HashedBarSize::Sec => "sec.json".to_string(),
            HashedBarSize::Sec5 => "sec5.json".to_string(),
            HashedBarSize::Sec15 => "sec15.json".to_string(),
            HashedBarSize::Sec30 => "sec30.json".to_string(),
            HashedBarSize::Min => "min.json".to_string(),
            HashedBarSize::Min2 => "min2.json".to_string(),
            HashedBarSize::Min3 => "min3.json".to_string(),
            HashedBarSize::Min5 => "min5.json".to_string(),
            HashedBarSize::Min15 => "min15.json".to_string(),
            HashedBarSize::Min20 => "min20.json".to_string(),
            HashedBarSize::Min30 => "min30.json".to_string(),
            HashedBarSize::Hour => "hour.json".to_string(),
            HashedBarSize::Hour2 => "hour2.json".to_string(),
            HashedBarSize::Hour3 => "hour3.json".to_string(),
            HashedBarSize::Hour4 => "hour4.json".to_string(),
            HashedBarSize::Hour8 => "hour8.json".to_string(),
            HashedBarSize::Day => "day.json".to_string(),
            HashedBarSize::Week => "week.json".to_string(),
            HashedBarSize::Month => "month.json".to_string(),
        }
    }
}

pub struct DataBroker<K, V> where K: Ord + Clone + Eq + Serialize + DeserializeOwned + StepLite, V: Serialize + Eq + Clone + DeserializeOwned{
    storage_directory: String, // the root directory of the data
    ticker_map: HashMap<String, Option<HashMap<HashedBarSize, RangeDataStorage<K, V>>>>, // map from tickers to bar sizes to data
}

/// lazily maps available tickers in storage directory to bar sizes.
/// initially the bar sizes point to None until the data is requested,
/// after which the data is retrieved from disk.
impl<K, V> DataBroker<K, V> where K: Ord + Clone + Eq + Serialize + DeserializeOwned + StepLite, V: Serialize + Eq + Clone + DeserializeOwned{
    pub fn new(storage_location: Option<String>) -> Result<Self, Error> {
        let loc = storage_location.unwrap_or_else(|| "@data".to_string());
        let path = Path::new(&loc);


        let exists = path.try_exists()?;
        return if !exists {
            // create the directory
            std::fs::create_dir_all(path)?;
            Ok(DataBroker {
                storage_directory: loc,
                ticker_map: HashMap::new(),
            })
        } else { // file exists, need to lazily evaluate hashmap
            let mut ticker_map = HashMap::new();
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let file_name_res = path.file_name();
                    if file_name_res.is_none() {
                        continue;
                    }
                    let dir_name = file_name_res.unwrap().to_str().unwrap().to_string();
                    ticker_map.insert(dir_name, None);
                }
            }
            Ok(DataBroker {
                storage_directory: loc,
                ticker_map,
            })
        }
    }

    /// Given some ticker, retrieves the data for that ticker and timeframe.
    pub fn retrieve_data(&mut self, ticker: String, timeframe: HashedBarSize, start_date: K, end_date: K) -> Result<&V, Error> {
        if let None = self.ticker_map.get(&ticker) { // the ticker directory does not exist
            let mut handler = IbapiHandler::new()?;
        }
        bail!("Not implemented");
    }

    // TODO
    pub fn has_range(&mut self, ticker: String, timeframe: HashedBarSize, start_date: K, end_date: K) -> Result<bool, Error> {
        if let Some(None) = self.ticker_map.get(&ticker) {
            // ticker exists but hasn't been fully serialized yet
            Ok(false)
        } else {
            Ok(false)
        }
    }

    pub fn insert(&mut self, timeframe: HashedBarSize, start_date: K, end_date: K, value: V) -> Result<(), Error>{
        Ok(())
    }

    pub fn get_num_tickers(&self) -> usize {
        self.ticker_map.len()
    }
}