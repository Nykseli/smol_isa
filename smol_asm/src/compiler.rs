use smol_file::{SmolFile, Storage, StorageItem};

use crate::ast::{ASTTree, Arg2, Instruction, R16Regs, R8Regs, Variable, A16, I16, I8, R16, R8};

trait Compile {
    fn compile(&self) -> Vec<u8>;
}

impl Compile for Arg2<R8, R8> {
    fn compile(&self) -> Vec<u8> {
        let arg = (self.arg2.compile()[0] << 4) | self.arg1.compile()[0];
        vec![arg]
    }
}

impl Compile for Arg2<R8, I8> {
    fn compile(&self) -> Vec<u8> {
        let arg = self.arg1.compile()[0];
        vec![arg, self.arg2.compile()[0]]
    }
}

impl Compile for Arg2<R16, R16> {
    fn compile(&self) -> Vec<u8> {
        let arg = (self.arg2.compile()[0] << 4) | self.arg1.compile()[0];
        vec![arg]
    }
}

impl Compile for Arg2<R16, I16> {
    fn compile(&self) -> Vec<u8> {
        let arg = self.arg1.compile()[0];
        let arg2 = self.arg2.compile();
        vec![arg, arg2[0], arg2[1]]
    }
}

impl Compile for Arg2<A16, R16> {
    fn compile(&self) -> Vec<u8> {
        let arg = self.arg1.compile();
        let arg2 = self.arg2.compile()[0];
        vec![arg[0], arg[1], arg2]
    }
}

impl Compile for Arg2<A16, I16> {
    fn compile(&self) -> Vec<u8> {
        let arg = self.arg1.compile();
        let arg2 = self.arg2.compile();
        vec![arg[0], arg[1], arg2[0], arg2[1]]
    }
}

impl Compile for Arg2<A16, R8> {
    fn compile(&self) -> Vec<u8> {
        let arg = self.arg1.compile();
        let arg2 = self.arg2.compile()[0];
        vec![arg[0], arg[1], arg2]
    }
}

impl Compile for Arg2<A16, I8> {
    fn compile(&self) -> Vec<u8> {
        let arg = self.arg1.compile();
        let arg2 = self.arg2.compile()[0];
        vec![arg[0], arg[1], arg2]
    }
}

impl Compile for R8 {
    fn compile(&self) -> Vec<u8> {
        let val: u8 = match self.register {
            R8Regs::R0 => 0b0000,
            R8Regs::R1 => 0b0001,
            R8Regs::R2 => 0b0010,
            R8Regs::R3 => 0b0011,
            R8Regs::R4 => 0b0100,
            R8Regs::R5 => 0b0101,
            R8Regs::R6 => 0b0110,
            R8Regs::R7 => 0b0111,
        };
        vec![val]
    }
}

impl Compile for R16 {
    fn compile(&self) -> Vec<u8> {
        let val: u8 = match self.register {
            R16Regs::L0 => 0b1001,
            R16Regs::L1 => 0b1010,
        };
        vec![val]
    }
}

impl Compile for I8 {
    fn compile(&self) -> Vec<u8> {
        vec![self.value]
    }
}

impl Compile for I16 {
    fn compile(&self) -> Vec<u8> {
        self.value.to_le_bytes().into()
    }
}

impl Compile for A16 {
    fn compile(&self) -> Vec<u8> {
        self.value.to_le_bytes().into()
    }
}

#[allow(dead_code)]
enum ALUType {
    Add,
    Subtract,
    And,
    Or,
    Xor,
    Not,
    Equality,
    IncrDecr,
}

/// If `opcode[2:4] != 0b111`:
/// `opcode[5]` - Source
///  * `0b0` - Register
///  * `0b1` - Immediate
///
/// Otherwise:
/// `opcode[5]` - Function
///  * `0b0` - Increment
///  * `0b1` - Decrement
#[allow(dead_code)]
enum ALUSrc {
    Register,
    Immidiate,
    Incerement,
    Decrement,
}

#[allow(dead_code)]
enum LoadStoreType {
    Load,
    Store,
    Swap,
}

#[allow(dead_code)]
enum BranchCall {
    Jump,
    BranchEq,
    BranchNe,
    BranchGt,
    BranchLt,
    Call,
    Ret,
}

