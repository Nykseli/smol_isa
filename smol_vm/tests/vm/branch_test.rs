use smol_vm::Vm;

#[test]
pub fn it_jumps_to_end_of_program() {
    let mut vm = Vm::default();
    vm.registers.r0 = 1;
    vm.registers.r1 = 2;
    vm.instructions.instructions = vec![
        // Jump to address
        0b11_000_0_0_0,
        // 16bit 5 (end of program)
        5,
        0,
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 1);
    assert_eq!(vm.registers.ic, 5);
}

#[test]
pub fn it_runs_without_jump() {
    let mut vm = Vm::default();
    vm.registers.r0 = 1;
    vm.registers.r1 = 2;
    vm.registers.r6 = 51;
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_0_0_0,
        // Register r7 and r6
        0b0110_0111,
        // Branch if equal (Won't be equal so the ALU code instr will be run)
        0b11_001_0_0_0,
        // 16bit 7 (end of program)
        7,
        0,
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 3);
    assert_eq!(vm.registers.ic, 7);
}

#[test]
pub fn it_branches_if_equal() {
    let mut vm = Vm::default();
    vm.registers.r0 = 1;
    vm.registers.r1 = 2;
    vm.registers.r6 = 50;
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_0_0_0,
        // Register r7 and r6
        0b0110_0111,
        // Branch if equal
        0b11_001_0_0_0,
        // 16bit 7 (end of program)
        7,
        0,
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 1);
    assert_eq!(vm.registers.ic, 7);
}

#[test]
pub fn it_branches_if_not_equal() {
    let mut vm = Vm::default();
    vm.registers.r0 = 1;
    vm.registers.r1 = 2;
    vm.registers.r6 = 51;
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_0_0_0,
        // Register r7 and r6
        0b0110_0111,
        // Branch if not equal
        0b11_010_0_0_0,
        // 16bit 7 (end of program)
        7,
        0,
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 1);
    assert_eq!(vm.registers.ic, 7);
}

#[test]
pub fn it_branches_if_greater_than() {
    let mut vm = Vm::default();
    vm.registers.r0 = 1;
    vm.registers.r1 = 2;
    vm.registers.r6 = 50;
    vm.registers.r7 = 51;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_0_0_0,
        // Register r7 and r6
        0b0110_0111,
        // Branch if greater than
        0b11_011_0_0_0,
        // 16bit 7 (end of program)
        7,
        0,
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 1);
    assert_eq!(vm.registers.ic, 7);
}

#[test]
pub fn it_branches_if_less_than() {
    let mut vm = Vm::default();
    vm.registers.r0 = 1;
    vm.registers.r1 = 2;
    vm.registers.r6 = 53;
    vm.registers.r7 = 51;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_0_0_0,
        // Register r7 and r6
        0b0110_0111,
        // Branch if greater than
        0b11_100_0_0_0,
        // 16bit 7 (end of program)
        7,
        0,
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 1);
    assert_eq!(vm.registers.ic, 7);
}
