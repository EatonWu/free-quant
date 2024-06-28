use ordered_float::OrderedFloat;
use std::collections::{HashMap};
use std::error::Error;

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

pub struct Strategy {
    context: HashMap<Metric, OrderedFloat<f64>>,
    buy_signal: fn(&HashMap<Metric, OrderedFloat<f64>>) -> Result<bool, Box<dyn Error>>,
    sell_signal: fn(&HashMap<Metric, OrderedFloat<f64>>) -> Result<bool, Box<dyn Error>>,
    on_buy: fn(),
    on_sell: fn(),
}

impl Strategy {
    pub fn new(
        context: HashMap<Metric, OrderedFloat<f64>>,
        buy_signal: fn(&HashMap<Metric, OrderedFloat<f64>>) -> Result<bool, Box<dyn Error>>,
        sell_signal: fn(&HashMap<Metric, OrderedFloat<f64>>) -> Result<bool, Box<dyn Error>>,
        on_buy: fn(),
        on_sell: fn(),
    ) -> Self {
        Self {
            context,
            buy_signal,
            sell_signal,
            on_buy,
            on_sell,
        }
    }

    pub fn should_buy(&self) -> bool {
        let result = (self.buy_signal)(&self.context);
        result.unwrap_or_else(|_| false)
    }

    pub fn should_sell(&self) -> bool {
        let result = (self.sell_signal)(&self.context);
        result.unwrap_or_else(|_| false)
    }
}