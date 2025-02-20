use json_minimal::Json;

use super::{dimensions::Position, Connection};

#[derive(Debug, Clone)]
pub struct Door {
    position_start: Position,
    position_end:   Position,
}

impl Door {
    pub fn new(position_start: Position, position_end: Position) -> Self { Self { position_start, position_end } }
}

impl Into<Json> for Door {
    fn into(self) -> Json {
        Json::JSON(vec![
            Json::OBJECT { name: "position_start".into(), value: Box::new(self.position_start.into()) },
            Json::OBJECT { name: "position_end".into(), value: Box::new(self.position_end.into()) }
        ])
    }
}

#[derive(Debug, Clone)]
pub struct Wall {
    position_start: Position,
    position_end:   Position,

    connection_start: Option<Connection>,
    connection_end:   Option<Connection>,

    doors: Option<Vec<Door>>
}

impl Into<Json> for Wall {
    fn into(self) -> Json {
        let position_start = Json::OBJECT { name: "position_start".into(), value: Box::new(self.position_start.into()) };
        let position_end = Json::OBJECT { name: "position_end".into(), value: Box::new(self.position_end.into()) };

        let connection_start = match self.connection_start {
            Some(connection) => Json::OBJECT { name: "connection_start".into(), value: Box::new(connection.into()) },
            None => Json::NULL,
        };
        let connection_end = match self.connection_end {
            Some(connection) => Json::OBJECT { name: "connection_end".into(), value: Box::new(connection.into()) },
            None => Json::NULL,
        };

        let doors = match self.doors {
            Some(door) => 
                Json::ARRAY(
                    door
                        .into_iter()
                        .map(|v| v.into())
                        .collect()
                ),
            None => todo!(),
        };

        Json::JSON(vec![
            position_start,
            position_end,

            connection_end,
            connection_start,

            doors
        ])
    }
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            position_start: (0,0).into(),
            position_end: (0,0).into(),
            connection_start: None,
            connection_end: None,
            doors: None
         }
    }
}

impl Wall {
    pub fn new(
        position_start: Position,
        position_end:   Position,
        connection_start: Option<Connection>,
        connection_end:   Option<Connection>,
        doors: Option<Vec<Door>>
    ) -> Self { 
        Self {
            position_start,
            position_end,
            connection_start,
            connection_end,
            doors
        }
    }

    pub fn from_connections(connections: Vec<Connection>) -> Vec<Wall> {
        let mut walls: Vec<Wall>  = Vec::new();

        let len = connections.len();
        for (idx, connection) in connections.clone().into_iter().enumerate() {
            walls.push(
                Wall::new(
                     connection.clone(),
                     connections[(idx+1)%len].clone(),
                     Some(connection),
                     Some(connections[(idx+1)%len].clone()),
                     None
                )
            )
        };

        walls
    }
}
