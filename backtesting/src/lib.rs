use ordered_float::OrderedFloat;
use std::collections::{HashMap};
use std::iter::Iterator;
use anyhow::{bail, Error};
use ibapi::{ market_data::historical::BarSize };
use ibapi::contracts::Contract;
use ibapi::orders::Order;
use fq_data_broker::{DataBroker, HashedBarSize};
use ibapi_handler::IBApiBar;
use time::OffsetDateTime;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BacktestingMetric {
    SMA(usize), // period in days
    EMA,
    RSI,
    MACD,
    DpRatio,
    Volatility,
    Volume,
}

// ignore signal sentinel
static IGNORE_SENTINEL: OrderedFloat<f64> = OrderedFloat(f64::INFINITY);

impl BacktestingMetric {

    // this function takes bars and then returns a vector of signals (buy, sell, hold)
    pub fn calculate_for_data(&self, data: &Vec<IBApiBar>, buy_signal: MetricsMapFn, sell_signal: MetricsMapFn, identifier: String) -> Result<Vec<Signal>, Error> {
        match self {
            BacktestingMetric::SMA(period) => {
                match (data.first(), data.last()) {
                    (Some(first), Some(last)) => {
                        let mut sma = OrderedFloat(0.0);
                        let mut sma_values = Vec::new();
                        // generate sma values
                        for i in 0..data.len() {
                            if i < *period {
                                sma = sma + data[i].close;
                                sma_values.push(IGNORE_SENTINEL);
                            } else {
                                sma = sma - data[i - *period].close + data[i].close;
                                sma_values.push(sma / *period as f64);
                            }
                        }
                        // generate signals
                        let signals = self.generate_signals(data, &sma_values, buy_signal, sell_signal, identifier)?;
                        return Ok(signals);
                    }
                    _ => bail!("No data")
                }
            },
            _ => bail!("Metric not implemented")
        }
    }

    pub fn generate_signals(&self, data: &Vec<IBApiBar>, metrics: &Vec<OrderedFloat<f64>>, buy_signal: MetricsMapFn, sell_signal: MetricsMapFn, identifier: String) -> Result<Vec<Signal>, Error> {
        let mut result = Vec::new();
        for i in 0..metrics.len() {
            if metrics[i] != IGNORE_SENTINEL {
                let buy = buy_signal(&metrics, i as u32)?;
                let sell = sell_signal(&metrics, i as u32)?;
                if buy && sell {
                    bail!("Buy and sell signals generated at the same time");
                }
                if buy {
                    result.push(Signal::new(SignalType::Buy, data[i].date, data[i].close, &identifier, None, None));
                }
                else if sell {
                    result.push(Signal::new(SignalType::Sell, data[i].date, data[i].close, &identifier, None, None));
                }
                else { // TODO: flesh out hold signal functionality
                    result.push(Signal::new(SignalType::Hold, data[i].date, data[i].close, &identifier, None, None));
                }
            }
        }
        if result.len() == 0 {
            bail!("No signals generated");
        }
        Ok(result)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BacktestingMeasure {
    NetProfit,
    TotalReturn,
    RiskAdjustedReturn,
}

// a function that takes some data and generates a vector of signals, the u32 is the current index in the data
type MetricsMapFn = fn(&Vec<OrderedFloat<f64>>, u32) -> Result<bool, Error>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
/// A strategy that can be executed on a set of metrics
pub struct Strategy {
    context: Vec<BacktestingMetric>,
    bar_size: HashedBarSize,
    contract_name: String,
    buy_signal: MetricsMapFn,
    sell_signal: MetricsMapFn,
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
    on_buy: fn() -> Result<(), Error>,
    on_sell: fn() -> Result<(), Error>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SignalType {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Signal {
    signal_type: SignalType,
    timestamp: i64,
    price: OrderedFloat<f64>,
    identifier: String,
    quantity: Option<i32>, // defaults to 1
    confidence: Option<OrderedFloat<f64>> // defaults to 1.0
}


// builder functions for Signals
impl Signal {
    pub fn new(signal_type: SignalType, timestamp: i64, price: OrderedFloat<f64>, identifier: &String, quantity: Option<i32>, confidence: Option<OrderedFloat<f64>>) -> Self {
        let quantity = quantity.unwrap_or(1);
        let confidence = confidence.unwrap_or(OrderedFloat(1.0));
        Self {
            signal_type,
            timestamp,
            price,
            identifier: identifier.clone(),
            quantity: Some(quantity),
            confidence: Some(confidence)
        }
    }
}

impl Strategy {

    /// Creates a new strategy.
    /// You must provide contexts, bar_size, contract_name, buy_signal, sell_signal, start_date, end_date, on_buy, on_sell
    pub fn new(
        context: Vec<BacktestingMetric>,
        bar_size: HashedBarSize,
        contract_name: String,
        buy_signal: MetricsMapFn,
        sell_signal: MetricsMapFn,
        start_date: OffsetDateTime,
        end_date: OffsetDateTime,
        on_buy: fn() -> Result<(), Error>,
        on_sell: fn() -> Result<(), Error>,
    ) -> Self {
        Self {
            context,
            bar_size,
            contract_name,
            buy_signal,
            sell_signal,
            start_date,
            end_date,
            on_buy,
            on_sell,
        }
    }

    pub fn should_buy(&self, context: Vec<OrderedFloat<f64>>, current: u32) -> Result<bool, Error> {
        (self.buy_signal)(&context, current)
    }

    pub fn should_sell(&self, context: Vec<OrderedFloat<f64>>, current:u32) -> Result<bool, Error> {
        (self.sell_signal)(&context, current)
    }

    // iterates over data, generating signals when some condition for a metric is met
    pub fn execute(&self, data: &Vec<IBApiBar>, ) -> Result<HashMap<BacktestingMetric, Result<Vec<Signal>, Error>>, Error> {
        let mut return_signals = HashMap::new();
        for metric in &self.context {
            println!("Calculating metric {:?}", metric);
            let signals_result = metric.calculate_for_data(data, self.buy_signal, self.sell_signal, self.contract_name.clone());
            match signals_result {
                Ok(signals) => {
                    return_signals.insert(metric.clone(), Ok(signals));
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(return_signals)
    }
}

pub struct BacktestExecutor {
    broker: DataBroker,
}

impl BacktestExecutor {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            broker: DataBroker::new(None)?,
        })
    }

    /// Get data, and generate signals
    /// results is a hashmap mapping each passed strategy to its strategy.execute() result.
    pub fn execute(&mut self, strategies: Vec<Strategy>) -> Result<HashMap<Strategy, Result<HashMap<BacktestingMetric, Result<Vec<Signal>, Error>>, Error>>, Error> {
        let mut results = HashMap::new();
        for strategy in strategies {
            let data = self.broker.retrieve_data(
                strategy.contract_name.clone(),
                strategy.bar_size,
                strategy.start_date,
                strategy.end_date);
            let data = match data {
                Ok(data) => data,
                Err(e) => {
                    results.insert(strategy.clone(), Err(e)); continue;
                },
            };
            let signals = strategy.execute(&data);
            results.insert(strategy, signals);
        }
        Ok(results)
    }
}