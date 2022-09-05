// DeviceInfoProvider is a trait to implement custom providers for devices info and building reports
pub trait DeviceInfoProvider {
    fn get_devices_info(&self) -> [(&str, &str); 4];
}
