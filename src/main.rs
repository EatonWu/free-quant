use ibapi::contracts::Contract;
use ibapi::market_data::historical::BarSize;
use ibapi_handler;
use range_data_storage::range_data_storage::RangeDataStorage;

fn main() {
    let client = ibapi_handler::connect_to_tws();
    match client {
        Ok(_) => { println!("Connected to TWS") },
        Err(e) => {
            println!("Error: {:?}", e);
            return
        }
    }

    // attempt to retrieve historical data
    let mut client = client.unwrap();
    let contract = Contract::stock("AAPL");
    let data = ibapi_handler::get_historical_data(
        &mut client,
        contract,
        365, // get daily data for last 365 days
        BarSize::Day
    );
    match data {
        Ok(_) => { println!("Historical data retrieved") },
        Err(e) => {
            println!("Error: {:?}", e);
            return
        }
    }

    let data = data.unwrap();

    for bar in &data.bars {
        println!("Bar: {:?}", bar);
        bar.date.unix_timestamp();
    }

    // collect bars
    let bars = data.bars.clone();


    let mut data_store = RangeDataStorage::new(Some("@data/data.json".to_string()));
    match data_store {
        Ok(_) => { println!("Data store created") },
        Err(e) => {
            println!("Error: {:?}", e);
            return
        }
    }
    let mut data_store = data_store.unwrap();
    let start_date = bars[0].date.unix_timestamp();
    let end_date = bars[bars.len() - 1].date.unix_timestamp();
    if data_store.contains_range(start_date, end_date) {
        println!("Already obtained this information");
    } else {
        let wrappers = ibapi_handler::convert_bar_vec_to_wrapper(bars);
        data_store.insert(start_date, end_date, wrappers);
    }
}
