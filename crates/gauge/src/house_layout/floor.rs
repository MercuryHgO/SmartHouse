use super::room::Room;


#[derive(Debug)]
pub struct Floor {
    rooms: Vec<Room>,
    number: u8
}

impl Floor {
    pub fn new(rooms: Vec<Room>, number: u8) -> Self { Self { rooms, number } }
}

impl Default for Floor {
    fn default() -> Self {
        Self { rooms: vec![], number: 0 }
    }
}
