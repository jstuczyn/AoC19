use crate::intcode_machine::{IntcodeMachine, Tape};

mod intcode_machine;
mod utils;

fn do_part1(tape: Tape) {
    let fake_input = b"1";
    let mut output = Vec::new();

    IntcodeMachine::new(tape, &fake_input[..], &mut output)
        .run()
        .unwrap();

    let parsed_output = utils::parse_multiple_utf8_num_repr_lns(&output)
        .last()
        .unwrap()
        .to_owned();

    println!("{:?}", parsed_output);
}

fn do_part2(tape: Tape) {
    let fake_input = b"2";
    let mut output = Vec::new();

    IntcodeMachine::new(tape, &fake_input[..], &mut output)
        .run()
        .unwrap();

    let parsed_output = utils::parse_multiple_utf8_num_repr_lns(&output)
        .last()
        .unwrap()
        .to_owned();

    println!("{:?}", parsed_output);
}

fn main() {
    let tape = Tape::new(utils::read_input_file("day9.input"));

    do_part1(tape.clone());
    do_part2(tape);
}
