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
const RLT_BASE_OFFSET_OP_CODE: isize = 9;
const HALT_OP_CODE: isize = 99;

const POSITION_MODE: usize = 0;
const IMMEDIATE_MODE: usize = 1;
const RELATIVE_MODE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

// TODO: replace usize with u64 and isize with i64 due to ever changing specs

impl TryFrom<usize> for ParamMode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use ParamMode::*;

        match value {
            POSITION_MODE => Ok(Position),
            IMMEDIATE_MODE => Ok(Immediate),
            RELATIVE_MODE => Ok(Relative),
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

#[derive(Debug)]
enum OpCode {
    Add(Vec<ParamMode>),
    Mul(Vec<ParamMode>),
    In(Vec<ParamMode>),
    Out(Vec<ParamMode>),
    Jt(Vec<ParamMode>),
    Jf(Vec<ParamMode>),
    Lt(Vec<ParamMode>),
    Eq(Vec<ParamMode>),
    Rbo(Vec<ParamMode>),
    Halt,
    Er(isize),
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
            INPUT_OP_CODE => 1,
            OUTPUT_OP_CODE => 1,
            RLT_BASE_OFFSET_OP_CODE => 1,
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
            INPUT_OP_CODE => In(param_modes_vec),
            OUTPUT_OP_CODE => Out(param_modes_vec),
            RLT_BASE_OFFSET_OP_CODE => Rbo(param_modes_vec),
            HALT_OP_CODE => Halt,
            _ => Er(code),
        }
    }
}

#[derive(Debug)]
enum TapeError {
    WriteOutOfRangeError,
    ReadOutOfRangeError,
    WriteInImmediateModeError,
}

#[derive(Debug, Clone)]
pub struct Tape(Vec<isize>);

impl Tape {
    pub(crate) fn new(input: Vec<isize>) -> Self {
        Tape(input)
    }

