use smol_vm::Vm;

#[test]
pub fn it_adds_r0_r1() {
    let mut vm = Vm::default();
    vm.registers.r0 = 1;
    vm.registers.r1 = 2;
    vm.instructions.instructions = vec![
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 3);
}

#[test]
pub fn it_adds_r2_r3() {
    let mut vm = Vm::default();
    vm.registers.r2 = 2;
    vm.registers.r3 = 6;
    vm.instructions.instructions = vec![
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r2 and r3
        0b0011_0010,
    ];
    vm.run();

    assert_eq!(vm.registers.r2, 8);
}

#[test]
pub fn it_adds_r7_r4() {
    let mut vm = Vm::default();
    vm.registers.r4 = 11;
    vm.registers.r7 = 100;
    vm.instructions.instructions = vec![
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r7 and r4
        0b0100_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 111);
}

#[test]
pub fn it_iadds_r7_123() {
    let mut vm = Vm::default();
    vm.registers.r7 = 100;
    vm.instructions.instructions = vec![
        // ALU Add from Immediate
        0b00_000_1_0_0,
        // Registers r7 and r4
        0b0100_0111,
        // Immediate 11
        11,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 111);
}

#[test]
pub fn it_substracts_r0_r1() {
    let mut vm = Vm::default();
    vm.registers.r0 = 5;
    vm.registers.r1 = 2;
    vm.instructions.instructions = vec![
        // ALU Subtract from Register
        0b00_001_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 3);
}

#[test]
pub fn it_substracts_r2_r3() {
    let mut vm = Vm::default();
    vm.registers.r2 = 14;
    vm.registers.r3 = 6;
    vm.instructions.instructions = vec![
        // ALU Subtract from Register
        0b00_001_0_0_0,
        // Registers r2 and r3
        0b0011_0010,
    ];
    vm.run();

    assert_eq!(vm.registers.r2, 8);
}

#[test]
pub fn it_substracts_r7_r4() {
    let mut vm = Vm::default();
    vm.registers.r4 = 100;
    vm.registers.r7 = 211;
    vm.instructions.instructions = vec![
        // ALU Subtract from Register
        0b00_001_0_0_0,
        // Registers r7 and r4
        0b0100_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 111);
}

#[test]
pub fn it_binary_ands_r0_r1() {
    let mut vm = Vm::default();
    vm.registers.r0 = 0b101;
    vm.registers.r1 = 0b100;
    vm.instructions.instructions = vec![
        // ALU Binary and from Register
        0b00_010_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 4);
}

#[test]
pub fn it_binary_ands_r2_r3() {
    let mut vm = Vm::default();
    vm.registers.r2 = 14;
    vm.registers.r3 = 8;
    vm.instructions.instructions = vec![
        // ALU Binary and from Register
        0b00_010_0_0_0,
        // Registers r2 and r3
        0b0011_0010,
    ];
    vm.run();

    assert_eq!(vm.registers.r2, 8);
}

#[test]
pub fn it_binary_ands_r7_r4() {
    let mut vm = Vm::default();
    vm.registers.r4 = 0b01111111;
    vm.registers.r7 = 0b11101111;
    vm.instructions.instructions = vec![
        // ALU Binary and from Register
        0b00_010_0_0_0,
        // Registers r7 and r4
        0b0100_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 111);
}

#[test]
pub fn it_binary_ors_r0_r1() {
    let mut vm = Vm::default();
    vm.registers.r0 = 0b101;
    vm.registers.r1 = 0b100;
    vm.instructions.instructions = vec![
        // ALU Binary or from Register
        0b00_011_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 5);
}

#[test]
pub fn it_binary_ors_r2_r3() {
    let mut vm = Vm::default();
    vm.registers.r2 = 14;
    vm.registers.r3 = 8;
    vm.instructions.instructions = vec![
        // ALU Binary or from Register
        0b00_011_0_0_0,
        // Registers r2 and r3
        0b0011_0010,
    ];
    vm.run();

    assert_eq!(vm.registers.r2, 14);
}

#[test]
pub fn it_binary_ors_r7_r4() {
    let mut vm = Vm::default();
    vm.registers.r4 = 0b01101111;
    vm.registers.r7 = 0b01101110;
    vm.instructions.instructions = vec![
        // ALU Binary or from Register
        0b00_011_0_0_0,
        // Registers r7 and r4
        0b0100_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 111);
}

#[test]
pub fn it_binary_xors_r0_r1() {
    let mut vm = Vm::default();
    vm.registers.r0 = 0b101;
    vm.registers.r1 = 0b100;
    vm.instructions.instructions = vec![
        // ALU Binary xor from Register
        0b00_100_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 1);
}

