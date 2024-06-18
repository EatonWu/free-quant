# Purpose

The purpose of this crate is to act as a wrapper around RangeMap for our date_start..date_end -> data mappings.
It even provides serialization and deserialization for the data!

General usage of this crate will be to:

1. Create a new RangeDataStorage, optionally providing a path to a file to load data from.
2. Insert data into the RangeDataStorage.
3. When we drop the RangeDataStorage, it will automatically save the data to a file.
