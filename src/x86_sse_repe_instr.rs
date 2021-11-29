use crate::x86_instr_def::OP_ARG2_IMM8;
use crate::x86_instr_arg_type::X86ArgType::{REG, RM, XM, XMM};
use crate::x86_instr_operand::Operand;

pub const INSTRUCTIONS_SSE_REPE: [Operand;25] = [
    Operand::new(0x10, 8,  0, "movss",      XMM,    XM, 0),
    Operand::new(0x11, 8,  0, "movss",      XM,     XMM, 0),
    Operand::new(0x12, 8,  0, "movsldup",   XMM,    XM, 0),
    Operand::new(0x16, 8,  0, "movshdup",   XMM,    XM, 0),
    Operand::new(0x2A, 8,  0, "cvtsi2ss",   XMM,    RM, 0),
    Operand::new(0x2C, 8,  0, "cvttss2si",  REG,    XM, 0),
    Operand::new(0x2D, 8,  0, "cvtss2si",   REG,    XM, 0),
    Operand::new(0x51, 8,  0, "sqrtss",     XMM,    XM, 0),
    Operand::new(0x52, 8,  0, "rsqrtss",    XMM,    XM, 0),
    Operand::new(0x53, 8,  0, "rcpss",      XMM,    XM, 0),
    Operand::new(0x58, 8,  0, "addss",      XMM,    XM, 0),
    Operand::new(0x59, 8,  0, "mulss",      XMM,    XM, 0),
    Operand::new(0x5A, 8,  0, "cvtss2sd",   XMM,    XM, 0),
    Operand::new(0x5B, 8,  0, "cvttps2dq",  XMM,    XM, 0),
    Operand::new(0x5C, 8,  0, "subss",      XMM,    XM, 0),
    Operand::new(0x5D, 8,  0, "minss",      XMM,    XM, 0),
    Operand::new(0x5E, 8,  0, "divss",      XMM,    XM, 0),
    Operand::new(0x5F, 8,  0, "maxss",      XMM,    XM, 0),
    Operand::new(0x6F, 8,  0, "movdqu",     XMM,    XM, 0),
    Operand::new(0x70, 8,  0, "pshufhw",    XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x7E, 8,  0, "movq",       XMM,    XM, 0),
    Operand::new(0x7F, 8,  0, "movdqu",     XM,     XMM, 0),
    Operand::new(0xB8, 8, 16, "popcnt",     REG,    RM, 0),    /* not SSE */
    Operand::new(0xC2, 8,  0, "cmpss",      XMM,    XM,     OP_ARG2_IMM8),
/*    Operand::new(0xD6, 8,  0, "movq2dq",    XMM,    MMX), */
    Operand::new(0xE6, 8,  0, "cvtdq2pd",   XMM,    XM, 0),
];
