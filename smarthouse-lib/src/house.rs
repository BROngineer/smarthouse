use super::providers::DeviceInfoProvider;

// Device defines the device object
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
pub struct SmartHouse<'a> {
    name: &'a str,
    rooms: [&'a Room<'a>; 2],
}

impl<'a> SmartHouse<'a> {
    pub fn new(name: &'a str, rooms: [&'a Room<'a>; 2]) -> Self {
        SmartHouse { name, rooms }
    }

    pub fn get_rooms(&self) -> [&str; 2] {
        [self.rooms[0].name, self.rooms[1].name]
    }

    pub fn devices(&self, room_name: &str) -> [&str; 2] {
        for room in self.rooms {
            if room.name == room_name {
                return [room.devices[0].name, room.devices[1].name];
            }
        }
        ["", ""]
    }

    pub fn is_room_exist(&self, room_name: &str) -> bool {
        let mut result = false;
        let rooms = self.get_rooms();
        for room in rooms {
            if room == room_name {
                result = true;
                return result;
            }
        }
        result
    }

    pub fn is_device_exist(&self, room_name: &str, device_name: &str) -> bool {
        let mut result = false;
        let rooms = self.get_rooms();
        for room in rooms {
            if room == room_name {
                let devices = self.devices(room);
                for dev in devices {
                    if dev == device_name {
                        result = true;
                        return result;
                    }
                }
            }
        }
        result
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
            if !self.is_room_exist(pair.0) {
                report.push_str("does not exist in the house.\n");
                continue;
            }
            for room in self.rooms {
                if room.name != pair.0 {
                    continue;
                }
                report.push_str(format!("\n    Device {}: ", pair.1).as_str());
                if !self.is_device_exist(pair.0, pair.1) {
                    report.push_str(format!("does not exist in the room {}.\n", pair.0).as_str());
                    continue;
                }
                for dev in room.devices {
                    if dev.name != pair.1 {
                        continue;
                    }
                    report.push_str(format!("{}\n", dev.state).as_str());
                }
            }
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
    fn test_get_rooms() {
        let sh = seed_house();
        assert_eq!(sh.get_rooms(), ["kitchen", "hall"]);
    }

    #[test]
    fn test_devices() {
        let sh = seed_house();
        assert_eq!(sh.devices("hall"), ["socket-1", "socket-2"]);
    }

    #[test]
    fn test_is_room_exist() {
        let sh = seed_house();
        assert_eq!(sh.is_room_exist("hall"), true);
        assert_eq!(sh.is_room_exist("bathroom"), false);
    }

    #[test]
    fn test_is_device_exist() {
        let sh = seed_house();
        assert_eq!(sh.is_device_exist("hall", "socket-1"), true);
        assert_eq!(sh.is_device_exist("kitchen", "thermo-1"), false);
    }
}