fn compile_alu_equality(tt: ALUType, source: ALUSrc, is_16b: bool, noop: bool) -> u8 {
    #[allow(clippy::unusual_byte_groupings)]
    let op = match tt {
        ALUType::Add => 0b00_000_0_0_0,
        ALUType::Subtract => 0b00_001_0_0_0,
        ALUType::And => 0b00_010_0_0_0,
        ALUType::Or => 0b00_011_0_0_0,
        ALUType::Xor => 0b00_100_0_0_0,
        ALUType::Not => 0b00_101_0_0_0,
        ALUType::Equality => 0b00_110_0_0_0,
        ALUType::IncrDecr => 0b00_111_0_0_0,
    };

    let mut op = match source {
        ALUSrc::Immidiate | ALUSrc::Decrement => op | 0b100,
        _ => op,
    };

    if is_16b {
        op |= 0b10;
    }

    if noop {
        op |= 0b1;
    }

    op
}

fn compile_branch_call<'a>(
    tt: BranchCall,
    label: &'a str,
    label_instrs: &mut Vec<(&'a str, usize, bool)>,
    labels: &mut [(&'a str, u16)],
    instr_len: usize,
) -> Vec<u8> {
    #[allow(clippy::unusual_byte_groupings)]
    let op: u8 = match tt {
        BranchCall::Jump => 0b11_000_000,
        BranchCall::BranchEq => 0b11_001_000,
        BranchCall::BranchNe => 0b11_010_000,
        BranchCall::BranchGt => 0b11_011_000,
        BranchCall::BranchLt => 0b11_100_000,
        BranchCall::Call => 0b11_101_000,
        BranchCall::Ret => 0b11_110_000,
    };

    let mut args = vec![op, 0, 0];

    if let Some(addr) = labels.iter().find(|(l, _)| l == &label) {
        let add_le = addr.1.to_le_bytes();
        args[1] = add_le[0];
        args[2] = add_le[1];
    } else {
        label_instrs.push((label, instr_len + 1, false));
    }

    args
}

fn compile_load_store(
    tt: LoadStoreType,
    is_memtarget: bool,
    is_immiddiate: bool,
    is_16b: bool,
    is_cr: bool,
) -> u8 {
    #[allow(clippy::unusual_byte_groupings)]
    let mut op = match tt {
        LoadStoreType::Store => 0b01_00_0_0_0_0,
        LoadStoreType::Load => 0b01_01_0_0_0_0,
        LoadStoreType::Swap => 0b01_11_0_0_0_0,
    };

    if is_memtarget {
        op |= 0b1000;
    }

    if is_immiddiate {
        op |= 0b100;
    }

    if is_16b {
        op |= 0b10;
    }

    if is_cr {
        op |= 0b1;
    }

    op
}

fn compile_variables(vars: &Vec<Variable>) -> Storage {
    // total size in bytes!
    let mut total_size = 0;
    let mut offset = 0;
    let mut items: Vec<StorageItem> = Vec::new();
    for var in vars {
        let mut size = var.size;
        let init_data = if let Some(data) = &var.bytes {
            if data.len() != var.size as usize {
                panic!(
                    "Variable '{}' initial value's length expected to be {}, was {}",
                    var.name,
                    var.size,
                    data.len()
                );
            }

            // Set the higest bit to signal initialised data
            size |= 0x8000;
            // Save space for the variable size
            total_size += var.size;
            Some(data)
        } else {
            None
        };

        items.push(StorageItem {
            size,
            offset,
            init_data: init_data.cloned(), // TODO: get rid of clone
        });

        // 4 bytes for the two u16
        total_size += 4;
        offset += var.size;
    }

    Storage { total_size, items }
}

fn variable_offset(name: &str, ast: &ASTTree, storage: &Storage) -> u16 {
    // Variable and storage items are handeled in order so they have the same indexes
    let idx = ast
        .variables
        .iter()
        .position(|v| v.name == name)
        .unwrap_or_else(|| panic!("Variable '{name}' is not defined"));

    storage.items[idx].offset
}

pub fn compile_ast(ast: ASTTree) -> SmolFile {
    let storage = compile_variables(&ast.variables);

    // When coming acorss a labe instruction, check if the label already exists
    // if it does, get the address and compile it
    // if it doesn't, save the label instr here and mutate when you find the label
    let mut labels: Vec<(&str, u16)> = Vec::new();
    let mut label_instrs: Vec<(&str, usize, bool)> = Vec::new();

    let mut instructions: Vec<u8> = Vec::new();

    for instr in &ast.instructions {
        let bytes = match instr {
            Instruction::Add(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_alu_equality(ALUType::Add, ALUSrc::Register, false, false);
                args.insert(0, op);
                args
            }
            Instruction::AddI(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_alu_equality(ALUType::Add, ALUSrc::Immidiate, false, false);
                args.insert(0, op);
                args
            }
            Instruction::EqR(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_alu_equality(ALUType::Equality, ALUSrc::Register, false, false);
                args.insert(0, op);
                args
            }
            Instruction::EqI(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_alu_equality(ALUType::Equality, ALUSrc::Immidiate, false, false);
                args.insert(0, op);
                args
            }
            Instruction::EqRL(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_alu_equality(ALUType::Equality, ALUSrc::Register, true, false);
                args.insert(0, op);
                args
            }
            Instruction::EqIL(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_alu_equality(ALUType::Equality, ALUSrc::Immidiate, true, false);
                args.insert(0, op);
                args
            }
            Instruction::Sv(name) => {
                let name = name.inner();
                let offset = variable_offset(name, &ast, &storage);
                let [li, mi] = offset.to_le_bytes();
                // Stack load variable immediate 16 bit
                [0b10101100, li, mi].into()
            }
            Instruction::Uv(_) => {
                // hardcoded UV
                [0b10110000].to_vec()
            }
            Instruction::Syscall(_) => {
                // hardcoded syscall binary
                [0b11101111].to_vec()
            }
            Instruction::St(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Store, false, false, false, false);
                args.insert(0, op);
                args
            }
            Instruction::StL(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Store, false, false, true, false);
                args.insert(0, op);
                args
            }
            Instruction::StI(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Store, false, true, false, false);
                args.insert(0, op);
                args
            }
            Instruction::StIL(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Store, false, true, true, false);
                args.insert(0, op);
                args
            }
            Instruction::Stm(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Store, true, true, false, false);
                args.insert(0, op);
                args
            }
            Instruction::StmL(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Store, true, true, true, false);
                args.insert(0, op);
                args
            }
            Instruction::Str(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Store, true, false, false, false);
                args.insert(0, op);
                args
            }
            Instruction::StrL(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Store, true, false, true, false);
                args.insert(0, op);
                args
            }
            Instruction::Ldm(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Load, true, false, false, false);
                args.insert(0, op);
                args
            }
            Instruction::LdmL(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_load_store(LoadStoreType::Load, true, false, true, false);
                args.insert(0, op);
                args
            }
            Instruction::Be(instr) => {
                let label = instr.inner();
                compile_branch_call(
                    BranchCall::BranchEq,
                    label,
                    &mut label_instrs,
                    &mut labels,
                    instructions.len(),
                )
            }
            Instruction::Bne(instr) => {
                let label = instr.inner();
                compile_branch_call(
                    BranchCall::BranchNe,
                    label,
                    &mut label_instrs,
                    &mut labels,
                    instructions.len(),
                )
            }
            Instruction::Bgt(instr) => {
                let label = instr.inner();
                compile_branch_call(
                    BranchCall::BranchGt,
                    label,
                    &mut label_instrs,
                    &mut labels,
                    instructions.len(),
                )
            }
            Instruction::Blt(instr) => {
                let label = instr.inner();
                compile_branch_call(
                    BranchCall::BranchLt,
                    label,
                    &mut label_instrs,
                    &mut labels,
                    instructions.len(),
                )
            }
            Instruction::Call(instr) => {
                let label = instr.inner();
                compile_branch_call(
                    BranchCall::Call,
                    label,
                    &mut label_instrs,
                    &mut labels,
                    instructions.len(),
                )
            }
            Instruction::Ret(_) => [0b11_110_000].into(),
            Instruction::Label(label) => {
                let exists = labels.iter().any(|(lab, _)| lab == label);
                if exists {
                    // TODO: AST should handle this
                    panic!("Duplicate label '{label}' found");
                }

                let addr = instructions.len() as u16;
                let add_le = addr.to_le_bytes();
                label_instrs.iter_mut().for_each(|(lab, addr, init)| {
                    if lab == label {
                        let bytes = &mut instructions;
                        bytes[*addr] = add_le[0];
                        bytes[*addr + 1] = add_le[1];
                        *init = true;
                    }
                });

                labels.push((label, addr));
                // TODO: handle the lable in a less hacky way
                [].into()
            }
        };

        instructions.extend(bytes.iter());
    }

    for label in label_instrs {
        if !label.2 {
            panic!("Label for '{}' was not found", label.0);
        }
    }

    let main_start = if let Some((_, addr)) = labels.iter().find(|(name, _)| *name == "main") {
        *addr
    } else {
        panic!("main label was not found");
    };

    SmolFile {
        storage,
        main_start,
        instructions,
    }
}
