use crate::runtime::vm::{Opcode, VM};

pub fn dissasemble_instruction(vm: &VM, offset: usize) {
    print!("{:04} ", offset);
    if offset > 0 && vm.instructions[offset].line == vm.instructions[offset - 1].line {
        print!("{:>3}", '|')
    } else {
        print!("{:3>}", vm.instructions[offset].line)
    }

    let instruction = vm.instructions[offset].opcode;
    use crate::runtime::vm::Opcode::*;
    match Opcode::try_from(instruction) {
        Ok(OpReturn) => println!(" Return"),
        Ok(OpConstant) => println!(
            " Constant {}: {}",
            vm.instructions[offset + 1].opcode,
            vm.constants[vm.instructions[offset + 1].opcode as usize]
        ),
        Ok(OpNegate) => println!(" Negate"),
        Ok(OpAdd) => println!(" Add"),
        Ok(OpSubtract) => println!(" Subtract"),
        Ok(OpMultiply) => println!(" Multiply"),
        Ok(OpDivide) => println!(" Divide"),
        Ok(OpTrue) => println!(" True"),
        Ok(OpFalse) => println!(" False"),
        Ok(OpNull) => println!(" Null"),
        Ok(OpNot) => println!(" Not"),
        Err(e) => panic!("{}", e),
    }
}

pub fn dissasemble_vm(vm: &VM, name: &str) {
    println!("===={}====", name);
    for i in 0..vm.instructions.len() {
        dissasemble_instruction(vm, i);
    }
}
