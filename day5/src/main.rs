use std::fs;

enum OpCode {
    Add,
    Mul,
    //    Input,
    //    Output,
    Halt,
    Err,
}

impl OpCode {
    fn new(val: isize) -> Self {
        use OpCode::*;
        match val {
            1 => Add,
            2 => Mul,
            99 => Halt,
            _ => Err,
        }
    }
}

#[derive(Debug)]
enum TapeError {
    WriteOutOfRangeError,
    ReadOutOfRangeError,
}

struct Tape(Vec<isize>);

impl Tape {
    fn new(input: Vec<isize>) -> Self {
        Tape(input)
    }

    fn write(&mut self, value: isize, position: usize) -> Result<(), TapeError> {
        if self.0.len() < position {
            return Err(TapeError::WriteOutOfRangeError);
        }

        self.0[position] = value;
        Ok(())
    }

    fn read(&self, position: usize) -> Result<isize, TapeError> {
        if self.0.len() < position {
            return Err(TapeError::ReadOutOfRangeError);
        }

        Ok(self.0[position])
    }
}

struct IntcodeMachine {
    tape: Tape,
    head_position: usize,
    output: isize,
}

impl IntcodeMachine {
    fn new(tape: Tape) -> Self {
        IntcodeMachine {
            tape,
            head_position: 0,
            output: 0,
        }
    }

    fn run(&self) -> isize {
        0
    }
}

fn read_input_file(path: &str) -> Vec<isize> {
    fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn do_part1(tape: Tape) {
    println!("Part 1 answer: {}", IntcodeMachine::new(tape).run());
}

fn main() {
    let tape = Tape::new(read_input_file("day5.input"));
    do_part1(tape);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod day2_intcode_machine_reimplementation {
        use super::*;

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode1() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![1, 0, 0, 0, 99])).run()
            )
        }

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode2() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![2, 3, 0, 3, 99])).run()
            )
        }

        #[test]
        fn produces_expected_output_for_average_size_input() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![2, 4, 4, 5, 99, 0])).run()
            )
        }

        #[test]
        fn produces_expected_output_for_longer_input() {
            assert_eq!(
                30,
                IntcodeMachine::new(Tape::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])).run()
            )
        }

        #[test]
        fn produces_expected_output_for_a_lengthy_input() {
            assert_eq!(
                3500,
                IntcodeMachine::new(Tape::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]))
                    .run()
            )
        }
    }
}
