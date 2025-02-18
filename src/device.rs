use super::house::{Device, Room};

pub struct SmartSocket {}

impl DeviceInfoProvider for SmartSocket {
    fn get_status(&self, room: &Room, device: &Device) -> Option<String> {
        use Device::*;

        match (room.name(), device) {
            ("Спальня", AC) => Some("on".into()),
            ("Спальня", Lamp) => Some("on".into()),
            ("Ельня", Stove) => Some("off".into()),
            ("Ельня", Fridge) => Some("off".into()),
            ("Ельня", Dishwasher) => Some("on".into()),
            ("Пильня", Stove) => Some("off".into()),
            ("Пильня", Fridge) => Some("off".into()),
            ("Пильня", Dishwasher) => Some("off".into()),
            ("Телевизор смотрельня", TV) => Some("off".into()),
            ("Телевизор смотрельня", Game) => Some("off".into()),
            ("Телевизор смотрельня", Router) => Some("off".into()),
            _ => None,
        }
    }
}

pub struct SmartThermometer {}

impl DeviceInfoProvider for SmartThermometer {
    fn get_status(&self, room: &Room, device: &Device) -> Option<String> {
        use Device::*;

        match (room.name(), device) {
            ("Спальня", AC) => Some("25".into()),
            ("Ельня", Stove) => Some("25".into()),
            ("Ельня", Fridge) => Some("-5".into()),
            ("Ельня", Dishwasher) => Some("40".into()),
            ("Пильня", Stove) => Some("25".into()),
            ("Пильня", Fridge) => Some("25".into()),
            ("Пильня", Dishwasher) => Some("25".into()),
            _ => None,
        }
    }
}

pub trait DeviceInfoProvider {
    fn get_status(&self, room: &Room, device: &Device) -> Option<String>;
}

pub struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl OwningDeviceInfoProvider {
    pub fn new(socket: SmartSocket) -> Self {
        Self { socket }
    }
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_status(&self, room: &Room, device: &Device) -> Option<String> {
        let mut result = String::new();

        result.push_str("socket: ");
        if let Some(status) = self.socket.get_status(room, device) {
            result.push_str(&status);
        } else {
            result.push_str("none");
        }

        Some(result)
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
    pub fn new(socket: &'a SmartSocket, thermo: &'b SmartThermometer) -> Self {
        Self { socket, thermo }
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_status(&self, room: &Room, device: &Device) -> Option<String> {
        let mut result = String::new();

        result.push_str("socket: ");
        if let Some(status) = self.socket.get_status(room, device) {
            result.push_str(&status);
        } else {
            result.push_str("none");
        }

        result.push_str(", thermo: ");
        if let Some(status) = self.thermo.get_status(room, device) {
            result.push_str(&status);
        } else {
            result.push_str("none");
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_smart_socket_get_status_returns_status() {
        let smart_socket = SmartSocket {};
        let device = Device::AC;
        let room = Room::new("Спальня".into(), vec![]);

        assert_eq!(smart_socket.get_status(&room, &device), Some("on".into()))
    }

    #[test]
    fn it_smart_socket_get_status_returns_none() {
        let smart_socket = SmartSocket {};
        let device = Device::AC;
        let room = Room::new("test".into(), vec![]);

        assert_eq!(smart_socket.get_status(&room, &device), None)
    }

    #[test]
    fn it_smart_thermometer_get_status_returns_status() {
        let smart_thermometer = SmartThermometer {};
        let device = Device::AC;
        let room = Room::new("Спальня".into(), vec![]);

        assert_eq!(
            smart_thermometer.get_status(&room, &device),
            Some("25".into())
        )
    }

    #[test]
    fn it_smart_thermometer_get_status_returns_none() {
        let smart_thermometer = SmartThermometer {};
        let device = Device::AC;
        let room = Room::new("test".into(), vec![]);

        assert_eq!(smart_thermometer.get_status(&room, &device), None)
    }

    #[test]
    fn it_owning_device_info_provider_get_status_returns_status() {
        let smart_socket = SmartSocket {};
        let owning_device_info_provider = OwningDeviceInfoProvider::new(smart_socket);
        let device = Device::AC;
        let room = Room::new("Спальня".into(), vec![]);

        assert_eq!(
            owning_device_info_provider.get_status(&room, &device),
            Some("socket: on".into())
        )
    }

    #[test]
    fn it_owning_device_info_provider_get_status_returns_none() {
        let smart_socket = SmartSocket {};
        let owning_device_info_provider = OwningDeviceInfoProvider::new(smart_socket);
        let device = Device::AC;
        let room = Room::new("test".into(), vec![]);

        assert_eq!(
            owning_device_info_provider.get_status(&room, &device),
            Some("socket: none".into())
        )
    }

    #[test]
    fn it_borrowing_device_info_provider_get_status_returns_status() {
        let smart_socket = SmartSocket {};
        let smart_thermometer = SmartThermometer {};
        let borrowing_device_info_provider =
            BorrowingDeviceInfoProvider::new(&smart_socket, &smart_thermometer);
        let device = Device::AC;
        let room = Room::new("Спальня".into(), vec![]);

        assert_eq!(
            borrowing_device_info_provider.get_status(&room, &device),
            Some("socket: on, thermo: 25".into())
        )
    }

    #[test]
    fn it_borrowing_device_info_provider_get_status_returns_none() {
        let smart_socket = SmartSocket {};
        let smart_thermometer = SmartThermometer {};
        let borrowing_device_info_provider =
            BorrowingDeviceInfoProvider::new(&smart_socket, &smart_thermometer);
        let device = Device::AC;
        let room = Room::new("test".into(), vec![]);

        assert_eq!(
            borrowing_device_info_provider.get_status(&room, &device),
            Some("socket: none, thermo: none".into())
        )
    }
}
