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
            assert_eq!(range_store.get(3), Some(&3));
        }

        #[test]
        pub fn insert_two_test() {
            let range_store = RangeDataStorage::new(None);
            let mut range_store = range_store.unwrap();
            range_store.insert(1, 3, 3);
            range_store.insert(5, 6, 6);
            assert_eq!(range_store.get(1), Some(&3));
            assert_eq!(range_store.get(2), Some(&3));
            assert_eq!(range_store.get(3), Some(&3));
            assert_eq!(range_store.get(5), Some(&6));
            assert_eq!(range_store.get(6), Some(&6));
            assert_eq!(range_store.len(), 2);
        }

        #[test]
        pub fn save_test() {
            let range_store = RangeDataStorage::new(None);
            let mut range_store = range_store.unwrap();
            range_store.insert(1, 3, 3);
            drop(range_store);

            let range_store = RangeDataStorage::new(Some("data.json".to_string()));
            let mut range_store = range_store.unwrap();
            assert_eq!(range_store.get(1), Some(&3));
            assert_eq!(range_store.get(2), Some(&3));
            assert_eq!(range_store.get(3), Some(&3));
        }
    }

    // // Strings currently don't work because of the lack of StepLite implementation
    // mod string_range_tests {
    //     use range_data_storage::range_data_storage::RangeDataStorage;
    //
    //     // test if works fine withs trings as keys
    //     #[test]
    //     pub fn insert_test() {
    //         let range_store = RangeDataStorage::new(None);
    //         let mut range_store = range_store.unwrap();
    //         range_store.insert("a", "c", "Hello".to_string());
    //         assert_eq!(range_store.get("a"), Some(&"Hello".to_string()));
    //         assert_eq!(range_store.get("b"), Some(&"Hello".to_string()));
    //         assert_eq!(range_store.get("c"), Some(&"Hello".to_string()));
    //     }
    // }
}