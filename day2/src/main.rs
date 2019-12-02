use crate::State::{Error, Halted, Running};
use std::fs;

struct IntcodeMachine {
    tape: Vec<usize>,
    head_position: usize,
    output: usize,
}

enum State {
    Running,
    Halted,
    Error,
}

impl IntcodeMachine {
    fn new(tape: Vec<usize>) -> Self {
        Self {
            tape,
            head_position: 0,
            output: 0,
        }
    }

    fn run(&mut self) -> usize {
        loop {
            match self.step_through() {
                Running => {} // continue execution
                Halted => return self.output,
                Error => panic!("intcode machine ended in invalid state"),
            }
            self.advance_head();
        }
    }

    fn advance_head(&mut self) {
        assert!(self.head_position + 4 < self.tape.len());
        self.head_position += 4
    }

    fn step_through(&mut self) -> State {
        match self.tape[self.head_position] {
            1 => {
                self.add_op();
                Running
            }
            2 => {
                self.mul_op();
                Running
            }
            99 => {
                self.halt_op();
                Halted
            }
            _ => Error,
        }
    }

    fn add_op(&mut self) {
        let input1_idx = self.tape[self.head_position + 1];
        let input2_idx = self.tape[self.head_position + 2];
        let output_idx = self.tape[self.head_position + 3];
        self.tape[output_idx] = self.tape[input1_idx] + self.tape[input2_idx];
    }

    fn mul_op(&mut self) {
        let input1_idx = self.tape[self.head_position + 1];
        let input2_idx = self.tape[self.head_position + 2];
        let output_idx = self.tape[self.head_position + 3];
        self.tape[output_idx] = self.tape[input1_idx] * self.tape[input2_idx];
    }

    fn halt_op(&mut self) {
        self.output = self.tape[0]
    }
}

fn read_input_file(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn part1_preparation(input: Vec<usize>) -> Vec<usize> {
    input
        .iter()
        .cloned()
        .take(1)
        .chain(vec![12usize, 2usize])
        .chain(input.iter().cloned().skip(3))
        .collect()
}

fn do_part1(input: Vec<usize>) {
    let prepared_tape = part1_preparation(input);
    println!(
        "Part 1 answer: {}",
        IntcodeMachine::new(prepared_tape).run()
    );
}

fn main() {
    let day1_input = read_input_file("day2.input");
    do_part1(day1_input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod intcode_machine {
        use super::*;

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode1() {
            assert_eq!(2, IntcodeMachine::new(vec![1, 0, 0, 0, 99]).run())
        }

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode2() {
            assert_eq!(2, IntcodeMachine::new(vec![2, 3, 0, 3, 99]).run())
        }

        #[test]
        fn produces_expected_output_for_average_size_input() {
            assert_eq!(2, IntcodeMachine::new(vec![2, 4, 4, 5, 99, 0]).run())
        }

        #[test]
        fn produces_expected_output_for_longer_input() {
            assert_eq!(
                30,
                IntcodeMachine::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).run()
            )
        }

        #[test]
        fn produces_expected_output_for_a_lengthy_input() {
            assert_eq!(
                3500,
                IntcodeMachine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).run()
            )
        }
    }
}
