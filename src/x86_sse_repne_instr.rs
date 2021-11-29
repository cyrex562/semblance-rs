use crate::x86_instr_def::OP_ARG2_IMM8;
use crate::x86_instr_arg_type::X86ArgType::{MEM, REG, RM, XM, XMM};
use crate::x86_instr_operand::Operand;

pub const INSTRUCTIONS_SSE_REPNE: [Operand;21] = [
    Operand::new(0x10, 8,  0, "movsd",      XMM,    XM, 0),
    Operand::new(0x11, 8,  0, "movsd",      XM,     XMM, 0),
    Operand::new(0x12, 8,  0, "movddup",    XMM,    XM, 0),

    Operand::new(0x2A, 8,  0, "cvtsi2sd",   XMM,    RM, 0),

    Operand::new(0x2C, 8,  0, "cvttsd2si",  REG,    XM, 0),
    Operand::new(0x2D, 8,  0, "cvtsd2si",   REG,    XM, 0),

    Operand::new(0x51, 8,  0, "sqrtsd",     XMM,    XM, 0),

    Operand::new(0x58, 8,  0, "addsd",      XMM,    XM, 0),
    Operand::new(0x59, 8,  0, "mulsd",      XMM,    XM, 0),
    Operand::new(0x5A, 8,  0, "cvtsd2ss",   XMM,    XM, 0),

    Operand::new(0x5C, 8,  0, "subsd",      XMM,    XM, 0),
    Operand::new(0x5D, 8,  0, "minsd",      XMM,    XM, 0),
    Operand::new(0x5E, 8,  0, "divsd",      XMM,    XM, 0),
    Operand::new(0x5F, 8,  0, "maxsd",      XMM,    XM, 0),

    Operand::new(0x70, 8,  0, "pshuflw",    XMM,    XM,     OP_ARG2_IMM8),

    Operand::new(0x7C, 8,  0, "haddps",     XMM,    XM, 0),
    Operand::new(0x7D, 8,  0, "hsubps",     XMM,    XM, 0),

    Operand::new(0xC2, 8,  0, "cmpsd",      XMM,    XM,     OP_ARG2_IMM8),

    Operand::new(0xD0, 8,  0, "addsubps",   XMM,    XM, 0),

/*    Operand::new(0xD6, 8,  0, "movdq2q",    MMX,    XMM), */

    Operand::new(0xE6, 8,  0, "cvtpd2dq",   XMM,    XM, 0),

    Operand::new(0xF0, 8,  0, "lddqu",      XMM,    MEM, 0),
];
