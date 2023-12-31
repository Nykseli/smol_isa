#![deny(clippy::undocumented_unsafe_blocks)]

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub,
    SubAssign,
};

mod registers;
pub mod syscall;

use registers::Registers;
use syscall::vm_syscall;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

type RegEither = Either<u8, u16>;

impl RegEither {
    fn as_u8(self) -> u8 {
        match self {
            Self::Left(v) => v,
            Self::Right(v) => v as u8,
        }
    }

    fn as_u16(self) -> u16 {
        match self {
            Self::Left(v) => v as u16,
            Self::Right(v) => v,
        }
    }
}

impl From<u8> for RegEither {
    fn from(value: u8) -> Self {
        RegEither::Left(value)
    }
}

impl From<u16> for RegEither {
    fn from(value: u16) -> Self {
        RegEither::Right(value)
    }
}

macro_rules! either_oper {
    ($lvalue:expr, $rvalue:expr, $op:tt) => {
        match $lvalue {
            Either::Left(value) => Either::Left(value $op $rvalue.as_u8()),
            Either::Right(value) => Either::Right(value $op $rvalue.as_u16()),
        }
    };
}

impl Add for RegEither {
    type Output = RegEither;

    fn add(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, +)
    }
}

impl Sub for RegEither {
    type Output = RegEither;

    fn sub(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, -)
    }
}

impl BitAnd for RegEither {
    type Output = RegEither;

    fn bitand(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, &)
    }
}

impl BitOr for RegEither {
    type Output = RegEither;

    fn bitor(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, |)
    }
}

impl BitXor for RegEither {
    type Output = RegEither;

    fn bitxor(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, ^)
    }
}

impl Not for RegEither {
    type Output = RegEither;

    fn not(self) -> Self::Output {
        match self {
            Either::Left(value) => Either::Left(!value),
            Either::Right(value) => Either::Right(!value),
        }
    }
}

