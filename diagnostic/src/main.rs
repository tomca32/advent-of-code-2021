mod diagnostic;

fn main() {
    let diag = diagnostic::DiagnosticReport::new("input.txt");
    println!("Gamma Rate is: {}", diag.get_gamma_rate());
    println!("Epsilon Rate is: {}", diag.get_epsilon_rate());
    println!(
        "Power Consumption is: {}",
        diag.get_gamma_rate() * diag.get_epsilon_rate()
    );

    let oxygen_generation = diag.get_oxygen_generation();
    let co2_scrubber_rating = diag.get_co2_scrubber_rating();

    println!(
        "Oxygen Generation is: {}",
        oxygen_generation
    );
    println!(
        "CO2 Scrubber Rating is is: {}",
        co2_scrubber_rating
    );
    println!(
        "Life Support Rating is: {}",
        oxygen_generation * co2_scrubber_rating
    );
}
