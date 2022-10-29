use crate::common::value::Value;
use crate::diagnostic::debug::dissasemble_instruction;
use crate::diagnostic::{PetrelError, VMError};

#[derive(Debug)]
#[repr(u8)]
pub enum Opcode {
    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

impl TryFrom<u8> for Opcode {
    type Error = VMError;
    fn try_from(src: u8) -> Result<Self, Self::Error> {
        match src {
            0 => Ok(Opcode::OpReturn),
            1 => Ok(Opcode::OpConstant),
            2 => Ok(Opcode::OpNegate),
            3 => Ok(Opcode::OpAdd),
            4 => Ok(Opcode::OpSubtract),
            5 => Ok(Opcode::OpMultiply),
            6 => Ok(Opcode::OpDivide),
            _ => Err(VMError::InvalidOpcodeConversion(src)),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(code: Opcode) -> Self {
        code as u8
    }
}

pub struct Operation {
    pub opcode: u8,
    pub line: usize,
    pub start: usize,
    pub length: usize,
}

pub struct VM {
    pub instructions: Vec<Operation>,
    pub constants: Vec<Value>,
    pub stack: Vec<Value>,
    pub ip: usize,
}

/// Macro for creating basic binary operations
macro_rules! binary_op {
    ($s:tt, $v:ident) => {
        {
            let a = $v.stack.pop().ok_or(VMError::EmptyStack)?;
            let b = $v.stack.pop().ok_or(VMError::EmptyStack)?;
            $v.stack.push(b $s a);
        }
    };
}

impl VM {
    pub fn new() -> Self {
        VM {
            instructions: vec![],
            constants: vec![],
            stack: vec![],
            ip: 0,
        }
    }

    pub fn run(&mut self, stack_trace: bool) -> Result<(), PetrelError> {
        loop {
            if stack_trace {
                for val in &self.stack {
                    println!("{:>10}[ {:?} ]", " ", val);
                }
            }
            let instruction = self.instructions.get(self.ip).ok_or(VMError::NoReturn)?;
            dissasemble_instruction(self, self.ip);
            use Opcode::*;
            match Opcode::try_from(instruction.opcode)? {
                OpReturn => break,
                OpAdd => binary_op!(+, self),
                OpSubtract => binary_op!(-, self),
                OpMultiply => binary_op!(*, self),
                OpDivide => binary_op!(/, self),
                OpNegate => {
                    #[allow(clippy::unnecessary_lazy_evaluations)]
                    let val = self.stack.pop().ok_or_else(|| VMError::EmptyStack)?;
                    self.stack.push(-val);
                }
                OpConstant => {
                    let val = self.constants[self.instructions[self.ip + 1].opcode as usize];
                    self.stack.push(val);
                    self.ip += 1;
                }
            }
            self.ip += 1;
        }
        Ok(())
    }

    pub fn write_constant(&mut self, constant: Value) -> u8 {
        self.constants.push(constant);
        (self.constants.len() - 1) as u8
    }

    pub fn write_operation(&mut self, code: u8, line: usize, start: usize, len: usize) {
        let op = Operation {
            opcode: code,
            length: len,
            start,
            line,
        };
        self.instructions.push(op);
    }
}

impl Default for VM {
    fn default() -> Self {
        VM::new()
    }
}

#[cfg(test)]
mod vm_test {
    use super::*;

    /// Test that our opcode is byte sized
    #[test]
    fn opcode_size() {
        assert_eq!(std::mem::size_of::<Opcode>(), 1);
    }

    #[test]
    fn basic_arithmatic() {
        let mut vm = VM::new();
        let a = vm.write_constant(2.5);
        let b = vm.write_constant(7.5);
        let c = vm.write_constant(2.0);
        vm.write_operation(Opcode::OpConstant.into(), 123, 0, 0);
        vm.write_operation(a, 123, 0, 0);
        vm.write_operation(Opcode::OpConstant.into(), 123, 0, 0);
        vm.write_operation(b, 123, 0, 0);
        vm.write_operation(Opcode::OpAdd.into(), 123, 0, 0);
        vm.write_operation(Opcode::OpConstant.into(), 123, 0, 0);
        vm.write_operation(c, 123, 0, 0);
        vm.write_operation(Opcode::OpDivide.into(), 123, 0, 0);
        vm.write_operation(Opcode::OpReturn.into(), 123, 0, 0);
        vm.run(true).unwrap();
    }
}
