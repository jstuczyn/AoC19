use itertools::Itertools;
use std::fs;
use std::iter::successors;

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
                State::Running => {} // continue execution
                State::Halted => return self.output,
                State::Error => panic!("intcode machine ended in an invalid state"),
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
                State::Running
            }
            2 => {
                self.mul_op();
                State::Running
            }
            99 => {
                self.halt_op();
                State::Halted
            }
            _ => State::Error,
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

fn do_part2(input: Vec<usize>) {
    // bruteforce possible noun, verb pairs
    // an alternative would be to reverse engineer the machine execution
    // or implement something like SAT solver
    // But even puzzle authors imply you should just try to bruteforce

    let part2_answer_vec: Vec<usize> = successors(Some(0usize), |x| Some(*x + 1))
        .take(100)
        .permutations(2)
        .map(|noun_verb_vec| {
            let noun = noun_verb_vec[0];
            let verb = noun_verb_vec[1];
            let machine_input: Vec<usize> = input
                .iter()
                .cloned()
                .take(1)
                .chain(vec![noun, verb])
                .chain(input.iter().cloned().skip(3))
                .collect();
            (noun, verb, IntcodeMachine::new(machine_input).run())
        })
        .skip_while(|(_, _, output)| *output != 19_690_720)
        .map(|(noun, verb, _)| 100 * noun + verb)
        .take(1)
        .collect();

    println!("Part 2 answer: {:?}", part2_answer_vec.first().unwrap());
}

fn main() {
    let day1_input = read_input_file("day2.input");
    do_part1(day1_input.clone());
    do_part2(day1_input.clone());
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
