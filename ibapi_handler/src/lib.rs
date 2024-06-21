use ibapi::client::Client;
use ibapi::contracts::Contract;
use ibapi::market_data::historical::{BarSize, HistoricalData, WhatToShow};
use ibapi::market_data::historical::Duration;
use serde::{Deserialize, Serialize};
use ordered_float::OrderedFloat;

// We use OrderedFloat to avoid dealing with RangeMap's Eq requirement
// Hopefully this doesn't cause any issues
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct IBApiBar {
    date: i64,
    open: OrderedFloat<f64>,
    high: OrderedFloat<f64>,
    low: OrderedFloat<f64>,
    close: OrderedFloat<f64>,
    volume: OrderedFloat<f64>,
    count: i32,
    wap: OrderedFloat<f64>,
}


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
pub fn get_historical_data(client: &mut Client, contract: Contract, days: i32, bar_size: BarSize) -> Result<HistoricalData, Box<dyn std::error::Error>> {
    // create OffsetDateTime object for last 30 days
    let duration = Duration::days(days);

    // check cache if it has data
    let data = client.historical_data_ending_now(&contract, duration,
        bar_size, WhatToShow::Trades, false);
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

pub fn convert_from_ibapi_bar_historical(bars: ibapi::market_data::historical::Bar)
        -> IBApiBar {
    IBApiBar {
        date: bars.date.unix_timestamp(),
        open: OrderedFloat::from(bars.open),
        high: OrderedFloat::from(bars.high),
        low: OrderedFloat::from(bars.low),
        close: OrderedFloat::from(bars.close),
        volume: OrderedFloat::from(bars.volume),
        count: bars.count,
        wap: OrderedFloat::from(bars.wap),
    }
}

pub fn convert_bar_vec_to_wrapper(bars: Vec<ibapi::market_data::historical::Bar>)
        -> Vec<IBApiBar> {
    let mut wrappers = Vec::new();
    for bar in bars {
        wrappers.push(convert_from_ibapi_bar_historical(bar));
    }
    wrappers
}