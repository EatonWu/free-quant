use ibapi;
use range_data_storage::range_data_storage::RangeDataStorage;
use std::collections::HashMap;
use anyhow::{bail, Error};
use std::path::Path;
use ibapi::market_data::historical::BarSize;
use time::OffsetDateTime;
use ibapi_handler::{IBApiBar, IbapiHandler};

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

    pub fn from_filename(filename: &String) -> Result<Self, Error> {
        match filename.as_str() {
            "sec.json" => Ok(HashedBarSize::Sec),
            "sec5.json" => Ok(HashedBarSize::Sec5),
            "sec15.json" => Ok(HashedBarSize::Sec15),
            "sec30.json" => Ok(HashedBarSize::Sec30),
            "min.json" => Ok(HashedBarSize::Min),
            "min2.json" => Ok(HashedBarSize::Min2),
            "min3.json" => Ok(HashedBarSize::Min3),
            "min5.json" => Ok(HashedBarSize::Min5),
            "min15.json" => Ok(HashedBarSize::Min15),
            "min20.json" => Ok(HashedBarSize::Min20),
            "min30.json" => Ok(HashedBarSize::Min30),
            "hour.json" => Ok(HashedBarSize::Hour),
            "hour2.json" => Ok(HashedBarSize::Hour2),
            "hour3.json" => Ok(HashedBarSize::Hour3),
            "hour4.json" => Ok(HashedBarSize::Hour4),
            "hour8.json" => Ok(HashedBarSize::Hour8),
            "day.json" => Ok(HashedBarSize::Day),
            "week.json" => Ok(HashedBarSize::Week),
            "month.json" => Ok(HashedBarSize::Month),
            _ => bail!("Could not get HashedBarSize from '{}'", filename)
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

pub struct DataBroker{
    storage_directory: String, // the root directory of the data
    ticker_map: HashMap<String, Option<HashMap<HashedBarSize, Option<RangeDataStorage<i64, Vec<IBApiBar>>>>>>, // map from tickers to bar sizes to data
    ibapi_handler: IbapiHandler,
}

/// lazily maps available tickers in storage directory to bar sizes.
/// initially the bar sizes point to None until the data is requested,
/// after which the data is retrieved from disk.
impl DataBroker {
    pub fn new(storage_location: Option<String>) -> Result<Self, Error> {
        let loc = storage_location.unwrap_or_else(|| "@data".to_string());
        let path = Path::new(&loc);
        // TODO: Do i really need to initialize this before I need it? Could be an Option instead.
        let handler = IbapiHandler::new()?;

        let exists = path.try_exists()?;
        return if !exists {
            // create the directory
            std::fs::create_dir_all(path)?;
            Ok(DataBroker {
                storage_directory: loc,
                ticker_map: HashMap::new(),
                ibapi_handler: handler,
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
                ibapi_handler: handler,
            })
        }
    }

    /// Returning None on this means that "this directory exists but i dont know what's in it".
    /// Returning Some(None) is not a valid state.
    /// Returning Some(
    fn ticker_exists(&self, ticker: String) -> bool {
        self.ticker_map.get(&ticker).is_some()
    }

    /// For a ticker/barsize pair, checks if the data has been realized
    /// (loaded into memory from disk)
    fn pull_range_data_store_from_disk(&self, ticker: &String, timeframe: &HashedBarSize) -> Result<RangeDataStorage<i64, Vec<IBApiBar>>, Error> {
        let path =
            Path::new(&self.storage_directory)
            .join(ticker)
            .join(timeframe.to_location());
        if !path.exists() {
            bail!("Timeframe data does not exist for ticker: {} at path: {}", ticker, path.display());
        }
        let data = std::fs::read_to_string(path)?;
        let data: RangeDataStorage<i64, Vec<IBApiBar>> = serde_json::from_str(&data)?;
        Ok(data)
    }

    pub fn convert_osstr_to_string(osstr: Option<&std::ffi::OsStr>) -> Result<String, Error> {
        let thing = match osstr {
            Some(thing) => thing,
            None => bail!("Could not convert OsStr to string")
        };
        let thing = thing.to_str();
        match thing {
            Some(thing) => Ok(thing.to_string()),
            None => bail!("Could not convert OsStr to string")
        }
    }

    fn realize_ticker_dir(&self, ticker: &String) -> Result<HashMap<HashedBarSize, Option<RangeDataStorage<i64, Vec<IBApiBar>>>>, Error> {
        // check if the ticker directory exists, else create it
        let path = Path::new(&self.storage_directory).join(ticker);
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        let mut bar_size_map = HashMap::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = Self::convert_osstr_to_string(path.file_name())?;
                let bar_size = HashedBarSize::from_filename(&file_name)?;
                bar_size_map.insert(bar_size, None);
            }
        }
        Ok(bar_size_map)
    }

    fn get_data_and_save(&mut self, ticker: &String, timeframe: &HashedBarSize, start_date: OffsetDateTime, end_date: OffsetDateTime) -> Result<RangeDataStorage<i64, Vec<IBApiBar>>, Error> {
        println!("Getting data for ticker: {} and timeframe: {:?}", ticker, timeframe);
        let contract = ibapi::contracts::Contract::stock(ticker.as_str());
        let data = self.ibapi_handler.get_historical_data(&contract, timeframe.to_bar_size(), start_date, end_date)?;
        let path = Path::new(&self.storage_directory).join(ticker).join(timeframe.to_location());
        let mut range_data_storage = RangeDataStorage::new(None)?;
        let vec_data = Vec::from(data);
        range_data_storage.insert(start_date.unix_timestamp(), end_date.unix_timestamp(), vec_data);
        let str_path = Self::convert_osstr_to_string(Some(path.as_os_str()))?;
        println!("Saving data to path: {}", str_path);
        let save_result = range_data_storage.save(str_path);
        match save_result {
            Ok(_) => Ok(range_data_storage),
            Err(e) => bail!(e)
        }
    }

    /// Given some ticker, retrieves the data for that ticker and timeframe.
    /// Checks if data exists in the ticker_map/range_data_storage, if so, retrieves from disk.
    /// Otherwise, retrieves the data using ibapi.
    /// The ticker_map is:
    /// map of tickers -> Option<Map of bar sizes -> Option<RangeDataStorage>>
    /// ticker_map.get(ticker) -> None, ticker does not exist in the map
    /// ticker_map.get(ticker) -> Some(None), ticker exists but has not been fully evaluated
    /// ticker_map.get(ticker) -> Some(Some(bar_size_map)), ticker exists, and has at least
    /// been partially realized (not all bar sizes are evaluated when a retrieval request is made)
    ///
    /// bar_size_map.get(BarSize) -> None, bar size does not exist in the map
    /// bar_size_map.get(BarSize) -> Some(None), bar size exists but has not been fully evaluated
    /// bar_size_map.get(BarSize) -> Some(Some(RangeDataStorage)), bar size exists and has been fully evaluated
    ///
    /// TODO: THIS FUNCTION SHOULD ONLY BE ALLOWED TO FAIL IF WE EXHAUST ALL POSSIBLE DATA SOURCES
    pub fn retrieve_data(&mut self, ticker: String, timeframe: HashedBarSize, start_date: OffsetDateTime, end_date: OffsetDateTime) -> Result<Vec<IBApiBar>, Error> {
        // step 1. check if the bar size map exists
        match self.ticker_map.get(&ticker) {
            // We do not have knowledge of this ticker, so we need request it using
            // the ibapi_handler.
            None | Some(None) => {
                let bar_size_map = self.realize_ticker_dir(&ticker)?;
                self.ticker_map.insert(ticker.clone(), Some(bar_size_map));
            },
            _ => {}
        }

        let bar_size_map = self.ticker_map.get_mut(&ticker);
        let bar_size_map = match bar_size_map {
            Some(Some(bar_map)) => bar_map,
            _ => bail!("Could not retrieve data for ticker: {} and timeframe: {:?}", ticker, timeframe)
        };

        let mut owned_bar_size_map = bar_size_map.clone();

        // step 2. check if the range data storage exists
        // bar size map maps the bar size (15 seconds, 30 seconds, etc) to the RangeDataStorage containing it

        match owned_bar_size_map.get(&timeframe) {
            // We do not have knowledge of this bar size, so we need to request it using
            // the ibapi_handler.
            None => {
                println!("Data does not exist in bar size map");
                let data = self.get_data_and_save(&ticker, &timeframe, start_date, end_date)?;
                owned_bar_size_map.insert(timeframe, Some(data));
            },
            Some(None) => {
                println!("Data partially realized");
                let data = self.pull_range_data_store_from_disk(&ticker, &timeframe)?;
                owned_bar_size_map.insert(timeframe, Some(data));
            },
            Some(Some(_)) => {
                println!("Data already instantiated")
            }
        }

        let data_store = owned_bar_size_map.get_mut(&timeframe);
        let data_store = match data_store {
            Some(Some(data)) => data,
            _ => bail!("Could not retrieve data for ticker: {} and timeframe: {:?}", ticker, timeframe)
        };

        let mut owned_data_store = data_store.clone();
        // step 3. check if the data exists in the range data storage
        if let Some(value) = owned_data_store.get(start_date.unix_timestamp()) {
            println!("Data exists in range data storage");
            return Ok(value.clone());
        }
        else {
            println!("Data does not exist in range data storage");
        }

        // step 4. data not in range data storage, need to retrieve from ibapi
        let contract = ibapi::contracts::Contract::stock(ticker.as_str());
        let data = self.ibapi_handler.get_historical_data(&contract, timeframe.to_bar_size(), start_date, end_date)?;
        owned_data_store.insert(start_date.unix_timestamp(), end_date.unix_timestamp(), Vec::from(data));
        let result = owned_data_store.get(start_date.unix_timestamp());
        owned_bar_size_map.insert(timeframe, Some(owned_data_store.clone()));
        self.ticker_map.insert(ticker.clone(), Some(owned_bar_size_map.clone()));
        match result {
            Some(value) => Ok(value.clone()),
            None => bail!("Could not retrieve data for ticker: {} and timeframe: {:?}", ticker, timeframe)
        }
    }

    // TODO
    pub fn has_range(&mut self, ticker: String, timeframe: HashedBarSize, start_date: i64, end_date: i64) -> Result<bool, Error>{
        !todo!("has_range not implemented yet");
        if let Some(None) = self.ticker_map.get(&ticker) {
            // ticker exists but hasn't been fully serialized yet
            Ok(false)
        } else {
            Ok(false)
        }
    }

    pub fn insert(&mut self, timeframe: HashedBarSize, start_date: i64, end_date: i64, value: IBApiBar) -> Result<(), Error>{
        Ok(())
    }

    pub fn get_num_tickers(&self) -> usize {
        self.ticker_map.len()
    }
}