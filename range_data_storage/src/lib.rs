pub mod range_data_storage {
    use std::error::Error;
    use std::fmt::Debug;
    use serde::{Deserialize, Serialize};
    use serde;
    use std::path::Path;
    use rangemap::{RangeInclusiveMap, StepLite};
    use serde::de::DeserializeOwned;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RangeDataStorage<K, V> where
        K: Ord + Clone + Eq + Serialize + DeserializeOwned + StepLite,
        V: Serialize + Eq + Clone + DeserializeOwned{
        #[serde(bound(deserialize = r"
        K: Ord + Clone + DeserializeOwned + StepLite,
        V: Eq + Clone + DeserializeOwned,
    "))]
        range_map: RangeInclusiveMap<K, V>,
    }

    impl<K, V> RangeDataStorage<K, V>
        where
            K: Ord + Clone + Eq + Serialize + DeserializeOwned + StepLite,
            V: Serialize + Eq + Clone + DeserializeOwned {

        pub fn new(location: Option<String>) -> Result<RangeDataStorage<K, V>, Box<dyn Error>> {
            if location.is_some() {
                // load from file
                let location = location.unwrap();

                // do directories for the file path exist?
                let path = Path::new(&location);
                if !path.exists() {
                    // create directories
                    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                }

                // does the file exist?
                if path.exists() {
                    let data = std::fs::read_to_string(location)?;
                    let data: RangeDataStorage<K, V> = serde_json::from_str(&data)?;

                    // clone so that the borrowed value can be returned
                    // let cloned_data = data.clone();
                    return Ok(data);
                }
            }
            Ok(
                RangeDataStorage {
                    range_map: RangeInclusiveMap::new(),
                }
            )
        }
        pub fn insert(&mut self, key_start: K, key_end: K, value: V) {
            let range = key_start..=key_end;
            self.range_map.insert(range, value);
        }

        pub fn save(&mut self, location: String) {
            let file = std::fs::File::create(location).unwrap();
            let writer = std::io::BufWriter::new(file);
            let res = serde_json::to_writer(writer, self);
            match res {
                Ok(_) => {},
                Err(e) => { println!("Error while saving file : {:?}", e); }
            }
        }

        pub fn get(&self, key: K) -> Option<&V> {
            self.range_map.get(&key)
        }
    }

    impl <K, V> Drop for RangeDataStorage<K, V> where
        K: Ord + Clone + Eq + Serialize + DeserializeOwned + StepLite,
        V: Serialize+ Eq + Clone + DeserializeOwned {
        fn drop(&mut self) {
            self.save("data.json".to_string());
        }
    }
}
