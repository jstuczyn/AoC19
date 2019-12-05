use std::fs;

struct Tape(Vec<isize>);

impl Tape {
    fn new(input: Vec<isize>) -> Self {
        Tape(input)
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
