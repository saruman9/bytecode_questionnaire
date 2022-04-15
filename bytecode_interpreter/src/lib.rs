use std::collections::HashMap;

mod instructions;
use instructions::{Ident, IndexedInstruction, Instruction, IteratorWrapper};

// TODO: There should be a hash number like `u256`
type Data = u128;
type Stack = Vec<Data>;
type Memory = HashMap<Ident, Data>;
type Address = Data;

#[derive(Debug, Default)]
pub struct ByteCode {
    instructions: Option<Vec<IndexedInstruction>>,
    stack: Stack,
    memory: Memory,
    position: Address,
    ret: Option<Data>,
}

impl ByteCode {
    pub fn new(instructions: Vec<IndexedInstruction>) -> Self {
        Self {
            instructions: Some(instructions),
            ..Default::default()
        }
    }

    pub fn from_bytecode_text(input: impl AsRef<str>) -> Result<Self, Vec<String>> {
        let (instructions, errors): (Vec<_>, Vec<_>) = input
            .as_ref()
            .lines()
            .enumerate()
            .filter(|(_, l)| !(l.starts_with("//") || l.is_empty()))
            .map(|(i, l)| (i, l.split_ascii_whitespace()))
            .map(|(i, l)| (i, Instruction::try_from(IteratorWrapper(l))))
            .partition(|(_, l)| l.is_ok());
        if !errors.is_empty() {
            return Err(errors
                .into_iter()
                .map(|(i, e)| format!("Line: {}, error: {}", i, e.unwrap_err()))
                .collect());
        }
        let instructions = instructions
            .into_iter()
            .map(|(i, ins)| IndexedInstruction::new(i, ins.unwrap()))
            .collect();
        Ok(Self::new(instructions))
    }

    pub fn instructions(&self) -> Option<&[IndexedInstruction]> {
        self.instructions.as_ref().map(AsRef::as_ref)
    }

    pub fn interpret(&mut self) -> Result<(), String> {
        let instructions = self
            .instructions
            .take()
            .ok_or("Instructions doesn't exist")?;
        while self.ret().is_none() {
            let instruction = instructions.get(self.position()).ok_or(format!(
                "Instruction doesn't exist at {} position",
                self.position
            ))?;
            instruction
                .instruction()
                .interpret(self)
                .map_err(|e| format!("Line: {}, error: {}", instruction.index(), e))?;
            dbg!(instruction, self.position, &self.stack);
        }
        Ok(())
    }

    pub fn ret(&self) -> Option<&Data> {
        self.ret.as_ref()
    }

    pub fn position(&self) -> usize {
        self.position as usize
    }
}

#[cfg(test)]
mod test {
    use crate::{instructions::IndexedInstruction, ByteCode, Instruction};

    #[test]
    fn parse_bytecode_example() {
        let input = r#"
// x = 1
LOAD_VAL 1
WRITE_VAR x
// y = 2
LOAD_VAL 2
WRITE_VAR y
// return (x + 1) * y
READ_VAR x
LOAD_VAL 1
ADD

READ_VAR y
MULTIPLY

RETURN_VALUE
"#;
        let output = [
            IndexedInstruction::new(2, Instruction::LoadVal(1)),
            IndexedInstruction::new(3, Instruction::WriteVar("x".into())),
            IndexedInstruction::new(5, Instruction::LoadVal(2)),
            IndexedInstruction::new(6, Instruction::WriteVar("y".into())),
            IndexedInstruction::new(8, Instruction::ReadVar("x".into())),
            IndexedInstruction::new(9, Instruction::LoadVal(1)),
            IndexedInstruction::new(10, Instruction::Add),
            IndexedInstruction::new(12, Instruction::ReadVar("y".into())),
            IndexedInstruction::new(13, Instruction::Mul),
            IndexedInstruction::new(15, Instruction::RetVal),
        ];
        assert!(&ByteCode::from_bytecode_text(input)
            .unwrap()
            .instructions()
            .unwrap()
            .iter()
            .eq(output.iter()));
    }

