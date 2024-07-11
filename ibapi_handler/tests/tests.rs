mod ibapi_tests {
    use range_data_storage::range_data_storage::RangeDataStorage;
    use time::OffsetDateTime;

    // range data storage test
    #[test]
    pub fn insert_test() {
        // set current working directory to parent
        let curr_dir_res = std::env::set_current_dir("..");
        assert!(curr_dir_res.is_ok());
        let range_store = RangeDataStorage::new(None);
        let now = OffsetDateTime::now_utc();
        let day = time::Duration::days(1);
        let tomorrow = now.checked_add(day).unwrap();
        let today_timestamp = now.unix_timestamp();
        let tomorrow_timestamp = tomorrow.unix_timestamp();
        let mut range_store = range_store.unwrap();
        range_store.insert(today_timestamp, tomorrow_timestamp, 3);
        assert_eq!(range_store.get(today_timestamp), Some(&3));
        assert_eq!(range_store.get(tomorrow_timestamp), Some(&3));
    }

    #[test]
    pub fn save_test() {
        let range_store = RangeDataStorage::new(None);
        let now = OffsetDateTime::now_utc();
        let day = time::Duration::days(1);
        let tomorrow = now.checked_add(day).unwrap();
        let today_timestamp = now.unix_timestamp();
        let tomorrow_timestamp = tomorrow.unix_timestamp();
        let mut range_store = range_store.unwrap();
        range_store.insert(today_timestamp, tomorrow_timestamp, 3);
        drop(range_store);

        let range_store = RangeDataStorage::new(Some("data.json".to_string()));
        let mut range_store = range_store.unwrap();
        assert_eq!(range_store.get(today_timestamp), Some(&3));
        assert_eq!(range_store.get(tomorrow_timestamp), Some(&3));
    }
}

#[cfg(test)]
mod handler_tests {
    use ibapi::contracts::Contract;
    use ibapi::market_data::historical::BarSize;
    use time::macros::datetime;
    use time::Month;
    use ibapi_handler::IbapiHandler;

    #[test]
    pub fn instantiate_test() {
        let handler = IbapiHandler::new();
        assert!(handler.is_ok());
        let mut handler = handler.unwrap();
        let contract = Contract::stock("AAPL");
        // set start date to 01/01/2020
        let date = time::Date::from_calendar_date(2020, Month::January, 1).unwrap();
        let start_date = time::OffsetDateTime::new_utc(date, time::Time::MIDNIGHT);
        // set end date to 01/01/2021
        let date = time::Date::from_calendar_date(2021, Month::January, 1).unwrap();
        // let alternate_date =  datetime!(2021-01-01 00:00:00 UTC);
        let end_date = time::OffsetDateTime::new_utc(date, time::Time::MIDNIGHT);
        let result = handler.get_historical_data(&contract, BarSize::Day, start_date, end_date);
        assert!(result.is_ok(), "Error: {:?}", result.err());
        let result = result.unwrap();
        for bar in &result {
            println!("Bar: {:?}", bar);
        }
        println!("Bars: {:?}", result.len());
    }
}