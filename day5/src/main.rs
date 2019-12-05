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
}

impl From<TapeError> for OpCodeExecutionError {
    fn from(_: TapeError) -> Self {
        OpCodeExecutionError::TapeError
    }
}

trait OpCodeExecutor {
    fn execute(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError>;

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
}

struct AddOp {}

impl OpCodeExecutor for AddOp {
    fn execute(
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
}

struct MulOp {}

impl OpCodeExecutor for MulOp {
    fn execute(
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
}

struct InputOp {}

impl OpCodeExecutor for InputOp {
    fn execute(
        &self,
        tape: &mut Tape,
        head_position: usize,
        _param_modes: Vec<ParamMode>,
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
}

struct OutputOp {}

impl OpCodeExecutor for OutputOp {
    fn execute(
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

struct JumpTrueOp {}

impl OpCodeExecutor for JumpTrueOp {
    fn execute(
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
}

struct JumpFalseOp {}

impl OpCodeExecutor for JumpFalseOp {
    fn execute(
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
}

struct LessThanOp {}

impl OpCodeExecutor for LessThanOp {
    fn execute(
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
}

struct EqualsOp {}

impl OpCodeExecutor for EqualsOp {
    fn execute(
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
}

enum OpCode<Ex>
where
    Ex: OpCodeExecutor + ?Sized,
{
    Add(Box<Ex>, Vec<ParamMode>),
    Mul(Box<Ex>, Vec<ParamMode>),
    In(Box<Ex>),
    Out(Box<Ex>, Vec<ParamMode>),
    Jt(Box<Ex>, Vec<ParamMode>),
    Jf(Box<Ex>, Vec<ParamMode>),
    Lt(Box<Ex>, Vec<ParamMode>),
    Eq(Box<Ex>, Vec<ParamMode>),
    Halt,
    Err(isize),
}

//impl<Ex> OpCode<Ex>
//where
//    Ex: OpCodeExecutor,
//{
//
//}

impl From<isize> for OpCode<dyn OpCodeExecutor> {
    fn from(code: isize) -> Self {
        use OpCode::*;

        // make sure the opcode itself is positive, otherwise we have an invalid execution
        if code < 0 {
            return Err(code);
        }

        let digits = utils::num_to_digits_vec(code as usize);

        let reversed_padded_digits_iterator = std::iter::repeat(0).chain(digits.into_iter()).rev();

        let mut opcode_digits: Vec<_> = reversed_padded_digits_iterator.clone().take(2).collect();
        opcode_digits.reverse();
        let op_code_value = utils::digits_vec_to_num(&opcode_digits);

        // FIXME: this match is so nasty...
        match op_code_value as isize {
            ADD_OP_CODE => {
                let param_modes_vec: Vec<_> = reversed_padded_digits_iterator
                    .skip(2)
                    .take(3)
                    .map(|x| ParamMode::try_from(x).unwrap())
                    .collect();

                assert_eq!(3, param_modes_vec.len());
                // "Parameters that an instruction writes to will never be in immediate mode."
                assert_eq!(param_modes_vec[2], ParamMode::Position);
                Add(Box::new(AddOp {}), param_modes_vec)
            }
            MUL_OP_CODE => {
                let param_modes_vec: Vec<_> = reversed_padded_digits_iterator
                    .skip(2)
                    .take(3)
                    .map(|x| ParamMode::try_from(x).unwrap())
                    .collect();

                assert_eq!(3, param_modes_vec.len());
                // "Parameters that an instruction writes to will never be in immediate mode."
                assert_eq!(param_modes_vec[2], ParamMode::Position);

                Mul(Box::new(MulOp {}), param_modes_vec)
            }
            JMP_TRUE_OP_CODE => {
                let param_modes_vec: Vec<_> = reversed_padded_digits_iterator
                    .skip(2)
                    .take(2)
                    .map(|x| ParamMode::try_from(x).unwrap())
                    .collect();

                assert_eq!(2, param_modes_vec.len());

                Jt(Box::new(JumpTrueOp {}), param_modes_vec)
            }
            JMP_FALSE_OP_CODE => {
                let param_modes_vec: Vec<_> = reversed_padded_digits_iterator
                    .skip(2)
                    .take(2)
                    .map(|x| ParamMode::try_from(x).unwrap())
                    .collect();

                assert_eq!(2, param_modes_vec.len());

                Jf(Box::new(JumpFalseOp {}), param_modes_vec)
            }
            LESS_THAN_OP_CODE => {
                let param_modes_vec: Vec<_> = reversed_padded_digits_iterator
                    .skip(2)
                    .take(3)
                    .map(|x| ParamMode::try_from(x).unwrap())
                    .collect();

                assert_eq!(3, param_modes_vec.len());

                Lt(Box::new(LessThanOp {}), param_modes_vec)
            }
            EQUALS_OP_CODE => {
                let param_modes_vec: Vec<_> = reversed_padded_digits_iterator
                    .skip(2)
                    .take(3)
                    .map(|x| ParamMode::try_from(x).unwrap())
                    .collect();

                assert_eq!(3, param_modes_vec.len());

                Eq(Box::new(EqualsOp {}), param_modes_vec)
            }
            INPUT_OP_CODE => In(Box::new(InputOp {})),
            OUTPUT_OP_CODE => {
                let param_mode_vec: Vec<_> = reversed_padded_digits_iterator
                    .skip(2)
                    .take(1)
                    .map(|x| ParamMode::try_from(x).unwrap())
                    .collect();

                assert_eq!(1, param_mode_vec.len());

                Out(Box::new(OutputOp {}), param_mode_vec)
            }
            HALT_OP_CODE => Halt,
            _ => Err(code),
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
            let (op, modes) = match OpCode::from(self.tape.read(self.head_position)?) {
                OpCode::Err(_) => return Err(IntcodeMachineError::ExecutionFailure),
                OpCode::Halt => {
                    self.output = self.tape.read(0)?;
                    return Ok(self.output);
                }
                OpCode::Add(op, modes) => (op, modes),
                OpCode::Mul(op, modes) => (op, modes),
                OpCode::Jt(op, modes) => (op, modes),
                OpCode::Jf(op, modes) => (op, modes),
                OpCode::Lt(op, modes) => (op, modes),
                OpCode::Eq(op, modes) => (op, modes),
                OpCode::In(op) => (op, vec![]),
                OpCode::Out(op, modes) => (op, modes),
            };

            let head_update = op.execute(&mut self.tape, self.head_position, modes)?;
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
    let tape = Tape::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
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
                OpCode::Add(_, param_vec) => {
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
                OpCode::Add(_, param_vec) => {
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
                OpCode::Add(_, param_vec) => {
                    assert_eq!(ParamMode::Immediate, param_vec[0]);
                    assert_eq!(ParamMode::Immediate, param_vec[1]);
                    assert_eq!(ParamMode::Position, param_vec[2]);
                }
                _ => panic!("expected Add"),
            }
        }
    }
}
