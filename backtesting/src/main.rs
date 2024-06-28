use std::collections::HashMap;
use ordered_float::OrderedFloat;
use backtesting::*;

fn buy_signal(context: &HashMap<Metric, OrderedFloat<f64>>) -> Result<bool, Box<dyn std::error::Error>> {
    let sma_50 = context.get(&Metric::SMA(50)).ok_or("SMA 50 not found")?;
    let sma_200 = context.get(&Metric::SMA(200)).ok_or("SMA 200 not found")?;
    Ok(*sma_50 > *sma_200)
}

fn sell_signal(context: &HashMap<Metric, OrderedFloat<f64>>) -> Result<bool, Box<dyn std::error::Error>> {
    let sma_50 = context.get(&Metric::SMA(50)).ok_or("SMA 50 not found")?;
    let sma_200 = context.get(&Metric::SMA(200)).ok_or("SMA 200 not found")?;
    Ok(*sma_50 < *sma_200)
}

fn main() {
    let chosen_security = "AAPL";
    let mut context = HashMap::new();

    // 50 day sma
    let sma_50 = Metric::SMA(50);
    // 200 day sma
    let sma_200 = Metric::SMA(200);
    context.insert(sma_50, OrderedFloat(1.0));
    context.insert(sma_200, OrderedFloat(0.0));

    let strategy = Strategy::new(
        context,
        buy_signal,
        sell_signal,
        || println!("Bought"),
        || println!("Sold"),
    );
    println!("Should buy: {}", strategy.should_buy());
}
