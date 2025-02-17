use super::{
    error::Error,
    device::DeviceInfoProvider
};

#[derive(Debug)]
pub struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    pub fn new(name: String, rooms: Vec<Room>) -> Self {
        Self { name, rooms }
    }

    pub fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(Room::name).collect()
    }

    pub fn devices(&self, room_name: &str) -> Result<Vec<&str>, Error> {
        for room in &self.rooms {
            if room.name == room_name {
                return Ok(room.get_devices())
            }
        }

        Err(Error::RoomNotFound(room_name.into()))
    }

    pub fn create_report(&self, device_info: &dyn DeviceInfoProvider) -> String {
        let mut result = String::new();

        result.push_str("House: ");
        result.push_str(&self.name);
        result.push('\n');

        for room in &self.rooms {
            result.push_str("\n  Room: ");
            result.push_str(&room.name);
            result.push('\n');

            for device in &room.devices {
                result.push_str("    Device: ");
                result.push_str(device.get_name());
                result.push_str(" (");
                if let Some(status) = device_info.get_status(room, device) {
                    result.push_str(&status);
                }
                result.push_str(")\n");
            }
        }

        result
    }
}

impl Default for House {
    fn default() -> Self {
        use Device::*;

        Self::new(
            "Дом".into(),
            vec![
                Room::new("Спальня".into(), vec![AC, Lamp, Ebook]),
                Room::new("Ельня".into(), vec![Stove, Fridge, Dishwasher]),
                Room::new("Пильня".into(), vec![Stove, Fridge, Dishwasher]),
                Room::new("Спальня".into(), vec![AC, Lamp, Ebook]),
                Room::new("Телевизор смотрельня".into(), vec![TV, Game, Router]),
            ]
        )
    }
}

#[derive(Debug)]
pub struct Room {
    name: String,
    devices: Vec<Device>,
}

impl Room {
    pub fn new(name: String, devices: Vec<Device>) -> Self {
        Self { name, devices }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_devices(&self) -> Vec<&str> {
        self.devices.iter().map(Device::get_name).collect()
    }
}

#[derive(Debug)]
pub enum Device {
    AC,
    Lamp,
    Ebook,
    Stove,
    Fridge,
    Dishwasher,
    TV,
    Game,
    Router,
}

impl Device {
    pub fn get_name(&self) -> &str {
        use Device::*;

        match self {
            AC => "Кондиционер",
            Lamp => "Лампа",
            Ebook => "Электронная книга",
            Stove => "Плита",
            Fridge => "Холодильник",
            Dishwasher => "Посудомоечная машина",
            TV => "Телевизор",
            Game => "Игровая приставка",
            Router => "Маршрутизатор",
        }
    }
}


