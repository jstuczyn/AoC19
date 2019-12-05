mod utils;

use std::convert::TryFrom;
use std::fs;

const ADD_OP_CODE: isize = 1;
const MUL_OP_CODE: isize = 2;
const INPUT_OP_CODE: isize = 3;
const OUTPUT_OP_CODE: isize = 4;
const JMP_TRUE_OP_CODE: isize = 5;
const JMP_FALSE_OP_CODE: isize = 6;
const LESS_THAN_OP_CODE: isize = 7;
const EQUALS_OP_CODE: isize = 8;
const HALT_OP_CODE: isize = 99;

const POSITION_MODE: usize = 0;
const IMMEDIATE_MODE: usize = 1;

// TODO: in hindsight stdout and stdio should have been injected with dependency injection to be
// able to actually test OP3 and OP4

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
}

impl TryFrom<usize> for ParamMode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use ParamMode::*;

        match value {
            POSITION_MODE => Ok(Position),
            IMMEDIATE_MODE => Ok(Immediate),
            _ => Err(()),
        }
    }
}

type HeadPositionUpdate = usize;

#[derive(Debug)]
enum OpCodeExecutionError {
    TapeError,
    InvalidOpArguments,
    ExecutionFailure,
    ExecutionFinished,
}

impl From<TapeError> for OpCodeExecutionError {
    fn from(_: TapeError) -> Self {
        OpCodeExecutionError::TapeError
    }
}

enum OpCode {
    Add(Vec<ParamMode>),
    Mul(Vec<ParamMode>),
    In,
    Out(Vec<ParamMode>),
    Jt(Vec<ParamMode>),
    Jf(Vec<ParamMode>),
    Lt(Vec<ParamMode>),
    Eq(Vec<ParamMode>),
    Halt,
    Er(isize),
}

impl OpCode {
    fn execute(
        &self,
        tape: &mut Tape,
        head_position: usize,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        use OpCode::*;
        match self {
            Add(param_modes) => self.execute_add(tape, head_position, param_modes.clone()),
            Mul(param_modes) => self.execute_mul(tape, head_position, param_modes.clone()),
            Jt(param_modes) => self.execute_less_than(tape, head_position, param_modes.clone()),
            Jf(param_modes) => self.execute_jump_false(tape, head_position, param_modes.clone()),
            Lt(param_modes) => self.execute_jump_true(tape, head_position, param_modes.clone()),
            Eq(param_modes) => self.execute_equals(tape, head_position, param_modes.clone()),

            In => self.execute_input(tape, head_position),
            Out(param_modes) => self.execute_output(tape, head_position, param_modes.clone()),

            Halt => Err(OpCodeExecutionError::ExecutionFinished),
            Er(_) => Err(OpCodeExecutionError::ExecutionFailure),
        }
    }

    fn get_param_value(
        &self,
        tape: &Tape,
        literal_value: isize,
        param_mode: ParamMode,
    ) -> Result<isize, OpCodeExecutionError> {
        match param_mode {
            ParamMode::Position => {
                if literal_value < 0 {
                    Err(OpCodeExecutionError::InvalidOpArguments)
                } else {
                    Ok(tape.read(literal_value as usize)?)
                }
            }
            ParamMode::Immediate => Ok(literal_value),
        }
    }

    fn execute_add(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        // read input literals
        let input1_val = tape.read(head_position + 1)?;
        let input2_val = tape.read(head_position + 2)?;
        let output_idx = tape.read(head_position + 3)?;

        let result = self.get_param_value(tape, input1_val, param_modes[0])?
            + self.get_param_value(tape, input2_val, param_modes[1])?;

        // finally write the result back to the tape
        tape.write(output_idx as usize, result)?;

        Ok(head_position + 4)
    }

    fn execute_mul(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        // read input literals
        let input1_val = tape.read(head_position + 1)?;
        let input2_val = tape.read(head_position + 2)?;
        let output_idx = tape.read(head_position + 3)?;

        let result = self.get_param_value(tape, input1_val, param_modes[0])?
            * self.get_param_value(tape, input2_val, param_modes[1])?;

        // finally write the result back to the tape
        tape.write(output_idx as usize, result)?;

        Ok(head_position + 4)
    }

