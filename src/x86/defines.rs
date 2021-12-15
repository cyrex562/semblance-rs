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

pub struct X86InstrModRM {
    /// mod[7:6]:
    /// reg/opcode[5:3]:
    /// R/M[2:0]:
    pub mode: u8,
    pub reg: u8,
    pub rm: u8,
}

// operand opcode:u16, subcode: u16, size: i8, name: String, arg0: ArgumentType, arg1: ArgumentType, flags: u32
pub struct X86Instruction {
    pub prefixes: [u8; 4],
    pub opcode: [u8; 3],
    pub mod_rm: u8,
    pub sib: u8,
    pub displacement: [u8; 4],
    pub immediate: [u8; 4],
    pub name: String,
}

impl X86Instruction {
    pub fn new(
        // up to four bytes of instruction prefixes
        prefixes: [u8; 4],
        // opcode: 1-, 2-, or 3- byte opcode
        opcode: [u8; 3],
        // 1 byte if required
        mod_rm: u8,
        // 1 byte if required
        sib: u8,
        // address displacement of 1, 2, or 4 bytes or none
        displacement: [u8; 4],
        // immediate data of 1, 2, or 4 bytes or none
        immediate: [u8; 4],
        // descriptive name of instruction
        name: &str,
    ) -> Self {
        Self {
            prefixes,
            opcode,
            mod_rm,
            sib,
            displacement,
            immediate,
            name: name.into_string(),
        }
    }

    pub fn set_sib(&mut self, scale: u8, index: u8, base: u8) {
        /// scale[7:6]: 2^scale = scale factor
        /// index[.X, 5:3]: reg containing index
        /// base[B, 2:0]: reg containing base
        /// eff_addr = scale * index + base + offset
        unimplemented!()
    }

    pub fn get_sib(&self) -> X86InstrSIB {
        unimplemented!()
    }

    pub fn get_sib_eff_addr(&self) -> usize {
        /// eff_addr = scale * index + base + offset
        unimplemented!()
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

pub const PFX_REX: u16 = 0x0800;
pub const PFX_REXB: u16 = 0x1000;
pub const PFX_REXX: u16 = 0x2000;
pub const PFX_REXR: u16 = 0x4000;
pub const PFX_REXW: u16 = 0x8000;

pub const ESCAPE_OPCODE: u8 = 0x0f;
pub const MAX_INSTR: usize = 16;
pub const INSTR_SCANNED: u8 = 0x01;
pub const INSTR_VALID: u8 = 0x02;
pub const INSTR_JUMP: u8 = 0x04;
pub const INSTR_FUNC: u8 = 0x08;
pub const INSTR_FAR: u8 = 0x10;
pub const INSTR_RELOC: u8 = 0x20;

#[derive(Default, Debug, Clone)]
pub struct Instruction {
    pub raw: [u8; 15],
    pub PFXes: [InstrPFX; 4],
    pub opcode: [u8; 3],
    pub opcode_field: u8, // really just 3 bits of the ModR/M byte
    // addressing-form specifier byte
    // mod field (bits 0-1): combines with r/m to form 32 possible values: eight reigsters and 24 addressing modes
    // reg/opcode field (bits 2-4): specifies either a register number or three more bits of opcode info
    // r/m field (bits 5-7): can specify a register as an operand or can be combined with the mod field to encode an addressing mode.
    pub mod_rm_byte: u8,
    // addresing byte used with certain encodings of the ModR/M byte, including base-plus-index and scale-plus-index forms of 32-bit addressing.
    // scale: scale factor
    // index: register number of the index register
    // base: register number of the base register
    pub sib_byte: u8,
    // displacement can be 1,2, or 4 bytes immediate following the ModR/M or SIB byte
    // if the instr specifies an immediate operand it always follows any dispalcement bytes. An immediate operand can be 1,2, or 4 bytes
    pub PFX_raw: [u8; 4],
    pub opcode_raw: [u8; 3],
    pub op: Instruction,
    pub args: [Argument; 3],
    pub addr_size: u8,
    pub modrm_disp: DisplacementType,
    pub modrm_reg: i8,
    pub sib_scale: u8,
    pub sib_index: i8,
    pub usedmem: bool,
    pub vex: bool,
    pub vex_reg: u8, // 3 bits
    pub vex_256: bool,
}

pub enum ModRmReg {
    AlAxEaxMm0Xmm0 = 0, // 000
    ClCxEcxMm1Xmm1 = 1, // 001
    DlDxEdxMm2Xmm2 = 2, // 010
    BlBxEbxMm3Xmm3 = 3, // 011
    AhSpEspMm4Xmm4 = 4, // 100
    ChBpEbpMm5Xmm5 = 5, // 101
    DhSiEsiMm6Xmm6 = 6, // 110
    BhDiEdiMm7Xmm7 = 7, // 111
}

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
