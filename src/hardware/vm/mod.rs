const MEMORY_SIZE: usize = u16::MAX as usize;
use super::registers::Registers; 
use std::io::Read; 

pub struct VM {
    pub memory: [u16; MEMORY_SIZE], 
    pub registers: Registers, 
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE], 
            registers: Registers::new(),     
        }
    }
    
    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value; 
    }
    
    pub fn read_memory(&mut self, address: u16) -> u16 {
        if address == MemoryMappedReg::Kbsr as u16 {
            self.handle_keyboard();
        }
        self.memory[address as usize]
    }
    
    fn handle_keyboard(&mut self) {
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.write_memory(MemoryMappedReg::Kbsr as usize, 1 << 15);
            self.write_memory(MemoryMappedReg::Kbdr as usize, buffer[0] as u16);
        } 
    }
}

pub enum MemoryMappedReg {
    Kbsr = 0xFE00,      // keyboard status for pressed key; 
    Kbdr = 0xFE02,      // keyboard identifies which key was pressed
}
