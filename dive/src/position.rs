use crate::movement::direction::Direction;
use crate::movement::Movement;
use std::fmt;
#[derive(Default)]
pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Position {
    pub fn step(mut self, movement: Movement) -> Self {
        match movement.direction {
            Direction::Up => {
                self.aim -= i32::from(movement.amount);
            }
            Direction::Down => {
                self.aim += i32::from(movement.amount);
            }
            Direction::Forward => {
                self.horizontal += i32::from(movement.amount);
                self.depth += i32::from(movement.amount) * self.aim;
            }
        }
        self
    }

    pub fn from_movements(movements: Vec<Movement>) -> Position {
        let mut position: Position = Default::default();

        for m in movements {
            position = position.step(m);
        }
        position
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Horizontal: {}, Depth: {}", self.horizontal, self.depth)
    }
}
