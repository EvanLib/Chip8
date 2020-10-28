struct CPU {
    // 16 8bit registers 
    registers: [u8; 16],

    // Program Counter
    program_counter: usize,

    // 4 kB (4096) bytes of RAM
    memory: [u8; 4096],

    // 16 16-bit values
    stack: [u16; 16],

    // stack ptr
    stack_pointer: usize,
}

impl CPU {
    fn read_operation(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // return operation
        op_byte1 << 8 | op_byte2
    }

    fn run_operation(&mut self, opcode: u16) {
        self.program_counter += 2;
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        let nnn = opcode & 0x0FFF;
        match (c, x, y, d) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jump(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
    }
               
    

    fn run(&mut self) {
        loop {
            let opcode = self.read_operation();
            self.program_counter += 2;
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;
            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x1, _, _, _) => self.jump(nnn),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow")
        }

        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    fn jump(&mut self, addr: u16) {
        // set program_counter to addr
        self.program_counter = addr as usize;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;

    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;
    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

#[cfg(test)]
#[path = "./processor_test.rs"]
mod processor_test;
