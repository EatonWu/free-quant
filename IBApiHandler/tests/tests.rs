mod ibapi_tests {
    use range_data_storage::range_data_storage::RangeDataStorage;
    use chrono::{Duration, NaiveDate};
    use time::OffsetDateTime;

    // range data storage test
    #[test]
    pub fn insert_test() {
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