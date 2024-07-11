#[cfg(test)]
mod data_broker_tests {
    use ordered_float::OrderedFloat;
    use fq_data_broker::DataBroker;

    #[test]
    fn instantiate_test() {
        let curr_dir_res = std::env::set_current_dir("..");
        assert!(curr_dir_res.is_ok());
        let broker = DataBroker::new(None);
        assert!(broker.is_ok());
        let broker: DataBroker<i64, OrderedFloat<f64>> = broker.unwrap();

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
        let broker: DataBroker<i64, OrderedFloat<f64>> = broker.unwrap();

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
        let broker: DataBroker<i64, OrderedFloat<f64>> = broker.unwrap();

        // check if @data directory exists
        let path = std::path::Path::new("@test_data");
        assert!(path.exists());

        // check if data broker contains any elements in its ticker map
        println!("Ticker map size: {}", broker.get_num_tickers());
    }
}