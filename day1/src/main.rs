use std::fs::File;
use std::io::{BufRead, BufReader};

struct FuelCalculator {}

impl FuelCalculator {
    fn required_fuel(mass: u64) -> Option<u64> {
        match (mass as f64 / 3.0).floor() - 2.0 {
            f if f <= 0.0 => None,
            f => Some(f as u64),
        }
    }
}

trait Fuelable {
    fn calculate_base_required_fuel(&self) -> u64;
    fn calculate_total_required_fuel(&self) -> u64;
}

struct Module {
    mass: u64,
}

impl Module {
    fn new(mass: u64) -> Self {
        Self { mass }
    }
}

impl Fuelable for Module {
    fn calculate_base_required_fuel(&self) -> u64 {
        FuelCalculator::required_fuel(self.mass).unwrap()
    }

    fn calculate_total_required_fuel(&self) -> u64 {
        let f = FuelCalculator::required_fuel;
        std::iter::successors(f(self.mass), |x| f(*x)).sum()
    }
}

struct FuelUpper {}

impl FuelUpper {
    fn determine_total_required_base_fuel<F: Fuelable>(fuelables: &[F]) -> u64 {
        fuelables
            .iter()
            .map(|f| f.calculate_base_required_fuel())
            .sum()
    }

    fn determine_total_required_fuel<F: Fuelable>(fuelables: &[F]) -> u64 {
        fuelables
            .iter()
            .map(|f| f.calculate_total_required_fuel())
            .sum()
    }
}

fn read_input_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut inputs = vec![];
    for line in reader.lines() {
        inputs.push(line.unwrap());
    }

    inputs
}

fn input_to_modules(inputs: Vec<String>) -> Vec<Module> {
    inputs
        .iter()
        .map(|i| i.parse::<u64>().unwrap())
        .map(Module::new)
        .collect()
}

fn do_part1(inputs_modules: &[Module]) {
    let required_base_fuel = FuelUpper::determine_total_required_base_fuel(inputs_modules);
    println!("Part 1 answer: {}", required_base_fuel);
}

fn do_part2(input_modules: &[Module]) {
    let required_total_fuel = FuelUpper::determine_total_required_fuel(input_modules);
    println!("Part 2 answer: {}", required_total_fuel);
}

fn main() {
    let day1_input = read_input_file("day1.input");
    let modules = input_to_modules(day1_input);
    do_part1(&modules);
    do_part2(&modules);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_calculates_correct_base_fuel_for_mass_of_12() {
        assert_eq!(2, Module::new(12).calculate_base_required_fuel(),);
    }

    #[test]
    fn module_calculates_correct_base_fuel_for_mass_of_14() {
        assert_eq!(2, Module::new(14).calculate_base_required_fuel());
    }

    #[test]
    fn module_calculates_correct_base_fuel_for_mass_of_1969() {
        assert_eq!(654, Module::new(1969).calculate_base_required_fuel());
    }

    #[test]
    fn module_calculates_correct_base_fuel_for_mass_of_100756() {
        assert_eq!(33583, Module::new(100_756).calculate_base_required_fuel());
    }

    #[test]
    fn module_calculates_correct_total_fuel_for_mass_of_14() {
        assert_eq!(2, Module::new(14).calculate_total_required_fuel());
    }

    #[test]
    fn module_calculates_correct_total_fuel_for_mass_of_1969() {
        assert_eq!(966, Module::new(1969).calculate_total_required_fuel());
    }

    #[test]
    fn module_calculates_correct_total_fuel_for_mass_of_100756() {
        assert_eq!(50346, Module::new(100_756).calculate_total_required_fuel());
    }
}
