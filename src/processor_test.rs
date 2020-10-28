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
