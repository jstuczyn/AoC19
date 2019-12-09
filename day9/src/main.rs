use crate::intcode_machine::{IntcodeMachine, IntcodeMachineError, State, Tape};

mod intcode_machine;
mod utils;

fn do_part1(tape: Tape) {}

fn do_part2(tape: Tape) {}

fn main() {
    let tape = Tape::new(utils::read_input_file("day9.input"));

    do_part1(tape.clone());
    do_part2(tape);
}
