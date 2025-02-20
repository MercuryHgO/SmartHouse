use std::rc::Rc;

use super::{dimensions::Centimeters, wall::Wall, RoomGauge};


#[derive(Debug)]
pub struct Room {
    walls: Vec<Wall>,
    hieght: Centimeters,
    gauges: Vec<Rc<RoomGauge>>
}

// TODO: Delete Rc
impl Room {
    pub fn new(walls: Vec<Wall>, hieght: Centimeters, gauges: Vec<Rc<RoomGauge>>) -> Self {
        Self { walls, hieght, gauges }
    }
}

impl Default for Room {
    fn default() -> Self {
        Self { walls: vec![], hieght: 0.into(), gauges: vec![] }
    }
}

