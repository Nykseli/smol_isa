#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Registers {
    // Special registers
    /// (16,ro) - Instruction Counter
    pub ic: u16,
    /// (16,ro) - Core Flags
    /// (1 >> 0) Is equal
    /// (1 >> 1) Is greater than
    /// (1 >> 2) Is less than
    pub fg: u16,
    /// (16,rw) - Call Register
    pub cr: u16,
    /// (16,rw) - Stack pointer
    pub sp: u16,
    /// (16,rw) - Variable pointer
    pub vp: u16,
    /// (16,rw) - Zero register (n-bits wide)
    pub zr: u16,

    // General purpose registers
    /// 0th 8-bit general
    pub r0: u8,
    /// 1st 8-bit general
    pub r1: u8,
    /// 2nd 8-bit general
    pub r2: u8,
    /// 3th 8-bit general
    pub r3: u8,
    /// 4th 8-bit general
    pub r4: u8,
    /// 5th 8-bit general
    pub r5: u8,
    /// 6th 8-bit general
    pub r6: u8,
    /// 7th 8-bit general
    pub r7: u8,
    /// 0th 16-bit general
    pub l0: u16,
    /// 1th 16-bit general
    pub l1: u16,
}
