use std::collections::HashMap;
use anyhow::Error;
use ibapi::contracts::Contract;
use ibapi::market_data::historical::BarSize;
use ordered_float::OrderedFloat;
use time::macros::datetime;
use backtesting::*;
use fq_data_broker::{DataBroker, HashedBarSize};

fn test_buy_signal(prices: &Vec<OrderedFloat<f64>>, index: u32) -> Result<bool, Error> {
    Ok(true)
}

fn test_sell_signal(prices: &Vec<OrderedFloat<f64>>, index: u32) -> Result<bool, Error> {
    Ok(false)
}

fn test_on_buy() -> Result<(), Error> {
    println!("Buy signal detected");
    Ok(())
}

fn test_on_sell() -> Result<(), Error>{
    println!("Sell signal detected");
    Ok(())
}

fn main() {
    let chosen_security = "AAPL".to_string();
    let metric = BacktestingMetric::SMA(50); // 50 day SMA for AAPL
    let measure = BacktestingMeasure::NetProfit;
    let start_date = datetime!(2021-01-01 00:00:00 UTC);
    let end_date = datetime!(2021-12-31 23:59:59 UTC);
    let bar_size = HashedBarSize::Min15;
    let mut data_broker = DataBroker::new(None).unwrap();
    let data = data_broker.retrieve_data(
        chosen_security,
        bar_size,
        start_date,
        end_date,
    );
    if data.is_err() {
        println!("{:?}", data.as_ref().unwrap());
    }
    assert!(data.is_ok());
    // print data
    let data = data.unwrap();

    // create the strategy
    let strat = Strategy::new(
        vec![BacktestingMetric::SMA(50)],
        HashedBarSize::Min15,
        "AAPL".to_string(),
        test_buy_signal,
        test_sell_signal,
        start_date,
        end_date,
        test_on_buy,
        test_on_sell,
    );

    let mut strat_result = strat.execute(&data);
    if strat_result.is_err() {
        println!("{:?}", strat_result.as_ref().unwrap());
    }
    assert!(strat_result.is_ok());
    let strat_result = strat_result.unwrap();
    let sma50 = strat_result.get(&BacktestingMetric::SMA(50));
    if sma50.is_none() {
        println!("No signals received");
    }
    assert!(sma50.is_some());
    let sma50 = match sma50 {
        Some(s) => s,
        None => panic!("No signals received"),
    };

    if sma50.is_err() {
        println!("{:?}", &sma50.as_ref().unwrap());
    }
    assert!(sma50.is_ok());
    let sma50 = sma50.as_ref().unwrap();

    if sma50.is_empty() {
        println!("No signals received for 50 day SMA");
    }
    for signal in sma50.iter() {
        println!("{:?}", signal);
    }
}
