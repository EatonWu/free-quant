use std::fmt::{Debug};
use ibapi::client::Client;
use ibapi::contracts::Contract;
use ibapi::market_data::historical::{BarSize, WhatToShow};
use ibapi::market_data::historical::{Duration};
use serde::{Deserialize, Serialize};
use ordered_float::OrderedFloat;
use anyhow::{Error, bail};
use time::OffsetDateTime;

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

pub struct IbapiHandler {
    client: Client,
}

impl IbapiHandler {
    pub fn new() -> Result<IbapiHandler, Error> {
        let client = connect_to_tws()?;
        Ok(IbapiHandler {
            client,
        })
    }

    pub fn get_historical_data(&self, contract: &Contract, bar_size: BarSize, start_date: OffsetDateTime, end_date: OffsetDateTime) -> Result<Vec<IBApiBar>, Error> {
        // calculate duration
        let duration = end_date - start_date;
        // convert seconds to ibapi duration
        let duration = convert_durations(duration, bar_size);
        let data = self.client.historical_data(
            contract,
            end_date,
            duration,
            bar_size,
            WhatToShow::Trades,
            true
        )?;
        let bars = convert_bar_vec_to_wrapper(data.bars);
        Ok(bars)
    }
}

pub fn connect_to_tws() -> Result<Client, Error> {
    let client = Client::connect("127.0.0.1:7497", 100);
    return match client {
        Ok(c) => {
            println!("Connected to TWS");
            Ok(c)
        },
        Err(e) => {
            println!("Error: {:?}", e);
            bail!(e)
        }
    }
}

/// takes a time duration and converts it into a ibapi duration
pub fn convert_durations(duration: time::Duration, bar_size: BarSize) -> Duration{

    let seconds = duration.whole_seconds() as i32;
    println!("Seconds: {:?}", seconds);
    return match bar_size {
        BarSize::Sec => Duration::seconds(seconds),
        BarSize::Sec5 => Duration::seconds(seconds),
        BarSize::Sec15 => Duration::seconds(seconds),
        BarSize::Sec30 => Duration::seconds(seconds),
        BarSize::Min => Duration::seconds(seconds),
        BarSize::Min2 => Duration::seconds(seconds),
        BarSize::Min3 => Duration::seconds(seconds),
        BarSize::Min5 => Duration::seconds(seconds),
        BarSize::Min15 => Duration::seconds(seconds),
        BarSize::Min20 => Duration::seconds(seconds),
        BarSize::Min30 => Duration::seconds(seconds),
        BarSize::Hour => Duration::seconds(seconds),
        BarSize::Hour2 => Duration::seconds(seconds),
        BarSize::Hour3 => Duration::seconds(seconds),
        BarSize::Hour4 => Duration::seconds(seconds),
        BarSize::Hour8 => Duration::seconds(seconds),
        BarSize::Day => {
            let days = seconds / 86400;
            if days > 365 {
                Duration::years(seconds / 31536000)
            }
            else {
                Duration::days(seconds / 86400)
            }
        },
        BarSize::Week => Duration::weeks(seconds / 604800),
        BarSize::Month => Duration::months(seconds / 2592000),
    };
}

// /// Given a `client` and a `contract`, retrieves historical data for the contract
// /// for the last `days` days.
// fn get_historical_data_from_days(client: &mut Client, contract: Contract, days: i32, bar_size: BarSize) -> Result<HistoricalData, Error> {
//     // create OffsetDateTime object for last 30 days
//     let duration = Duration::days(days);
//
//     // check cache if it has data
//     let data = client.historical_data_ending_now(&contract, duration,
//         bar_size, WhatToShow::Trades, false);
//     return match data {
//         Ok(d) => {
//             Ok(d)
//         },
//         Err(e) => {
//             println!("Error: {:?}", e);
//             bail!(e)
//         }
//     }
// }

fn convert_from_ibapi_bar_historical(bars: ibapi::market_data::historical::Bar)
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

fn convert_bar_vec_to_wrapper(bars: Vec<ibapi::market_data::historical::Bar>) -> Vec<IBApiBar> {
    let mut wrappers = Vec::new();
    for bar in bars {
        wrappers.push(convert_from_ibapi_bar_historical(bar));
    }
    wrappers
}

impl IBApiBar {

}