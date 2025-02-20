use std::fmt::Display;

use super::floor::Floor;


#[derive(Debug)]
pub struct House {
    floors: Vec<Floor>
}

impl House {
    pub fn new(floors: Vec<Floor>) -> Self { Self { floors } }
}

impl Default for House {
    fn default() -> Self {
        Self { floors: vec![] }
    }
}

impl Display for House {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }
}
