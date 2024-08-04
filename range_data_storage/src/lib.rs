pub mod range_data_storage {
    use std::fmt::Debug;
    use std::io::Write;
    use serde::{Deserialize, Serialize};
    use serde;
    use std::path::Path;
    use rangemap::{RangeInclusiveMap, StepLite};
    use serde::de::DeserializeOwned;
    use anyhow::{bail, Error};

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

        pub fn new(location: Option<String>) -> Result<RangeDataStorage<K, V>, Error> {
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


        pub fn save(&mut self, location: String) -> Result<(), Error>{
            // create all parent directories
            let path = Path::new(&location);
            if !path.exists() {
                let parent = path.parent();
                if parent.is_none() {
                    bail!("Parent directory does not exist");
                }
                let parent = parent.unwrap();
                let res = std::fs::create_dir_all(parent);
                match res {
                    Ok(_) => {},
                    Err(e) => { bail!(e)}
                }
            }
            let file = std::fs::File::create(location);
            if file.is_err() {
                bail!("Error creating file");
            }
            let file = file.unwrap();
            let mut writer = std::io::BufWriter::new(file);
            // convert self to pretty json and write to file
            let res = serde_json::to_string_pretty(self);
            match res {
                Ok(data) => {
                    let res = writer.write_all(data.as_bytes());
                    match res {
                        Ok(_) => Ok(()),
                        Err(e) => { bail!(e)}
                    }
                },
                Err(e) => {
                    bail!(e)
                }
            }
        }

        pub fn contains(&mut self, key: K) -> bool {
            self.range_map.contains_key(&key)
        }

        pub fn contains_range(&self, key_start: &K, key_end: &K) -> bool {
            let has_start_and_end = self.range_map.contains_key(&key_start) && self.range_map.contains_key(&key_end);
            return has_start_and_end && self.range_map.get(&key_start) == self.range_map.get(&key_end);
        }

        pub fn get(&self, key: K) -> Option<&V> {
            self.range_map.get(&key)
        }
    }

    impl <K, V> Drop for RangeDataStorage<K, V> where
        K: Ord + Clone + Eq + Serialize + DeserializeOwned + StepLite,
        V: Serialize+ Eq + Clone + DeserializeOwned {
        fn drop(&mut self) {
            match self.save("@data/data.json".to_string()) {
                Ok(_) => {},
                Err(e) => {println!("Error while dropping RangeDataStorage: {:?}", e)}
            }
        }
    }
}
