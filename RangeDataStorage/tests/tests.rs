mod all_tests {
    mod range_data_storage_tests_integer {
        use RangeDataStorage::range_data_storage::{RangeDataStore};

        #[test]
        fn test_range_add() {
            let mut storage = RangeDataStore::new();
            storage.add_range(vec![1, 2, 3]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 3);
        }

        #[test]
        fn test_range_add_overlap() {
            let mut storage = RangeDataStore::new();
            storage.add_range(vec![1, 2, 3]);
            storage.add_range(vec![3, 4, 5]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 5);
        }

        #[test]
        fn test_range_add_no_overlap() {
            let mut storage = RangeDataStore::new();
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
            let mut storage = RangeDataStore::new();
            storage.add_range(vec!["a", "b", "c"]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 3);
        }

        #[test]
        fn test_range_add_overlap() {
            let mut storage = RangeDataStore::new();
            storage.add_range(vec!["a", "b", "c"]);
            storage.add_range(vec!["c", "d", "e"]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 5);
        }

        #[test]
        fn test_range_add_no_overlap() {
            let mut storage = RangeDataStore::new();
            storage.add_range(vec!["a", "b", "c"]);
            storage.add_range(vec!["d", "e", "f"]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 6);
        }
    }

    mod range_data_storage_tests_offset_datetime {
        use RangeDataStorage::range_data_storage::{RangeDataStore};
        use time::{OffsetDateTime};

        #[test]
        fn test_range_add() {
            let mut storage = RangeDataStore::new();
            storage.add_range(vec![OffsetDateTime::now_utc()]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 1);
        }

        #[test]
        fn test_range_add_overlap() {
            let mut storage = RangeDataStore::new();
            let current_time = OffsetDateTime::now_utc();
            storage.add_range(vec![current_time]);
            storage.add_range(vec![current_time]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 1);
        }
    }
}