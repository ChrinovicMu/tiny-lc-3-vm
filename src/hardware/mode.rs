use vm:VM; 

pub const MEMORY_SIZE: usize = std::16::MAX as usize; 

pub fn execute_program(vm: &mut VM){
`   
    while vm.registers.pc < MEMORY_SIZE as u16{

        let instruction = vm.read_memory(vm.registers.pc); 

        vm.registers.pc += 1; 

        instructions::execute_instruction(instruction, vm);
    }
}
