pub enum X86ArgType {
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
   SEG16,      /* Segment register */
   REG32,      /* 32-bit only register, used for cr/dr/tr */
   CR32,       /* control register */
   DR32,       /* debug register */
   TR32,       /* test register */
   /* floating point regs */
   ST,         /* top of stack aka st(0) */
   STX,        /* element of stack given by lowest three bytes of "modrm" */
}
