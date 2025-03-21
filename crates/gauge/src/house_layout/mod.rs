use dimensions::Position;

pub mod dimensions;
pub mod wall;
pub mod room;
pub mod floor;
pub mod house;

type Connection = Position;

#[cfg(test)]
mod tests {
    use crate::house_layout::{wall::Wall, Connection};

    #[test]
    fn connect_walls() {
        let connections: Vec<Connection> =  vec![
            ( 0, 0).into(),
            ( 0,10).into(),
            (10,10).into(),
            (10, 0).into(),
        ];

        let walls = Wall::from_connections(connections);

   
        // let mut walls = vec![
        //     Wall::new(( 0, 0).into(), ( 0,10).into(), None),
        //     Wall::new(( 0,10).into(), (10,10).into(), None),
        //     Wall::new((10,10).into(), (10, 0).into(), None),
        //     Wall::new((10, 0).into(), ( 0, 0).into(), None),
        // ];

        // Wall::connect_walls(&mut walls);

        assert!(true)
    }

    // #[test]
    // fn create_house() {
    //     let walls = 

    //     let House = House::new(
    //         Floor::new(
    //             vec![
                    
    //             ]
    //             ,1)
    //     );
    // }
}
