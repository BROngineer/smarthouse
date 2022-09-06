use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum SmartHouseError {
    NoError,
    RoomNotExist,
    DeviceNotExist,
}

impl Display for SmartHouseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            SmartHouseError::RoomNotExist => f.write_str("room does not exist"),
            SmartHouseError::DeviceNotExist => f.write_str("device does not exist"),
            SmartHouseError::NoError => Ok(()),
        }
    }
}

impl Error for SmartHouseError {}
