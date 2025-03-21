use json_minimal::Json;

use super::floor::Floor;


#[derive(Clone,Debug)]
pub struct House {
    pub(crate) floors: Vec<Floor>
}

impl House {
    pub fn new(floors: Vec<Floor>) -> Self { Self { floors } }

    pub fn push_floor(&mut self) {
        self.floors
            .push(
                Floor {
                    number: self.floors.len() as u8,
                    ..Default::default()
                }
            )
    }

    pub fn floors(&self) -> &Vec<Floor> {
        &self.floors
    }
}

impl Into<Json> for House {
    fn into(self) -> Json {
        let floors: Vec<Json> = self.floors
            .into_iter()
            .map(|v| v.into())
            .collect()
        ;

        Json::OBJECT {
            name: "floors".to_string(),
            value: Box::new(Json::ARRAY(floors))
        }
    }
}

impl Default for House {
    fn default() -> Self {
        Self { floors: vec![ Floor::default() ] }
    }
}
