use json_minimal::Json;

use super::room::Room;


#[derive(Debug)]
pub struct Floor {
    rooms: Vec<Room>,
    number: u8
}

impl Floor {
    pub fn new(rooms: Vec<Room>, number: u8) -> Self { Self { rooms, number } }
}

// impl Into<Json> for Floor {
//     fn into(self) -> Json {
//         let rooms = self.rooms
//             .into_iter()
//             .map(|v| Json::from(v.into()))
//         Json::JSON(vec![
//             Json::OBJECT { name: "rooms", value: Json::ARRAY() }
//         ])
//     }
// }

impl Default for Floor {
    fn default() -> Self {
        Self { rooms: vec![], number: 0 }
    }
}
