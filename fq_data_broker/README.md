# Purpose
The purpose of the data broker crate is to be responsible for retrieving and requesting data.

If it does not contain the requested bar data, it will retrieve it from ibapi for you.

If it does have it, and it is cached using a range data storage, it will return it to you.