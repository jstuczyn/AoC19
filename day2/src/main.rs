use std::fs;

// The below code could be made slightly nicer by introducing Tape type and defining methods on it.
// And also by properly defining OpCodes and operations on them
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

fn prepare_tape(input_tape: Vec<usize>, subs: (usize, usize)) -> Vec<usize> {
    input_tape
        .iter()
        .cloned()
        .take(1)
        .chain(vec![subs.0, subs.1])
        .chain(input_tape.iter().cloned().skip(3))
        .collect()
}

fn do_part1(input: Vec<usize>) {
    let prepared_tape = prepare_tape(input, (12, 2));
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
    let part2_answer_vec: Vec<usize> = (0..99)
        .flat_map(|noun| (0..99).map(move |verb| (noun, verb)))
        .map(|noun_verb_pair| {
            let machine_input = prepare_tape(input.clone(), noun_verb_pair);
            (noun_verb_pair, IntcodeMachine::new(machine_input).run())
        })
        .skip_while(|(_, output)| *output != 19_690_720)
        .map(|(noun_verb_pair, _)| 100 * noun_verb_pair.0 + noun_verb_pair.1)
        .take(1)
        .collect();

    println!("Part 2 answer: {:?}", part2_answer_vec.first().unwrap());
}

fn main() {
    let day2_input = read_input_file("day2.input");
    do_part1(day2_input.clone());
    do_part2(day2_input.clone());
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
