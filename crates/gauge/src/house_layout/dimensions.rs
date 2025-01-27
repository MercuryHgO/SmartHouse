#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Centimeters(u16);

pub struct Meters(&Centimeters);

pub struct Position {
    pub x: Meters,
    pub y: Meters
}

#[cfg(test)]
mod test {
    use super::Centimeters;

    #[test]
    fn some() {
        let first: Centimeters = 5;
        let second: Centimeters = 5;

        assert_eq!(first + second, 10);
    }
}
