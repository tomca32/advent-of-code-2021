mod movement;
mod position;

use movement::Movement;
use position::Position;

fn main() {
    let movements = Movement::from_input("./input.txt");

    let position: Position = Position::from_movements(movements);

    println!("{}", position);
    println!("Answer: {}", position.depth * position.horizontal);
}
