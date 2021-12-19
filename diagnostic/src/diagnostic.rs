use std::{cmp, fmt};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct DiagnosticLine {
    pub digits: Vec<bool>,
}


impl DiagnosticLine {
    fn to_u32(&self) -> u32 {
        self.digits.iter().rev().enumerate().fold(0, |acc, (i, digit)| {
            acc + (if *digit {2u32.pow(i as u32)} else {0})
        })
    }

    pub fn new(input: String) -> DiagnosticLine {
        let mut line = DiagnosticLine { digits: Vec::new() };
        for char in input.chars() {
            line.digits.push(char == '1')
        }
        line
    }
}

impl fmt::Display for DiagnosticLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line Digits: {}", self.digits.iter().map(|e| if *e {'1'} else {'0'}).collect::<String>())
    }
}

pub struct DiagnosticReport {
    pub lines: Vec<DiagnosticLine>,
    number_of_columns: u32,
}

impl DiagnosticReport {
    pub fn new(path: &str) -> DiagnosticReport {
        let file = File::open(path).expect("Error reading input file");
        let reader = BufReader::new(file);

        let mut diagnostic = DiagnosticReport {
            lines: Vec::new(),
            number_of_columns: 0,
        };
        for line in reader.lines() {
            let line_value = line.expect("Error reading file");
            let line_length = line_value.len() as u32;
            diagnostic
                .lines
                .push(DiagnosticLine::new(line_value));
            diagnostic.number_of_columns = cmp::max(diagnostic.number_of_columns, line_length)
        }
        diagnostic
    }

    fn get_column(lines: &Vec<&DiagnosticLine>, n: usize) -> Vec<bool> {
        lines.iter().fold(Vec::new(), |mut acc, row| {
            acc.push(row.digits[n]);
            acc
        })
    }

    fn get_most_common_column_value(lines: &Vec<&DiagnosticLine>, column_index: usize) -> bool {
        let column_sum: u32 =
            Self::get_column(&lines, column_index).iter().fold(
                0,
                |acc, digit| {
                    if *digit {
                        acc + 1
                    } else {
                        acc
                    }
                },
            );
        column_sum as f64 >= lines.len() as f64 / 2.0
    }

    fn get_least_common_column_value(lines: &Vec<&DiagnosticLine>, column_index: usize) -> bool {
        !Self::get_most_common_column_value(lines, column_index)
    }

    fn diagnostic_values(&self, column_evaluator: impl Fn(&Vec<&DiagnosticLine>, usize) -> bool) -> Vec<bool> {
        (0..self.number_of_columns)
            .map(|i| column_evaluator(&self.lines.iter().collect(), i as usize))
            .collect()
    }

    pub fn get_gamma_rate(&self) -> u32 {
        let v = DiagnosticLine { digits: self.diagnostic_values(Self::get_most_common_column_value) };
        println!("Gamma Line: {}", v);
        v.to_u32()
    }

    pub fn get_epsilon_rate(&self) -> u32 {
        DiagnosticLine { digits: self.diagnostic_values(Self::get_least_common_column_value) }.to_u32()
    }

    fn filter_lines_by_column_value<'a>(lines: Vec<&'a DiagnosticLine>, column_evaluator: &impl Fn(&Vec<&DiagnosticLine>, usize) -> bool, i: usize) -> Vec<&'a DiagnosticLine> {
        let value = column_evaluator(&lines, i);
        lines.into_iter().filter(|line| line.digits[i] == value).collect()
    }

    fn advance_diagnostic_values(lines: Vec<&DiagnosticLine>, column_evaluator: impl Fn(&Vec<&DiagnosticLine>, usize) -> bool) -> &DiagnosticLine {
        let mut filtered_lines = lines;
        let mut index = 0;
        loop {
            filtered_lines = Self::filter_lines_by_column_value(filtered_lines, &column_evaluator, index);
            index += 1;
            println!("lines: {:?}", filtered_lines.len());
            match filtered_lines.len() {
                0 => panic!(),
                1 => return filtered_lines[0],
                _ => continue
            }
        }
    }

    pub fn get_oxygen_generation(&self) -> u32 {
        Self::advance_diagnostic_values(self.lines.iter().collect(), Self::get_most_common_column_value).to_u32()
    }

    pub fn get_co2_scrubber_rating(&self) -> u32 {
        Self::advance_diagnostic_values(self.lines.iter().collect(), Self::get_least_common_column_value).to_u32()
    }
}
