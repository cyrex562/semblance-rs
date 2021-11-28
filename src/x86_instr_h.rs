// #ifndef __X86_INSTR_H
// #define __X86_INSTR_H
//
// #include "semblance.h"



pub enum ArgumentType {
    NONE = 0,
    /* the literal value 1, used for bit shift ops */
    ONE,
    /* specific registers */
    AL, CL, DL, BL, AH, CH, DH, BH,
    AX, CX, DX, BX, SP, BP, SI, DI,
    ES, CS, SS, DS, FS, GS,
    ALS, AXS,   /* the same as AL/AX except MASM doesn't print them */
    DXS,        /* the same as DX except GAS puts it in parentheses */
    /* absolute or relative numbers, given as 1/2/4 bytes */
    IMM8, IMM16, IMM,   /* immediate number */
    REL8, REL,          /* relative to current instruction */
    SEGPTR,     /* absolute instruction, used for far calls/jumps */
    MOFFS,      /* absolute location in memory, for A0-A3 MOV */
    /* specific memory addresses for string operations */
    DSBX, DSSI, ESDI,
    /* to be read from ModRM, appropriately */
    RM,         /* register/memory */
    MM,         /* MMX register/memory */
    XM,         /* SSE register/memory */
    MEM,        /* memory only (using 0x11xxxxxx is invalid) */
    REGONLY,    /* register only (not using 0x11xxxxxx is invalid) */
    MMXONLY,    /* MMX register only (not using 0x11xxxxxx is invalid) */
    XMMONLY,    /* SSE register only (not using 0x11xxxxxx is invalid) */
    REG,        /* register */
    MMX,        /* MMX register */
    XMM,        /* SSE register */
    SEG16,      /* segment register */
    REG32,      /* 32-bit only register, used for cr/dr/tr */
    CR32,       /* control register */
    DR32,       /* debug register */
    TR32,       /* test register */
    /* floating point regs */
    ST,         /* top of stack aka st(0) */
    STX,        /* element of stack given by lowest three bytes of "modrm" */
}

/* opcode flags */

pub const OP_ARG2_IMM: u16   =  0x0001;  /* has IMM16/32 as third argument */
pub const OP_ARG2_IMM8: u16 =    0x0002;  /* has IMM8 as third argument */
pub const OP_ARG2_CL: u16 =      0x0004;  /* has CL as third argument */
pub const OP_64: u16 =           0x0008;  /* opcodes which are 64-bit by default (call, jmp), most being 32-bit */

pub const OP_REPNE: u16 =        0x0010;  /* repne prefix valid */
pub const OP_REPE: u16 =         0x0020;  /* repe prefix valid */
pub const OP_REP: u16 =          OP_REPE; /* rep prefix valid */
pub const OP_OP32_REGONLY: u16 = 0x0040;  /* operand-size prefix only valid if used with reg */
pub const OP_LOCK: u16 =          0x0080;  /* lock prefix valid */

pub const OP_STACK: u16 = 0x0100;  /* only marked for size if overridden */
pub const OP_STRING: u16 = 0x0200;  /* string operations */
pub const OP_FAR: u16 = 0x0400;  /* far operation */
pub const OP_IMM64: u16 = 0x0800;  /* IMM argument can be 64-bit */

pub const OP_S: u16 = 0x1000;  /* (FPU) op takes -s if GCC */
pub const OP_L: u16 = 0x2000;  /* (FPU) op takes -l if GCC */
pub const OP_LL: u16 = 0x3000;  /* (FPU) op takes -ll if GCC */
/* -t doesn't need to be marked */

pub const OP_STOP: u16 = 0x4000;  /* stop scanning (jmp, ret) */
pub const OP_BRANCH: u16 = 0x8000;  /* branch to target (jmp, jXX) */

