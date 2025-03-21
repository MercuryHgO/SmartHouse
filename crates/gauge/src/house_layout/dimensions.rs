use std::ops::{AddAssign, SubAssign};

use json_minimal::Json;

use crate::helpers::json_check_object;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default,Clone, Copy)]
pub struct Centimeters(u16);

impl std::ops::Add for Centimeters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Centimeters(self.0 + rhs.0)
    }
}

impl AddAssign for Centimeters {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0
    }
}

impl std::ops::Sub for Centimeters {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Centimeters(self.0 + rhs.0)
    }
}

impl SubAssign for Centimeters {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0
    }
}

impl From<u16> for Centimeters {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<f64> for Centimeters {
    fn from(value: f64) -> Self {
        Self(value as u16)
    }
}

impl Into<Json> for Centimeters {
    fn into(self) -> Json {
        Json::NUMBER(self.0.into())
    }
}

pub struct Meters<'a>(&'a Centimeters);


#[derive(Debug,Clone)]
pub struct Position {
    pub x: Centimeters,
    pub y: Centimeters
}

impl Position {
    pub fn new(x: Centimeters, y: Centimeters) -> Self { Self { x, y } }
}

impl Into<Json> for Position {
    fn into(self) -> Json {
        Json::JSON(vec![
            Json::OBJECT { name: "x".to_string(), value: Box::new(self.x.into()) },
            Json::OBJECT { name: "y".to_string(), value: Box::new(self.y.into()) }
        ])
    }
}

impl TryFrom<&Json> for Position {
    type Error = crate::Error;

    fn try_from(value: &Json) -> Result<Self, Self::Error> {
        const POSITIONS_REQUIRED: &str = "both x and y position requied";
        const NOT_A_NUMBER: &str = "coordinate must be a number";

        let value = json_check_object(value)?;

        let x = value.get("x")
            .ok_or(POSITIONS_REQUIRED)?;
        let y = value.get("y")
            .ok_or(POSITIONS_REQUIRED)?;

        let x: Centimeters = match json_check_object(x)? {
            Json::NUMBER(n) => (*n).into(),
            _ => Err(NOT_A_NUMBER)?
        };

        let y: Centimeters = match json_check_object(y)? {
            Json::NUMBER(n) => (*n).into(),
            _ => Err(NOT_A_NUMBER)?
        };

        Ok( Self { x, y } )
    }
}

impl<ICX,ICY> From<(ICX,ICY)> for Position
where
    ICX: Into<Centimeters>,
    ICY: Into<Centimeters>
{
    fn from(value: (ICX,ICY)) -> Self {
        Self { x: value.0.into(), y: value.1.into() }
    }
}

#[cfg(test)]
mod test {
    use super::Centimeters;

    #[test]
    fn some() {
        let first: Centimeters = Centimeters(5);
        let second: Centimeters = Centimeters(5);

        assert_eq!(first + second, Centimeters(10));
    }
}
