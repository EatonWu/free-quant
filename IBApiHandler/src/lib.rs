pub mod ibapi_handler {
    use ibapi::client::Client;
    use ibapi::contracts::Contract;
    use ibapi::market_data::historical::{Bar, BarSize, HistoricalData, WhatToShow};
    use ibapi::orders::{order_builder, Action, OrderNotification};
    use time::{OffsetDateTime};
    use ibapi::market_data::historical::Duration;
    use time::ext::NumericalDuration;
    use serde::{Deserialize, Serialize};
    use RangeDataStorage::range_data_storage::{RangeDataStore, RangeDataEntry};

    pub fn connect_to_tws() -> Result<Client, Box<dyn std::error::Error>> {
        let client = Client::connect("127.0.0.1:7497", 100);
        return match client {
            Ok(c) => {
                println!("Connected to TWS");
                Ok(c)
            },
            Err(e) => {
                println!("Error: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    /// Given a `client` and a `contract`, retrieves historical data for the contract
    /// for the last `days` days.
    pub fn get_historical_data(client: &mut Client, contract: Contract, days: i32) -> Result<HistoricalData, Box<dyn std::error::Error>> {
        // create OffsetDateTime object for last 30 days
        let duration = Duration::days(days);

        // check cache if it has data
        let data = client.historical_data_ending_now(&contract, duration,
            BarSize::Day, WhatToShow::Trades, false);
        return match data {
            Ok(d) => {
                Ok(d)
            },
            Err(e) => {
                println!("Error: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub fn convert_bars_to_range_data(bars: Vec<Bar>) -> Vec<RangeDataEntry<Bar>> {
        let mut range_data: Vec<RangeDataEntry<Bar>> = Vec::new();
        let mut start = bars.first().unwrap().clone();
        let mut end = bars.last().unwrap().clone();
        let mut data: Vec<Bar> = Vec::new();

        for bar in bars {
            if bar.date < start.date {
                start = bar.clone();
            }
            if bar.date > end.date {
                end = bar.clone();
            }
            data.push(bar.clone());
        }

        range_data.push(RangeDataEntry::new(start, end, data));
        range_data
    }
}