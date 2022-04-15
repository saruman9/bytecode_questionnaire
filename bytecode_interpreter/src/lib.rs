use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, mpsc, Arc},
};

mod instructions;
use instructions::{Ident, IndexedInstruction, Instruction, IteratorWrapper};

// TODO: There should be a hash number like `u256`
type Data = u128;
type Stack = Vec<Data>;
type Memory = HashMap<Ident, Data>;
type Address = Data;
// TODO: We should use UUID for example or another unique id
type Id = usize;

#[derive(Debug, Default)]
pub struct ByteCode {
    id: Id,
    // FIXME: dirty hack
    count_of_threads: Arc<AtomicUsize>,
    instructions: Vec<IndexedInstruction>,
    stack: Stack,
    memory: Memory,
    position: Address,
    senders: HashMap<Id, mpsc::SyncSender<Data>>,
    receivers: HashMap<Id, mpsc::Receiver<Data>>,
    ret: Option<Data>,
}

impl ByteCode {
    pub fn new(instructions: Vec<IndexedInstruction>) -> Self {
        Self {
            instructions,
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

    pub fn instructions(&self) -> &[IndexedInstruction] {
        &self.instructions
    }

    pub fn interpret(&mut self) -> Result<(), String> {
        let instructions = self.instructions.clone();
        while self.ret().is_none() {
            let instruction = instructions.get(self.position()).ok_or(format!(
                "Instruction doesn't exist at {} position",
                self.position
            ))?;
            instruction
                .instruction()
                .interpret(self)
                .map_err(|e| format!("Line: {}, error: {}", instruction.index(), e))?;
            // TODO: Remove me pls
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

    pub(crate) fn stack_pop(&mut self) -> Result<Data, &'static str> {
        self.stack.pop().ok_or("Stack is empty")
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
    fn parse_and_interpret() {
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
    fn jump_ret_x() {
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

// ret_x_label: return x
READ_VAR x
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 1);
    }

    #[test]
    fn jump_ret_sum_of_xyw() {
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
    fn jump_and_jump() {
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

    #[test]
    fn jump_less_than_0() {
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

// if 256 < w {
//   return 1337
// }
// return 0
LOAD_VAL 256
READ_VAR w
LOAD_VAL 19
JUMP_LESS_THAN
LOAD_VAL 0
LOAD_VAL 20
JUMP
LOAD_VAL 1337
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 0);
    }

    #[test]
    fn jump_less_than_1() {
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

// if w < 256 {
//   return 1337
// }
// return 0
READ_VAR w
LOAD_VAL 256
LOAD_VAL 19
JUMP_LESS_THAN
LOAD_VAL 0
LOAD_VAL 20
JUMP
LOAD_VAL 1337
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 1337);
    }

    #[test]
    fn jump_greater_than_0() {
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

// if 256 > w {
//   return 1337
// }
// return 0
LOAD_VAL 256
READ_VAR w
LOAD_VAL 19
JUMP_GREATER_THAN
LOAD_VAL 0
LOAD_VAL 20
JUMP
LOAD_VAL 1337
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 1337);
    }

    #[test]
    fn jump_greater_than_1() {
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

// if w > 256 {
//   return 1337
// }
// return 0
READ_VAR w
LOAD_VAL 256
LOAD_VAL 19
JUMP_GREATER_THAN
LOAD_VAL 0
LOAD_VAL 20
JUMP
LOAD_VAL 1337
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 0);
    }

    #[test]
    fn jump_equal() {
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

// if w == 59 {
//   return 1337
// }
// return 0
READ_VAR w
LOAD_VAL 59
LOAD_VAL 19
JUMP_EQUAL
LOAD_VAL 0
LOAD_VAL 20
JUMP
LOAD_VAL 1337
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap() as u32, 1337);
    }

    #[test]
    fn pow() {
        let input = r#"
// base = 12
LOAD_VAL 12
WRITE_VAR base

// exponent = 15
LOAD_VAL 15
WRITE_VAR exponent

// result = 1
LOAD_VAL 1
WRITE_VAR result

// while (exponent > 0) {
//   result = result * base
//   exponent =- 1
// }
READ_VAR exponent
LOAD_VAL 0
LOAD_VAL 12
JUMP_GREATER_THAN

// return result
READ_VAR result
RETURN_VALUE

// body from while statement
READ_VAR result
READ_VAR base
MULTIPLY
WRITE_VAR result
READ_VAR exponent
LOAD_VAL 1
SUB
WRITE_VAR exponent
LOAD_VAL 6
JUMP
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap(), 15_407_021_574_586_368);
    }

    #[test]
    fn fibonacci_space_optimized() {
        let input = r#"
// fib(33)
// n = 33
LOAD_VAL 33
WRITE_VAR n

// a = 0
LOAD_VAL 0
WRITE_VAR a

// b = 1
LOAD_VAL 1
WRITE_VAR b

// if n == 0 {
//   return a
// }
READ_VAR n
LOAD_VAL 0
LOAD_VAL 36
JUMP_EQUAL

// for(i = 2; i <= n; i++) {
//   c = a + b
//   a = b
//   b = c
// }
LOAD_VAL 2
WRITE_VAR i
READ_VAR i
READ_VAR n
LOAD_VAL 22
JUMP_LESS_THAN
READ_VAR i
READ_VAR n
LOAD_VAL 22
JUMP_EQUAL

// return b
READ_VAR b
RETURN_VALUE

// body from for statement
READ_VAR a
READ_VAR b
ADD
WRITE_VAR c
READ_VAR b
WRITE_VAR a
READ_VAR c
WRITE_VAR b
READ_VAR i
LOAD_VAL 1
ADD
WRITE_VAR i
LOAD_VAL 12
JUMP

// label: if n == 0 than return 0
READ_VAR a
LOAD_VAL 17
JUMP
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap(), 3_524_578);
    }

    #[test]
    fn spawn() {
        let input = r#"
// spawn(f1, f2)
LOAD_VAL 5
LOAD_VAL 7
SPAWN
LOAD_VAL 0
RETURN_VALUE

// return 1
LOAD_VAL 1
RETURN_VALUE

// return 2
LOAD_VAL 2
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap(), 0);
    }

    #[test]
    fn send_recv() {
        let input = r#"
// spawn(f_recv, f_send)
LOAD_VAL 9
LOAD_VAL 14
SPAWN

// return recv(1) + recv(2)
LOAD_VAL 1
RECV_CHANNEL
LOAD_VAL 2
RECV_CHANNEL
ADD
RETURN_VALUE

// send(20)
LOAD_VAL 20
LOAD_VAL 0
SEND_CHANNEL
LOAD_VAL 0
RETURN_VALUE

// send(22)
LOAD_VAL 22
LOAD_VAL 0
SEND_CHANNEL
LOAD_VAL 0
RETURN_VALUE
"#;

        let mut bytecode = ByteCode::from_bytecode_text(input).unwrap();
        bytecode.interpret().unwrap();
        assert_eq!(*bytecode.ret().unwrap(), 42);
    }

}
