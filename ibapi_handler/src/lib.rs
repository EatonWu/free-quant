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
    pub date: i64,
    open: OrderedFloat<f64>,
    high: OrderedFloat<f64>,
    low: OrderedFloat<f64>,
    pub close: OrderedFloat<f64>,
    volume: OrderedFloat<f64>,
    count: i32,
    wap: OrderedFloat<f64>,
}

impl IBApiBar {
    pub fn date(&self) -> i64 {
        self.date.clone()
    }

    pub fn open(&self) -> OrderedFloat<f64> {
        self.open.clone()
    }

    pub fn high(&self) -> OrderedFloat<f64> {
        self.high.clone()
    }

    pub fn low(&self) -> OrderedFloat<f64> {
        self.low.clone()
    }

    pub fn close(&self) -> OrderedFloat<f64> {
        self.close.clone()
    }

    pub fn volume(&self) -> OrderedFloat<f64> {
        self.volume.clone()
    }

    pub fn count(&self) -> i32 {
        self.count.clone()
    }

    pub fn wap(&self) -> OrderedFloat<f64> {
        self.wap.clone()
    }
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

fn clamp_duration(seconds: i32) -> Duration {
    if seconds > 86400 {
        Duration::days(seconds / 86400)
    } else {
        Duration::seconds(seconds)
    }
}

/// takes a time duration and converts it into a ibapi duration
pub fn convert_durations(duration: time::Duration, bar_size: BarSize) -> Duration{

    let seconds = duration.whole_seconds() as i32;
    println!("Seconds: {:?}", seconds);

    return match bar_size {
        BarSize::Sec => clamp_duration(seconds),
        BarSize::Sec5 => clamp_duration(seconds),
        BarSize::Sec15 => clamp_duration(seconds),
        BarSize::Sec30 => clamp_duration(seconds),
        BarSize::Min => clamp_duration(seconds),
        BarSize::Min2 => clamp_duration(seconds),
        BarSize::Min3 => clamp_duration(seconds),
        BarSize::Min5 => clamp_duration(seconds),
        BarSize::Min15 => clamp_duration(seconds),
        BarSize::Min20 => clamp_duration(seconds),
        BarSize::Min30 => clamp_duration(seconds),
        BarSize::Hour => clamp_duration(seconds),
        BarSize::Hour2 => clamp_duration(seconds),
        BarSize::Hour3 => clamp_duration(seconds),
        BarSize::Hour4 => clamp_duration(seconds),
        BarSize::Hour8 => clamp_duration(seconds),
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