use crate::x86_instr_def::OP_ARG2_IMM8;
use crate::x86_instr_arg_type::X86ArgType::{IMM8, MEM, MM, MMX, REGONLY, RM, XM, XMM, XMMONLY};
use crate::x86_instr_operand::Operand;

pub const INSTRUCTIONS_SSE_OP32: [Operand;114] = [
    Operand::new(0x10, 8,  0, "movupd",     XMM,    XM, 0),
    Operand::new(0x11, 8,  0, "movupd",     XM,     XMM, 0),
    Operand::new(0x12, 8,  0, "movlpd",     XMM,    XM, 0),    /* fixme: movhlps */
    Operand::new(0x13, 8,  0, "movlpd",     MEM,    XMM, 0),
    Operand::new(0x14, 8,  0, "unpcklpd",   XMM,    XM, 0),
    Operand::new(0x15, 8,  0, "unpckhpd",   XMM,    XM, 0),
    Operand::new(0x16, 8,  0, "movhpd",     XMM,    XM, 0),    /* fixme: movlhps */
    Operand::new(0x17, 8,  0, "movhpd",     MEM,    XMM, 0),
    Operand::new(0x28, 8,  0, "movapd",     XMM,    XM, 0),
    Operand::new(0x29, 8,  0, "movapd",     XM,     XMM, 0),
    Operand::new(0x2A, 8,  0, "cvtpi2pd",   XMM,    MM, 0),
    Operand::new(0x2B, 8,  0, "movntpd",    MEM,    XMM, 0),
    Operand::new(0x2C, 8,  0, "cvttpd2pi",  MMX,    XM, 0),
    Operand::new(0x2D, 8,  0, "cvtpd2pi",   MMX,    XM, 0),
    Operand::new(0x2E, 8,  0, "ucomisd",    XMM,    XM, 0),
    Operand::new(0x2F, 8,  0, "comisd",     XMM,    XM, 0),
    Operand::new(0x50, 8, 32, "movmskpd",   REGONLY,XMM, 0),
    Operand::new(0x51, 8,  0, "sqrtpd",     XMM,    XM, 0),
    /* 52/3 unused */
    Operand::new(0x54, 8,  0, "andpd",      XMM,    XM, 0),
    Operand::new(0x55, 8,  0, "andnpd",     XMM,    XM, 0),
    Operand::new(0x56, 8,  0, "orpd",       XMM,    XM, 0),
    Operand::new(0x57, 8,  0, "xorpd",      XMM,    XM, 0),
    Operand::new(0x58, 8,  0, "addpd",      XMM,    XM, 0),
    Operand::new(0x59, 8,  0, "mulpd",      XMM,    XM, 0),
    Operand::new(0x5A, 8,  0, "cvtpd2ps",   XMM,    XM, 0),
    Operand::new(0x5B, 8,  0, "cvtps2dq",   XMM,    XM, 0),
    Operand::new(0x5C, 8,  0, "subpd",      XMM,    XM, 0),
    Operand::new(0x5D, 8,  0, "minpd",      XMM,    XM, 0),
    Operand::new(0x5E, 8,  0, "divpd",      XMM,    XM, 0),
    Operand::new(0x5F, 8,  0, "maxpd",      XMM,    XM, 0),
    Operand::new(0x60, 8,  0, "punpcklbw",  XMM,    XM, 0),
    Operand::new(0x61, 8,  0, "punpcklwd",  XMM,    XM, 0),
    Operand::new(0x62, 8,  0, "punpckldq",  XMM,    XM, 0),
    Operand::new(0x63, 8,  0, "packsswb",   XMM,    XM, 0),
    Operand::new(0x64, 8,  0, "pcmpgtb",    XMM,    XM, 0),
    Operand::new(0x65, 8,  0, "pcmpgtw",    XMM,    XM, 0),
    Operand::new(0x66, 8,  0, "pcmpgtd",    XMM,    XM, 0),
    Operand::new(0x67, 8,  0, "packuswb",   XMM,    XM, 0),
    Operand::new(0x68, 8,  0, "punpckhbw",  XMM,    XM, 0),
    Operand::new(0x69, 8,  0, "punpckhwd",  XMM,    XM, 0),
    Operand::new(0x6A, 8,  0, "punpckhdq",  XMM,    XM, 0),
    Operand::new(0x6B, 8,  0, "packssdw",   XMM,    XM, 0),
    Operand::new(0x6C, 8,  0, "punpcklqdq", XMM,    XM, 0),
    Operand::new(0x6D, 8,  0, "punpckhqdq", XMM,    XM, 0),
    Operand::new(0x6E, 8, -1, "mov",        XMM,    RM, 0),
    Operand::new(0x6F, 8,  0, "movdqa",     XMM,    XM, 0),
    Operand::new(0x70, 8,  0, "pshufd",     XMM,    XM,    OP_ARG2_IMM8),
    Operand::new(0x71, 2,  0, "psrlw",      XMMONLY,IMM8, 0),
    Operand::new(0x71, 4,  0, "psraw",      XMMONLY,IMM8, 0),
    Operand::new(0x71, 6,  0, "psllw",      XMMONLY,IMM8, 0),
    Operand::new(0x72, 2,  0, "psrld",      XMMONLY,IMM8, 0),
    Operand::new(0x72, 4,  0, "psrad",      XMMONLY,IMM8, 0),
    Operand::new(0x72, 6,  0, "pslld",      XMMONLY,IMM8, 0),
    Operand::new(0x73, 2,  0, "psrlq",      XMMONLY,IMM8, 0),
    Operand::new(0x73, 3,  0, "psrldq",     XMMONLY,IMM8, 0),
    Operand::new(0x73, 6,  0, "psllq",      XMMONLY,IMM8, 0),
    Operand::new(0x73, 7,  0, "pslldq",     XMMONLY,IMM8, 0),
    Operand::new(0x74, 8,  0, "pcmpeqb",    XMM,    XM, 0),
    Operand::new(0x75, 8,  0, "pcmpeqw",    XMM,    XM, 0),
    Operand::new(0x76, 8,  0, "pcmpeqd",    XMM,    XM, 0),

    Operand::new(0x7C, 8,  0, "haddpd",     XMM,    XM, 0),
    Operand::new(0x7D, 8,  0, "hsubpd",     XMM,    XM, 0),
    Operand::new(0x7E, 8, -1, "mov",        RM,     XMM, 0),
    Operand::new(0x7F, 8,  0, "movdqa",     XM,     XMM, 0),

    Operand::new(0xC2, 8,  0, "cmppd",      XMM,    XM,     OP_ARG2_IMM8),
    /* C3 unused */
    Operand::new(0xC4, 8,  0, "pinsrw",     XMM,    RM,     OP_ARG2_IMM8),
    Operand::new(0xC5, 8,  0, "pextrw",     REGONLY,XMM,    OP_ARG2_IMM8),
    Operand::new(0xC6, 8,  0, "shufpd",     XMM,    XM,     OP_ARG2_IMM8),

    Operand::new(0xD0, 8,  0, "addsubpd",   XMM,    XM, 0),
    Operand::new(0xD1, 8,  0, "psrlw",      XMM,    XM, 0),
    Operand::new(0xD2, 8,  0, "psrld",      XMM,    XM, 0),
    Operand::new(0xD3, 8,  0, "psrlq",      XMM,    XM, 0),
    Operand::new(0xD4, 8,  0, "paddd",      XMM,    XM, 0),
    Operand::new(0xD5, 8,  0, "pmullw",     XMM,    XM, 0),
    Operand::new(0xD6, 8,  0, "movq",       XM,     XMM, 0),
    Operand::new(0xD7, 8, 32, "pmovmskb",   REGONLY,XMM, 0),
    Operand::new(0xD8, 8,  0, "psubusb",    XMM,    XM, 0),
    Operand::new(0xD9, 8,  0, "psubusw",    XMM,    XM, 0),
    Operand::new(0xDA, 8,  0, "pminub",     XMM,    XM, 0),
    Operand::new(0xDB, 8,  0, "pand",       XMM,    XM, 0),
    Operand::new(0xDC, 8,  0, "paddusb",    XMM,    XM, 0),
    Operand::new(0xDD, 8,  0, "paddusw",    XMM,    XM, 0),
    Operand::new(0xDE, 8,  0, "pmaxub",     XMM,    XM, 0),
    Operand::new(0xDF, 8,  0, "pandn",      XMM,    XM, 0),
    Operand::new(0xE0, 8,  0, "pavgb",      XMM,    XM, 0),
    Operand::new(0xE1, 8,  0, "psraw",      XMM,    XM, 0),
    Operand::new(0xE2, 8,  0, "psrad",      XMM,    XM, 0),
    Operand::new(0xE3, 8,  0, "pavgw",      XMM,    XM, 0),
    Operand::new(0xE4, 8,  0, "pmulhuw",    XMM,    XM, 0),
    Operand::new(0xE5, 8,  0, "pmulhw",     XMM,    XM, 0),
    Operand::new(0xE6, 8,  0, "cvttpd2dq",  XMM,    XM, 0),
    Operand::new(0xE7, 8,  0, "movntdq",    MEM,    XMM, 0),
    Operand::new(0xE8, 8,  0, "psubsb",     XMM,    XM, 0),
    Operand::new(0xE9, 8,  0, "psubsw",     XMM,    XM, 0),
    Operand::new(0xEA, 8,  0, "pminsw",     XMM,    XM, 0),
    Operand::new(0xEB, 8,  0, "por",        XMM,    XM, 0),
    Operand::new(0xEC, 8,  0, "paddsb",     XMM,    XM, 0),
    Operand::new(0xED, 8,  0, "paddsw",     XMM,    XM, 0),
    Operand::new(0xEE, 8,  0, "pmaxsw",     XMM,    XM, 0),
    Operand::new(0xEF, 8,  0, "pxor",       XMM,    XM, 0),
    /* F0 unused */
    Operand::new(0xF1, 8,  0, "psllw",      XMM,    XM, 0),
    Operand::new(0xF2, 8,  0, "pslld",      XMM,    XM, 0),
    Operand::new(0xF3, 8,  0, "psllq",      XMM,    XM, 0),
    Operand::new(0xF4, 8,  0, "pmuludq",    XMM,    XM, 0),
    Operand::new(0xF5, 8,  0, "pmaddwd",    XMM,    XM, 0),
    Operand::new(0xF6, 8,  0, "psadbw",     XMM,    XM, 0),
    Operand::new(0xF7, 8,  0, "maskmovdqu", XMM,    XMMONLY, 0),
    Operand::new(0xF8, 8,  0, "psubb",      XMM,    XM, 0),
    Operand::new(0xF9, 8,  0, "psubw",      XMM,    XM, 0),
    Operand::new(0xFA, 8,  0, "psubd",      XMM,    XM, 0),
    Operand::new(0xFB, 8,  0, "psubq",      XMM,    XM, 0),
    Operand::new(0xFC, 8,  0, "paddb",      XMM,    XM, 0),
    Operand::new(0xFD, 8,  0, "paddw",      XMM,    XM, 0),
    Operand::new(0xFE, 8,  0, "paddd",      XMM,    XM, 0),
];
