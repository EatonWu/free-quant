use ordered_float::OrderedFloat;
use std::collections::{HashMap};
use anyhow::{bail, Error};
use ibapi::{ market_data::historical::BarSize };

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Metric {
    SMA(usize),
    EMA,
    RSI,
    MACD,
    DpRatio,
    Volatility,
    Volume,
}

type MetricsMapFn = fn(&HashMap<Metric, OrderedFloat<f64>>) -> Result<bool, Error>;

pub struct Strategy {
    context: HashMap<Metric, OrderedFloat<f64>>,
    bar_size: BarSize,
    buy_signal: MetricsMapFn,
    sell_signal: MetricsMapFn,
    on_buy: fn() -> Result<(), Error>,
    on_sell: fn() -> Result<(), Error>
}

impl Strategy {
    pub fn new(
        context: HashMap<Metric, OrderedFloat<f64>>,
        bar_size: BarSize,
        buy_signal: MetricsMapFn,
        sell_signal: MetricsMapFn,
        on_buy: fn() -> Result<(), Error>,
        on_sell: fn() -> Result<(), Error>,
    ) -> Self {
        Self {
            context,
            bar_size,
            buy_signal,
            sell_signal,
            on_buy,
            on_sell,
        }
    }

    pub fn should_buy(&self) -> Result<bool, Error> {
        (self.buy_signal)(&self.context)
    }

    pub fn should_sell(&self) -> Result<bool, Error> {
        (self.sell_signal)(&self.context)
    }

    pub fn execute(&self) -> Result<(), Error>{
        let should_buy = self.should_buy()?;
        let should_sell = self.should_sell()?;

        if should_buy && should_sell {
            bail!("Both buy and sell signals are true");
        }
        else if should_buy {
            (self.on_buy)()?;
        }
        else if should_sell {
            (self.on_sell)()?;
        }
        Ok(())
    }
}

pub struct BacktestExecutor {
    strategies: Vec<Strategy>
}

impl BacktestExecutor {
    pub fn new(strategies: Vec<Strategy>) -> Self {
        Self {
            strategies
        }
    }

    pub fn execute(&self) -> Result<(), Error> {
        for strategy in &self.strategies {
            strategy.execute()?;
        }
        Ok(())
    }
}