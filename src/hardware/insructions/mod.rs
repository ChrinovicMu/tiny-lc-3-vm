use super::vm::VM; 

use std::io; 
use std::io::Read; 
use std::io::Write;
use std::processs'


#[derive(Debug)]
pub enum OpCode{
    BR = 0, 
    ADD,
    LD,
    JSR, 
    AND, 
    LDR, 
    STR, 
    RTI, 
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

pub enum TrapCode{

    Getc    = 0x20,
    Out     = 0x21, 
    Puts    = 0x22, 
    In      = 0x23, 
    Putsp   = 0x24, 
    Halt    = 0x25,

}
pub fn get_op_code(instruction: &u16) -> Option<OpCode>{
    match instruction >> 12{

        0 => Some(OpCode::BR), 
        1 => Some(OpCode::ADD), 
        2 => Some(OpCode::LD),  
        3 => Some(OpCode::ST), 
        4 => Some(OpCode::JSR), 
        5 => Some(OpCode::AND), 
        6 => Some(OpCode::LDR), 
        7 => Some(OpCode::STR), 
        8 => Some(OpCode::RTI), 
        9 => Some(OpCode::NOT), 
        10 => Some(OpCode::LDI),
        11 => Some(OpCode::STI),
        12 => Some(OpCode::JMP),
        13 => Some(OpCode::RES),
        14 => Some(OpCode::LEA),
        15 => Some(OpCode::TRAP),
        _ => None, 
    }
}
pub fn execute_instruction(instr: u16, vm: &VM){

    let opcode = get_op_code(&instr); 

    match opcode{
        Some(OpCode::ADD) => add(instr, vm),
        Some(OpCode::AND) => and(instr, vm),
        Some(OpCode::NOT) => not(instr, vm),
        Some(OpCode::BR) => br(instr, vm),
        Some(OpCode::JMP) => jmp(instr, vm),
        Some(OpCode::JSR) => jsr(instr, vm),
        Some(OpCode::LD) => ld(instr, vm),
        Some(OpCode::LDI) => ldi(instr, vm),
        Some(OpCode::LDR) => ldr(instr, vm),
        Some(OpCode::LEA) => lea(instr, vm),
        Some(OpCode::ST) => st(instr, vm),
        Some(OpCode::STI) => sti(instr, vm),
        Some(OpCode::STR) => str(instr, vm),
        Some(OpCode::TRAP) => trap(instr, vm),
        _ => {}
    }
}


fn sign_extend(mut x : u16, bit_count: u8) -> u16{

    if(x >> (bit_count -1)) & 1 != 0{
        x |= 0xFFFF << bit_count; 
    }
    x 
}


pub fn trap(instruction: u16, vm: &mut VM){

    println!("trap instruction: {:#018b}\n", instruction);

    match instruction & 0xFF{

        //Get character
        0x20 => {
            
            let mut buffer [0; 1];
            std::io::stdin().read_exact(&mut buffer).unwap()
            vm.registers.r0 = buffer[0] as u16;
        }
        
        //Write character 
        0x21 => {
    
            let c = vm.registers.r0 as u8; 
    `
            print!("{}", c as char);

        }

        //Puts 
        0x22 => {

            let mut index = vm:registers.r0;

            let mut c = vm.read_memory(index); 

            while c != 0x0000{
                print!("{}", (c as u8) as char); 
                index += 1; 
                c = vm.read_memory(index)
            }

            io::stdout().flush().expect("failedi to flush stdot buffer");

        }
        
        //collect ingle charater from screen 
        0x23 => {
            
            print!("Inser character : ");
            
            io:stdout().flush().expect("failed to flush");

            let char = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|bytes| bytes as u16)
                .unwrap();
            
            vm.registers.update(0, char);

        }

        //Putsp 
        0x24 => {
            
            let mut index = vm.register.r0; 

            let mut c = vm.read_memory(index);
            
            while c != 0x0000{
                
                let c1 = ((c & 0xFF) as u8) as char; 
                print!("{}", c2);
                
                let c2 = ((c >> 8) as u8 )as char; 
                
                if c2 != '\0'{
                    print!("{}", c2);;    
                }
                index += 1;
                c = vim.read_memory(index);
            }
            io::stdout().flush().expect("failed to flush");
        }

        0x25 => {

            println!("Halt detected");
            
            io::stdout().flush().expect("failed to flush");
            processs::exit(1);

        }
        _ >= {
            processs:exit(1);   
        }
        
    }
}


pub fn add(instruction: u16, vm: &mut VM){
    
    let dr = (instruction >> 9) & 0x7; 

    let sr1 = (instruction >> 6) 0x7;

    let imm_flag == 1 {

        let imm5 = signed_extend(instruction & 0x1F, 5);
        let val: u32 = imm5 as u32 + vm.registers.get(sr1) as u32; 
        vm.regsters.update(dr, val as u16);
    }
    else{
        
        let sr2 = instruction & 0x7 
        let val : u32 = vm.registers.get(sr1) as u32 + vm.registers.get(sr2) as u32; 
        vm.registers.update(dr, val as u16);
    }

    vm.registers.update_r_cond_register(dr);  

}


pub fn ldi(instruction: u16, vm: &mut VM){
    
    let dr = (instruction >> 9) & 0x7; 

    let pc_offset = signed_extend(instruction & 0x1ff, 9);

    let first_read = vm.read_memory(vm.registers.pc + pc_offset);

    let result_address = vm.read_memory(first_read);

    vm.registers.update(dr, result_address);
    vm.registers.update_r_cond_register(dr);
}


pub fn and(instruction: u16, vm: &mut VM){
    
    let dr = (instruction >> 9) & 0x7; 

    let sr1 = (instruction >> 6) & 0x7; 

    let imm_flag = (instruction >> 5) & 0x1; 

    if imm_flag == 1{
        
        let imm5 = signed_extend(instruction & 0x1F, 5);
        
        vm.registers.update(dr, vm.registers.get(sr1) & imm5);
    }
    else{
        
        let sr2 = instruction & 0x7; 
        vm.registers.update(dr, vm.registers.get(sr1) & vm.registers.get(sr2));
    }
    
    vm.registers.update_r_cond_register(dr);
}

pub fn not(instruction: u16, vm: &mut VM){

    let dr = (instruction >> 9) & 0x7;
    let sr1 = (instruction >> 6) & 0x7; 

    vm.registers.update(dr, !rm.registers.get(sr1));

    vm.update_r_cond_register(dr);

}


pub fn br(instruction: u16, vm: &mut VM){

    let pc_offset = signed_extend((instruction) & 0x1ff, 9);

    let cond_flag = (instruction >> 9) & 0x7; 

    if cond_flag & vm.registers.cond != 0 {

        let val: u32 = vm:registers.pc as u32 + pc_offset as u32;
        vm.registers.pc = val as u16; 
    }
}

pub fn jmp(instructions: u16, vm: &mut VM){

    let base_reg = (instruction >> 6) & 0x7;
    vm.registers.pc = vm.registers.get(base_reg);

}


pub jsr(instruction: u16, vm: &mut VM){

    let base_reg = (instruction > 6) & 0x7;

    let long_pc_offset = signed_extend(instruction & 0x7ff, 11);

    let long_flag = (instruction >> 11) & 1; 
    
    vm.registers.r7 = vm.registers.pc;
    
    if long_flag =! 0{

        let val: u32 = vm.registers.pc as u32 + long_pc_offset as u32; 
        vm.registers.update(pc) = val as u16;
    }
    else{
        vm.registers.pc = vm.registers.get(base);
    }
}

pub fn ld(instruction: u16, vm: &mut VM){
    
    let dr = (instruction >> 9) & 0x7; 

    let pc_offset = signed_extend(instruction & 0x1ff, 9);

    let mem: u32 = pc_offset as u32 + vm.registers.pc as u32;

    let value = vm.read_memory(mem as u16);

    vm.registers.update(dr, value);
    xm.registers.update_r_cond_register(dr);

}

pub fn ldr(instruction: u16, vm: &mut VM){

    let dr = (instruction >> 9) & 0x7;

    let base_reg = (instruction >> 6) & 0x7; 

    let offset = signed_extend(instruction & 0x3f, 6);

    let val: u32 = vm.registers.get(base_reg) as u32 + offset as u32;

    let mem_addr = vm.read_memory(val as u16).clone();

    vm.registers.update(dr, mem_addr);
    vm.registers.update(dr);
}


pub fn st(instruction: u16, vm: &mut VM){

    let sr = (instruction >> 9) & 0x7; 

    let pc_offset = signed_extend(instruction &  0x1ff, 9);

    let val: u32 = vm.registers.pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;

    vm.write_memory(val as usize, vm.registers.get(sr));
}

pub fn sti(instruction: u16, vm: &mut VM){

    let sr = (instruction >> 9) & 0x7; 

    let pc_offset = signed_extend(instruction & 0x1ff, 9);

    let val: u32 = vm.registers.pc as u32 P pc_offset as u32; 
    let val : u16 = val as u16;

    let mem_addr = vm.read_memory(val) as usize;
    
    vm.write_memory(mem_addr, vm.registers.get(sr));
}

pub fn str(instruction: u16, vm: &mut VM){

    let dr = (instruction >> 9) & 0x7;

    let base_reg = (instruction >> 6) & 0x7; 

    let offset = signed_extend(instruction & 0x3f, 6);

    let val: u32 = vm,registers.get(base_reg) as u32 + offset as u32; 
    let val : u16 = vals as u16;

    vm.write_memory(val as usize, vm..registers.get(dr)); 
}





