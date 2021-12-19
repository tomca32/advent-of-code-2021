use std::fmt;
use std::str::FromStr;

pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input.to_lowercase().as_ref() {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "forward" => Ok(Self::Forward),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "up",
                Direction::Down => "down",
                Direction::Forward => "forward",
            }
        )
    }
}
