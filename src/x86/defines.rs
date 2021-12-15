use crate::x86::x86_instr_PFX::InstrPFX;

pub const PFX_GRP_1_LOCK_PFX: u8 = 0xF0;
pub const PFX_GRP_1_REPNE_REPNZ_PFX: u8 = 0xF2;
pub const PFX_GRP_1_REPE_REPZ_PFX: u8 = 0xF3;
pub const PFX_GRP_2_CS_SEG_OVRD: u8 = 0x2E;
pub const PFX_GRP_2_SS_SEG_OVRD: u8 = 0x36;
pub const PFX_GRP_2_DS_SEG_OVRD: u8 = 0x3E;
pub const PFX_GRP_2_ES_SEG_OVRD: u8 = 0x26;
pub const PFX_GRP_2_FS_SEG_OVRD: u8 = 0x64;
pub const PFX_GRP_2_GS_SEG_OVRD: u8 = 0x65;
pub const PFX_GRP_2_BR_NOT_TAKEN: u8 = 0x2E;
pub const PFX_GRP_2_BR_TAKEN: u8 = 0x3E;
pub const PFX_GRP_3_OP_SZ_OVRD_PFX: u8 = 0x66;
pub const PFX_GRP_4_ADDR_SZ_OVRD_PFX: u8 = 0x67;

#[derive(Default, Debug, Clone)]
pub struct X86InstrSIB {
    /// scale[7:6]
    /// index[5:3] (reg)
    /// base[2:0] (reg)
    pub scale: u8,
    pub index: u8,
    pub base: u8,
}

impl X86InstrSIB {
    pub fn get_eff_addr(&self) -> usize {
        todo!()
    }
}

impl From<u8> for X86InstrSIB {
    fn from(_: u8) -> Self {
        todo!()
    }
}

impl Into<u8> for X86InstrSIB {
    fn into(self) -> u8 {
        todo!()
    }
}

#[derive(Clone, Debug, Default)]
pub struct X86InstrModRM {
    /// mod[7:6]:
    /// reg/opcode[5:3]:
    /// R/M[2:0]:
    pub mode: u8,
    pub reg: u8,
    pub rm: u8,
}

impl From<u8> for X86InstrModRM {
    fn from(_: u8) -> Self {
        todo!()
    }
}

