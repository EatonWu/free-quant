mod all_tests {
    mod range_data_storage_tests_integer {
        use RangeDataStorage::range_data_storage::{RangeDataStore};

        #[test]
        fn test_range_add() {
            let mut storage = RangeDataStore::new("@data".to_string(),"int_test".to_string());
            storage.add_range(vec![1, 2, 3]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 3);
        }

        #[test]
        fn test_range_add_overlap() {
            let mut storage = RangeDataStore::new("@data".to_string(),"int_test".to_string());
            storage.add_range(vec![1, 2, 3]);
            storage.add_range(vec![3, 4, 5]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 5);
        }

        #[test]
        fn test_range_add_no_overlap() {
            let mut storage = RangeDataStore::new("@data".to_string(),"int_test".to_string());
            storage.add_range(vec![1, 2, 3]);
            storage.add_range(vec![4, 5, 6]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 6);
        }
    }

    mod range_data_storage_tests_string {
        use RangeDataStorage::range_data_storage::{RangeDataStore};

        #[test]
        fn test_range_add() {
            let mut storage = RangeDataStore::new("@data".to_string(), "string_test".to_string());
            storage.add_range(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 3);
        }

        #[test]
        fn test_range_add_overlap() {
            let mut storage = RangeDataStore::new("@data".to_string(), "string_test".to_string());
            storage.add_range(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
            storage.add_range(vec!["c".to_string(), "d".to_string(), "e".to_string()]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 5);
        }

        #[test]
        fn test_range_add_no_overlap() {
            let mut storage = RangeDataStore::new("@data".to_string(), "string_test".to_string());
            storage.add_range(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
            storage.add_range(vec!["d".to_string(), "e".to_string(), "f".to_string()]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 6);
        }
    }

    mod range_data_storage_tests_offset_datetime {
        use RangeDataStorage::range_data_storage::{RangeDataStore};
        use time::{OffsetDateTime};

        #[test]
        fn test_range_add() {
            let mut storage = RangeDataStore::new("@data".to_string(), "offset_datetime_test".to_string());
            storage.add_range(vec![OffsetDateTime::now_utc()]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 1);
        }

        #[test]
        fn test_range_add_overlap() {
            let mut storage = RangeDataStore::new("@data".to_string(), "offset_datetime_test".to_string());
            let current_time = OffsetDateTime::now_utc();
            storage.add_range(vec![current_time]);
            storage.add_range(vec![current_time]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 1);
        }
    }
}