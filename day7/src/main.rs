use crate::intcode_machine::{IntcodeMachine, IntcodeMachineError, State, Tape};
use itertools::enumerate;
use permutohedron::LexicalPermutation;

mod intcode_machine;
mod utils;

struct AmplifierPhaseSequence(Vec<usize>);

impl AmplifierPhaseSequence {
    fn new(v: Vec<usize>) -> Self {
        assert_eq!(5, v.len());
        Self(v)
    }

    fn test_sequence_with_state_dump(&self, tape: Tape) -> (isize, Vec<State>) {
        let mut output_signal = 0;
        let mut amp_states = vec![State::default(); 5];

        for (i, phase_seq) in enumerate(&self.0) {
            let input_string = format!("{}\n{}\n", phase_seq, output_signal);
            let input = input_string.as_bytes();
            let mut output_reader = Vec::new();

            match IntcodeMachine::new(tape.clone(), &input[..], &mut output_reader).run() {
                Ok(_) => (),
                Err(IntcodeMachineError::InputFailure(state)) => amp_states[i] = state,
                _ => panic!("unexpected machine failure"),
            };

            output_signal = String::from_utf8(output_reader)
                .unwrap()
                .parse::<isize>()
                .unwrap();
        }

        (output_signal, amp_states)
    }

    fn test_sequence(&self, tape: Tape) -> isize {
        let (output_signal, _) = self.test_sequence_with_state_dump(tape);
        output_signal
    }

    fn test_feedback_sequence(&self, tape: Tape) -> isize {
        let (mut output_signal, mut amp_states) = self.test_sequence_with_state_dump(tape);

        // main feedback loop
        let mut i = 0;
        loop {
            let input_string = format!("{}\n", output_signal);
            let input = input_string.as_bytes();
            let mut output_reader = Vec::new();

            let machine_output =
                IntcodeMachine::load_state(amp_states[i].clone(), &input[..], &mut output_reader)
                    .run();

            // get any outputs
            output_signal = String::from_utf8(output_reader)
                .unwrap()
                .parse::<isize>()
                .unwrap();

            match machine_output {
                // if amp E halted, return
                Ok(_) => {
                    if i == 4 {
                        break;
                    }
                }
                Err(IntcodeMachineError::InputFailure(state)) => amp_states[i] = state, // save current memory dump to resume later
                _ => panic!("unexpected machine failure"),
            };

            i = (i + 1) % 5;
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

fn do_part2(tape: Tape) {
    let mut highest_signal = 0;

    let mut data = [5, 6, 7, 8, 9];
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
        let amp_out = amp_seq.test_feedback_sequence(tape.clone());
        if amp_out > highest_signal {
            highest_signal = amp_out;
        }
    }

    println!("Part 2 answer: {}", highest_signal);
}

fn main() {
    let tape = Tape::new(utils::read_input_file("day7.input"));

    do_part1(tape.clone());
    do_part2(tape);
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

    #[test]
    fn it_produces_thruster_signal_139629729_from_feedback_seq_98765_with_sample_input() {
        let amp_seq = AmplifierPhaseSequence::new(vec![9, 8, 7, 6, 5]);
        let tape = Tape::new(vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);

        assert_eq!(139_629_729, amp_seq.test_feedback_sequence(tape));
    }

    #[test]
    fn it_produces_thruster_signal_18216_from_feedback_seq_97856_with_sample_input() {
        let amp_seq = AmplifierPhaseSequence::new(vec![9, 7, 8, 5, 6]);
        let tape = Tape::new(vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);

        assert_eq!(18216, amp_seq.test_feedback_sequence(tape));
    }
}
