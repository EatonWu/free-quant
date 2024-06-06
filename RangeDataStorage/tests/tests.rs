mod all_tests {
    mod range_data_storage_tests_integer {
        use RangeDataStorage::range_data_storage::{RangeDataStore};
        use RangeDataStorage::range_data_storage::RangeDataGetResult::{Found, Partial, SpansMultiple};

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

        #[test]
        fn test_range_get_subset() {
            let mut storage = RangeDataStore::new("@data".to_string(),"int_test".to_string());
            storage.add_range(vec![1, 2, 3]);
            storage.add_range(vec![5, 6]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 5);
            let subset = storage.get_range(&2, &3);
            assert_eq!(subset, Found(vec![2, 3]));
        }

        /// What should happen if we can locate only part of the range?
        /// This case implies there's some missing data.
        /// We can simply request data from the last result to the end of the range.
        #[test]
        fn test_range_get_subset_partial() {
            let mut storage = RangeDataStore::new("@data".to_string(),"int_test".to_string());
            storage.add_range(vec![1, 2, 3, 4, 5, 6]);
            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 6);
            let subset = storage.get_range(&5, &7);
            assert_eq!(subset, Partial(vec![5, 6]));
            println!("{:?}", subset);
        }

        /// What should happen if our requested range spans two different ranges?
        /// This case implies there's some missing data.
        /// How the user handles this is up to them. (Request full data?)
        #[test]
        fn test_range_get_subset_spans_two_ranges() {
            let mut storage = RangeDataStore::new("@data".to_string(),"int_test".to_string());
            storage.add_range(vec![1, 2, 3]);
            storage.add_range(vec![5, 6]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 5);
            let subset = storage.get_range(&2, &5);
            assert_eq!(subset, SpansMultiple(vec![2, 3, 5]));
            println!("{:?}", subset);

        }
    }

    mod range_data_storage_tests_string {
        use RangeDataStorage::range_data_storage::{RangeDataStore};
        use RangeDataStorage::range_data_storage::RangeDataGetResult::*;

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

        #[test]
        fn test_range_get_subset() {
            let mut storage = RangeDataStore::new("@data".to_string(), "string_test".to_string());
            storage.add_range(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
            storage.add_range(vec!["d".to_string(), "e".to_string(), "f".to_string()]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 6);
            let subset = storage.get_range(&"b".to_string(), &"c".to_string());
            assert_eq!(subset, Found(vec!["b".to_string(), "c".to_string()]));
        }

        #[test]
        fn test_range_get_subset_partial() {
            let mut storage = RangeDataStore::new("@data".to_string(), "string_test".to_string());
            storage.add_range(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

            assert_eq!(storage.len(), 1);
            assert_eq!(storage.total_len(), 3);
            let subset = storage.get_range(&"c".to_string(), &"f".to_string());
            assert_eq!(subset, Partial(vec!["c".to_string()]));
        }

        #[test]
        fn test_range_get_subset_spans_two_ranges() {
            let mut storage = RangeDataStore::new("@data".to_string(), "string_test".to_string());
            storage.add_range(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
            storage.add_range(vec!["d".to_string(), "e".to_string(), "f".to_string()]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 6);
            let subset = storage.get_range(&"b".to_string(), &"e".to_string());
            assert_eq!(subset, SpansMultiple(vec!["b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]));
        }
    }

    mod range_data_storage_tests_offset_datetime {
        use std::ops::Add;
        use RangeDataStorage::range_data_storage::{RangeDataStore};
        use time::{Duration, OffsetDateTime};
        use RangeDataStorage::range_data_storage::RangeDataGetResult::*;

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

        #[test]
        fn test_range_add_no_overlap() {
            let mut storage = RangeDataStore::new("@data".to_string(), "offset_datetime_test".to_string());
            let current_time = OffsetDateTime::now_utc();
            let add_duration = Duration::hours(1);
            let new_time = current_time.add(add_duration);
            storage.add_range(vec![current_time]);
            storage.add_range(vec![new_time]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 2);
        }

        #[test]
        fn test_range_get_subset() {
            let mut storage = RangeDataStore::new("@data".to_string(), "offset_datetime_test".to_string());
            let current_time = OffsetDateTime::now_utc();
            let add_duration = Duration::hours(1);
            let new_time = current_time.add(add_duration);
            storage.add_range(vec![current_time]);
            storage.add_range(vec![new_time]);
            assert_eq!(storage.len(), 2);
            assert_eq!(storage.total_len(), 2);
            let subset = storage.get_range(&current_time, &new_time);
            assert_eq!(subset, SpansMultiple(vec![current_time, new_time]));
        }
    }
}