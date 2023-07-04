use smol_vm::Vm;

#[test]
pub fn is_stores_register_in_register() {
    let mut vm = Vm::default();
    vm.registers.r2 = 3;
    vm.instructions.instructions = vec![
        //  ST   r/8  r/8  - Store register in register
        0b01_00_0_0_0_0,
        // Register r1 and r2
        0b0010_0001,
    ];
    vm.run();
    assert_eq!(vm.registers.r1, 3);
}

pub fn is_stores_16_bit_register_in_register() {
    let mut vm = Vm::default();
    vm.registers.l0 = 256;
    vm.instructions.instructions = vec![
        // STL  r/16 r/16 - Store 16-bit register into 16-bit register
        0b01_00_0_1_1_0,
        // Register l1 and l0
        0b1001_1010,
    ];
    vm.run();
    assert_eq!(vm.registers.l1, 256);
}

#[test]
pub fn is_stores_immediate_in_register() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // STI  r/8  i/8  - Store immediate in register
        0b01_00_0_1_0_0,
        // Register r1
        0b0000_0001,
        3,
    ];
    vm.run();
    assert_eq!(vm.registers.r1, 3);
}

pub fn is_stores_16_bit_immediate_in_register() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // STIL r/16 i/16 - Store 16-bit immediate into 16-bit register
        0b01_00_0_1_1_0,
        // Register l1
        0b0000_1010,
        // Value of 256 in 16 bit little endian
        0b00000000,
        0b00000001,
    ];
    vm.run();
    assert_eq!(vm.registers.l1, 256);
}

#[test]
pub fn is_stores_immediate_in_memory() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // STM  a/16 i/8  - Store immediate to memory
        0b01_00_1_1_0_0,
        // address of 256 in 16 bit little endian
        0b00000000,
        0b00000001,
        25,
    ];
    vm.run();
    assert_eq!(vm.stack.memory()[256], 25);
}

#[test]
pub fn is_stores_16b_immediate_in_memory() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // STML a/16 i/16 - Store 16-bit immediate into memory
        0b01_00_1_1_1_0,
        // address of 256 in 16 bit little endian
        0b00000000,
        0b00000001,
        // address of 258 in 16 bit little endian
        0b00000010,
        0b00000001,
    ];
    vm.run();

    let val = u16::from_le_bytes([vm.stack.memory()[256], vm.stack.memory()[257]]);
    assert_eq!(val, 258);
}

#[test]
pub fn is_stores_register_in_memory() {
    let mut vm = Vm::default();
    vm.registers.r2 = 5;
    vm.instructions.instructions = vec![
        // STR  a/16 r/8  - Store register into memory
        0b01_00_1_0_0_0,
        // address of 256 in 16 bit little endian
        0b00000000,
        0b00000001,
        // register r2
        0b0000_0010,
    ];
    vm.run();
    assert_eq!(vm.stack.memory()[256], 5);
}

#[test]
pub fn is_stores_16b_bregister_in_memory() {
    let mut vm = Vm::default();
    vm.registers.l1 = 258;
    vm.instructions.instructions = vec![
        // STR  a/16 r/16  - Store 16-bit register into memory
        0b01_00_1_0_1_0,
        // address of 256 in 16 bit little endian
        0b00000000,
        0b00000001,
        // Register l1
        0b0000_1010,
    ];
    vm.run();
    let val = u16::from_le_bytes([vm.stack.memory()[256], vm.stack.memory()[257]]);
    assert_eq!(val, 258);
}
