use crate::intcode_machine::{IntcodeMachine, Tape};
use permutohedron::LexicalPermutation;
mod intcode_machine;
mod utils;

struct AmplifierPhaseSequence(Vec<usize>);

impl AmplifierPhaseSequence {
    fn new(v: Vec<usize>) -> Self {
        assert_eq!(5, v.len());
        Self(v)
    }

    fn test_sequence(&self, tape: Tape) -> isize {
        let mut output_signal = 0;

        for phase_seq in &self.0 {
            let input_string = format!("{}\n{}\n", phase_seq, output_signal);
            let input = input_string.as_bytes();
            let mut output_reader = Vec::new();

            IntcodeMachine::new(tape.clone(), &input[..], &mut output_reader)
                .run()
                .unwrap();

            output_signal = String::from_utf8(output_reader)
                .unwrap()
                .parse::<isize>()
                .unwrap();
        }

        output_signal
    }
}

fn do_part1(tape: Tape) {
    let mut highest_signal = 0;

    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    loop {
        permutations.push(data.to_vec());
        if !data.next_permutation() {
            break;
        }
    }

    for perm in permutations {
        println!("testing permutation: {:?}", perm);
        let amp_seq = AmplifierPhaseSequence::new(perm);
        let amp_out = amp_seq.test_sequence(tape.clone());
        if amp_out > highest_signal {
            highest_signal = amp_out;
        }
    }

    println!("Part 1 answer: {}", highest_signal);
}

fn main() {
    let tape = Tape::new(utils::read_input_file("day7.input"));

    do_part1(tape);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_produces_thruster_signal_43210_from_seq_43210_with_sample_input() {
        let amp_seq = AmplifierPhaseSequence::new(vec![4, 3, 2, 1, 0]);
        let tape = Tape::new(vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);

        assert_eq!(43210, amp_seq.test_sequence(tape));
    }

    #[test]
    fn it_produces_thruster_signal_54321_from_seq_01234_with_sample_input() {
        let amp_seq = AmplifierPhaseSequence::new(vec![0, 1, 2, 3, 4]);
        let tape = Tape::new(vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);

        assert_eq!(54321, amp_seq.test_sequence(tape));
    }

    #[test]
    fn it_produces_thruster_signal_65210_from_seq_10432_with_sample_input() {
        let amp_seq = AmplifierPhaseSequence::new(vec![1, 0, 4, 3, 2]);
        let tape = Tape::new(vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);

        assert_eq!(65210, amp_seq.test_sequence(tape));
    }
}