    fn execute_less_than(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let arg1 = tape.read(head_position + 1)?;
        let param1 = self.get_param_value(tape, arg1, param_modes[0])?;

        let arg2 = tape.read(head_position + 2)?;
        let param2 = self.get_param_value(tape, arg2, param_modes[1])?;

        let store_target = tape.read(head_position + 3)?;

        if param1 < param2 {
            tape.write(store_target as usize, 1)?;
        } else {
            tape.write(store_target as usize, 0)?;
        }

        Ok(head_position + 4)
    }

    fn execute_jump_true(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let arg = tape.read(head_position + 1)?;
        let param = self.get_param_value(tape, arg, param_modes[0])?;

        let jump_target_val = tape.read(head_position + 2)?;
        let jump_target = self.get_param_value(tape, jump_target_val, param_modes[1])?;

        if param != 0 {
            Ok(jump_target as usize)
        } else {
            Ok(head_position + 3)
        }
    }

    fn execute_jump_false(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let arg = tape.read(head_position + 1)?;
        let param = self.get_param_value(tape, arg, param_modes[0])?;

        let jump_target_val = tape.read(head_position + 2)?;
        let jump_target = self.get_param_value(tape, jump_target_val, param_modes[1])?;

        if param == 0 {
            Ok(jump_target as usize)
        } else {
            Ok(head_position + 3)
        }
    }

    fn execute_equals(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let arg1 = tape.read(head_position + 1)?;
        let param1 = self.get_param_value(tape, arg1, param_modes[0])?;

        let arg2 = tape.read(head_position + 2)?;
        let param2 = self.get_param_value(tape, arg2, param_modes[1])?;

        let store_target = tape.read(head_position + 3)?;

        if param1 == param2 {
            tape.write(store_target as usize, 1)?;
        } else {
            tape.write(store_target as usize, 0)?;
        }

        Ok(head_position + 4)
    }

    fn execute_input(
        &self,
        tape: &mut Tape,
        head_position: usize,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        // read input literals
        let output_idx = tape.read(head_position + 1)?;

        // Read the user input
        println!("Provide the system required input...");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let input_value = buffer.trim().parse::<isize>().unwrap();

        // finally write the result back to the tape
        tape.write(output_idx as usize, input_value)?;

        Ok(head_position + 2)
    }

    fn execute_output(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let output_source_idx = tape.read(head_position + 1)?;
        let output_val = self.get_param_value(tape, output_source_idx, param_modes[0])?;
        println!("Test result: {}", output_val);
        Ok(head_position + 2)
    }
}

