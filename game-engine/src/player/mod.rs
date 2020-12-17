use std::io;

use crate::board::case::Coordinate;
use crate::board::piece::Color;

pub trait Player {
    fn ask_next_move(&self) -> (Coordinate, Coordinate);

    fn get_color(&self) -> &Color;

    fn get_name(&self) -> &str;
}

pub struct HumanPlayer {
    color: Color,
    name: String,
}

impl Player for HumanPlayer {
    fn ask_next_move(&self) -> (Coordinate, Coordinate) {
        println!("{:?} is playing", self.color);
        println!("from ? (y/x) (split with , e.g 1,2)");
        let from: Vec<i16> = read_from_input()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<i16>().unwrap())
            .collect();

        println!("to ? (y/x) (split with , e.g 1,2)");

        let to: Vec<i16> = read_from_input()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<i16>().unwrap())
            .collect();

        //println!("{:?}|{:?}", from, to);

        (
            Coordinate::new(from.get(0).unwrap().clone(), from.get(1).unwrap().clone()),
            Coordinate::new(to.get(0).unwrap().clone(), to.get(1).unwrap().clone()),
        )
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl HumanPlayer {
    pub fn new(color: Color, name: String) -> Self {
        HumanPlayer { color, name }
    }
}

fn read_from_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    input.pop();
    Ok(input)
}

#[cfg(test)]
mod test {
    use crate::board::piece::Color;

    use super::{HumanPlayer, Player};

    #[test]
    fn should_create_player() {
        let hp = HumanPlayer::new(Color::Red, String::from("Tigran"));
        assert_eq!(hp.get_name(), "Tigran");
        assert_eq!(hp.get_color(), &Color::Red);
    }
}