    #[test]
    fn interpret_example() {
        let instructions = vec![
            IndexedInstruction::new(0, Instruction::LoadVal(1)),
            IndexedInstruction::new(0, Instruction::WriteVar("x".into())),
            IndexedInstruction::new(0, Instruction::LoadVal(2)),
            IndexedInstruction::new(0, Instruction::WriteVar("y".into())),
            IndexedInstruction::new(0, Instruction::ReadVar("x".into())),
            IndexedInstruction::new(0, Instruction::LoadVal(1)),
            IndexedInstruction::new(0, Instruction::Add),
            IndexedInstruction::new(0, Instruction::ReadVar("y".into())),
            IndexedInstruction::new(0, Instruction::Mul),
            IndexedInstruction::new(0, Instruction::RetVal),
        ];
        let mut bytecode = ByteCode::new(instructions);
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 4);
    }

    #[test]
    fn parse_interpret_0() {
        let input = r#"
// x = 1
LOAD_VAL 1
WRITE_VAR x

// y = 2
LOAD_VAL 2
WRITE_VAR y

// z = 56
LOAD_VAL 56
WRITE_VAR z

// w = z + x + y
READ_VAR z
READ_VAR x
ADD
READ_VAR y
ADD
WRITE_VAR w

// return (x + 1) * y * z + (w + 33)
READ_VAR w
LOAD_VAL 33
ADD
READ_VAR x
LOAD_VAL 1
ADD
READ_VAR y
MULTIPLY
READ_VAR z
MULTIPLY
ADD
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 316);
    }

    #[test]
    fn parse_interpret_jump_ret_x() {
        let input = r#"
// x = 1
LOAD_VAL 1
WRITE_VAR x

// goto ret_x_label
LOAD_VAL 26
JUMP

// y = 2
LOAD_VAL 2
WRITE_VAR y

//z = 56
LOAD_VAL 56
WRITE_VAR z

// w = z + x + y
READ_VAR z
READ_VAR x
ADD
READ_VAR y
ADD
WRITE_VAR w

// return (x + 1) * y * z + (w + 33)
READ_VAR w
LOAD_VAL 33
ADD
READ_VAR x
LOAD_VAL 1
ADD
READ_VAR y
MULTIPLY
READ_VAR z
MULTIPLY
ADD
RETURN_VALUE

// ret_x_label: return x
READ_VAR x
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 1);
    }

    #[test]
    fn parse_interpret_jump_ret_sum_of_xyw() {
        let input = r#"
// x = 1
LOAD_VAL 1
WRITE_VAR x

// y = 2
LOAD_VAL 2
WRITE_VAR y

//z = 56
LOAD_VAL 56
WRITE_VAR z

// w = z + x + y
READ_VAR z
READ_VAR x
ADD
READ_VAR y
ADD
WRITE_VAR w

// goto ret_xyw_label
LOAD_VAL 28
JUMP

// return (x + 1) * y * z + (w + 33)
READ_VAR w
LOAD_VAL 33
ADD
READ_VAR x
LOAD_VAL 1
ADD
READ_VAR y
MULTIPLY
READ_VAR z
MULTIPLY
ADD
RETURN_VALUE

// ret_x_label: return x
READ_VAR x
RETURN_VALUE

// ret_xyw_label: return x + y + w
READ_VAR x
READ_VAR y
ADD
READ_VAR w
ADD
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 62);
    }

    #[test]
    fn parse_interpret_jump_and_jump() {
        let input = r#"
// x = 1
LOAD_VAL 1
WRITE_VAR x

// y = 2
LOAD_VAL 2
WRITE_VAR y

//z = 56
LOAD_VAL 56
WRITE_VAR z

// w = z + x + y
READ_VAR z
READ_VAR x
ADD
READ_VAR y
ADD
WRITE_VAR w

// goto set_x_label
LOAD_VAL 34
JUMP

// return (x + 1) * y * z + (w + 33)
READ_VAR w
LOAD_VAL 33
ADD
READ_VAR x
LOAD_VAL 1
ADD
READ_VAR y
MULTIPLY
READ_VAR z
MULTIPLY
ADD
RETURN_VALUE

// ret_x_label: return x
READ_VAR x
RETURN_VALUE

// ret_xyw_label: return x + y + w
READ_VAR x
READ_VAR y
ADD
READ_VAR w
ADD
RETURN_VALUE

// set_x_label: x = 42
LOAD_VAL 42
WRITE_VAR x

// goto ret_x_label
LOAD_VAL 26
JUMP
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 42);
    }
}
