use self::dimensions::{Meters, Position};

pub mod dimensions;

struct Door {
    position_first: Position,
    position_second: Position,
}

impl Door {
    fn new(position_first: Position, position_second: Position) -> Self { Self { position_first, position_second } }
}

struct Wall {
    position_first: Position,
    position_second: Position,

    connection_first: &Wall,
    connection_second: &Wall,

    doors: Option<Vec<Door>>
}

impl Wall {
    fn new(
        position_first: Position,
        position_second: Position,
        connection_first: &Wall,
        connection_second: &Wall,
        doors: Option<Vec<Door>>
    ) -> Self { 
        Self {
            position_first,
            position_second,
            connection_first,
            connection_second,
            doors
        }
    }
}

struct Room {
    walls: Vec<Wall>,
    hieght: Meters
}

impl Room {
    fn new(walls: Vec<Wall>, hieght: Meters) -> Self { Self { walls, hieght } }
}

struct Floor {
    rooms: Vec<Room>,
    number: u8
}

impl Floor {
    fn new(rooms: Vec<Room>, number: u8) -> Self { Self { rooms, number } }
}

struct House {
    floors: Vec<Floor>
}

impl House {
    fn new(floors: Vec<Floor>) -> Self { Self { floors } }
}

