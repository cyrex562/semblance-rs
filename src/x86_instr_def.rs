 // #ifndef __X86_INSTR_H
// #define __X86_INSTR_H
//
// #include "semblance.h"



 use crate::x86_instr_operand::Operand;
 use crate::x86_instr_prefix::InstrPrefix;


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

 pub const ESCAPE_OPCODE: u8 = 0x0f;


#[derive(Default,Debug,Clone)]
pub struct Instruction {
    pub raw: [u8;15],
    pub prefixes: [InstrPrefix;4],
    pub opcode: [u8;3],
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

    pub prefix_raw: [u8;4],
    pub opcode_raw: [u8;3],
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
