#[cfg(test)]
mod data_broker_tests {
    use ordered_float::OrderedFloat;
    use time::macros::datetime;
    use fq_data_broker::{DataBroker, HashedBarSize};

    #[test]
    fn instantiate_test() {
        let curr_dir_res = std::env::set_current_dir("..");
        assert!(curr_dir_res.is_ok());
        let broker = DataBroker::new(None);
        assert!(broker.is_ok());
        let broker = broker.unwrap();

        // check if @data directory exists
        let path = std::path::Path::new("@data");
        assert!(path.exists());

        // check if data broker contains any elements in its ticker map
        println!("Ticker map size: {}", broker.get_num_tickers());
    }

    #[test]
    fn instantiate_test_custom_path() {
        let curr_dir_res = std::env::set_current_dir("..");
        assert!(curr_dir_res.is_ok());

        let broker = DataBroker::new(Some("@test_data".to_string()));
        assert!(broker.is_ok());
        let broker = broker.unwrap();

        // check if @data directory exists
        let path = std::path::Path::new("@test_data");
        assert!(path.exists());

        // check if data broker contains any elements in its ticker map
        println!("Ticker map size: {}", broker.get_num_tickers());
    }

    #[test]
    fn instantiate_test_custom_path_with_data() {
        let curr_dir_res = std::env::set_current_dir("..");
        assert!(curr_dir_res.is_ok());

        // delete the directory if it exists
        let path = std::path::Path::new("@test_data");
        if path.exists() {
            std::fs::remove_dir_all(path).unwrap();
        }

        // create the directory
        let dir_create_res = std::fs::create_dir_all(path);
        assert!(dir_create_res.is_ok());

        // add apple ticker
        let apple_path = path.join("AAPL");
        let apple_path_res = std::fs::create_dir_all(apple_path);
        assert!(apple_path_res.is_ok());

        let broker = DataBroker::new(Some("@test_data".to_string()));
        assert!(broker.is_ok());
        let broker = broker.unwrap();

        // check if @data directory exists
        let path = std::path::Path::new("@test_data");
        assert!(path.exists());

        // check if data broker contains any elements in its ticker map
        println!("Ticker map size: {}", broker.get_num_tickers());
    }

    ///
    #[test]
    fn retrieve_data_test() {

        let curr_dir_res = std::env::set_current_dir("..");
        assert!(curr_dir_res.is_ok());

        // delete the directory if it exists
        let path = std::path::Path::new("@test_data");
        if path.exists() {
            std::fs::remove_dir_all(path).unwrap();
        }
        
        // create the directory
        let dir_create_res = std::fs::create_dir_all(path);
        assert!(dir_create_res.is_ok());
        
        // add apple ticker
        let apple_path = path.join("AAPL");
        let apple_path_res = std::fs::create_dir_all(apple_path);
        assert!(apple_path_res.is_ok());
        
        // call retrieve data
        let broker = DataBroker::new(Some("@test_data".to_string()));
        assert!(broker.is_ok());
        
        let mut broker = broker.unwrap();
        let data = broker.retrieve_data(
            "AAPL".to_string(),
            HashedBarSize::Day,
            datetime!(2021-01-01 00:00:00 UTC),
            datetime!(2021-01-02 00:00:00 UTC)
        );

        assert!(data.is_ok());
        let data = data.unwrap();
        for bar in data {
            println!("Bar: {:?}", bar);
        }

        // check that data has been saved to disk
        let path = std::path::Path::new("@test_data").join("AAPL").join(HashedBarSize::Day.to_location());
        assert!(path.exists());
    }

    #[test]
    fn retrieve_data_test_no_delete() {

        let curr_dir_res = std::env::set_current_dir("..");
        assert!(curr_dir_res.is_ok());

        // check that data exists
        let path = std::path::Path::new("@test_data").join("AAPL").join(HashedBarSize::Day.to_location());
        assert!(path.exists());

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

        assert!(data.is_ok());
        let data = data.unwrap();
        for bar in data {
            println!("Bar: {:?}", bar);
        }

    }
}