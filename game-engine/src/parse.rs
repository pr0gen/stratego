use std::io;

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

#[test]
fn should_parse_input() {
    assert_eq!((0, 0), parse_input("0A"));
    assert_eq!((4, 1), parse_input("4B"));
    assert_eq!((3, 7), parse_input("3H"));
}