#[test]
pub fn it_binary_xors_r2_r3() {
    let mut vm = Vm::default();
    vm.registers.r2 = 14;
    vm.registers.r3 = 8;
    vm.instructions.instructions = vec![
        // ALU Binary xor from Register
        0b00_100_0_0_0,
        // Registers r2 and r3
        0b0011_0010,
    ];
    vm.run();

    assert_eq!(vm.registers.r2, 6);
}

#[test]
pub fn it_binary_xors_r7_r4() {
    let mut vm = Vm::default();
    vm.registers.r4 = 0b01101111;
    vm.registers.r7 = 0b01101110;
    vm.instructions.instructions = vec![
        // ALU Binary xor from Register
        0b00_100_0_0_0,
        // Registers r7 and r4
        0b0100_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 1);
}

#[test]
pub fn it_binary_nots_r0() {
    // TODO: fix this since it's not working due to u8 -> u16 stuffs
    let mut vm = Vm::default();
    vm.registers.r0 = 0b1111_1101;
    vm.instructions.instructions = vec![
        // ALU Binary not from Register
        0b00_101_0_0_0,
        // Register r0
        0b000_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 0b0000010);
}

#[test]
pub fn it_binary_nots_r2() {
    let mut vm = Vm::default();
    vm.registers.r2 = 123;
    vm.instructions.instructions = vec![
        // ALU Binary not from Register
        0b00_101_0_0_0,
        // Register r2
        0b0000_0010,
    ];
    vm.run();

    assert_eq!(vm.registers.r2, 132);
}

#[test]
pub fn it_binary_nots_r7() {
    let mut vm = Vm::default();
    vm.registers.r7 = 0b0110_1111;
    vm.instructions.instructions = vec![
        // ALU Binary not from Register
        0b00_101_0_0_0,
        // Register r 7
        0b0000_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 0b1001_0000);
}

#[test]
pub fn it_increments_r0() {
    let mut vm = Vm::default();
    vm.registers.r0 = 0b111101;
    vm.instructions.instructions = vec![
        // ALU Incerement from Register
        0b00_111_0_0_0,
        // Register r0
        0b000_0000,
    ];
    vm.run();

    assert_eq!(vm.registers.r0, 0b111110);
}

#[test]
pub fn it_increments_r2() {
    let mut vm = Vm::default();
    vm.registers.r2 = 123;
    vm.instructions.instructions = vec![
        // ALU Incerement from Register
        0b00_111_0_0_0,
        // Register r2
        0b0000_0010,
    ];
    vm.run();

    assert_eq!(vm.registers.r2, 124);
}

#[test]
pub fn it_decrements_r7() {
    let mut vm = Vm::default();
    vm.registers.r7 = 0b01101111;
    vm.instructions.instructions = vec![
        // ALU Decrement from Register
        0b00_111_1_0_0,
        // Register r 7
        0b0000_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 0b01101110);
}

#[test]
pub fn it_sets_eq_flag_register() {
    let mut vm = Vm::default();
    vm.registers.r6 = 50;
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_0_0_0,
        // Register r7 and r6
        0b0110_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.fg, 0b1);
}

#[test]
pub fn it_sets_eq_flag_immidiate() {
    let mut vm = Vm::default();
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_1_0_0,
        // Register r7
        0b0000_0111,
        50,
    ];
    vm.run();

    assert_eq!(vm.registers.fg, 0b1);
}

#[test]
pub fn it_sets_gt_flag_register() {
    let mut vm = Vm::default();
    vm.registers.r6 = 49;
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_0_0_0,
        // Register r7 and r6
        0b0110_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.fg, 0b10);
}

#[test]
pub fn it_sets_gt_flag_immidiate() {
    let mut vm = Vm::default();
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_1_0_0,
        // Register r7
        0b0000_0111,
        49,
    ];
    vm.run();

    assert_eq!(vm.registers.fg, 0b10);
}

#[test]
pub fn it_sets_lt_flag_register() {
    let mut vm = Vm::default();
    vm.registers.r6 = 60;
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_0_0_0,
        // Register r7 and r6
        0b0110_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.fg, 0b100);
}

#[test]
pub fn it_sets_lt_flag_immidiate() {
    let mut vm = Vm::default();
    vm.registers.r7 = 50;
    vm.instructions.instructions = vec![
        // ALU EQR
        0b00_110_1_0_0,
        // Register r7
        0b0000_0111,
        60,
    ];
    vm.run();

    assert_eq!(vm.registers.fg, 0b100);
}
