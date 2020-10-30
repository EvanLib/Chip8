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
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match (c, x, y, d) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jump(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.op_3xnn(x, nn),
            (0x4, _, _, _) => self.op_4xnn(x, nn),
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),
            (0x6, _, _, 0x0) => self.op_6xnn(x, nn),
            (0x7, _, _, 0x0) => self.op_7xnn(x, nn),
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            (0x8, _, _, 0x6) => self.op_8xy6(x, y),
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            (0x8, _, _, 0xE) => self.op_8xyE(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_operation();
            self.run_operation(opcode);
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

    /// Skip the following instruction if the value of register VX equals NN
    fn op_3xnn(&mut self, x: u8, nn: u8) {
        let value = self.registers[x as usize];
        if value == nn {
            self.program_counter += 2;
        }
    }

    /// Skip the following instruction if the value of register VX is not equal to NN
    fn op_4xnn(&mut self, x: u8, nn: u8) {
        let value = self.registers[x as usize];
        if value != nn {
            self.program_counter += 2;
        }
    }

    /// Skip the following instruction if the value of register VX is equal to the value of register VY
    fn op_5xy0(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        if vx == vy {
            println!("{}", vx);
            println!("{}", vy);
            self.program_counter += 2;
        }
    }
    /// Store number NN in register VX
    fn op_6xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
    }

    /// Add value nn to register VX
    fn op_7xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] += nn;
    }

    /// Store the value of register VY in register VX
    fn op_8xy0(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    /// Set VX to VX OR VY
    fn op_8xy1(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }

    /// Set VX to VX OR VY
    fn op_8xy2(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
    }

    /// Set VX to VX OR VY
    fn op_8xy3(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
    }

    /// Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    fn op_8xy4(&mut self, x: u8, y: u8) {
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

    /// Subtract the value of register VY from register VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn op_8xy5(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_sub(arg2);
        self.registers[x as usize] = val;

        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    /// Store the value of register VY shifted right one bit in register VX
    /// Set register VF to the least significant bit prior to the shift
    fn op_8xy6(&mut self, x: u8, y: u8) {
        let arg2 = self.registers[y as usize];
        let vf = arg2 & 0x01;

        // shift one bit
        self.registers[x as usize] = arg2 >> 1;
        self.registers[0xF] = vf;
    }

    ///	Set register VX to the value of VY minus VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn op_8xy7(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg2.overflowing_sub(arg1);
        if overflow_detected {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }
        self.registers[x as usize] = val;
    }

    /// Store the value of register VY shifted left one bit in register VX
    /// Set register VF to the most significant bit prior to the shift
    fn op_8xyE(&mut self, x: u8, y: u8) {
        // shift one bit
        let arg2: u8 = self.registers[y as usize];
        self.registers[x as usize] = arg2 << 1;

        // set register VF to most significan bit prioir to the shift.
        self.registers[0x0F] = (self.registers[y as usize] & 0b10000000) >> 7;
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
