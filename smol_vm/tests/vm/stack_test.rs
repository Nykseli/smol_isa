use smol_vm::Vm;

#[test]
pub fn it_loads_immediate_variable_address() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // Stack load variable immediate 8bit
        0b10_10_1_0_00,
        // Value of 10
        10,
    ];
    vm.run();

    assert_eq!(vm.registers.vp, (u16::MAX / 2) + 10);
}

#[test]
pub fn it_loads_immediate_variable_address_16bit() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // Stack load variable immediate 16 bit
        0b10_10_1_1_00,
        // Value of 256 in 16 bit little endian
        0b00000000,
        0b00000001,
    ];
    vm.run();

    assert_eq!(vm.registers.vp, (u16::MAX / 2) + 256);
}

#[test]
pub fn it_loads_register_variable_address() {
    let mut vm = Vm::default();
    vm.registers.r6 = 5;
    vm.instructions.instructions = vec![
        // Stack load variable regsiter
        0b10_10_0_0_00,
        // Register r6
        0b0000_0110,
    ];
    vm.run();

    assert_eq!(vm.registers.vp, (u16::MAX / 2) + 5);
}

#[test]
pub fn it_loads_16b_register_variable_address() {
    let mut vm = Vm::default();
    vm.registers.l1 = 700;
    vm.instructions.instructions = vec![
        // Stack load variable register 16 bit
        0b10_10_0_1_00,
        // Register l1
        0b0000_1010,
    ];
    vm.run();
    assert_eq!(vm.registers.vp, (u16::MAX / 2) + 700);
}

#[test]
pub fn it_resets_variablepointer() {
    let mut vm = Vm::default();
    vm.registers.vp = (u16::MAX / 2) + 123;
    // First make sure that the SP has been changed
    vm.instructions.instructions = vec![
        // Stack load variable immediate 8bit
        0b10_10_1_0_00,
        // Value of 10
        10,
    ];
    vm.run();
    assert_eq!(vm.registers.vp, (u16::MAX / 2) + 10);

    vm.registers.ic = 0;
    // Then make sure we actually reset it
    vm.instructions.instructions = vec![
        // Stack reset the variable pointer
        0b10_11_0_0_00,
    ];
    vm.run();
    assert_eq!(vm.registers.vp, (u16::MAX / 2));
}

#[test]
pub fn it_pushes_register() {
    let mut vm = Vm::default();
    vm.registers.r3 = 5;
    vm.instructions.instructions = vec![
        // PUR  r/8  - Push register onto stack
        0b10_00_0_0_00,
        // Register r3
        0b0000_0011,
    ];
    vm.run();
    assert_eq!(vm.registers.sp, 1);
    assert_eq!(vm.stack.memory()[0], 5);
}

#[test]
pub fn it_pushes_register_16b() {
    let mut vm = Vm::default();
    vm.registers.l0 = 258;
    vm.instructions.instructions = vec![
        // PURL r/16 - Push 16-bit register onto stack
        0b10_00_0_1_00,
        // Register l0
        0b0000_1001,
    ];
    vm.run();
    assert_eq!(vm.registers.sp, 2);
    assert_eq!(vm.stack.memory()[0], 2);
    assert_eq!(vm.stack.memory()[1], 1);
}

#[test]
pub fn it_pushes_immidiate() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // PUI  i/8  - Push immidiate onto stack
        0b10_00_1_0_00,
        5,
    ];
    vm.run();

    assert_eq!(vm.registers.sp, 1);
    assert_eq!(vm.stack.memory()[0], 5);
}

#[test]
pub fn it_pushes_immidiate_16b() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // PURL i/16 - Push 16-bit immidiate onto stack
        0b10_00_1_1_00,
        2,
        1,
    ];
    vm.run();
    assert_eq!(vm.registers.sp, 2);
    assert_eq!(vm.stack.memory()[0], 2);
    assert_eq!(vm.stack.memory()[1], 1);
}

#[test]
pub fn it_pops() {
    let mut vm = Vm::default();
    vm.registers.sp = 1;
    vm.stack.memory_mut()[0] = 5;
    vm.instructions.instructions = vec![
        // POR  r/8  - Pop stack into register
        0b10_01_0_0_00,
        2,
    ];
    vm.run();

    assert_eq!(vm.registers.sp, 0);
    assert_eq!(vm.registers.r2, 5);
}

#[test]
pub fn it_pops_16b() {
    let mut vm = Vm::default();
    vm.registers.sp = 2;
    vm.stack.memory_mut()[0] = 2;
    vm.stack.memory_mut()[1] = 1;
    vm.instructions.instructions = vec![
        // PORL  r/16  - Pop stack into 16-bit register
        0b10_01_0_1_00,
        // Register l0
        0b1001,
    ];
    vm.run();

    assert_eq!(vm.registers.sp, 0);
    assert_eq!(vm.registers.l0, 258);
}
