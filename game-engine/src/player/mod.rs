use std::io;

use crate::board::case::Coordinate;
use crate::board::piece::Color;

pub mod ai_player;

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
        println!("from ? (e.g 0A)");
        let from: (i16, i16) = parse_input(read_from_input().unwrap().as_str());

        println!("to ? (e.g 0A)");

        let to: (i16, i16) = parse_input(read_from_input().unwrap().as_str());
        //println!("{:?}|{:?}", from, to);

        (Coordinate::new(from.0, from.1), Coordinate::new(to.0, to.1))
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

fn parse_input(input: &str) -> (i16, i16) {
    let (number, letter) = input.split_at(1);
    (parse_str_to_i16(number), parse_letter_to_i16(letter))
}

fn parse_letter_to_i16(letter: &str) -> i16 {
    letter.as_bytes()[0] as i16 - 65
}

fn parse_str_to_i16(s: &str) -> i16 {
    s.parse::<i16>().unwrap()
}

#[test]
fn should_parse_input() {
    assert_eq!((0, 0), parse_input("0A"));
    assert_eq!((4, 1), parse_input("4B"));
    assert_eq!((3, 7), parse_input("3H"));
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
