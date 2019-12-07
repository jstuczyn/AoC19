use crate::intcode_machine::{IntcodeMachine, Tape};
use std::io;
use std::io::{BufRead, Write};

mod intcode_machine;
mod utils;

fn run_machine<R, W>(tape: Tape, reader: R, writer: W)
where
    R: BufRead,
    W: Write,
{
    // answer will be printed (as per specs) to output (here STDOUT)
    // part1 requires input of 1, part2 of 5
    IntcodeMachine::new(tape, reader, writer).run().unwrap();
}

fn main() {
    let tape = Tape::new(utils::read_input_file("day5.input"));

    let stdio = io::stdin();
    let input = stdio.lock();
    let output = io::stdout();

    run_machine(tape, input, output);
}
