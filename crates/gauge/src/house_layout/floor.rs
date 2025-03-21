use json_minimal::Json;

use super::room::Room;


#[derive(Clone, Debug)]
pub struct Floor {
    pub(crate) rooms: Vec<Room>,
    pub(crate) number: u8
}

impl Floor {
    pub fn new(rooms: Vec<Room>, number: u8) -> Self { Self { rooms, number } }
}

impl Into<Json> for Floor {
    fn into(self) -> Json {
        let rooms: Vec<Json> = self.rooms
            .into_iter()
            .map(|v| v.into())
            .collect()
        ;

        Json::JSON(vec![
            Json::OBJECT {
                name: "rooms".to_string(),
                value: Box::new(Json::ARRAY(
                    rooms
                ))
            },

            Json::OBJECT {
                name: "number".into(),
                value: Box::new(Json::NUMBER(
                    self.number.into()
                ))
            }

        ])
    }
}

impl Default for Floor {
    fn default() -> Self {
        Self { rooms: vec![ Room::default() ], number: 0 }
    }
}
