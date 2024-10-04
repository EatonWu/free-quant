use anyhow::bail;
use anyhow::Error;
use ordered_float::OrderedFloat;
use time::macros::datetime;
use fq_data_broker::{DataBroker, HashedBarSize};
use ibapi_handler::IBApiBar;
fn print_data(res: Result<Vec<IBApiBar>, Error>) {
    match res {
        Ok(d) => {
            for bar in &d {
                println!("Bar: {:?}", bar);
            }
            println!("Bars: {:?}", d.len());
        },
        Err(e) => {
            println!("Error: {:?}", e);
            assert!(false);
        }
    }
}

fn main() {
// delete the directory if it exists
    let path = std::path::Path::new("@test_data");
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
    }

    // create the directory
    let dir_create_res = std::fs::create_dir_all(path);
    assert!(dir_create_res.is_ok());

    // instantiate data broker
    let broker = DataBroker::new(Some("@test_data".to_string()));
    assert!(broker.is_ok());

    let mut broker = broker.unwrap();
    let data = broker.retrieve_data(
        "AAPL".to_string(),
        HashedBarSize::Day,
        datetime!(2021-01-01 00:00:00 UTC),
        datetime!(2021-01-02 00:00:00 UTC)
    );

    let data = broker.retrieve_data(
        "AAPL".to_string(),
        HashedBarSize::Day,
        datetime!(2021-01-02 00:00:00 UTC),
        datetime!(2021-01-03 00:00:00 UTC)
    );
    print_data(data);

    let data = broker.retrieve_data(
        "AAPL".to_string(),
        HashedBarSize::Day,
        datetime!(2021-01-03 00:00:00 UTC),
        datetime!(2021-01-04 00:00:00 UTC)
    );
    print_data(data);

    let data = broker.retrieve_data(
        "AAPL".to_string(),
        HashedBarSize::Day,
        datetime!(2021-01-06 00:00:00 UTC),
        datetime!(2021-01-10 00:00:00 UTC)
    );

    // check that data exists
    // let path = std::path::Path::new("@test_data").join("AAPL").join(HashedBarSize::Day.to_location());
    // assert!(path.exists());

    print_data(data);
}