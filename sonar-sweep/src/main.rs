mod input;

fn main() {
    let result: i32 = input::SONAR_INPUT
        .windows(3)
        .map(|w| w.iter().fold(0, |acc, item| acc + item)).collect::<Vec<i32>>()
        .windows(2).fold(0, |acc, e| -> i32 {
            match e[1] > e[0] {
                true => acc + 1,
                false => acc
            }
    });

    println!("Increased times: {}", result)
}
