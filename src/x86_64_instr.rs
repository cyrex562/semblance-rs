use crate::x86_instr_arg_type::X86ArgType::{AH, AL, ALS, AX, AXS, BH, BL, BP, BX, CH, CL, CX, DH, DI, DL, DSBX, DSSI, DX, DXS, ESDI, IMM, IMM16, IMM8, MEM, MOFFS, NONE, REG, REL, REL8, RM, SI, SP};
use crate::x86_instr_def::{OP_ARG2_IMM, OP_ARG2_IMM8, OP_BRANCH, OP_FAR, OP_IMM64, OP_LOCK, OP_OP32_REGONLY, OP_REP, OP_REPE, OP_REPNE, OP_STACK, OP_STOP, OP_STRING};
use crate::x86_instr_operand::Operand;

pub const X86_64_INSTRUCTIONS: [Operand;256] = [
    Operand::new(0x00, 8,  8, "add",        RM,     REG,    OP_LOCK),
    Operand::new(0x01, 8, -1, "add",        RM,     REG,    OP_LOCK),
    Operand::new(0x02, 8,  8, "add",        REG,    RM, 0),
    Operand::new(0x03, 8, -1, "add",        REG,    RM, 0),
    Operand::new(0x04, 8,  8, "add",        AL,     IMM, 0),
    Operand::new(0x05, 8, -1, "add",        AX,     IMM, 0),
    Operand::new(0x06, 8, 0, "", NONE, NONE, 0),  /* undefined (was push es) */
    Operand::new(0x07, 8, 0, "", NONE, NONE, 0),  /* undefined (was pop es) */
    Operand::new(0x08, 8,  8, "or",         RM,     REG,    OP_LOCK),
    Operand::new(0x09, 8, -1, "or",         RM,     REG,    OP_LOCK),
    Operand::new(0x0A, 8,  8, "or",         REG,    RM, 0),
    Operand::new(0x0B, 8, -1, "or",         REG,    RM, 0),
    Operand::new(0x0C, 8,  8, "or",         AL,     IMM, 0),
    Operand::new(0x0D, 8, -1, "or",         AX,     IMM, 0),
    Operand::new(0x0E, 8, 0, "", NONE, NONE, 0),  /* undefined (was push cs) */
    Operand::new(0x0F, 8, 0, "", NONE, NONE, 0),  /* two-byte codes */
    Operand::new(0x10, 8,  8, "adc",        RM,     REG,    OP_LOCK),
    Operand::new(0x11, 8, -1, "adc",        RM,     REG,    OP_LOCK),
    Operand::new(0x12, 8,  8, "adc",        REG,    RM, 0),
    Operand::new(0x13, 8, -1, "adc",        REG,    RM, 0),
    Operand::new(0x14, 8,  8, "adc",        AL,     IMM, 0),
    Operand::new(0x15, 8, -1, "adc",        AX,     IMM, 0),
    Operand::new(0x16, 8, 0, "", NONE, NONE, 0),  /* undefined (was push ss) */
    Operand::new(0x17, 8, 0, "", NONE, NONE, 0),  /* undefined (was pop ss) */
    Operand::new(0x18, 8,  8, "sbb",        RM,     REG,    OP_LOCK),
    Operand::new(0x19, 8, -1, "sbb",        RM,     REG,    OP_LOCK),
    Operand::new(0x1A, 8,  8, "sbb",        REG,    RM, 0),
    Operand::new(0x1B, 8, -1, "sbb",        REG,    RM, 0),
    Operand::new(0x1C, 8,  8, "sbb",        AL,     IMM,0),
    Operand::new(0x1D, 8, -1, "sbb",        AX,     IMM,0),
    Operand::new(0x1E, 8, 0, "", NONE, NONE, 0),  /* undefined (was push ds) */
    Operand::new(0x1F, 8, 0, "", NONE, NONE, 0),  /* undefined (was pop ds) */
    Operand::new(0x20, 8,  8, "and",        RM,     REG,    OP_LOCK),
    Operand::new(0x21, 8, -1, "and",        RM,     REG,    OP_LOCK),
    Operand::new(0x22, 8,  8, "and",        REG,    RM, 0),
    Operand::new(0x23, 8, -1, "and",        REG,    RM, 0),
    Operand::new(0x24, 8,  8, "and",        AL,     IMM, 0),
    Operand::new(0x25, 8, -1, "and",        AX,     IMM, 0),
    Operand::new(0x26, 80, 0, "", NONE, NONE, 0),  /* undefined (was es:) */
    Operand::new(0x27, 8, 0, "", NONE, NONE, 0),  /* undefined (was daa) */
    Operand::new(0x28, 8,  8, "sub",        RM,     REG,    OP_LOCK),
    Operand::new(0x29, 8, -1, "sub",        RM,     REG,    OP_LOCK),
    Operand::new(0x2A, 8,  8, "sub",        REG,    RM, 0),
    Operand::new(0x2B, 8, -1, "sub",        REG,    RM, 0),
    Operand::new(0x2C, 8,  8, "sub",        AL,     IMM, 0),
    Operand::new(0x2D, 8, -1, "sub",        AX,     IMM, 0),
    Operand::new(0x2E, 8, 0, "", NONE, NONE, 0),  /* undefined (was cs:) */
    Operand::new(0x2F, 8, 0, "", NONE, NONE, 0),  /* undefined (was das) */
    Operand::new(0x30, 8,  8, "xor",        RM,     REG,    OP_LOCK),
    Operand::new(0x31, 8, -1, "xor",        RM,     REG,    OP_LOCK),
    Operand::new(0x32, 8,  8, "xor",        REG,    RM, 0),
    Operand::new(0x33, 8, -1, "xor",        REG,    RM, 0),
    Operand::new(0x34, 8,  8, "xor",        AL,     IMM, 0),
    Operand::new(0x35, 8, -1, "xor",        AX,     IMM, 0),
    Operand::new(0x36, 8, 0, "", NONE, NONE, 0),  /* undefined (was ss:) */
    Operand::new(0x37, 8, 0, "", NONE, NONE, 0),  /* undefined (was aaa) */
    Operand::new(0x38, 8,  8, "cmp",        RM,     REG, 0),
    Operand::new(0x39, 8, -1, "cmp",        RM,     REG, 0),
    Operand::new(0x3A, 8,  8, "cmp",        REG,    RM, 0),
    Operand::new(0x3B, 8, -1, "cmp",        REG,    RM, 0),
    Operand::new(0x3C, 8,  8, "cmp",        AL,     IMM, 0),
    Operand::new(0x3D, 8, -1, "cmp",        AX,     IMM, 0),
    Operand::new(0x3E, 8, 0, "", NONE, NONE, 0),  /* undefined (was ds:) */
    Operand::new(0x3F, 8, 0, "", NONE, NONE, 0),  /* undefined (was aas) */
    Operand::new(0x40, 8,  0, "rex", NONE, NONE, 0),
    Operand::new(0x41, 8,  0, "rex.B", NONE, NONE, 0),
    Operand::new(0x42, 8,  0, "rex.X", NONE, NONE, 0),
    Operand::new(0x43, 8,  0, "rex.XB", NONE, NONE, 0),
    Operand::new(0x44, 8,  0, "rex.R", NONE, NONE, 0),
    Operand::new(0x45, 8,  0, "rex.RB", NONE, NONE, 0),
    Operand::new(0x46, 8,  0, "rex.RX", NONE, NONE, 0),
    Operand::new(0x47, 8,  0, "rex.RXB", NONE, NONE, 0),
    Operand::new(0x48, 8,  0, "rex.W", NONE, NONE, 0),
    Operand::new(0x49, 8,  0, "rex.WB", NONE, NONE, 0),
    Operand::new(0x4A, 8,  0, "rex.WX", NONE, NONE, 0),
    Operand::new(0x4B, 8,  0, "rex.WXB", NONE, NONE, 0),
    Operand::new(0x4C, 8,  0, "rex.WR", NONE, NONE, 0),
    Operand::new(0x4D, 8,  0, "rex.WRB", NONE, NONE, 0),
    Operand::new(0x4E, 8,  0, "rex.WRX", NONE, NONE, 0),
    Operand::new(0x4F, 8,  0, "rex.WRXB", NONE, NONE, 0),
    Operand::new(0x50, 8, -1, "push",       AX,     NONE,      OP_STACK),
    Operand::new(0x51, 8, -1, "push",       CX,     NONE,      OP_STACK),
    Operand::new(0x52, 8, -1, "push",       DX,     NONE,      OP_STACK),
    Operand::new(0x53, 8, -1, "push",       BX,     NONE,      OP_STACK),
    Operand::new(0x54, 8, -1, "push",       SP,     NONE,      OP_STACK),
    Operand::new(0x55, 8, -1, "push",       BP,     NONE,      OP_STACK),
    Operand::new(0x56, 8, -1, "push",       SI,     NONE,      OP_STACK),
    Operand::new(0x57, 8, -1, "push",       DI,     NONE,      OP_STACK),
    Operand::new(0x58, 8, -1, "pop",        AX,     NONE,      OP_STACK),
    Operand::new(0x59, 8, -1, "pop",        CX,     NONE,      OP_STACK),
    Operand::new(0x5A, 8, -1, "pop",        DX,     NONE,      OP_STACK),
    Operand::new(0x5B, 8, -1, "pop",        BX,     NONE,      OP_STACK),
    Operand::new(0x5C, 8, -1, "pop",        SP,     NONE,      OP_STACK),
    Operand::new(0x5D, 8, -1, "pop",        BP,     NONE,      OP_STACK),
    Operand::new(0x5E, 8, -1, "pop",        SI,     NONE,      OP_STACK),
    Operand::new(0x5F, 8, -1, "pop",        DI,     NONE,      OP_STACK),
    Operand::new(0x60, 8, 0, "", NONE, NONE, 0),  /* undefined (was pusha) */
    Operand::new(0x61, 8, 0, "", NONE, NONE, 0),  /* undefined (was popa) */
    Operand::new(0x62, 8, 0, "", NONE, NONE, 0),  /* undefined (was bound) */
    Operand::new(0x63, 8, -1, "movsx",      REG,    RM, 0),
    Operand::new(0x64, 8,  0, "fs", NONE, NONE, 0),  /* FS prefix */
    Operand::new(0x65, 8,  0, "gs", NONE, NONE, 0),  /* GS prefix */
    Operand::new(0x66, 8,  0, "data", NONE, NONE, 0),  /* op-size prefix */
    Operand::new(0x67, 8,  0, "addr", NONE, NONE, 0),  /* addr-size prefix */
    Operand::new(0x68, 8, -1, "push",       IMM,    NONE,      OP_STACK),
    Operand::new(0x69, 8, -1, "imul",       REG,    RM,     OP_ARG2_IMM),
    Operand::new(0x6A, 8, -1, "push",       IMM8,   NONE,      OP_STACK),
    Operand::new(0x6B, 8, -1, "imul",       REG,    RM,     OP_ARG2_IMM8),
    Operand::new(0x6C, 8,  8, "ins",        ESDI,   DXS,    OP_STRING|OP_REP),
    Operand::new(0x6D, 8, -1, "ins",        ESDI,   DXS,    OP_STRING|OP_REP),
    Operand::new(0x6E, 8,  8, "outs",       DXS,    DSSI,   OP_STRING|OP_REP),
    Operand::new(0x6F, 8, -1, "outs",       DXS,    DSSI,   OP_STRING|OP_REP),
    Operand::new(0x70, 8,  0, "jo",         REL8,   NONE,      OP_BRANCH),
    Operand::new(0x71, 8,  0, "jno",        REL8,   NONE,      OP_BRANCH),
    Operand::new(0x72, 8,  0, "jb",         REL8,   NONE,      OP_BRANCH),
    Operand::new(0x73, 8,  0, "jae",        REL8,   NONE,      OP_BRANCH),
    Operand::new(0x74, 8,  0, "jz",         REL8,   NONE,      OP_BRANCH),
    Operand::new(0x75, 8,  0, "jnz",        REL8,   NONE,      OP_BRANCH),
    Operand::new(0x76, 8,  0, "jbe",        REL8,   NONE,      OP_BRANCH),
    Operand::new(0x77, 8,  0, "ja",         REL8,   NONE,      OP_BRANCH),
    Operand::new(0x78, 8,  0, "js",         REL8,   NONE,      OP_BRANCH),
    Operand::new(0x79, 8,  0, "jns",        REL8,   NONE,      OP_BRANCH),
    Operand::new(0x7A, 8,  0, "jp",         REL8,   NONE,      OP_BRANCH),
    Operand::new(0x7B, 8,  0, "jnp",        REL8,   NONE,      OP_BRANCH),
    Operand::new(0x7C, 8,  0, "jl",         REL8,   NONE,      OP_BRANCH),
    Operand::new(0x7D, 8,  0, "jge",        REL8,   NONE,      OP_BRANCH),
    Operand::new(0x7E, 8,  0, "jle",        REL8,   NONE,      OP_BRANCH),
    Operand::new(0x7F, 8,  0, "jg",         REL8,   NONE,      OP_BRANCH),
    Operand::new(0x80, 8, 0, "", NONE, NONE, 0),  /* arithmetic operations */
    Operand::new(0x81, 8, 0, "", NONE, NONE, 0),
    Operand::new(0x82, 8, 0, "", NONE, NONE, 0),  /* undefined (was alias for 80) */
    Operand::new(0x83, 8,  0, "", NONE, NONE, 0),
    Operand::new(0x84, 8,  8, "test",       RM,     REG, 0),
    Operand::new(0x85, 8, -1, "test",       RM,     REG, 0),
    Operand::new(0x86, 8,  8, "xchg",       REG,    RM,     OP_LOCK),
    Operand::new(0x87, 8, -1, "xchg",       REG,    RM,     OP_LOCK),
    Operand::new(0x88, 8,  8, "mov",        RM,     REG, 0),
    Operand::new(0x89, 8, -1, "mov",        RM,     REG, 0),
    Operand::new(0x8A, 8,  8, "mov",        REG,    RM, 0),
    Operand::new(0x8B, 8, -1, "mov",        REG,    RM, 0),
    Operand::new(0x8C, 8, -1, "mov",        RM,     SEG16, 0),
    Operand::new(0x8D, 8, -1, "lea",        REG,    MEM, 0),
    Operand::new(0x8E, 8, -1, "mov",        SEG16,  RM,     OP_OP32_REGONLY),
    Operand::new(0x8F, 8,  0, "", NONE, NONE, 0),  /* pop (subcode 0 only) */
    Operand::new(0x90, 8, -1, "nop",        NONE,      NONE,      OP_REP),
    Operand::new(0x91, 8, -1, "xchg",       AX,     CX, 0),
    Operand::new(0x92, 8, -1, "xchg",       AX,     DX, 0),
    Operand::new(0x93, 8, -1, "xchg",       AX,     BX, 0),
    Operand::new(0x94, 8, -1, "xchg",       AX,     SP, 0),
    Operand::new(0x95, 8, -1, "xchg",       AX,     BP, 0),
    Operand::new(0x96, 8, -1, "xchg",       AX,     SI, 0),
    Operand::new(0x97, 8, -1, "xchg",       AX,     DI, 0),
    Operand::new(0x98, 8, -1, "cbw", NONE, NONE, 0),       /* handled separately */
    Operand::new(0x99, 8, -1, "cwd",NONE, NONE, 0),       /* handled separately */
    Operand::new(0x9A, 8, 0, "", NONE, NONE, 0),  /* undefined (was call SEGPTR) */
    Operand::new(0x9B, 8,  0, "wait", NONE, NONE, 0),  /* wait ~prefix~ */
    Operand::new(0x9C, 8, -1, "pushf",      NONE,      NONE,      OP_STACK),
    Operand::new(0x9D, 8, -1, "popf",       NONE,      NONE,      OP_STACK),
    Operand::new(0x9E, 8,  0, "sahf", NONE, NONE, 0),
    Operand::new(0x9F, 8,  0, "lahf", NONE, NONE, 0),
    Operand::new(0xA0, 8,  8, "mov",        AL,     MOFFS, 0),
    Operand::new(0xA1, 8, -1, "mov",        AX,     MOFFS, 0),
    Operand::new(0xA2, 8,  8, "mov",        MOFFS,  AL, 0),
    Operand::new(0xA3, 8, -1, "mov",        MOFFS,  AX, 0),
    Operand::new(0xA4, 8,  8, "movs",       DSSI,   ESDI,   OP_STRING|OP_REP),
    Operand::new(0xA5, 8, -1, "movs",       DSSI,   ESDI,   OP_STRING|OP_REP),
    Operand::new(0xA6, 8,  8, "cmps",       DSSI,   ESDI,   OP_STRING|OP_REPNE|OP_REPE),
    Operand::new(0xA7, 8, -1, "cmps",       DSSI,   ESDI,   OP_STRING|OP_REPNE|OP_REPE),
    Operand::new(0xA8, 8,  8, "test",       AL,     IMM, 0),
    Operand::new(0xA9, 8, -1, "test",       AX,     IMM, 0),
    Operand::new(0xAA, 8,  8, "stos",       ESDI,   ALS,    OP_STRING|OP_REP),
    Operand::new(0xAB, 8, -1, "stos",       ESDI,   AXS,    OP_STRING|OP_REP),
    Operand::new(0xAC, 8,  8, "lods",       ALS,    DSSI,   OP_STRING|OP_REP),
    Operand::new(0xAD, 8, -1, "lods",       AXS,    DSSI,   OP_STRING|OP_REP),
    Operand::new(0xAE, 8,  8, "scas",       ALS,    ESDI,   OP_STRING|OP_REPNE|OP_REPE),
    Operand::new(0xAF, 8, -1, "scas",       AXS,    ESDI,   OP_STRING|OP_REPNE|OP_REPE),
    Operand::new(0xB0, 8,  8, "mov",        AL,     IMM, 0),
    Operand::new(0xB1, 8,  8, "mov",        CL,     IMM, 0),
    Operand::new(0xB2, 8,  8, "mov",        DL,     IMM, 0),
    Operand::new(0xB3, 8,  8, "mov",        BL,     IMM, 0),
    Operand::new(0xB4, 8,  8, "mov",        AH,     IMM, 0),
    Operand::new(0xB5, 8,  8, "mov",        CH,     IMM, 0),
    Operand::new(0xB6, 8,  8, "mov",        DH,     IMM, 0),
    Operand::new(0xB7, 8,  8, "mov",        BH,     IMM, 0),
    Operand::new(0xB8, 8, -1, "mov",        AX,     IMM,    OP_IMM64),
    Operand::new(0xB9, 8, -1, "mov",        CX,     IMM,    OP_IMM64),
    Operand::new(0xBA, 8, -1, "mov",        DX,     IMM,    OP_IMM64),
    Operand::new(0xBB, 8, -1, "mov",        BX,     IMM,    OP_IMM64),
    Operand::new(0xBC, 8, -1, "mov",        SP,     IMM,    OP_IMM64),
    Operand::new(0xBD, 8, -1, "mov",        BP,     IMM,    OP_IMM64),
    Operand::new(0xBE, 8, -1, "mov",        SI,     IMM,    OP_IMM64),
    Operand::new(0xBF, 8, -1, "mov",        DI,     IMM,    OP_IMM64),
    Operand::new(0xC0, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xC1, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xC2, 8,  0, "ret",        IMM16,  NONE,      OP_STOP),
    Operand::new(0xC3, 8,  0, "ret",        NONE,      NONE,      OP_STOP|OP_REPE|OP_REPNE),
    Operand::new(0xC4, 8, 0, "", NONE, NONE, 0),  /* undefined (was les) */
    Operand::new(0xC5, 8, 0, "", NONE, NONE, 0),  /* undefined (was lds) */
    Operand::new(0xC6, 0, 0, "", NONE, NONE, 0),  /* mov (subcode 0 only) */
    Operand::new(0xC7, 0, 0, "", NONE, NONE, 0),  /* mov (subcode 0 only) */
    Operand::new(0xC8, 8,  0, "enter",      IMM16,  IMM8, 0),
    Operand::new(0xC9, 8,  0, "leave", NONE, NONE, 0),
    Operand::new(0xCA, 8,  0, "ret",        IMM16,  NONE,      OP_STOP|OP_FAR),    /* a change in bitness should only happen across Segment boundaries */
    Operand::new(0xCB, 8,  0, "ret",        NONE,      NONE,      OP_STOP|OP_FAR),
    Operand::new(0xCC, 8,  0, "int3",       NONE,      NONE,      OP_STOP),
    Operand::new(0xCD, 8,  0, "int",        IMM8, NONE, 0),
    Operand::new(0xCE, 8,  0, "into", NONE, NONE, 0),
    Operand::new(0xCF, 8,  0, "iret",       NONE,      NONE,      OP_STOP),
    Operand::new(0xD0, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xD1, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xD2, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xD3, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xD4, 8, 0, "", NONE, NONE, 0),  /* undefined (was aam) */
    Operand::new(0xD5, 8, 0, "", NONE, NONE, 0),  /* undefined (was aad) */
    Operand::new(0xD6, 8, 0, "", NONE, NONE, 0),  /* undefined (was salc?) */
    Operand::new(0xD7, 8,  0, "xlatb",      DSBX, NONE,0),
    Operand::new(0xD8, 8, 0, "", NONE, NONE, 0),  /* float ops */
    Operand::new(0xD9, 8, 0, "", NONE, NONE, 0),  /* float ops */
    Operand::new(0xDA, 8, 0, "", NONE, NONE, 0),  /* float ops */
    Operand::new(0xDB, 8, 0, "", NONE, NONE, 0),  /* float ops */
    Operand::new(0xDC, 8, 0, "", NONE, NONE, 0),  /* float ops */
    Operand::new(0xDD, 8, 0, "", NONE, NONE, 0),  /* float ops */
    Operand::new(0xDE, 8, 0, "", NONE, NONE, 0),  /* float ops */
    Operand::new(0xDF, 8, 0, "", NONE, NONE, 0),  /* float ops */
    Operand::new(0xE0, 8,  0, "loopnz",     REL8,   NONE,      OP_BRANCH),  /* fixme: how to print this? */
    Operand::new(0xE1, 8,  0, "loopz",      REL8,   NONE,      OP_BRANCH),
    Operand::new(0xE2, 8,  0, "loop",       REL8,   NONE,      OP_BRANCH),
    Operand::new(0xE3, 8, -1, "jcxz",       REL8,   NONE,      OP_BRANCH),  /* name handled separately */
    Operand::new(0xE4, 8,  8, "in",         AL,     IMM, 0),
    Operand::new(0xE5, 8, -1, "in",         AX,     IMM, 0),
    Operand::new(0xE6, 8,  8, "out",        IMM,    AL, 0),
    Operand::new(0xE7, 8, -1, "out",        IMM,    AX, 0),
    Operand::new(0xE8, 8, -1, "call",       REL,
                 NONE,      OP_BRANCH),
    Operand::new(0xE9, 8, -1, "jmp",        REL,    NONE,      OP_BRANCH|OP_STOP),
    Operand::new(0xEA, 8, 0, "", NONE, NONE, 0),  /* undefined (was jmp/SEGPTR) */
    Operand::new(0xEB, 8,  0, "jmp",        REL8,
                 NONE,      OP_BRANCH|OP_STOP),
    Operand::new(0xEC, 8,  8, "in",         AL,     DXS, 0),
    Operand::new(0xED, 8, -1, "in",         AX,     DXS, 0),
    Operand::new(0xEE, 8,  8, "out",        DXS,    AL, 0),
    Operand::new(0xEF, 8, -1, "out",        DXS,    AX, 0),
    Operand::new(0xF0, 8,  0, "lock", NONE, NONE, 0),      /* lock prefix */
    Operand::new(0xF1, 8, 0, "", NONE, NONE, 0),  /* undefined (fixme: int1/icebp?) */
    Operand::new(0xF2, 8,  0, "repne", NONE,
    NONE, 0),     /* repne prefix */
    Operand::new(0xF3, 8,  0, "repe", NONE, NONE, 0),      /* repe prefix */
    Operand::new(0xF4, 8,  0, "hlt", NONE, NONE, 0),
    Operand::new(0xF5, 8,  0, "cmc", NONE, NONE, 0),
    Operand::new(0xF6, 8, 0, "", NONE, NONE, 0),  /* group #3 */
    Operand::new(0xF7, 8, 0, "", NONE, NONE, 0),  /* group #3 */
    Operand::new(0xF8, 8,  0, "clc", NONE, NONE, 0),
    Operand::new(0xF9, 8,  0, "stc", NONE, NONE, 0),
    Operand::new(0xFA, 8,  0, "cli", NONE, NONE, 0),
    Operand::new(0xFB, 8,  0, "sti", NONE, NONE, 0),
    Operand::new(0xFC, 8,  0, "cld", NONE, NONE, 0),
    Operand::new(0xFD, 8,  0, "std", NONE, NONE, 0),
    Operand::new(0xFE, 8, 0, "", NONE, NONE, 0),  /* inc/dec */
    Operand::new(0xFF, 8, 0, "", NONE, NONE, 0),  /* group #5 */
];
