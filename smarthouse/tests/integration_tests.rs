use smarthouse_lib::house::{Device, Room, SmartHouse};
use smarthouse_lib::providers::DeviceInfoProvider;

// CustomProvider implements DeviceInfoProvider
struct CustomProvider<'a> {
    devices: [(&'a str, &'a str); 4],
}

impl<'a> CustomProvider<'a> {
    fn new(devices: [(&'a str, &'a str); 4]) -> Self {
        CustomProvider { devices }
    }
}

impl DeviceInfoProvider for CustomProvider<'_> {
    fn get_devices_info(&self) -> [(&str, &str); 4] {
        self.devices
    }
}

#[test]
fn test_report_ok() {
    let socket1_kitchen = Device::new("socket-1", "on");
    let socket2_kitchen = Device::new("socket-2", "off");
    let socket1_hall = Device::new("socket-1", "off");
    let socket2_hall = Device::new("socket-2", "on");
    let room1 = Room::new("kitchen", [&socket1_kitchen, &socket2_kitchen]);
    let room2 = Room::new("hall", [&socket1_hall, &socket2_hall]);
    let sh = SmartHouse::new("sample SmartHouse", [&room1, &room2]);

    // case 1: existing rooms and devices
    let provider1 = CustomProvider::new([
        ("kitchen", "socket-1"),
        ("kitchen", "socket-2"),
        ("hall", "socket-1"),
        ("hall", "socket-2"),
    ]);

    let report = "Report for house: sample SmartHouse
  Room kitchen: 
    Device socket-1: on
  Room kitchen: 
    Device socket-2: off
  Room hall: 
    Device socket-1: off
  Room hall: 
    Device socket-2: on
";

    assert_eq!(sh.create_report(provider1), report);
}

#[test]
fn test_report_device_invalid() {
    let socket1_kitchen = Device::new("socket-1", "on");
    let socket2_kitchen = Device::new("socket-2", "off");
    let socket1_hall = Device::new("socket-1", "off");
    let socket2_hall = Device::new("socket-2", "on");
    let room1 = Room::new("kitchen", [&socket1_kitchen, &socket2_kitchen]);
    let room2 = Room::new("hall", [&socket1_hall, &socket2_hall]);
    let sh = SmartHouse::new("sample SmartHouse", [&room1, &room2]);

    // case 2: existing rooms but some devices are invalid
    let provider2 = CustomProvider::new([
        ("kitchen", "socket-1"),
        ("kitchen", "thermo-1"),
        ("hall", "socket-1"),
        ("hall", "socket-3"),
    ]);

    let report = "Report for house: sample SmartHouse
  Room kitchen: 
    Device socket-1: on
  Room kitchen: 
    Device thermo-1: does not exist in the room kitchen.
  Room hall: 
    Device socket-1: off
  Room hall: 
    Device socket-3: does not exist in the room hall.
";

    assert_eq!(sh.create_report(provider2), report);
}

#[test]
fn test_report_room_invalid() {
    let socket1_kitchen = Device::new("socket-1", "on");
    let socket2_kitchen = Device::new("socket-2", "off");
    let socket1_hall = Device::new("socket-1", "off");
    let socket2_hall = Device::new("socket-2", "on");
    let room1 = Room::new("kitchen", [&socket1_kitchen, &socket2_kitchen]);
    let room2 = Room::new("hall", [&socket1_hall, &socket2_hall]);
    let sh = SmartHouse::new("sample SmartHouse", [&room1, &room2]);

    // case 3: one room does not exist
    let provider3 = CustomProvider::new([
        ("bedroom", "socket-1"),
        ("bedroom", "socket-2"),
        ("kitchen", "socket-1"),
        ("kitchen", "socket-2"),
    ]);

    let report = "Report for house: sample SmartHouse
  Room bedroom: does not exist in the house.
  Room bedroom: does not exist in the house.
  Room kitchen: 
    Device socket-1: on
  Room kitchen: 
    Device socket-2: off
";

    assert_eq!(sh.create_report(provider3), report);
}
