use super::*;

fn build_cpu() -> CPU {
    let cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };
    cpu
}

#[test]
fn test_initial_state() {
    let cpu = build_cpu();
    assert_eq!(cpu.stack_pointer, 0);
}

#[test]
fn test_2nnn() {
    let mut cpu = build_cpu();
    cpu.run_operation(0x2666);
    assert_eq!(cpu.program_counter, 0x0666);
}

#[test]
fn test_3xnn() {
    // should jump next instruction
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 1;
    cpu.run_operation(0x3101);
    assert_eq!(cpu.program_counter, 0x0004);

    // should not jump next instruction
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 2;
    cpu.run_operation(0x3101);
    assert_eq!(cpu.program_counter, 0x0002);
}

#[test]
fn test_4xnn() {
    // should not jump next instruction
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 1;
    cpu.run_operation(0x4101);
    assert_eq!(cpu.program_counter, 0x0002);

    // should not jump next instruction
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 2;
    cpu.run_operation(0x4101);
    assert_eq!(cpu.program_counter, 0x0004);
}

#[test]
fn test_5xy0() {
    // should jump next instruction
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 1;
    cpu.registers[0x0002 as usize] = 1;
    cpu.run_operation(0x5120);
    assert_eq!(cpu.program_counter, 0x0004);

    // should not jump next instruction
    let mut cpu2 = build_cpu();
    cpu2.registers[0x0001 as usize] = 1;
    cpu2.registers[0x0002 as usize] = 2;
    cpu2.run_operation(0x5120);
    assert_eq!(cpu2.program_counter, 0x0002);
}

#[test]
fn test_6xnn() {
    // should set register value
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 1;
    cpu.run_operation(0x6120);
    assert_eq!(cpu.registers[0x0001], 0x0020);
}

#[test]
fn test_7xnn() {
    // should set register value
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 1;
    cpu.run_operation(0x7120);
    assert_eq!(cpu.registers[0x0001], 0x0021);
}

#[test]
fn test_8xy0() {
    // should set register value
    let mut cpu = build_cpu();
    // x
    cpu.registers[0x0001 as usize] = 1;
    // y
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8120);
    assert_eq!(cpu.registers[0x0001], cpu.registers[0x0002]);
}

#[test]
fn test_8xy1() {
    // should set register value
    let mut cpu = build_cpu();
    let testval = 2 | 2;

    // x
    cpu.registers[0x0001 as usize] = 2;
    // y
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8121);
    assert_eq!(cpu.registers[0x0001], testval);
}

#[test]
fn test_8xy2() {
    // should set register value
    let mut cpu = build_cpu();
    let testval = 1 & 2;

    // x
    cpu.registers[0x0001 as usize] = 1;
    // y
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8122);
    assert_eq!(cpu.registers[0x0001], testval);
}

#[test]
fn test_8xy3() {
    // should set register value
    let mut cpu = build_cpu();
    let testval = 2 ^ 2;

    // x
    cpu.registers[0x0001 as usize] = 2;
    // y
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8123);
    assert_eq!(cpu.registers[0x0001], testval);
}

#[test]
fn test_8xy4() {
    // should set register value
    let mut cpu = build_cpu();
    let testval = 4;

    // test for addition
    cpu.registers[0x0001 as usize] = 2;
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8124);
    assert_eq!(cpu.registers[0x0001], testval);
    assert_eq!(cpu.registers[0x000f], 0);

    // test for carry
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 0xff;
    cpu.registers[0x0002 as usize] = 0x01;
    cpu.run_operation(0x8124);
    assert_eq!(cpu.registers[0x000f], 1);
}

#[test]
fn test_8xy5() {
    // should set register value
    let mut cpu = build_cpu();
    let testval = 4;

    // test for addition
    cpu.registers[0x0001 as usize] = 6;
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8125);
    assert_eq!(cpu.registers[0x0001], testval);
    assert_eq!(cpu.registers[0x000f], 0);

    // test for borrow
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 0x01;
    cpu.registers[0x0002 as usize] = 0x0f;
    cpu.run_operation(0x8125);
    assert_eq!(cpu.registers[0x000f], 1);
}

#[test]
fn test_8xy6() {
    // should set register value
    let mut cpu = build_cpu();
    let testval = 2 >> 1;

    // test for addition
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8126);
    assert_eq!(cpu.registers[0x0001], testval);
    assert_eq!(cpu.registers[0x000f], 0);

    // should set register value
    let mut cpu = build_cpu();
    let testval = 3 >> 1;

    // test for addition
    cpu.registers[0x0002 as usize] = 3;
    cpu.run_operation(0x8126);
    assert_eq!(cpu.registers[0x0001], testval);
    assert_eq!(cpu.registers[0x000f], 1);
}


#[test]
fn test_8xy7() {
    // should set register value
    let mut cpu = build_cpu();
    let testval = 2 - 2;
    cpu.registers[0x0001 as usize] = 2;
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8127);
    assert_eq!(cpu.registers[0x0001], testval);
    assert_eq!(cpu.registers[0x000f], 1);

    // should set register value
    let mut cpu = build_cpu();
    cpu.registers[0x0001 as usize] = 4;
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x8127);
    assert_eq!(cpu.registers[0x000f], 0);
}

#[test]
fn test_8xye() {
    // should set register value
    let mut cpu = build_cpu();
    let testval = 2 << 1;

    // test for addition
    cpu.registers[0x0001 as usize] = 2;
    cpu.registers[0x0002 as usize] = 2;
    cpu.run_operation(0x812E);
    assert_eq!(cpu.registers[0x0001], testval);
    assert_eq!(cpu.registers[0x000f], 0);

    // should set register value
    let mut cpu = build_cpu();
    let testval: u8 = 255 << 1;

    // test for addition
    cpu.registers[0x0002 as usize] = 255;
    cpu.run_operation(0x812E);
    assert_eq!(cpu.registers[0x0001], testval);
    assert_eq!(cpu.registers[0x000F], 1);
}
