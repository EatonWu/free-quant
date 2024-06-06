use std::ops::Add;
use std::time;
use std::time::Duration;
use ibapi::client::Client;
use ibapi::contracts::Contract;
// use ibapi::market_data::historical::Duration;
use ibapi::orders::{order_builder, Action, OrderNotification};
use IBApiHandler::ibapi_handler;
use RangeDataStorage::range_data_storage::RangeDataStore;

fn main() {
    let client = ibapi_handler::connect_to_tws();
    match client {
        Ok(_) => { println!("Connected to TWS") },
        Err(e) => { println!("Error: {:?}", e); return }
    }

    // attempt to retrieve historical data
    let mut client = client.unwrap();
    let contract = Contract::stock("AAPL");
    let data = ibapi_handler::get_historical_data(&mut client, contract, 7);
    match data {
        Ok(_) => { println!("Historical data retrieved") },
        Err(e) => { println!("Error: {:?}", e); return }
    }

    let data = data.unwrap();

    for bar in &data.bars {
        println!("Bar: {:?}", bar);
    }

    // collect bars
    let bars = data.bars.clone();

    // let mut data_store = RangeDataStore::new("@data".to_string(), "int_test".to_string());
    // data_store.add_range(bars)
    // data_store.save_to_file("@data", "int_test");
}
