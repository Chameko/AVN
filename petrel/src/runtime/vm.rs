use crate::common::value::Value;
use crate::diagnostic::debug::dissasemble_instruction;
use crate::diagnostic::{Context, PetrelError, VMError};

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
    OpNull,
    OpTrue,
    OpFalse,
    OpNot,
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

#[derive(Debug)]
pub struct Operation {
    pub opcode: u8,
    pub line: usize,
}

#[derive(Debug)]
pub struct VM {
    pub instructions: Vec<Operation>,
    pub constants: Vec<Value>,
    pub stack: Vec<Value>,
    pub ip: usize,
}

/// Macro for creating basic binary operations
macro_rules! binary_op {
    ($s:tt, $v:ident, $i:ident) => {
        {
            if let Value::Number(_) = $v.peek(0)? {
                if let Value::Number(_) = $v.peek(1)? {
                    // Pop off the values
                    if let Value::Number(an) = $v.pop()? {
                        if let Value::Number(bn) = $v.pop()? {
                            // Push on the result
                            $v.stack.push(Value::Number(bn $s an));
                        }
                    }
                } else {
                    Err(VMError::Runtime(VM::create_context(&$i, "Operands must be numbers")))?;
                }
            }
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
                OpAdd => binary_op!(+, self, instruction),
                OpSubtract => binary_op!(-, self, instruction),
                OpMultiply => binary_op!(*, self, instruction),
                OpDivide => binary_op!(/, self, instruction),
                OpNegate => {
                    if let Value::Number(_) = self.peek(0)? {
                        // Actually pop the value off the stack
                        if let Value::Number(n) = self.pop()? {
                            // Add the negated value to the stack
                            self.stack.push(Value::Number(-n));
                        }
                    } else {
                        // Error out
                        Err(VMError::Runtime(Self::create_context(
                            instruction,
                            "Attempted to negate a non number",
                        )))?;
                    }
                }
                OpConstant => {
                    let val =
                        self.constants[self.instructions[self.ip + 1].opcode as usize].clone();
                    self.stack.push(val);
                    self.ip += 1;
                }
                OpNull => self.stack.push(Value::Null),
                OpTrue => self.stack.push(Value::Bool(true)),
                OpFalse => self.stack.push(Value::Bool(false)),
                OpNot => {
                    // Check if it is a bool
                    match self.peek(0)? {
                        Value::Bool(_) => {
                            // Use logical not
                            if let Value::Bool(b) = self.pop()? {
                                self.stack.push(Value::Bool(!b));
                            }
                        }
                        // !null == null so we do nothing
                        Value::Null => {}
                        _ => Err(VMError::Runtime(Self::create_context(
                            instruction,
                            "Attempted to use logical not on a non boolean",
                        )))?,
                    }
                }
            }
            self.ip += 1;
        }
        Ok(())
    }

    /// Peek at the opcode distance from the top of the stack. Use 0 for top
    fn peek(&self, distance: usize) -> Result<&Value, PetrelError> {
        #[allow(clippy::unnecessary_lazy_evaluations)]
        let v = self
            .stack
            .get(self.stack.len() - 1 - distance)
            .ok_or_else(|| VMError::EmptyStack)?;
        Ok(v)
    }

    fn pop(&mut self) -> Result<Value, PetrelError> {
        #[allow(clippy::unnecessary_lazy_evaluations)]
        let v = self.stack.pop().ok_or_else(|| VMError::EmptyStack)?;
        Ok(v)
    }

    fn create_context(ins: &Operation, message: &str) -> Context {
        Context::new("Unknown".to_string(), ins.line, message.to_string())
    }

    pub fn write_constant(&mut self, constant: Value) -> u8 {
        self.constants.push(constant);
        (self.constants.len() - 1) as u8
    }

    pub fn write_operation(&mut self, code: u8, line: usize) {
        let op = Operation { opcode: code, line };
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
        let a = vm.write_constant(Value::Number(2.5));
        let b = vm.write_constant(Value::Number(7.5));
        let c = vm.write_constant(Value::Number(2.0));
        vm.write_operation(Opcode::OpConstant.into(), 123);
        vm.write_operation(a, 123);
        vm.write_operation(Opcode::OpConstant.into(), 123);
        vm.write_operation(b, 123);
        vm.write_operation(Opcode::OpAdd.into(), 123);
        vm.write_operation(Opcode::OpConstant.into(), 123);
        vm.write_operation(c, 123);
        vm.write_operation(Opcode::OpDivide.into(), 123);
        vm.write_operation(Opcode::OpReturn.into(), 123);
        vm.run(true).unwrap();
    }
}
