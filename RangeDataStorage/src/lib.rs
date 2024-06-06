pub mod range_data_storage {
    use std::cmp::{max, min};
    use std::collections::{BTreeMap, HashSet};
    use serde::{Deserialize, Serialize};
    use serde;
    use std::hash::Hash;
    use std::io::Write;
    use serde::de::DeserializeOwned;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct RangeDataEntry<T> where T: PartialEq {
        start: T,
        end: T,
        data: Vec<T>
    }

    impl<T> RangeDataEntry<T>
        where T: Ord + PartialEq + Clone {
        pub fn new(start: T, end: T, data: Vec<T>) -> Self {
            assert!(start <= end, "Start must be less than or equal to end");
            RangeDataEntry { start, end, data }
        }

        pub fn contains(&self, start: &T, end: &T) -> bool {
            &self.start <= start && &self.end >= end
        }

        fn overlaps(&self, start: &T, end: &T) -> bool {
            !(self.end < *start || *end < self.start)
        }

        fn merge(&mut self, start: T, end: T, data: Vec<T>) {
            self.start = min(self.start.clone(), start);
            self.end = max(self.end.clone(), end);
            self.data.extend(data);
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RangeDataStore<T: PartialEq + Clone + Ord> {
        directory: String,
        file_name: String,
        entries: BTreeMap<T, RangeDataEntry<T>>
    }

    /// User should be able to ask RangeDataStorage if:
    /// - A range already exists in the storage
    impl<T> RangeDataStore<T> where T: Ord + Clone + Hash + Serialize + DeserializeOwned {
        pub fn new(directory: String, file_name: String) -> Self {
            if !std::path::Path::new(&directory).exists() {
                std::fs::create_dir(&directory).unwrap();
            }

            let file_path = format!("{}/{}", directory, file_name);

            if std::path::Path::new(&file_path).exists() {
                println!("Loading from file");
                return Self::load_from_file(&directory, &file_name);
            }

            RangeDataStore {
                directory,
                file_name,
                entries: BTreeMap::new(),
            }
        }

        // Add a new range with associated data to the storage
        pub fn add_range(&mut self, mut data: Vec<T>) {
            let mut to_merge = vec![];
            if data.is_empty() {
                return;
            }
            data.sort_unstable();
            let start = data.first().unwrap().clone();
            let end = data.last().unwrap().clone();
            let mut new_start = start.clone();
            let mut new_end = end.clone();
            let mut new_data: HashSet<T> = data.into_iter().collect();

            // Find overlapping ranges
            for (key, entry) in self.entries.range(..=end.clone()) {
                if entry.overlaps(&start, &end) {
                    to_merge.push(key.clone());
                    new_start = min(new_start.clone(), entry.start.clone());
                    new_end = max(new_end.clone(), entry.end.clone());
                    new_data.extend(entry.data.clone());
                }
            }

            // Remove overlapping ranges
            for key in to_merge {
                self.entries.remove(&key);
            }

            // revert to vector
            let new_data: Vec<T> = new_data.into_iter().collect();

            // Insert the merged range
            self.entries.insert(new_start.clone(), RangeDataEntry::new(new_start, new_end, new_data));
        }

        pub fn add_range_entry(&mut self, entry: RangeDataEntry<T>) {
            self.add_range(entry.data.clone());
        }

        // Check if a range is contained within any of the stored ranges
        fn contains(&self, start: &T, end: &T) -> bool {
            // iterate over each range entry in the storage, could maybe be a binary search(?)
            for (_, entry) in self.entries.range(..=start) {
                if entry.contains(start, end) {
                    return true;
                }
            }
            false
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn total_len(&self) -> usize {
            self.entries.iter().fold(0, |acc, (_, entry)| acc + entry.data.len())
        }

        pub fn load_from_file(directory: &str, file_name: &str) -> Self {
            let file_path = format!("{}/{}", directory, file_name);
            let file = std::fs::File::open(file_path).unwrap();
            let reader = std::io::BufReader::new(file);
            let data= serde_json::from_reader(reader);
            match data {
                Ok(data) => {
                    data
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    RangeDataStore {
                        directory: directory.to_string(),
                        file_name: file_name.to_string(),
                        entries: BTreeMap::new(),
                    }
                }
            }
        }

        pub fn save_to_file(&self, directory: &str, file_name: &str) {
            let json = serde_json::to_string_pretty(&self).unwrap();
            let mut file = std::fs::File::create(format!("{}/{}", directory, file_name)).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
    }
}
