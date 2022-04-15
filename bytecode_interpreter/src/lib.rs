use std::collections::HashMap;

mod instructions;
use instructions::{Ident, Instruction, IteratorWrapper};

type Data = u128;
type Stack = Vec<Data>;
type Memory = HashMap<Ident, Data>;

#[derive(Debug, Default)]
pub struct ByteCode {
    instructions: Option<Vec<(usize, Instruction)>>,
    stack: Stack,
    memory: Memory,
    ret: Option<Data>,
}

impl ByteCode {
    pub fn new(instructions: Vec<(usize, Instruction)>) -> Self {
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
            .map(|(i, ins)| (i, ins.unwrap()))
            .collect();
        Ok(Self::new(instructions))
    }

    pub fn instructions(&self) -> Option<&[(usize, Instruction)]> {
        self.instructions.as_ref().map(AsRef::as_ref)
    }

    pub fn interpret(&mut self) -> Result<(), String> {
        let instructions = self.instructions.take();
        for instruction in instructions
            .ok_or("Instructions doesn't exist")?
            .into_iter()
        {
            instruction
                .1
                .interpret(self)
                .map_err(|e| format!("Line: {}, error: {}", instruction.0, e))?;
        }
        Ok(())
    }

    pub fn ret(&self) -> Option<&Data> {
        self.ret.as_ref()
    }
}

#[cfg(test)]
mod test {
    use crate::{ByteCode, Instruction};

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
            (2, Instruction::LoadVal(1)),
            (3, Instruction::WriteVar("x".into())),
            (5, Instruction::LoadVal(2)),
            (6, Instruction::WriteVar("y".into())),
            (8, Instruction::ReadVar("x".into())),
            (9, Instruction::LoadVal(1)),
            (10, Instruction::Add),
            (12, Instruction::ReadVar("y".into())),
            (13, Instruction::Mul),
            (15, Instruction::RetVal),
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
            (0, Instruction::LoadVal(1)),
            (0, Instruction::WriteVar("x".into())),
            (0, Instruction::LoadVal(2)),
            (0, Instruction::WriteVar("y".into())),
            (0, Instruction::ReadVar("x".into())),
            (0, Instruction::LoadVal(1)),
            (0, Instruction::Add),
            (0, Instruction::ReadVar("y".into())),
            (0, Instruction::Mul),
            (0, Instruction::RetVal),
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
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 316);
    }
}
