use crate::x86_instr_arg_type::X86ArgType::{MEM, NONE};
use crate::x86_instr_def::{OP_L, OP_LL, OP_S};
use crate::x86_instr_operand::Operand;

pub const INSTRUCTIONS_FPU_M: [Operand;64] = [
    Operand::new(0xD8, 0, 32, "fadd",       MEM,    NONE,      OP_S),
    Operand::new(0xD8, 1, 32, "fmul",       MEM,    NONE,      OP_S),
    Operand::new(0xD8, 2, 32, "fcom",       MEM,    NONE,      OP_S),
    Operand::new(0xD8, 3, 32, "fcomp",      MEM,    NONE,      OP_S),
    Operand::new(0xD8, 4, 32, "fsub",       MEM,    NONE,      OP_S),
    Operand::new(0xD8, 5, 32, "fsubr",      MEM,    NONE,      OP_S),
    Operand::new(0xD8, 6, 32, "fdiv",       MEM,    NONE,      OP_S),
    Operand::new(0xD8, 7, 32, "fdivr",      MEM,    NONE,      OP_S),
    Operand::new(0xD9, 0, 32, "fld",        MEM,    NONE,      OP_S),
    Operand::new(0xD9, 1, 0, "", NONE, NONE, 0),
    Operand::new(0xD9, 2, 32, "fst",        MEM,    NONE,      OP_S),
    Operand::new(0xD9, 3, 32, "fstp",       MEM,    NONE,      OP_S),
    Operand::new(0xD9, 4,  0, "fldenv",     MEM, NONE, 0),   /* 14/28 */
    Operand::new(0xD9, 5,  0, "fldcw",      MEM, NONE, 0),   /* 16 */
    Operand::new(0xD9, 6,  0, "fnstenv",    MEM, NONE, 0),   /* 14/28 */
    Operand::new(0xD9, 7,  0, "fnstcw",     MEM, NONE, 0),   /* 16 */
    Operand::new(0xDA, 0, 32, "fiadd",      MEM,    NONE,      OP_L),
    Operand::new(0xDA, 1, 32, "fimul",      MEM,    NONE,      OP_L),
    Operand::new(0xDA, 2, 32, "ficom",      MEM,    NONE,      OP_L),
    Operand::new(0xDA, 3, 32, "ficomp",     MEM,    NONE,      OP_L),
    Operand::new(0xDA, 4, 32, "fisub",      MEM,    NONE,      OP_L),
    Operand::new(0xDA, 5, 32, "fisubr",     MEM,    NONE,      OP_L),
    Operand::new(0xDA, 6, 32, "fidiv",      MEM,    NONE,      OP_L),
    Operand::new(0xDA, 7, 32, "fidivr",     MEM,    NONE,      OP_L),
    Operand::new(0xDB, 0, 32, "fild",       MEM,    NONE,      OP_L),
    Operand::new(0xDB, 1, 32, "fisttp",     MEM,    NONE,      OP_L),
    Operand::new(0xDB, 2, 32, "fist",       MEM,    NONE,      OP_L),
    Operand::new(0xDB, 3, 32, "fistp",      MEM,    NONE,      OP_L),
    Operand::new(0xDB, 4, 0, "", NONE, NONE, 0),
    Operand::new(0xDB, 5, 80, "fld",        MEM, NONE, 0),
    Operand::new(0xDB, 6, 0, "", NONE, NONE, 0),
    Operand::new(0xDB, 7, 80, "fstp",       MEM, NONE, 0),
    Operand::new(0xDC, 0, 64, "fadd",       MEM,    NONE,      OP_L),
    Operand::new(0xDC, 1, 64, "fmul",       MEM,    NONE,      OP_L),
    Operand::new(0xDC, 2, 64, "fcom",       MEM,    NONE,      OP_L),
    Operand::new(0xDC, 3, 64, "fcomp",      MEM,    NONE,      OP_L),
    Operand::new(0xDC, 4, 64, "fsub",       MEM,    NONE,      OP_L),
    Operand::new(0xDC, 5, 64, "fsubr",      MEM,    NONE,      OP_L),
    Operand::new(0xDC, 6, 64, "fdiv",       MEM,    NONE,      OP_L),
    Operand::new(0xDC, 7, 64, "fdivr",      MEM,    NONE,      OP_L),
    Operand::new(0xDD, 0, 64, "fld",        MEM,    NONE,      OP_L),
    Operand::new(0xDD, 1, 64, "fisttp",     MEM,    NONE,      OP_LL),
    Operand::new(0xDD, 2, 64, "fst",        MEM,    NONE,      OP_L),
    Operand::new(0xDD, 3, 64, "fstp",       MEM,    NONE,      OP_L),
    Operand::new(0xDD, 4,  0, "frstor",     MEM, NONE, 0),   /* 94/108 */
    Operand::new(0xDD, 5, 0, "", NONE, NONE, 0),
    Operand::new(0xDD, 6,  0, "fnsave",     MEM, NONE, 0),   /* 94/108 */
    Operand::new(0xDD, 7,  0, "fnstsw",     MEM, NONE, 0),   /* 16 */
    Operand::new(0xDE, 0, 16, "fiadd",      MEM,    NONE,      OP_S),
    Operand::new(0xDE, 1, 16, "fimul",      MEM,    NONE,      OP_S),
    Operand::new(0xDE, 2, 16, "ficom",      MEM,    NONE,      OP_S),
    Operand::new(0xDE, 3, 16, "ficomp",     MEM,    NONE,      OP_S),
    Operand::new(0xDE, 4, 16, "fisub",      MEM,    NONE,      OP_S),
    Operand::new(0xDE, 5, 16, "fisubr",     MEM,    NONE,      OP_S),
    Operand::new(0xDE, 6, 16, "fidiv",      MEM,    NONE,      OP_S),
    Operand::new(0xDE, 7, 16, "fidivr",     MEM,    NONE,      OP_S),
    Operand::new(0xDF, 0, 16, "fild",       MEM,    NONE,      OP_S),
    Operand::new(0xDF, 1, 16, "fisttp",     MEM,    NONE,      OP_S),
    Operand::new(0xDF, 2, 16, "fist",       MEM,    NONE,      OP_S),
    Operand::new(0xDF, 3, 16, "fistp",      MEM,    NONE,      OP_S),
    Operand::new(0xDF, 4,  0, "fbld",       MEM, NONE, 0),   /* 80 */
    Operand::new(0xDF, 5, 64, "fild",       MEM,    NONE,      OP_LL),
    Operand::new(0xDF, 6,  0, "fbstp",      MEM, NONE, 0),   /* 80 */
    Operand::new(0xDF, 7, 64, "fistp",      MEM,    NONE,      OP_LL),
];