impl From<isize> for OpCode {
    fn from(code: isize) -> Self {
        use OpCode::*;

        // make sure the opcode itself is positive, otherwise we have an invalid execution
        if code < 0 {
            return Er(code);
        }

        let digits = utils::num_to_digits_vec(code as usize);

        let mut opcode_digits: Vec<_> = std::iter::repeat(0)
            .chain(digits.clone().into_iter())
            .rev()
            .take(2)
            .collect();
        opcode_digits.reverse();
        let op_code_value = utils::digits_vec_to_num(&opcode_digits);

        let num_args = match op_code_value as isize {
            ADD_OP_CODE => 3,
            MUL_OP_CODE => 3,
            JMP_TRUE_OP_CODE => 2,
            JMP_FALSE_OP_CODE => 2,
            LESS_THAN_OP_CODE => 3,
            EQUALS_OP_CODE => 3,
            INPUT_OP_CODE => 0,
            OUTPUT_OP_CODE => 1,
            HALT_OP_CODE => 0,
            _ => 0,
        };

        let param_modes_vec: Vec<_> = std::iter::repeat(0)
            .chain(digits.into_iter())
            .rev()
            .skip(2)
            .take(num_args)
            .map(|x| ParamMode::try_from(x).unwrap())
            .collect();

        match op_code_value as isize {
            ADD_OP_CODE => Add(param_modes_vec),
            MUL_OP_CODE => Mul(param_modes_vec),
            JMP_TRUE_OP_CODE => Jt(param_modes_vec),
            JMP_FALSE_OP_CODE => Jf(param_modes_vec),
            LESS_THAN_OP_CODE => Lt(param_modes_vec),
            EQUALS_OP_CODE => Eq(param_modes_vec),
            INPUT_OP_CODE => In,
            OUTPUT_OP_CODE => Out(param_modes_vec),
            HALT_OP_CODE => Halt,
            _ => Er(code),
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

    fn len(&self) -> usize {
        self.0.len()
    }

    fn write(&mut self, position: usize, value: isize) -> Result<(), TapeError> {
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

#[derive(Debug)]
enum IntcodeMachineError {
    TapeOutOfBoundsError,
    ExecutionFailure,
}

impl From<TapeError> for IntcodeMachineError {
    fn from(_: TapeError) -> Self {
        IntcodeMachineError::TapeOutOfBoundsError
    }
}

impl From<OpCodeExecutionError> for IntcodeMachineError {
    fn from(_: OpCodeExecutionError) -> Self {
        IntcodeMachineError::ExecutionFailure
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

    fn advance_head(&mut self, val: HeadPositionUpdate) -> Result<(), IntcodeMachineError> {
        // check if new head is within 0..tape.len()
        if !(0..self.tape.len()).contains(&val) {
            return Err(IntcodeMachineError::TapeOutOfBoundsError);
        }

        self.head_position = val;
        Ok(())
    }

    fn run(&mut self) -> Result<isize, IntcodeMachineError> {
        loop {
            let op = OpCode::from(self.tape.read(self.head_position)?);
            let head_update = match op.execute(&mut self.tape, self.head_position) {
                Err(err) => match (err) {
                    OpCodeExecutionError::ExecutionFinished => {
                        self.output = self.tape.read(0)?;
                        return Ok(self.output);
                    }
                    _ => return Err(IntcodeMachineError::ExecutionFailure),
                },
                Ok(head_update) => head_update,
            };

            self.advance_head(head_update)?;
        }
    }
}

fn read_input_file(path: &str) -> Vec<isize> {
    fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn run_machine(tape: Tape) {
    // answer will be printed (as per specs) to output (here STDOUT)
    // part1 requires input of 1, part2 of 5
    IntcodeMachine::new(tape).run().unwrap();
}

fn main() {
    let tape = Tape::new(read_input_file("day5.input"));
    run_machine(tape);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_works_on_negative_values() {
        assert_eq!(
            1101,
            IntcodeMachine::new(Tape::new(vec![1101, 100, -1, 4, 0]))
                .run()
                .unwrap()
        );
    }

    #[cfg(test)]
    mod day2_intcode_machine_reimplementation {
        use super::*;

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode1() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![1, 0, 0, 0, 99]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode2() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![2, 3, 0, 3, 99]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_average_size_input() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![2, 4, 4, 5, 99, 0]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_longer_input() {
            assert_eq!(
                30,
                IntcodeMachine::new(Tape::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_a_lengthy_input() {
            assert_eq!(
                3500,
                IntcodeMachine::new(Tape::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_day2_input() {
            let mut day2_tape = Tape::new(read_input_file("day2.input"));
            // do the substitutions
            day2_tape.0[1] = 12;
            day2_tape.0[2] = 2;
            assert_eq!(4_138_687, IntcodeMachine::new(day2_tape).run().unwrap())
        }
    }

    #[cfg(test)]
    mod opcode_parsing {
        use super::*;

        #[test]
        fn works_for_basic_addition() {
            match OpCode::from(1) {
                OpCode::Add(param_vec) => {
                    assert_eq!(ParamMode::Position, param_vec[0]);
                    assert_eq!(ParamMode::Position, param_vec[1]);
                    assert_eq!(ParamMode::Position, param_vec[2]);
                }

                _ => panic!("expected Add"),
            }
        }

        #[test]
        fn works_for_basic_addition_with_zero_prefix() {
            match OpCode::from(101) {
                OpCode::Add(param_vec) => {
                    assert_eq!(ParamMode::Immediate, param_vec[0]);
                    assert_eq!(ParamMode::Position, param_vec[1]);
                    assert_eq!(ParamMode::Position, param_vec[2]);
                }
                _ => panic!("expected Add"),
            }
        }

        #[test]
        fn work_for_addition_with_implicit_mode() {
            match OpCode::from(1101) {
                OpCode::Add(param_vec) => {
                    assert_eq!(ParamMode::Immediate, param_vec[0]);
                    assert_eq!(ParamMode::Immediate, param_vec[1]);
                    assert_eq!(ParamMode::Position, param_vec[2]);
                }
                _ => panic!("expected Add"),
            }
        }
    }
}