    fn resize(&mut self, lower_bound: usize) {
        self.0.resize(lower_bound, 0);
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn write(&mut self, position: usize, value: isize) -> Result<(), TapeError> {
        if self.0.len() <= position {
            // according to day9 specs, write should always succeed (unless to negative index)
            self.resize(position + 1);
        }

        self.0[position] = value;
        Ok(())
    }

    fn read(&mut self, position: usize) -> isize {
        if self.len() < position {
            // according to day9 specs, read should always succeed (unless on negative index)
            self.resize(position + 1);
        }

        self.0[position]
    }

    fn mode_read(
        &mut self,
        position: usize,
        relative_base: isize,
        param_mode: ParamMode,
    ) -> Result<isize, TapeError> {
        let literal_value = self.read(position);
        match param_mode {
            ParamMode::Position => {
                if literal_value < 0 {
                    Err(TapeError::ReadOutOfRangeError)
                } else {
                    Ok(self.read(literal_value as usize))
                }
            }
            ParamMode::Relative => {
                if (literal_value + relative_base) < 0 {
                    Err(TapeError::ReadOutOfRangeError)
                } else {
                    Ok(self.read((literal_value + relative_base) as usize))
                }
            }

            ParamMode::Immediate => Ok(literal_value),
        }
    }

    fn mode_write(
        &mut self,
        position: usize,
        relative_base: isize,
        param_mode: ParamMode,
        value: isize,
    ) -> Result<(), TapeError> {
        match param_mode {
            ParamMode::Position => self.write(position, value),
            ParamMode::Relative => {
                if position as isize + relative_base < 0 {
                    return Err(TapeError::WriteOutOfRangeError);
                }
                self.write((position as isize + relative_base) as usize, value)
            }

            ParamMode::Immediate => Err(TapeError::WriteInImmediateModeError),
        }
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
    relative_base: isize,
    head_position: usize,
}

impl State {
    pub fn new_from_tape(tape: Tape) -> Self {
        State {
            tape,
            relative_base: 0,
            head_position: 0,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            tape: Tape(Vec::new()),
            relative_base: 0,
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
    relative_base: isize,

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
            relative_base: 0,
            input: reader,
            output: writer,
        }
    }

    pub fn load_state(state: State, reader: R, writer: W) -> Self {
        IntcodeMachine {
            tape: state.tape,
            head_position: state.head_position,
            relative_base: state.relative_base,
            input: reader,
            output: writer,
        }
    }

    pub fn dump_state(&self) -> State {
        State {
            tape: self.tape.clone(),
            relative_base: self.relative_base,
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

    fn execute_add(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let result =
            self.tape
                .mode_read(self.head_position + 1, self.relative_base, param_modes[0])?
                + self.tape.mode_read(
                    self.head_position + 2,
                    self.relative_base,
                    param_modes[1],
                )?;

        let output_idx = self.tape.read(self.head_position + 3);
        self.tape.write(output_idx as usize, result);

        Ok(self.head_position + 4)
    }

    fn execute_mul(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let result =
            self.tape
                .mode_read(self.head_position + 1, self.relative_base, param_modes[0])?
                * self.tape.mode_read(
                    self.head_position + 2,
                    self.relative_base,
                    param_modes[1],
                )?;

        let output_idx = self.tape.read(self.head_position + 3);
        self.tape.write(output_idx as usize, result);

        Ok(self.head_position + 4)
    }

    fn execute_less_than(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param1 =
            self.tape
                .mode_read(self.head_position + 1, self.relative_base, param_modes[0])?;
        let param2 =
            self.tape
                .mode_read(self.head_position + 2, self.relative_base, param_modes[1])?;
        let store_target = self.tape.read(self.head_position + 3);

        if param1 < param2 {
            self.tape.write(store_target as usize, 1);
        } else {
            self.tape.write(store_target as usize, 0);
        }

        Ok(self.head_position + 4)
    }

    fn execute_jump_true(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param =
            self.tape
                .mode_read(self.head_position + 1, self.relative_base, param_modes[0])?;
        let jump_target =
            self.tape
                .mode_read(self.head_position + 2, self.relative_base, param_modes[1])?;

        if param != 0 {
            Ok(jump_target as usize)
        } else {
            Ok(self.head_position + 3)
        }
    }

    fn execute_jump_false(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param =
            self.tape
                .mode_read(self.head_position + 1, self.relative_base, param_modes[0])?;
        let jump_target =
            self.tape
                .mode_read(self.head_position + 2, self.relative_base, param_modes[1])?;

        if param == 0 {
            Ok(jump_target as usize)
        } else {
            Ok(self.head_position + 3)
        }
    }

    fn execute_equals(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param1 =
            self.tape
                .mode_read(self.head_position + 1, self.relative_base, param_modes[0])?;
        let param2 =
            self.tape
                .mode_read(self.head_position + 2, self.relative_base, param_modes[1])?;
        let store_target = self.tape.read(self.head_position + 3);

        if param1 == param2 {
            self.tape.write(store_target as usize, 1);
        } else {
            self.tape.write(store_target as usize, 0);
        }

        Ok(self.head_position + 4)
    }

    fn execute_adjust_relative_base(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param =
            self.tape
                .mode_read(self.head_position + 1, self.relative_base, param_modes[0])?;

        self.relative_base += param;
        Ok(self.head_position + 2)
    }

    fn execute_input(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let output_idx = self.tape.read(self.head_position + 1);

        let mut buffer = String::new();
        self.input.read_line(&mut buffer).unwrap();
        let input_value = match buffer.trim().parse::<isize>() {
            Ok(val) => val,
            _ => return Err(OpCodeExecutionError::InputFailure),
        };

        self.tape.write(output_idx as usize, input_value);

        Ok(self.head_position + 2)
    }

    fn execute_output(
        &mut self,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let output_val =
            self.tape
                .mode_read(self.head_position + 1, self.relative_base, param_modes[0])?;
        println!("writing {}", output_val);
        //        write!(&mut self.output, "{}", output_val).unwrap();
        writeln!(&mut self.output, "{}", output_val).unwrap();

        Ok(self.head_position + 2)
    }

    fn execute_op(&mut self, op: OpCode) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        use OpCode::*;
        match op {
            Add(param_modes) => self.execute_add(param_modes),
            Mul(param_modes) => self.execute_mul(param_modes),
            Jt(param_modes) => self.execute_jump_true(param_modes),
            Jf(param_modes) => self.execute_jump_false(param_modes),
            Lt(param_modes) => self.execute_less_than(param_modes),
            Eq(param_modes) => self.execute_equals(param_modes),
            Rbo(param_modes) => self.execute_adjust_relative_base(param_modes),
            In(param_modes) => self.execute_input(param_modes),
            Out(param_modes) => self.execute_output(param_modes.clone()),

            Halt => Err(OpCodeExecutionError::ExecutionFinished),
            Er(_) => Err(OpCodeExecutionError::ExecutionFailure),
        }
    }

    pub(crate) fn run(&mut self) -> Result<isize, IntcodeMachineError> {
        loop {
            let op = OpCode::from(self.tape.read(self.head_position));

            println!("executing: {:?}", op);

            let head_update = match self.execute_op(op) {
                Err(err) => match err {
                    OpCodeExecutionError::ExecutionFinished => {
                        return Ok(self.tape.read(0));
                    }
                    OpCodeExecutionError::InputFailure => {
                        return Err(IntcodeMachineError::InputFailure(self.dump_state()))
                    }
                    _ => {
                        println!("execution failure");
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
    use crate::intcode_machine::IntcodeMachineError::InputFailure;
    use std::io;

    #[test]
    fn intcode_machine_still_works_for_day2_part1() {
        let mut day2_tape = Tape::new(utils::read_input_file("day2.input"));
        day2_tape.write(1, 12).unwrap();
        day2_tape.write(2, 2).unwrap();

        let dummy_in = b"";
        let mut dummy_out = Vec::new();
        assert_eq!(
            4_138_687,
            IntcodeMachine::new(day2_tape, &dummy_in[..], &mut dummy_out)
                .run()
                .unwrap()
        )
    }

    #[test]
    fn injecting_io_works_for_day_5_input_part1() {
        let tape = Tape::new(utils::read_input_file("day5.input"));

        let input = b"1";
        let mut output = Vec::new();

        IntcodeMachine::new(tape, &input[..], &mut output)
            .run()
            .unwrap();

        let output = utils::parse_multiple_utf8_num_repr_lns(&output)
            .last()
            .unwrap()
            .to_owned();
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

        let output = utils::parse_multiple_utf8_num_repr_lns(&output)
            .last()
            .unwrap()
            .to_owned();
        assert_eq!(584_126, output);
    }

    #[test]
    fn example_1_outputs_itself() {
        let tape_input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let tape = Tape::new(tape_input.clone());

        let dummy_in = b"";
        let mut dummy_out = Vec::new();

        let foo = IntcodeMachine::new(tape, &dummy_in[..], &mut dummy_out).run();
        match foo {
            Err(e) => panic!(e),
            _ => (),
        }

        let full_out = utils::parse_multiple_utf8_num_repr_lns(&dummy_out);

        println!("out: {:?}", full_out);
        //        panic!(dummy_out)

        //        assert!(foo.is_ok());
    }

    #[test]
    fn example_2_outputs_16_digit_number() {
        let tape = Tape::new(vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0]);

        let dummy_in = b"";
        let mut dummy_out = Vec::new();

        IntcodeMachine::new(tape, &dummy_in[..], &mut dummy_out)
            .run()
            .unwrap();

        assert_eq!(
            1_219_070_632_396_864,
            utils::parse_multiple_utf8_num_repr_lns(&dummy_out)
                .last()
                .unwrap()
                .to_owned()
        )
    }

    #[test]
    fn example_3_outputs_1125899906842624() {
        let tape = Tape::new(vec![104, 1_125_899_906_842_624, 99]);

        let dummy_in = b"";
        let mut dummy_out = Vec::new();

        IntcodeMachine::new(tape, &dummy_in[..], &mut dummy_out)
            .run()
            .unwrap();

        assert_eq!(
            1_125_899_906_842_624,
            utils::parse_multiple_utf8_num_repr_lns(&dummy_out)
                .last()
                .unwrap()
                .to_owned()
        );
    }
}
