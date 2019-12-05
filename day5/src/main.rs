use std::fs;

const ADD_OP_CODE: isize = 1;
const MUL_OP_CODE: isize = 2;
const HALT_OP_CODE: isize = 99;

//type Foo: Fn(Vec<isize>) -> isize;

type HeadAdvancement = isize;

#[derive(Debug)]
enum OpCodeExecutionError {
    TapeError,
    InvalidOpCodeError,
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
    ) -> Result<HeadAdvancement, OpCodeExecutionError>;
}

struct AddOp {}

impl OpCodeExecutor for AddOp {
    fn execute(
        &self,
        tape: &mut Tape,
        head_position: usize,
    ) -> Result<HeadAdvancement, OpCodeExecutionError> {
        // make sure we are actually supposed to be executing this op
        match tape.read(head_position)? {
            ADD_OP_CODE => (),
            _ => return Err(OpCodeExecutionError::InvalidOpCodeError),
        }

        // read input indices
        let input1_idx = tape.read(head_position + 1)?;
        let input2_idx = tape.read(head_position + 2)?;
        let output_idx = tape.read(head_position + 3)?;

        // make sure all indices are positive
        if input1_idx < 0 {
            return Err(OpCodeExecutionError::InvalidOpArguments);
        }
        if input2_idx < 0 {
            return Err(OpCodeExecutionError::InvalidOpArguments);
        }
        if output_idx < 0 {
            return Err(OpCodeExecutionError::InvalidOpArguments);
        }

        // now it's safe to cast them to usize
        let result = tape.read(input1_idx as usize)? + tape.read(input2_idx as usize)?;

        // finally write the result back to the tape
        tape.write(output_idx as usize, result)?;

        Ok(4)
    }
}

struct MulOp {}

impl OpCodeExecutor for MulOp {
    fn execute(
        &self,
        tape: &mut Tape,
        head_position: usize,
    ) -> Result<HeadAdvancement, OpCodeExecutionError> {
        // make sure we are actually supposed to be executing this op
        match tape.read(head_position)? {
            MUL_OP_CODE => (),
            _ => return Err(OpCodeExecutionError::InvalidOpCodeError),
        }

        // read input indices
        let input1_idx = tape.read(head_position + 1)?;
        let input2_idx = tape.read(head_position + 2)?;
        let output_idx = tape.read(head_position + 3)?;

        // make sure all indices are positive
        if input1_idx < 0 {
            return Err(OpCodeExecutionError::InvalidOpArguments);
        }
        if input2_idx < 0 {
            return Err(OpCodeExecutionError::InvalidOpArguments);
        }
        if output_idx < 0 {
            return Err(OpCodeExecutionError::InvalidOpArguments);
        }

        // now it's safe to cast them to usize
        let result = tape.read(input1_idx as usize)? * tape.read(input2_idx as usize)?;

        // finally write the result back to the tape
        tape.write(output_idx as usize, result)?;

        Ok(4)
    }
}

enum OpCode<Ex>
where
    Ex: OpCodeExecutor + ?Sized,
{
    Add(Box<Ex>),
    Mul(Box<Ex>),
    //    Input,
    //    Output,
    Halt,
    Err(isize),
}

impl From<isize> for OpCode<dyn OpCodeExecutor> {
    fn from(code: isize) -> Self {
        use OpCode::*;

        match code {
            ADD_OP_CODE => Add(Box::new(AddOp {})),
            MUL_OP_CODE => Mul(Box::new(MulOp {})),
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

    fn advance_head(&mut self, val: HeadAdvancement) -> Result<(), IntcodeMachineError> {
        // check if new head is within 0..tape.len()
        let proposed_head_position = self.head_position as isize + val;
        if !(0..self.tape.len() as isize).contains(&proposed_head_position) {
            return Err(IntcodeMachineError::TapeOutOfBoundsError);
        }

        self.head_position += val as usize;
        Ok(())
    }

    fn run(&mut self) -> Result<isize, IntcodeMachineError> {
        loop {
            let op = match OpCode::from(self.tape.read(self.head_position)?) {
                OpCode::Err(_) => return Err(IntcodeMachineError::ExecutionFailure),
                OpCode::Halt => {
                    self.output = self.tape.read(0)?;
                    return Ok(self.output);
                }
                OpCode::Add(op) => op,
                OpCode::Mul(op) => op,
            };

            let head_adv = op.execute(&mut self.tape, self.head_position)?;
            self.advance_head(head_adv)?;
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

fn do_part1(tape: Tape) {
    println!(
        "Part 1 answer: {}",
        IntcodeMachine::new(tape).run().unwrap()
    );
}

fn main() {
    let tape = Tape::new(read_input_file("day5.input"));
    do_part1(tape);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