impl AddAssign for RegEither {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl SubAssign for RegEither {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl BitXorAssign for RegEither {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl BitOrAssign for RegEither {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitAndAssign for RegEither {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

#[derive(Debug)]
enum Register {
    Ic,
    Fg,
    Cr,
    Sp,
    Vp,
    Zr,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    L0,
    L1,
}

#[derive(Debug)]
struct RegisterValue {
    value: RegEither,
    register: Register,
}

impl RegisterValue {
    fn new(value: RegEither, register: Register) -> Self {
        Self { value, register }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Stack {
    /// Stack size of 64kib since that's what the u16 stack pointer allows.
    memory: [u8; u16::MAX as usize],
}

impl Stack {
    // Get the memory space from the stack pointer
    pub fn from_sp(&self, sp: u16) -> &[u8] {
        &self.memory[sp as usize..]
    }

    // Get the memory space from the stack pointer
    pub fn from_sp_mut(&mut self, sp: u16) -> &mut [u8] {
        &mut self.memory[sp as usize..]
    }

    /// Memory is the whole 64kib memory
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    /// Memory is the whole 64kib memory
    pub fn memory_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }

    /// Stack is the first 32kib bytes
    pub fn stack(&self) -> &[u8] {
        &self.memory[..(u16::MAX / 2) as usize]
    }

    /// Stack is the first 32kib bytes
    pub fn stack_mut(&mut self) -> &mut [u8] {
        &mut self.memory[..(u16::MAX / 2) as usize]
    }

    /// Variable space is the last 32kib bytes
    pub fn variable(&self) -> &[u8] {
        &self.memory[..(u16::MAX / 2) as usize]
    }

    /// Variable space is the last 32kib bytes
    pub fn variable_mut(&mut self) -> &mut [u8] {
        &mut self.memory[(u16::MAX / 2) as usize..]
    }

    pub fn save_value(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn save_value_16(&mut self, addr: u16, value: u16) {
        let [li, mi] = value.to_le_bytes();
        self.memory[addr as usize] = li;
        self.memory[addr as usize + 1] = mi;
    }

    pub fn load_value(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn load_value_16(&mut self, addr: u16) -> u16 {
        let li = self.memory[addr as usize];
        let mi = self.memory[addr as usize + 1];
        u16::from_le_bytes([li, mi])
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            memory: [0; u16::MAX as usize],
        }
    }
}

#[derive(Debug, Default)]
pub struct Instructions {
    /// Linear set of instructions.
    /// [Registers.ic] points here.
    pub instructions: Vec<u8>,
}

impl Instructions {
    pub fn size(&self) -> usize {
        self.instructions.len()
    }

    pub fn get(&self, idx: u16) -> u8 {
        self.instructions[idx as usize]
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Vm {
    pub registers: Registers,
    pub stack: Stack,
    pub instructions: Instructions,
}

impl Vm {
    fn stack_pop(&mut self) -> u8 {
        self.registers.sp -= 1;
        self.stack.load_value(self.registers.sp)
    }

    fn stack_pop_16b(&mut self) -> u16 {
        self.registers.sp -= 2;
        self.stack.load_value_16(self.registers.sp)
    }

    fn stack_push(&mut self, val: u8) {
        self.stack.save_value(self.registers.sp, val);
        self.registers.sp += 1;
    }

    fn stack_push_16b(&mut self, val: u16) {
        self.stack.save_value_16(self.registers.sp, val);
        self.registers.sp += 2;
    }

    fn register_val(&self, reg: u8) -> RegisterValue {
        match reg {
            0b0000 => RegisterValue::new(self.registers.r0.into(), Register::R0),
            0b0001 => RegisterValue::new(self.registers.r1.into(), Register::R1),
            0b0010 => RegisterValue::new(self.registers.r2.into(), Register::R2),
            0b0011 => RegisterValue::new(self.registers.r3.into(), Register::R3),
            0b0100 => RegisterValue::new(self.registers.r4.into(), Register::R4),
            0b0101 => RegisterValue::new(self.registers.r5.into(), Register::R5),
            0b0110 => RegisterValue::new(self.registers.r6.into(), Register::R6),
            0b0111 => RegisterValue::new(self.registers.r7.into(), Register::R7),
            0b1000 => RegisterValue::new(self.registers.vp.into(), Register::Vp),
            0b1001 => RegisterValue::new(self.registers.l0.into(), Register::L0),
            0b1010 => RegisterValue::new(self.registers.l1.into(), Register::L1),
            0b1011 => RegisterValue::new(self.registers.ic.into(), Register::Ic),
            0b1100 => RegisterValue::new(self.registers.fg.into(), Register::Fg),
            0b1101 => RegisterValue::new(self.registers.cr.into(), Register::Cr),
            0b1110 => RegisterValue::new(self.registers.sp.into(), Register::Sp),
            0b1111 => RegisterValue::new(self.registers.zr.into(), Register::Zr),
            _ => unreachable!("Tried to access nonexsisting register {reg}"),
        }
    }

    fn register_save(&mut self, reg: RegisterValue) {
        match reg.register {
            Register::R0 => self.registers.r0 = reg.value.as_u8(),
            Register::R1 => self.registers.r1 = reg.value.as_u8(),
            Register::R2 => self.registers.r2 = reg.value.as_u8(),
            Register::R3 => self.registers.r3 = reg.value.as_u8(),
            Register::R4 => self.registers.r4 = reg.value.as_u8(),
            Register::R5 => self.registers.r5 = reg.value.as_u8(),
            Register::R6 => self.registers.r6 = reg.value.as_u8(),
            Register::R7 => self.registers.r7 = reg.value.as_u8(),
            Register::L0 => self.registers.l0 = reg.value.as_u16(),
            Register::L1 => self.registers.l1 = reg.value.as_u16(),
            Register::Ic => self.registers.ic = reg.value.as_u16(),
            Register::Fg => self.registers.fg = reg.value.as_u16(),
            Register::Cr => self.registers.cr = reg.value.as_u16(),
            Register::Sp => self.registers.sp = reg.value.as_u16(),
            Register::Vp => self.registers.vp = reg.value.as_u16(),
            Register::Zr => self.registers.zr = reg.value.as_u16(),
        }
    }

    fn immediate_instr(&self, ic: u16) -> u8 {
        self.instructions.instructions[ic as usize]
    }

    /// Turn instructions from ic and ic+1 into u16
    fn immediate_instr_16b(&self, ic: u16) -> u16 {
        // TODO: make this faster with unsafe
        let instrs = &self.instructions.instructions[ic as usize..];
        // The archicture is little endian so we need to create u16 from le bytes
        u16::from_le_bytes([instrs[0], instrs[1]])
    }

    fn decode_register(&self, regs: u8) -> RegisterValue {
        let r0 = regs & 0b1111;
        self.register_val(r0)
    }

    fn decode_registers(&self, regs: u8) -> (RegisterValue, RegisterValue) {
        let r0 = regs & 0b1111;
        let r1 = (regs >> 4) & 0b1111;
        (self.register_val(r0), self.register_val(r1))
    }

    fn decode_alu_instr(&mut self, instr: u8) -> u16 {
        // TODO: 16 bit support
        let (used, source_vals) = match instr & 0b100 {
            0b000 => {
                let regs = self.instructions.get(self.registers.ic + 1);
                (2, self.decode_registers(regs))
            }
            // TODO: Do something less hacky
            0b100 if (instr >> 3) & 0b111 == 0b111 => {
                let regs = self.instructions.get(self.registers.ic + 1);
                (2, self.decode_registers(regs))
            }
            0b100 => {
                // TODO: Don't hackily ignore the second encoded register
                let regs = self.instructions.get(self.registers.ic + 1);
                let value = self.instructions.get(self.registers.ic + 2);
                let mut regs = self.decode_registers(regs);
                regs.1.value = value.into();
                (3, regs)
            }
            _ => unreachable!(),
        };

        let mut source_vals = source_vals;

        match (instr >> 3) & 0b111 {
            // Add
            0b000 => source_vals.0.value += source_vals.1.value,
            // Subtract
            0b001 => source_vals.0.value -= source_vals.1.value,
            // Binary and
            0b010 => source_vals.0.value &= source_vals.1.value,
            // Binary or
            0b011 => source_vals.0.value |= source_vals.1.value,
            // Binary xor
            0b100 => source_vals.0.value ^= source_vals.1.value,
            // Binary not
            0b101 => source_vals.0.value = !source_vals.0.value,
            // Equality
            0b110 => {
                // reset equailty flags
                self.registers.fg &= 0b000;
                if source_vals.0.value == source_vals.1.value {
                    self.registers.fg |= 0b1;
                } else if source_vals.0.value > source_vals.1.value {
                    self.registers.fg |= 0b10;
                } else if source_vals.0.value < source_vals.1.value {
                    self.registers.fg |= 0b100;
                }
            }
            0b111 => {
                // Decode the increment/decrement function
                source_vals.0.value = match instr & 0b100 {
                    // Increment
                    0b000 => source_vals.0.value + (1_u8).into(),
                    // Decrement
                    0b100 => source_vals.0.value - (1_u8).into(),
                    _ => unreachable!("This statement should be literally impossible"),
                }
            }

            // Since we use and (&) we limit ourself to values 0-3
            _ => unimplemented!("Only Add AluFamily is implemnted"),
        }
        self.register_save(source_vals.0);

        used
    }

    fn decode_load_store_instr(&mut self, instr: u8) -> u16 {
        let mut used: u16 = 1;

        let ic = self.registers.ic + 1;
        match (instr >> 4) & 0b11 {
            // Store
            0b00 => {
                let has_memory_target = (instr >> 3) & 0b1 == 1;

                // If the target is memory, the source value can be decoded
                // after the two addrees bytes
                let src_ic = if has_memory_target { ic + 1 } else { ic };

                let src_value = match (instr >> 1) & 0b11 {
                    // 8 bit register or 16 bit register
                    0b00 | 0b01 => {
                        // If there's a memory target, the register is not encoded in the same byte as source
                        if has_memory_target {
                            used += 1;
                            // If the target is memory, we need to skip the second memory addres byte
                            self.decode_registers(self.instructions.get(src_ic + 1))
                                .0
                                .value
                        } else {
                            self.decode_registers(self.instructions.get(src_ic)).1.value
                        }
                    }
                    // 8 bit immediate
                    0b10 => {
                        used += 1;
                        // If the source is not a register, immideate is stored in the next instruction
                        self.immediate_instr(src_ic + 1).into()
                    }
                    // 16 bit immediate
                    0b11 => {
                        used += 2;
                        // If the source is not a register, immideate is stored in the next instruction
                        self.immediate_instr_16b(src_ic + 1).into()
                    }
                    _ => unreachable!(),
                };

                match (instr >> 3) & 0b1 {
                    // Register target
                    0b0 => {
                        used += 1;
                        let mut register = self.decode_register(self.instructions.get(ic));
                        register.value = src_value;
                        self.register_save(register);
                    }
                    // Memory target target
                    0b1 => {
                        used += 2;
                        // register addess is encoded the same way immediates are
                        let addr = self.immediate_instr_16b(ic);
                        match src_value {
                            Either::Left(value) => self.stack.save_value(addr, value),
                            Either::Right(value) => self.stack.save_value_16(addr, value),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            // Load
            0b01 => {
                // Address is decoded in the same way as immideates
                let addr = self.immediate_instr_16b(ic);
                // The target register is always after the two address bytes
                let mut target = self.decode_register(self.instructions.get(ic + 2));

                // We can only load into registers
                match instr & 0b1111 {
                    // 8 bit register
                    0b1000 => {
                        let val = self.stack.load_value(addr);
                        target.value = val.into();
                    }
                    // 16 bit register
                    0b1010 => {
                        let val = self.stack.load_value_16(addr);
                        target.value = val.into();
                    }

                    _ => unreachable!("Invalid load instruction"),
                }

                self.register_save(target);
                used += 3;
            }
            0b11 => unimplemented!("Swap is not implemented"),
            _ => unreachable!(),
        }

        used
    }

    /// First tuple value is true if a jump happens
    fn decode_branch_instr(&mut self, instr: u8) -> (bool, u16) {
        let start_ic = self.registers.ic;

        let address = if (instr >> 3) & 0b111 <= 0b100 {
            self.immediate_instr_16b(self.registers.ic + 1)
        } else {
            // Since Only Branch/Jump will use the address, this value doesn't matter
            0
        };

        match (instr >> 3) & 0b111 {
            // Relative jump
            0b000 => self.registers.ic = address,
            // Branch if equal
            0b001 => {
                if self.registers.fg & 0b1 == 1 {
                    self.registers.ic = address
                }
            }
            // Branch if not equal
            0b010 => {
                if self.registers.fg & 0b1 == 0 {
                    self.registers.ic = address
                }
            }
            // Branch if greater than
            0b011 => {
                if (self.registers.fg >> 1) & 0b1 == 1 {
                    self.registers.ic = address
                }
            }
            // Branch if less than
            0b100 => {
                if (self.registers.fg >> 2) & 0b1 == 1 {
                    self.registers.ic = address
                }
            }
            // Call
            0b101 => {
                if instr & 0b111 == 0b111 {
                    vm_syscall(&mut self.registers, &mut self.stack);
                } else {
                    // Set the offset so we can return to after the call
                    self.stack_push_16b(start_ic + 3);
                    self.registers.ic = address
                }
            }
            // Return from call
            0b110 => self.registers.ic = self.stack_pop_16b(),
            // Return from interrupt
            0b111 => unimplemented!("Return from interrupt is not implemented"),
            // Since we use and (&) we limit ourself to values 0-3
            _ => unimplemented!("Only Add AluFamily is implemnted"),
        }

        let has_jumped = start_ic != self.registers.ic;
        // syscall and ret use 1 instruction, others use 3
        if instr == 0b11101111 || (instr >> 3) & 0b110 == 0b110 {
            (has_jumped, 1)
        } else {
            (has_jumped, 3)
        }
    }

    fn decode_stack_instr(&mut self, instr: u8) -> u16 {
        let mut used: u16;
        match (instr >> 4) & 0b11 {
            // Push
            0b00 => {
                used = 2;
                let value = match (instr >> 2) & 0b11 {
                    // 8 bit and 16 register has the same logic
                    0b00 | 0b01 => {
                        let reg = self.instructions.get(self.registers.ic + 1);
                        self.decode_register(reg).value
                    }
                    // 8 bit immideate
                    0b10 => self.immediate_instr(self.registers.ic + 1).into(),
                    // 16 bit immideate
                    0b11 => {
                        used = 3;
                        self.immediate_instr_16b(self.registers.ic + 1).into()
                    }
                    _ => unreachable!(),
                };

                match value {
                    Either::Left(val) => self.stack_push(val),
                    Either::Right(val) => self.stack_push_16b(val),
                }
            }
            // Pop
            0b01 => {
                // Can only pop into a register
                used = 2;
                let reg = self.instructions.get(self.registers.ic + 1);
                let mut reg = self.decode_register(reg);
                let popped = match (instr >> 2) & 0b1 == 0 {
                    true => self.stack_pop().into(),
                    false => self.stack_pop_16b().into(),
                };
                reg.value = popped;
                self.register_save(reg);
            }
            // Load variable
            0b10 => {
                self.registers.vp = u16::MAX / 2;
                // Set used to two since only 16 immideate uses 3 (self + 1/2)
                used = 2;
                match (instr >> 2) & 0b11 {
                    // 8 bit and 16 register has the same logic
                    0b00 | 0b01 => {
                        let reg = self.instructions.get(self.registers.ic + 1);
                        self.registers.vp += self.decode_register(reg).value.as_u16();
                    }
                    // 8 bit immideate
                    0b10 => {
                        self.registers.vp += self.immediate_instr(self.registers.ic + 1) as u16;
                    }
                    // 16 bit immideate
                    0b11 => {
                        self.registers.vp += self.immediate_instr_16b(self.registers.ic + 1);
                        used = 3;
                    }
                    _ => unreachable!(),
                }
            }
            // Unload variable
            0b11 => {
                self.registers.vp = u16::MAX / 2;
                used = 1;
            }
            // Since we use and (&) we limit ourself to values 0-3
            _ => unimplemented!("decoding stack instr logic is broken"),
        }

        used
    }

    fn decode_next_instr(&mut self) {
        let instr = self.instructions.get(self.registers.ic);

        match (instr >> 6) & 0b11 {
            0b00 => {
                let used = self.decode_alu_instr(instr);
                self.registers.ic += used;
            }
            0b01 => {
                let used = self.decode_load_store_instr(instr);
                self.registers.ic += used;
            }
            0b10 => {
                let used = self.decode_stack_instr(instr);
                self.registers.ic += used;
            }
            0b11 => {
                let ret = self.decode_branch_instr(instr);
                if let (false, used) = ret {
                    self.registers.ic += used;
                }
            }
            // Since we use and (&) we limit ourself to values 0-3
            _ => unreachable!(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let ic = self.registers.ic;

            // Break after the last instruction
            if ic as usize == self.instructions.size() {
                break;
            }

            if ic as usize > self.instructions.size() {
                panic!("Tried to access non-exsisitng instruction {}", ic);
            }

            self.decode_next_instr();
        }
    }
}