// operand opcode:u16, subcode: u16, size: i8, name: String, arg0: ArgumentType, arg1: ArgumentType, flags: u32
pub struct Operand {
    pub opcode: u16,
    pub subcode: u8,
    pub size: i8,
    pub name: String,
    pub arg0: ArgumentType,
    pub arg1: ArgumentType,
    pub flags: u16,
}

impl Operand {
    pub fn new(opcode: u16, subcode: u8, size: i8, name: &str, arg0: ArgumentType, arg1: ArgumentType, flags: u16) -> Self {
        Self {
            opcode,
            subcode,
            size,
            name: name.into_string(),
            arg0,
            arg1,
            flags,
        }
    }
}

pub const PREFIX_ES: u16 = 0x0001;  /* 26 */
pub const PREFIX_CS: u16 = 0x0002;  /* 2E */
pub const PREFIX_SS: u16 = 0x0003;  /* 36 */
pub const PREFIX_DS: u16 = 0x0004;  /* 3E */
pub const PREFIX_FS: u16 = 0x0005;  /* 64 */
pub const PREFIX_GS: u16 = 0x0006;  /* 65 */
pub const PREFIX_SEG_MASK: u16 = 0x0007;

pub const PREFIX_OP32: u16 = 0x0008;  /* 66 */
pub const PREFIX_ADDR32: u16 = 0x0010;  /* 67 */
pub const PREFIX_LOCK: u16 = 0x0020;  /* F0 */
pub const PREFIX_REPNE: u16 = 0x0040;  /* F2 */
pub const PREFIX_REPE: u16 = 0x0080;  /* F3 */
pub const PREFIX_WAIT: u16 = 0x0100;  /* 9B */

pub const PREFIX_REX: u16 = 0x0800;  /* 40 */
pub const PREFIX_REXB: u16 = 0x1000;  /* 41 */
pub const PREFIX_REXX: u16 = 0x2000;  /* 42 */
pub const PREFIX_REXR: u16 = 0x4000;  /* 44 */
pub const PREFIX_REXW: u16 = 0x8000;  /* 48 */

pub enum DisplacementType {
    DispNone = 0,      /* no disp, i.e. mod == 0 && m != 6 */
    Disp8 = 1,      /* one byte */
    Disp16 = 2,      /* two bytes */
    DispReg = 3,      /* register, i.e. mod == 3 */
}

// extern const char SEG16[6][3];

pub struct Argument {
    pub arg_string: String,
    pub ip: u32,
    pub value: u64,
    pub arg_type: ArgType,
}

#[derive(Default,Debug,Clone)]
pub struct Instruction {
    pub prefix: u16,
    pub op: Operand,
    pub args: [Argument;3],
    pub addr_size: u8,
    pub modrm_disp: DisplacementType,
    pub modrm_reg: i8,
    pub sib_scale: u8,
    pub sib_index: i8,
    pub usedmem: bool,
    pub vex: bool,
    pub vex_reg: u8, // 3 bits
    pub vex_256: bool
}

// extern int get_instr(dword ip, const byte *p, struct instr *instr, int bits);
// extern void print_instr(char *ip, const byte *p, int len, byte flags, struct instr *instr, const char *comment, int bits);

/* 66 + 67 + seg + lock/rep + 2 bytes opcode + modrm + sib + 4 bytes displacement + 4 bytes immediate */
pub const MAX_INSTR: usize =       16;

/* flags relating to specific instructions */
pub const INSTR_SCANNED: u8 =  0x01;    /* byte has been scanned */
pub const INSTR_VALID: u8 =     0x02;    /* byte begins an instruction */
pub const INSTR_JUMP: u8 =      0x04;    /* instruction is jumped to */
pub const INSTR_FUNC: u8 =      0x08;    /* instruction begins a function */
pub const INSTR_FAR: u8 =       0x10;    /* instruction is target of far call/jmp */
pub const INSTR_RELOC: u8 =     0x20;    /* byte has relocation data */

// #endif /* __X86_INSTR_H */
