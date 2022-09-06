use super::error::SmartHouseError;
use super::providers::DeviceInfoProvider;

// Device defines the device object
#[derive(Debug, PartialEq, Eq)]
pub struct Device<'a> {
    name: &'a str,
    state: &'a str,
}

impl<'a> Device<'a> {
    pub fn new(name: &'a str, state: &'a str) -> Self {
        Device { name, state }
    }
}

// Room defines room object and devices it contains
#[derive(Debug, PartialEq, Eq)]
pub struct Room<'a> {
    name: &'a str,
    devices: [&'a Device<'a>; 2],
}

impl<'a> Room<'a> {
    pub fn new(name: &'a str, devices: [&'a Device<'a>; 2]) -> Self {
        Room { name, devices }
    }
}

// SmartHOuse defines house object and rooms it contains
#[derive(Debug)]
pub struct SmartHouse<'a> {
    name: &'a str,
    rooms: [&'a Room<'a>; 2],
}

impl<'a> SmartHouse<'a> {
    pub fn new(name: &'a str, rooms: [&'a Room<'a>; 2]) -> Self {
        SmartHouse { name, rooms }
    }

    pub fn rooms(&self) -> [&str; 2] {
        [self.rooms[0].name, self.rooms[1].name]
    }

    pub fn get_room(&self, room_name: &str) -> Result<&Room, SmartHouseError> {
        for room in self.rooms {
            if room.name == room_name {
                return Ok(room);
            }
        }
        Err(SmartHouseError::RoomNotExist)
    }

    pub fn devices(&self, room_name: &str) -> Result<[&str; 2], SmartHouseError> {
        for room in self.rooms {
            if room.name == room_name {
                return Ok([room.devices[0].name, room.devices[1].name]);
            }
        }
        Err(SmartHouseError::RoomNotExist)
    }

    pub fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&Device, SmartHouseError> {
        let room = match self.get_room(room_name) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        for device in room.devices {
            if device.name == device_name {
                return Ok(device);
            }
        }
        Err(SmartHouseError::DeviceNotExist)
    }

    pub fn create_report<T>(&self, provider: T) -> String
    where
        T: DeviceInfoProvider,
    {
        let mut report = String::new();
        report.push_str(format!("Report for house: {}\n", self.name).as_str());
        let devices_info = provider.get_devices_info();
        for pair in devices_info {
            report.push_str(format!("  Room {}: ", pair.0).as_str());
            let room = match self.get_room(pair.0) {
                Ok(r) => r,
                Err(e) => {
                    report.push_str(format!("{}\n", e).as_str());
                    continue;
                }
            };
            report.push_str(format!("\n    Device {}: ", pair.1).as_str());
            match self.get_device(room.name, pair.1) {
                Ok(d) => report.push_str(format!("{}\n", d.state).as_str()),
                Err(e) => report.push_str(format!("{}\n", e).as_str()),
            };
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_house() -> &'static SmartHouse<'static> {
        &SmartHouse {
            name: "sample SmartHouse",
            rooms: [
                &Room {
                    name: "kitchen",
                    devices: [
                        &Device {
                            name: "socket-1",
                            state: "on",
                        },
                        &Device {
                            name: "socket-2",
                            state: "off",
                        },
                    ],
                },
                &Room {
                    name: "hall",
                    devices: [
                        &Device {
                            name: "socket-1",
                            state: "off",
                        },
                        &Device {
                            name: "socket-2",
                            state: "on",
                        },
                    ],
                },
            ],
        }
    }

    #[test]
    fn test_new_device() {
        let _device = Device::new("socket", "on");
    }

    #[test]
    fn test_new_room() {
        let device1 = Device::new("socket-1", "on");
        let device2 = Device::new("socket-2", "on");
        let _room = Room::new("hall", [&device1, &device2]);
    }

    #[test]
    fn test_new_smarthouse() {
        let socket1_kitchen = Device::new("socket-1", "on");
        let socket2_kitchen = Device::new("socket-2", "off");
        let socket1_hall = Device::new("socket-1", "off");
        let socket2_hall = Device::new("socket-2", "on");
        let room1 = Room::new("kitchen", [&socket1_kitchen, &socket2_kitchen]);
        let room2 = Room::new("hall", [&socket1_hall, &socket2_hall]);
        let _sh = SmartHouse::new("sample SmartHouse", [&room1, &room2]);
    }

    #[test]
    fn test_rooms() {
        let sh = seed_house();
        let expected = ["kitchen", "hall"];
        assert_eq!(sh.rooms(), expected);
    }

    #[test]
    fn test_get_room() {
        let socket1_kitchen = Device::new("socket-1", "on");
        let socket2_kitchen = Device::new("socket-2", "off");
        let socket1_hall = Device::new("socket-1", "off");
        let socket2_hall = Device::new("socket-2", "on");
        let room1 = Room::new("kitchen", [&socket1_kitchen, &socket2_kitchen]);
        let room2 = Room::new("hall", [&socket1_hall, &socket2_hall]);
        let sh = SmartHouse::new("sample SmartHouse", [&room1, &room2]);
        assert_eq!(sh.get_room("hall").unwrap(), &room2);
        assert_eq!(
            sh.get_room("bedroom").unwrap_err(),
            SmartHouseError::RoomNotExist
        )
    }

    #[test]
    fn test_devices() {
        let sh = seed_house();
        let expected = ["socket-1", "socket-2"];
        assert_eq!(sh.devices("hall").unwrap(), expected);
        assert_eq!(
            sh.devices("bedroom").unwrap_err(),
            SmartHouseError::RoomNotExist
        )
    }

    #[test]
    fn test_get_device() {
        let socket1_kitchen = Device::new("socket-1", "on");
        let socket2_kitchen = Device::new("socket-2", "off");
        let socket1_hall = Device::new("socket-1", "off");
        let socket2_hall = Device::new("socket-2", "on");
        let room1 = Room::new("kitchen", [&socket1_kitchen, &socket2_kitchen]);
        let room2 = Room::new("hall", [&socket1_hall, &socket2_hall]);
        let sh = SmartHouse::new("sample SmartHouse", [&room1, &room2]);
        assert_eq!(sh.get_device("hall", "socket-1").unwrap(), &socket1_hall);
        assert_eq!(
            sh.get_device("bedroom", "socket-1").unwrap_err(),
            SmartHouseError::RoomNotExist
        );
        assert_eq!(
            sh.get_device("kitchen", "socket-3").unwrap_err(),
            SmartHouseError::DeviceNotExist
        )
    }
}
