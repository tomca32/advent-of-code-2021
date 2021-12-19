#[path = "movement/direction.rs"]
pub mod direction;
use direction::Direction;
use std::fmt;
use std::fs;
use std::str::FromStr;

pub struct Movement {
    pub direction: Direction,
    pub amount: i16,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Direction: {} - Amount: {}",
            self.direction.to_string(),
            self.amount
        )
    }
}

impl Movement {
    pub fn from_input(path: &str) -> Vec<Movement> {
        let input_string = fs::read_to_string(path).expect("Failed reading input");
        input_string
            .split('\n')
            .map(|line| -> Movement {
                if let [direction, amount] = line.split(' ').collect::<Vec<&str>>()[..] {
                    return Movement {
                        direction: Direction::from_str(direction).unwrap(),
                        amount: amount.parse::<i16>().unwrap(),
                    };
                }

                Movement {
                    direction: Direction::Forward,
                    amount: 0,
                }
            })
            .collect::<Vec<Movement>>()
    }
}
