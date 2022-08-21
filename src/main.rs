// Device defines the device object
struct Device<'a> {
    name: &'a str,
    state: &'a str,
}

impl<'a> Device<'a> {
    fn new(name: &'a str, state: &'a str) -> Self {
        Device { name, state }
    }
}

// Room defines room object and devices it contains
struct Room<'a> {
    name: &'a str,
    devices: [&'a Device<'a>; 2],
}

impl<'a> Room<'a> {
    fn new(name: &'a str, devices: [&'a Device<'a>; 2]) -> Self {
        Room { name, devices }
    }
}

// SmartHOuse defines house object and rooms it contains
struct SmartHouse<'a> {
    name: &'a str,
    rooms: [&'a Room<'a>; 2],
}

impl<'a> SmartHouse<'a> {
    fn new(name: &'a str, rooms: [&'a Room<'a>; 2]) -> Self {
        SmartHouse { name, rooms }
    }

    fn get_rooms(&self) -> [&str; 2] {
        [self.rooms[0].name, self.rooms[1].name]
    }

    fn devices(&self, room_name: &str) -> [&str; 2] {
        for room in self.rooms {
            if room.name == room_name {
                return [room.devices[0].name, room.devices[1].name];
            }
        }
        ["", ""]
    }

    fn is_room_exist(&self, room_name: &str) -> bool {
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

    fn is_device_exist(&self, room_name: &str, device_name: &str) -> bool {
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

    fn create_report<T>(&self, provider: T)
    where
        T: DeviceInfoProvider,
    {
        println!("Report for house: {}", self.name);
        let devices_info = provider.get_devices_info();
        for pair in devices_info {
            print!("  Room {}: ", pair.0);
            if !self.is_room_exist(pair.0) {
                println!("does not exist in the house.");
                continue;
            }
            println!();
            for room in self.rooms {
                if room.name != pair.0 {
                    continue;
                }
                print!("    Device {}: ", pair.1);
                if !self.is_device_exist(pair.0, pair.1) {
                    println!("does not exist in the room {}.", pair.0);
                    continue;
                }
                for dev in room.devices {
                    if dev.name != pair.1 {
                        continue;
                    }
                    println!("{}", dev.state);
                }
            }
        }
        println!()
    }
}

// DeviceInfoProvider is a trait to implement custom providers for devices info and building reports
trait DeviceInfoProvider {
    fn get_devices_info(&self) -> [(&str, &str); 4];
}

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

fn main() {
    //
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
    // case 2: existing rooms but some devices are invalid
    let provider2 = CustomProvider::new([
        ("kitchen", "socket-1"),
        ("kitchen", "thermo-1"),
        ("hall", "socket-1"),
        ("hall", "socket-3"),
    ]);
    // case 3: one room does not exist
    let provider3 = CustomProvider::new([
        ("bedroom", "socket-1"),
        ("bedroom", "socket-2"),
        ("kitchen", "socket-1"),
        ("kitchen", "socket-2"),
    ]);

    sh.create_report(provider1);
    sh.create_report(provider2);
    sh.create_report(provider3);
}