impl Into<u8> for X86InstrModRM {
    fn into(self) -> u8 {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum REXPrefix {
    REX,
    REXW,
    REXR,
    REXX,
    REXB,
    REXWR,
    REXWX,
    REXWB,
    REXRX,
    REXRB,
    REXXB,
    REXRXB,
    REXWXB,
    REXWRB,
    REXWRX,
    REXWRXB,
}

impl REXPrefix {}

impl Into<u8> for REXPrefix {
    fn into(self) -> u8 {
        match self {
            REXPrefix::REX => 0b01000000,
            REXPrefix::REXW => 0b01001000,
            REXPrefix::REXR => 0b01000100,
            REXPrefix::REXX => 0b01000010,
            REXPrefix::REXB => 0b01000001,
            REXPrefix::REXWR => 0b01001100,
            REXPrefix::REXWX => 0b01001010,
            REXPrefix::REXWB => 0b01001001,
            REXPrefix::REXRX => 0b01000110,
            REXPrefix::REXRB => 0b01000101,
            REXPrefix::REXXB => 0b01000011,
            REXPrefix::REXRXB => 0b01000111,
            REXPrefix::REXWXB => 0b01001011,
            REXPrefix::REXWRB => 0b01001101,
            REXPrefix::REXWRX => 0b01001110,
            REXPrefix::REXWRXB => 0b01001111,
        }
    }
}

pub enum OpEn {
    ZO,
}

impl OpEn {}

impl Into<[u8; 4]> for OpEn {
    fn into(self) -> [u8; 4] {
        match self {
            OpEn::ZO => [0, 0, 0, 0],
        }
    }
}

pub struct X86Instruction {
    pub prefixes: [u8; 4],
    pub opcode: [u8; 3],
    pub name: String,
    pub mod_rm: Option<X86InstrModRM>,
    pub sib: Option<X86InstrSIB>,
    pub op_en: OpEn,
    pub x64_mode: String,
    pub compat_leg_mode: String,
    pub displacement: [u8; 4],
    pub immediate: [u8; 4],
    pub description: String,
}

impl X86Instruction {
    pub fn new(
        // up to four bytes of instruction prefixes
        prefixes: [u8; 4],
        // opcode: 1-, 2-, or 3- byte opcode
        opcode: [u8; 3],
        name: &str,
        // description,
        // 1 byte if required
        mod_rm: Option<X86InstrModRM>,
        // 1 byte if required
        sib: Option<X86InstrSIB>,
        op_en: OpEn,
        x64_mode: &str,
        compat_leg_mode: &str,
        // address displacement of 1, 2, or 4 bytes or none
        displacement: [u8; 4],
        // immediate data of 1, 2, or 4 bytes or none
        immediate: [u8; 4],
        // descriptive name of instruction
        description: &str,
    ) -> Self {
        Self {
            prefixes,
            opcode,
            mod_rm,
            sib,
            displacement,
            immediate,
            name: name.into_string(),
            description: description.into_string(),
            compat_leg_mode: compat_leg_mode.into_string(),
            x64_mode: x64_mode.into_string(),
            op_en,
        }
    }
}

impl From<&[u8]> for X86Instruction {
    fn from(_: &[u8]) -> Self {
        todo!()
    }
}

pub enum X86ArgType {
    NONE = 0,
    /* the literal value 1, used for bit shift ops */
    ONE,
    /* specific registers */
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    ES,
    CS,
    SS,
    DS,
    FS,
    GS,
    ALS,
    AXS, /* the same as AL/AX except MASM doesn't print them */
    DXS, /* the same as DX except GAS puts it in parentheses */
    /* absolute or relative numbers, given as 1/2/4 bytes */
    IMM8,
    IMM16,
    IMM, /* immediate number */
    REL8,
    REL,    /* relative to current instruction */
    SEGPTR, /* absolute instruction, used for far calls/jumps */
    MOFFS,  /* absolute location in memory, for A0-A3 MOV */
    /* specific memory addresses for string operations */
    DSBX,
    DSSI,
    ESDI,
    /* to be read from ModRM, appropriately */
    RM,      /* register/memory */
    MM,      /* MMX register/memory */
    XM,      /* SSE register/memory */
    MEM,     /* memory only (using 0x11xxxxxx is invalid) */
    REGONLY, /* register only (not using 0x11xxxxxx is invalid) */
    MMXONLY, /* MMX register only (not using 0x11xxxxxx is invalid) */
    XMMONLY, /* SSE register only (not using 0x11xxxxxx is invalid) */
    REG,     /* register */
    MMX,     /* MMX register */
    XMM,     /* SSE register */
    SEG16,   /* Segment register */
    REG32,   /* 32-bit only register, used for cr/dr/tr */
    CR32,    /* control register */
    DR32,    /* debug register */
    TR32,    /* test register */
    /* floating point regs */
    ST,  /* top of stack aka st(0) */
    STX, /* element of stack given by lowest three bytes of "modrm" */
}

pub struct Argument {
    pub arg_string: String,
    pub ip: u32,
    pub value: u64,
    pub arg_type: ArgType,
}

pub enum DisplacementType {
    DispNone = 0, /* no disp, i.e. mod == 0 && m != 6 */
    Disp8 = 1,    /* one byte */
    Disp16 = 2,   /* two bytes */
    DispReg = 3,  /* register, i.e. mod == 3 */
}

pub const OP_ARG2_IMM: u16 = 0x0001;
pub const OP_ARG2_IMM8: u16 = 0x0002;
pub const OP_ARG2_CL: u16 = 0x0004;
pub const OP_64: u16 = 0x0008;

pub const OP_REPNE: u16 = 0x0010;
pub const OP_REPE: u16 = 0x0020;
pub const OP_REP: u16 = OP_REPE;
pub const OP_OP32_REGONLY: u16 = 0x0040;
pub const OP_LOCK: u16 = 0x0080;

pub const OP_STACK: u16 = 0x0100;
pub const OP_STRING: u16 = 0x0200;
pub const OP_FAR: u16 = 0x0400;
pub const OP_IMM64: u16 = 0x0800;

pub const OP_S: u16 = 0x1000;
pub const OP_L: u16 = 0x2000;
pub const OP_LL: u16 = 0x3000;

pub const OP_STOP: u16 = 0x4000;
pub const OP_BR: u16 = 0x8000;

pub const PFX_ES: u16 = 0x0001;
pub const PFX_CS: u16 = 0x0002;
pub const PFX_SS: u16 = 0x0003;
pub const PFX_DS: u16 = 0x0004;
pub const PFX_FS: u16 = 0x0005;
pub const PFX_GS: u16 = 0x0006;
pub const PFX_SEG_MASK: u16 = 0x0007;

pub const PFX_OP32: u16 = 0x0008;
pub const PFX_ADDR32: u16 = 0x0010;
pub const PFX_LOCK: u16 = 0x0020;
pub const PFX_REPNE: u16 = 0x0040;
pub const PFX_REPE: u16 = 0x0080;
pub const PFX_WAIT: u16 = 0x0100;

pub const ESCAPE_OPCODE: u8 = 0x0f;
pub const MAX_INSTR: usize = 16;
pub const INSTR_SCANNED: u8 = 0x01;
pub const INSTR_VALID: u8 = 0x02;
pub const INSTR_JUMP: u8 = 0x04;
pub const INSTR_FUNC: u8 = 0x08;
pub const INSTR_FAR: u8 = 0x10;
pub const INSTR_RELOC: u8 = 0x20;

pub enum ModRmDisp {
    NoDisp = 0,
    Disp8 = 1,
    Disp16 = 2,
    DispReg = 3,
}

pub enum ModRmEffAddr {
    BxSi = 0,
    BxDi = 1,
    BpSi = 2,
    BpDi = 3,
    Si = 4,
    Di = 5,
    BpDisp16 = 6,
    Bx = 7,
}
