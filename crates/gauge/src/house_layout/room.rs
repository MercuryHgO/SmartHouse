use json_minimal::Json;

use crate::{helpers::{json_check_array, json_check_number}, types::{fire_alarm::{FireAlarm, FireAlarmState}, Gauge, GaugeJson}};

use super::{dimensions::Centimeters, wall::Wall};

#[derive(Clone, Debug)]
pub struct Room {
    pub walls: Vec<Wall>,
    pub height: Centimeters,
    pub gauges: Vec<GaugeJson>
}

impl Into<Json> for Room {
    fn into(self) -> Json {
        Json::JSON(vec![
            Json::OBJECT { 
                name: "walls".to_string(),
                value: Box::new(Json::ARRAY(
                    self.walls
                        .into_iter()
                        .map(|v| v.into())
                        .collect()
                ))
            },

            Json::OBJECT {
                name: "height".to_string(),
                value: Box::new(self.height.into())
            },

            Json::OBJECT { 
                name: "gauges".to_string(),
                value: Box::new(Json::ARRAY(
                        self.gauges
                            .into_iter()
                            .map(|v| v.json())
                            .collect()
                    )
                )
            }
        ])
    }
}

impl TryFrom<&Json> for Room {
    type Error = crate::Error;

    /// Requered fields:
    /// `height`
    /// Additional fields:
    /// `gauges`
    /// `walls`
    fn try_from(value: &Json) -> Result<Self, Self::Error> {
        let height: Centimeters =
            json_check_number(value, "height",Err("height field required".into()))?;
        let walls: Vec<Wall> =
            json_check_array(value, "walls",Ok(vec![]))?;
        let gauges: Vec<GaugeJson> =
            json_check_array(value, "gauges",Ok(vec![]))?;

        Ok(
            Self {
                walls,
                height,
                gauges,
            }
        )
    }
}

impl Room {
    pub fn new(walls: Vec<Wall>, height: Centimeters, gauges: Vec<GaugeJson>) -> Self {
        Self { walls, height, gauges }
    }
}

impl Default for Room {
    fn default() -> Self {
        let gauge = FireAlarm::new("Test".to_string(), FireAlarmState::Disabled);
        let gauge = GaugeJson::new(&gauge.id(), gauge.name(), gauge.state());
        Self {
            walls: vec![ Wall::default()],
            height: 0.into(),
            gauges: vec![ gauge ]
        }
    }
}

