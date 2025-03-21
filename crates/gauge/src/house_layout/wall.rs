use json_minimal::Json;

use crate::helpers::{json_check_array, json_check_number, json_check_object};

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

impl TryFrom<&Json> for Door {
    type Error = crate::Error;

    fn try_from(value: &Json) -> Result<Self, Self::Error> {
        const NO_POSITION: &str = "postion star and end required";

        let value = json_check_object(value)?;

        let position_start: Position = value
            .get("position_start")
            .ok_or(NO_POSITION)?
            .try_into()?
        ;
        let position_end: Position = value
            .get("position_end")
            .ok_or(NO_POSITION)?
            .try_into()?
        ;

        Ok( Self { position_start, position_end })
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

        let connection_start: Json = match self.connection_start {
            Some(connection) => connection.into(),
            None => Json::NULL,
        };
        let connection_start = Json::OBJECT {
            name: "connection_start".into(),
            value: Box::new(connection_start)
        };

        let connection_end: Json = match self.connection_end {
            Some(connection) => connection.into(),
            None => Json::NULL,
        };
        let connection_end = Json::OBJECT {
            name: "connection_end".into(),
            value: Box::new(connection_end)
        };

        let doors = match self.doors {
            Some(door) => 
                Json::ARRAY(
                    door
                        .into_iter()
                        .map(|v| v.into())
                        .collect()
                ),
            None => Json::NULL,
        };
        let doors = Json::OBJECT {
            name: "doors".to_string(),
            value: Box::new(doors)
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

impl TryFrom<&Json> for Wall {
    type Error = crate::Error;

    fn try_from(value: &Json) -> Result<Self, Self::Error> {
        const POSTION_REQUIRED: &str = "postion start and end required";

        let position_start: Position = value.get("postition_start")
            .ok_or(POSTION_REQUIRED)?
            .try_into()?
        ;

        let position_end: Position = value.get("postition_end")
            .ok_or(POSTION_REQUIRED)?
            .try_into()?
        ;

        let connection_start: Option<Connection> =
        match value.get("connection_start") {
            Some(v) => Some(v.try_into()?),
            None => None,
        };

        let connection_end: Option<Connection> =
        match value.get("connection_end") {
            Some(v) => Some(v.try_into()?),
            None => None,
        };

        let doors: Vec<Door> =
        json_check_array(value, "doors", Ok(vec![]))?;
        let doors = doors.is_empty()
            .then_some(doors)
        ;

        Ok(
            Self
            {
                position_start,
                position_end,
                connection_start,
                connection_end,
                doors,
            }
        )
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
