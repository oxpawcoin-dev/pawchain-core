use serde::{Deserialize, Serialize};
use std::io;

pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, io::Error> {
    bincode::serialize(value).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn deserialize<T: DeserializeOwned>(data: &[u8]) -> Result<T, io::Error> {
    bincode::deserialize(data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
