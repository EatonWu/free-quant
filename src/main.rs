use ibapi::client::Client;
use ibapi::contracts::Contract;
use ibapi::orders::{order_builder, Action, OrderNotification};

fn main() {
    let client = Client::connect("127.0.0.1:7497", 100);
    let res_client;
    match client {
        Ok(mut c) => {
            println!("Connected to TWS");
            res_client = c;
        },
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    }





}
