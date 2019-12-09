use std::convert::TryFrom;
use std::io::{BufRead, Write};

use crate::utils;

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
    InputFailure,
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
    fn execute<R, W>(
        &self,
        tape: &mut Tape,
        head_position: usize,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError>
    where
        R: BufRead,
        W: Write,
    {
        use OpCode::*;
        match self {
            Add(param_modes) => self.execute_add(tape, head_position, param_modes.clone()),
            Mul(param_modes) => self.execute_mul(tape, head_position, param_modes.clone()),
            Jt(param_modes) => self.execute_jump_true(tape, head_position, param_modes.clone()),
            Jf(param_modes) => self.execute_jump_false(tape, head_position, param_modes.clone()),
            Lt(param_modes) => self.execute_less_than(tape, head_position, param_modes.clone()),
            Eq(param_modes) => self.execute_equals(tape, head_position, param_modes.clone()),

            In => self.execute_input(tape, head_position, reader),
            Out(param_modes) => {
                self.execute_output(tape, head_position, param_modes.clone(), writer)
            }

            Halt => Err(OpCodeExecutionError::ExecutionFinished),
            Er(_) => Err(OpCodeExecutionError::ExecutionFailure),
        }
    }

    fn mode_tape_read(
        &self,
        tape: &Tape,
        tape_idx: usize,
        param_mode: ParamMode,
    ) -> Result<isize, OpCodeExecutionError> {
        let literal_value = tape.read(tape_idx)?;
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
        let result = self.mode_tape_read(tape, head_position + 1, param_modes[0])?
            + self.mode_tape_read(tape, head_position + 2, param_modes[1])?;

        let output_idx = tape.read(head_position + 3)?;
        tape.write(output_idx as usize, result)?;

        Ok(head_position + 4)
    }

    fn execute_mul(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let result = self.mode_tape_read(tape, head_position + 1, param_modes[0])?
            * self.mode_tape_read(tape, head_position + 2, param_modes[1])?;

        let output_idx = tape.read(head_position + 3)?;
        tape.write(output_idx as usize, result)?;

        Ok(head_position + 4)
    }

    fn execute_less_than(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param1 = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        let param2 = self.mode_tape_read(tape, head_position + 2, param_modes[1])?;
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
        let param = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        let jump_target = self.mode_tape_read(tape, head_position + 2, param_modes[1])?;

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
        let param = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        let jump_target = self.mode_tape_read(tape, head_position + 2, param_modes[1])?;

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
        let param1 = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        let param2 = self.mode_tape_read(tape, head_position + 2, param_modes[1])?;
        let store_target = tape.read(head_position + 3)?;

        if param1 == param2 {
            tape.write(store_target as usize, 1)?;
        } else {
            tape.write(store_target as usize, 0)?;
        }

        Ok(head_position + 4)
    }

    fn execute_input<R>(
        &self,
        tape: &mut Tape,
        head_position: usize,
        mut reader: R,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError>
    where
        R: BufRead,
    {
        let output_idx = tape.read(head_position + 1)?;

        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let input_value = match buffer.trim().parse::<isize>() {
            Ok(val) => val,
            _ => return Err(OpCodeExecutionError::InputFailure),
        };

        tape.write(output_idx as usize, input_value)?;

        Ok(head_position + 2)
    }

    fn execute_output<W>(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
        mut writer: W,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError>
    where
        W: Write,
    {
        let output_val = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        write!(&mut writer, "{}", output_val).unwrap();
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

#[derive(Debug, Clone)]
pub struct Tape(Vec<isize>);

impl Tape {
    pub(crate) fn new(input: Vec<isize>) -> Self {
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
pub enum IntcodeMachineError {
    TapeOutOfBoundsError,
    ExecutionFailure,
    InputFailure(State),
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

#[derive(Debug, Clone)]
pub struct State {
    tape: Tape,
    head_position: usize,
}

impl State {
    pub fn new_from_tape(tape: Tape) -> Self {
        State {
            tape,
            head_position: 0,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            tape: Tape(Vec::new()),
            head_position: 0,
        }
    }
}

pub struct IntcodeMachine<R, W>
where
    R: BufRead,
    W: Write,
{
    tape: Tape,
    head_position: usize,
    input: R,
    output: W,
}

impl<R, W> IntcodeMachine<R, W>
where
    R: BufRead,
    W: Write,
{
    pub(crate) fn new(tape: Tape, reader: R, writer: W) -> Self {
        IntcodeMachine {
            tape,
            head_position: 0,
            input: reader,
            output: writer,
        }
    }

    pub fn load_state(state: State, reader: R, writer: W) -> Self {
        IntcodeMachine {
            tape: state.tape,
            head_position: state.head_position,
            input: reader,
            output: writer,
        }
    }

    pub fn dump_state(&self) -> State {
        State {
            tape: self.tape.clone(),
            head_position: self.head_position,
        }
    }

    fn update_head(&mut self, val: HeadPositionUpdate) -> Result<(), IntcodeMachineError> {
        // check if new head is within 0..tape.len()
        if !(0..self.tape.len()).contains(&val) {
            return Err(IntcodeMachineError::TapeOutOfBoundsError);
        }

        self.head_position = val;
        Ok(())
    }

    pub(crate) fn run(&mut self) -> Result<isize, IntcodeMachineError> {
        loop {
            let op = OpCode::from(self.tape.read(self.head_position)?);
            let head_update = match op.execute(
                &mut self.tape,
                self.head_position,
                &mut self.input,
                &mut self.output,
            ) {
                Err(err) => match err {
                    OpCodeExecutionError::ExecutionFinished => {
                        return Ok(self.tape.read(0)?);
                    }
                    OpCodeExecutionError::InputFailure => {
                        return Err(IntcodeMachineError::InputFailure(self.dump_state()))
                    }
                    _ => {
                        return Err(IntcodeMachineError::ExecutionFailure);
                    }
                },
                Ok(head_update) => head_update,
            };

            self.update_head(head_update)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intcode_machine_still_works_for_day2_part1() {
        fn produces_expected_output_for_day2_input() {
            let mut day2_tape = Tape::new(utils::read_input_file("day2.input"));
            day2_tape.write(1, 12);
            day2_tape.write(2, 2);

            let dummy_in = b"";
            let mut dummy_out = Vec::new();
            assert_eq!(
                4_138_687,
                IntcodeMachine::new(day2_tape, &dummy_in[..], &mut dummy_out)
                    .run()
                    .unwrap()
            )
        }
    }

    #[test]
    fn injecting_io_works_for_day_5_input_part1() {
        let tape = Tape::new(utils::read_input_file("day5.input"));

        let input = b"1";
        let mut output = Vec::new();

        IntcodeMachine::new(tape, &input[..], &mut output)
            .run()
            .unwrap();

        let output = String::from_utf8(output).unwrap().parse::<isize>().unwrap();
        assert_eq!(13_210_611, output);
    }

    #[test]
    fn injecting_io_works_for_day_5_input_part2() {
        let tape = Tape::new(utils::read_input_file("day5.input"));

        let input = b"5";
        let mut output = Vec::new();

        IntcodeMachine::new(tape, &input[..], &mut output)
            .run()
            .unwrap();

        let output = String::from_utf8(output).unwrap().parse::<isize>().unwrap();
        assert_eq!(584_126, output);
    }
}
