use std::io;
use std::str;

pub fn read_from_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    input.pop();
    Ok(input)
}

pub fn parse_input(input: &str) -> (i16, i16) {
    let (number, letter) = input.split_at(1);
    (parse_str_to_i16(number), parse_letter_to_i16(letter))
}

pub fn parse_letter_to_i16(letter: &str) -> i16 {
    letter.as_bytes()[0] as i16 - 65
}

fn parse_str_to_i16(s: &str) -> i16 {
    s.parse::<i16>().unwrap()
}

pub fn parse_i16_to_str(x: i16) -> String {
    let x = x.to_be_bytes();
    String::from_utf8(x.to_vec()).unwrap()
}

#[test]
fn should_parse_input() {
    assert_eq!((0, 0), parse_input("0A"));
    assert_eq!((4, 1), parse_input("4B"));
    assert_eq!((3, 7), parse_input("3H"));
}

#[test]
fn should_parse_i16_to_str() {
    assert_eq!("\u{0}A", parse_i16_to_str(65));
    assert_eq!("\u{0}B", parse_i16_to_str(66));
    assert_eq!("\u{0}K", parse_i16_to_str(75));
}
