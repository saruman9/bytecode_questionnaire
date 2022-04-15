use crate::{ByteCode, Data};

pub type Ident = String;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    LoadVal(Data),
    WriteVar(Ident),
    ReadVar(Ident),
    Add,
    Mul,
    RetVal,
    Jump,
    Unk,
}

impl Default for Instruction {
    fn default() -> Self {
        Self::Unk
    }
}

// FIXME: Ugly `TryFrom` trait with a wrapper, because Rust doesn't have the specialization
//
// https://github.com/rust-lang/rust/issues/50133
pub struct IteratorWrapper<'a, T: std::iter::Iterator<Item = &'a str>>(pub T);

impl<'a, T> TryFrom<IteratorWrapper<'a, T>> for Instruction
where
    T: std::iter::Iterator<Item = &'a str>,
{
    type Error = &'static str;

    fn try_from(iter_w: IteratorWrapper<'a, T>) -> Result<Self, Self::Error> {
        let mut iter = iter_w.0;
        let instruction = iter.next().ok_or("Empty instruction")?;
        let instruction = match instruction {
            "LOAD_VAL" => Self::LoadVal(
                iter.next()
                    .ok_or("Empty operand for LOAD_VAL")?
                    .parse()
                    .map_err(|_| "Parsing of operand of LOAD_VAL")?,
            ),
            "WRITE_VAR" => Self::WriteVar(iter.next().ok_or("Empty operand for WRITE_VAR")?.into()),
            "READ_VAR" => Self::ReadVar(iter.next().ok_or("Empty operand for READ_VAR")?.into()),
            "ADD" => Self::Add,
            "MULTIPLY" => Self::Mul,
            "RETURN_VALUE" => Self::RetVal,
            "JUMP" => Self::Jump,
            _ => return Err("Unknown instruction"),
        };
        Ok(instruction)
    }
}

impl Instruction {
    pub fn interpret(&self, bytecode: &mut ByteCode) -> Result<(), &'static str> {
        match self {
            Instruction::LoadVal(value) => {
                bytecode.stack.push(*value);
                bytecode.position += 1;
            }
            Instruction::WriteVar(ident) => {
                let value = bytecode.stack.pop().ok_or("Stack is empty")?;
                bytecode.memory.insert(ident.clone(), value);
                bytecode.position += 1;
            }
            Instruction::ReadVar(ident) => {
                let value = bytecode.memory.get(ident).ok_or("Variable doesn't exist")?;
                bytecode.stack.push(*value);
                bytecode.position += 1;
            }
            Instruction::Add => {
                let lhs = bytecode.stack.pop().ok_or("Stack is empty")?;
                let rhs = bytecode.stack.pop().ok_or("Stack is empty")?;
                bytecode
                    .stack
                    .push(lhs.checked_add(rhs).ok_or("Addition overflow occurred")?);
                bytecode.position += 1;
            }
            Instruction::Mul => {
                let lhs = bytecode.stack.pop().ok_or("Stack is empty")?;
                let rhs = bytecode.stack.pop().ok_or("Stack is empty")?;
                bytecode.stack.push(
                    lhs.checked_mul(rhs)
                        .ok_or("Multiplication overflow occurred")?,
                );
                bytecode.position += 1;
            }
            Instruction::RetVal => {
                bytecode.ret = Some(bytecode.stack.pop().ok_or("Stack is empty")?);
                bytecode.position += 1;
            }
            Instruction::Jump => {
                bytecode.position = bytecode.stack.pop().ok_or("Stack is empty")?;
            }
            Instruction::Unk => unreachable!(),
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct IndexedInstruction {
    index: usize,
    instruction: Instruction,
}

impl IndexedInstruction {
    pub fn new(index: usize, instruction: Instruction) -> Self {
        Self { index, instruction }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn instruction(&self) -> &Instruction {
        &self.instruction
    }
}
