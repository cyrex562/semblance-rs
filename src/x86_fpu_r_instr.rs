use crate::x86_instr_arg_type::X86ArgType::{NONE, ST, STX};
use crate::x86_instr_operand::Operand;

pub const instructions_fpu_r: [Operand;64] = [
    Operand::new(0xD8, 0,  0, "fadd",       ST,     STX, 0),
    Operand::new(0xD8, 1,  0, "fmul",       ST,     STX, 0),
    Operand::new(0xD8, 2,  0, "fcom",       STX,    NONE, 0),
    Operand::new(0xD8, 3,  0, "fcomp",      STX,    NONE, 0),
    Operand::new(0xD8, 4,  0, "fsub",       ST,     STX, 0),
    Operand::new(0xD8, 5,  0, "fsubr",      ST,     STX, 0),
    Operand::new(0xD8, 6,  0, "fdiv",       ST,     STX, 0),
    Operand::new(0xD8, 7,  0, "fdivr",      ST,     STX, 0),
    Operand::new(0xD9, 0,  0, "fld",        STX,    NONE, 0),
    Operand::new(0xD9, 1,  0, "fxch",       STX,    NONE, 0),
    Operand::new(0xD9, 2,  0, "",          NONE,      NONE, 0),     /* fnop */
    Operand::new(0xD9, 3,  0, "fstp",       STX,    NONE, 0),     /* partial alias - see ref.x86asm.net */
    Operand::new(0xD9, 4,  0, "",          NONE,      NONE, 0),     /* fchs, fabs, ftst, fxam */
    Operand::new(0xD9, 5,  0, "",          NONE,      NONE, 0),     /* fldXXX */
    Operand::new(0xD9, 6,  0, "",          NONE,      NONE, 0),     /* f2xm1, fyl2x, ... */
    Operand::new(0xD9, 7,  0, "",          NONE,      NONE, 0),     /* fprem, fyl2xp1, ... */
    Operand::new(0xDA, 0,  0, "fcmovb",     ST,     STX, 0),
    Operand::new(0xDA, 1,  0, "fcmove",     ST,     STX, 0),
    Operand::new(0xDA, 2,  0, "fcmovbe",    ST,     STX, 0),
    Operand::new(0xDA, 3,  0, "fcmovu",     ST,     STX, 0),
    Operand::new(0xDA, 4,  0, "",          NONE,      NONE, 0),
    Operand::new(0xDA, 5,  0, "",          NONE,      NONE, 0),     /* fucompp */
    Operand::new(0xDA, 6,  0, "",          NONE,      NONE, 0),
    Operand::new(0xDA, 7,  0, "",          NONE,      NONE, 0),
    Operand::new(0xDB, 0,  0, "fcmovnb",    ST,     STX, 0),
    Operand::new(0xDB, 1,  0, "fcmovne",    ST,     STX, 0),
    Operand::new(0xDB, 2,  0, "fcmovnbe",   ST,     STX, 0),
    Operand::new(0xDB, 3,  0, "fcmovnu",    ST,     STX, 0),
    Operand::new(0xDB, 4,  0, "",          NONE,      NONE, 0),     /* fneni, fndisi, fnclex, fninit, fnsetpm */
    Operand::new(0xDB, 5,  0, "fucomi",     ST,     STX, 0),
    Operand::new(0xDB, 6,  0, "fcomi",      ST,     STX, 0),
    Operand::new(0xDB, 7,  0, "",          NONE,      NONE, 0),
    Operand::new(0xDC, 0,  0, "fadd",       STX,    ST, 0),
    Operand::new(0xDC, 1,  0, "fmul",       STX,    ST, 0),
    Operand::new(0xDC, 2,  0, "fcom",       STX,    NONE, 0),     /* alias */
    Operand::new(0xDC, 3,  0, "fcomp",      STX,    NONE, 0),     /* alias */
    Operand::new(0xDC, 4,  0, "fsubr",      STX,    ST, 0),    /* nasm, masm, sandpile have these reversed, gcc doesn't */
    Operand::new(0xDC, 5,  0, "fsub",       STX,    ST, 0),
    Operand::new(0xDC, 6,  0, "fdivr",      STX,    ST, 0),
    Operand::new(0xDC, 7,  0, "fdiv",       STX,    ST, 0),
    Operand::new(0xDD, 0,  0, "ffree",      STX,    NONE, 0),
    Operand::new(0xDD, 1,  0, "fxch",       STX,    NONE, 0),     /* alias */
    Operand::new(0xDD, 2,  0, "fst",        STX,    NONE, 0),
    Operand::new(0xDD, 3,  0, "fstp",       STX,    NONE, 0),
    Operand::new(0xDD, 4,  0, "fucom",      STX,    NONE, 0),
    Operand::new(0xDD, 5,  0, "fucomp",     STX,    NONE, 0),
    Operand::new(0xDD, 6,  0, "",          NONE,      NONE, 0),
    Operand::new(0xDD, 7,  0, "",          NONE,      NONE, 0),
    Operand::new(0xDE, 0,  0, "faddp",      STX,    ST, 0),
    Operand::new(0xDE, 1,  0, "fmulp",      STX,    ST, 0),
    Operand::new(0xDE, 2,  0, "fcomp",      STX,    NONE, 0),     /* alias */
    Operand::new(0xDE, 3,  0, "",          NONE,      NONE, 0),     /* fcompp */
    Operand::new(0xDE, 4,  0, "fsubrp",     STX,    ST, 0),    /* nasm, masm, sandpile have these reversed, gcc doesn't */
    Operand::new(0xDE, 5,  0, "fsubp",      STX,    ST, 0),
    Operand::new(0xDE, 6,  0, "fdivrp",     STX,    ST, 0),
    Operand::new(0xDE, 7,  0, "fdivp",      STX,    ST, 0),
    Operand::new(0xDF, 0,  0, "ffreep",     STX,    NONE, 0),     /* unofficial name */
    Operand::new(0xDF, 1,  0, "fxch",       STX,    NONE, 0),     /* alias */
    Operand::new(0xDF, 2,  0, "fstp",       STX,    NONE, 0),     /* alias */
    Operand::new(0xDF, 3,  0, "fstp",       STX,    NONE, 0),     /* alias */
    Operand::new(0xDF, 4,  0, "",          NONE,      NONE, 0),     /* fnstsw */
    Operand::new(0xDF, 5,  0, "fucomip",    ST,     STX, 0),
    Operand::new(0xDF, 6,  0, "fcomip",     ST,     STX, 0),
    Operand::new(0xDF, 7,  0, "",          NONE,      NONE, 0),
];
