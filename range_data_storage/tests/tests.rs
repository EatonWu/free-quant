mod all_tests {
    mod integer_range_tests {
        use range_data_storage::range_data_storage::RangeDataStorage;

        #[test]
        pub fn insert_test() {
            let range_store = RangeDataStorage::new(None);
            let mut range_store = range_store.unwrap();
            range_store.insert(1, 3, 3);
            assert_eq!(range_store.get(1), Some(&3));
            assert_eq!(range_store.get(2), Some(&3));

        }
    }
}