/*
 * Functions to parse and print x86 instructions
 *
 * Copyright 2017-2020 Zebediah Figura
 *
 * This file is part of Semblance.
 *
 * Semblance is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Semblance is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Semblance; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301, USA
 */

// #include <string.h>
// #include "x86_instr.h"

/* this is easier than doing bitfields */
// #define MODOF(x)    ((x) >> 6)
pub fn MODOF(x: u8) -> u8 {
    x >> 6
}
// #define REGOF(x)    (((x) >> 3) & 7)
pub fn REGOF(x: u8) -> u8 {
    (x >> 3) & 7
}
// #define MEMOF(x)    ((x) & 7)
pub fn MEMOF(x: u8) -> u8 {
    x & 7
}

use std::mem;
use libc::printf;
use crate::semblance::AsmSyntax::{GAS, MASM, NASM};
use crate::semblance::{COMPILABLE, NO_SHOW_ADDRESSES, NO_SHOW_RAW_INSN};
use crate::x86_instr_h::{Argument, ArgumentType, INSTR_FAR, INSTR_JUMP, Instruction, OP_64, OP_ARG2_CL, OP_ARG2_IMM, OP_ARG2_IMM8, OP_BRANCH, OP_FAR, OP_IMM64, OP_L, OP_LL, OP_LOCK, OP_OP32_REGONLY, OP_REP, OP_REPE, OP_REPNE, OP_S, OP_STACK, OP_STOP, OP_STRING, Operand, PREFIX_ADDR32, PREFIX_CS, PREFIX_DS, PREFIX_ES, PREFIX_FS, PREFIX_GS, PREFIX_LOCK, PREFIX_OP32, PREFIX_REPE, PREFIX_REPNE, PREFIX_REX, PREFIX_REXB, PREFIX_REXR, PREFIX_REXX, PREFIX_SEG_MASK, PREFIX_SS, PREFIX_WAIT};
use crate::x86_instr_h::ArgumentType::{AH, AL, ALS, AX, AXS, BH, BL, BP, BX, CH, CL, CR32, CS, CX, DH, DI, DL, DR32, DS, DSBX, DSSI, DX, DXS, ES, ESDI, FS, GS, IMM, IMM16, IMM8, MEM, MM, MMX, MMXONLY, MOFFS, NONE, ONE, REG, REG32, REGONLY, REL, REL8, RM, SEG16, SEGPTR, SI, SP, SS, ST, STX, TR32, XM, XMM, XMMONLY};

// operand opcode:u16, subcode: u16, size: i8, name: String, arg0: ArgumentType, arg1: ArgumentType, flags: u32
pub const INSTRUCTIONS32: [Operand;256] = [
    Operand::new(0x00, 8,  8, "add",        RM,     REG,    OP_LOCK),
    Operand::new(0x01, 8, -1, "add",        RM,     REG,    OP_LOCK),
    Operand::new(0x02, 8,  8, "add",        REG,    RM, 0),
    Operand::new(0x03, 8, -1, "add",        REG,    RM, 0),
    Operand::new(0x04, 8,  8, "add",        AL,     IMM, 0),
    Operand::new(0x05, 8, -1, "add",        AX,     IMM, 0),
    Operand::new(0x06, 8, -1, "push",       ES,     NONE,      OP_STACK),
    Operand::new(0x07, 8, -1, "pop",        ES,     NONE,      OP_STACK),
    Operand::new(0x08, 8,  8, "or",         RM,     REG,    OP_LOCK),
    Operand::new(0x09, 8, -1, "or",         RM,     REG,    OP_LOCK),
    Operand::new(0x0A, 8,  8, "or",         REG,    RM, 0),
    Operand::new(0x0B, 8, -1, "or",         REG,    RM, 0),
    Operand::new(0x0C, 8,  8, "or",         AL,     IMM, 0),
    Operand::new(0x0D, 8, -1, "or",         AX,     IMM, 0),
    Operand::new(0x0E, 8, -1, "push",       CS,     NONE,      OP_STACK),
    Operand::new(0x0F, 8, 0, "", NONE, NONE, 0),  /* two-byte codes */
    Operand::new(0x10, 8,  8, "adc",        RM,     REG,    OP_LOCK),
    Operand::new(0x11, 8, -1, "adc",        RM,     REG,    OP_LOCK),
    Operand::new(0x12, 8,  8, "adc",        REG,    RM, 0),
    Operand::new(0x13, 8, -1, "adc",        REG,    RM, 0),
    Operand::new(0x14, 8,  8, "adc",        AL,     IMM, 0),
    Operand::new(0x15, 8, -1, "adc",        AX,     IMM, 0),
    Operand::new(0x16, 8, -1, "push",       SS,     NONE,      OP_STACK),
    Operand::new(0x17, 8, -1, "pop",        SS,     NONE,      OP_STACK),
    Operand::new(0x18, 8,  8, "sbb",        RM,     REG,    OP_LOCK),
    Operand::new(0x19, 8, -1, "sbb",        RM,     REG,    OP_LOCK),
    Operand::new(0x1A, 8,  8, "sbb",        REG,    RM, 0),
    Operand::new(0x1B, 8, -1, "sbb",        REG,    RM, 0),
    Operand::new(0x1C, 8,  8, "sbb",        AL,     IMM, 0),
    Operand::new(0x1D, 8, -1, "sbb",        AX,     IMM, 0),
    Operand::new(0x1E, 8, -1, "push",       DS,     NONE,      OP_STACK),
    Operand::new(0x2F, 8, -1, "pop",        DS,     NONE,      OP_STACK),
    Operand::new(0x20, 8,  8, "and",        RM,     REG,    OP_LOCK),
    Operand::new(0x21, 8, -1, "and",        RM,     REG,    OP_LOCK),
    Operand::new(0x22, 8,  8, "and",        REG,    RM, 0),
    Operand::new(0x23, 8, -1, "and",        REG,    RM, 0),
    Operand::new(0x24, 8,  8, "and",        AL,     IMM, 0),
    Operand::new(0x25, 8, -1, "and",        AX,     IMM, 0),
    Operand::new(0x26, 8,  0, "es", NONE, NONE, 0),  /* ES prefix */
    Operand::new(0x27, 8,  0, "daa", NONE, NONE, 0),
    Operand::new(0x28, 8,  8, "sub",        RM,     REG,    OP_LOCK),
    Operand::new(0x29, 8, -1, "sub",        RM,     REG,    OP_LOCK),
    Operand::new(0x2A, 8,  8, "sub",        REG,    RM, 0),
    Operand::new(0x2B, 8, -1, "sub",        REG,    RM, 0),
    Operand::new(0x2C, 8,  8, "sub",        AL,     IMM, 0),
    Operand::new(0x2D, 8, -1, "sub",        AX,     IMM, 0),
    Operand::new(0x2E, 8,  0, "cs", NONE, NONE, 0),  /* CS prefix */
    Operand::new(0x2F, 8,  0, "das", NONE, NONE, 0),
    Operand::new(0x30, 8,  8, "xor",        RM,     REG,    OP_LOCK),
    Operand::new(0x31, 8, -1, "xor",        RM,     REG,    OP_LOCK),
    Operand::new(0x32, 8,  8, "xor",        REG,    RM, 0),
    Operand::new(0x33, 8, -1, "xor",        REG,    RM, 0),
    Operand::new(0x34, 8,  8, "xor",        AL,     IMM, 0),
    Operand::new(0x35, 8, -1, "xor",        AX,     IMM, 0),
    Operand::new(0x36, 8,  0, "ss", NONE, NONE, 0),  /* SS prefix */
    Operand::new(0x37, 8,  0, "aaa", NONE, NONE, 0),
    Operand::new(0x38, 8,  8, "cmp",        RM,     REG, 0),
    Operand::new(0x39, 8, -1, "cmp",        RM,     REG, 0),
    Operand::new(0x3A, 8,  8, "cmp",        REG,    RM, 0),
    Operand::new(0x3B, 8, -1, "cmp",        REG,    RM, 0),
    Operand::new(0x3C, 8,  8, "cmp",        AL,     IMM, 0),
    Operand::new(0x3D, 8, -1, "cmp",        AX,     IMM, 0),
    Operand::new(0x3E, 8,  0, "ds", NONE, NONE, 0),  /* DS prefix */
    Operand::new(0x3F, 8,  0, "aas", NONE, NONE, 0),
    Operand::new(0x40, 8, -1, "inc",        AX, NONE, 0),
    Operand::new(0x41, 8, -1, "inc",        CX, NONE, 0),
    Operand::new(0x42, 8, -1, "inc",        DX, NONE, 0),
    Operand::new(0x43, 8, -1, "inc",        BX, NONE, 0),
    Operand::new(0x44, 8, -1, "inc",        SP, NONE, 0),
    Operand::new(0x45, 8, -1, "inc",        BP, NONE, 0),
    Operand::new(0x46, 8, -1, "inc",        SI, NONE, 0),
    Operand::new(0x47, 8, -1, "inc",        DI, NONE, 0),
    Operand::new(0x48, 8, -1, "dec",        AX, NONE, 0),
    Operand::new(0x49, 8, -1, "dec",        CX, NONE, 0),
    Operand::new(0x4A, 8, -1, "dec",        DX, NONE, 0),
    Operand::new(0x4B, 8, -1, "dec",        BX, NONE, 0),
    Operand::new(0x4C, 8, -1, "dec",        SP, NONE, 0),
    Operand::new(0x4D, 8, -1, "dec",        BP, NONE, 0),
    Operand::new(0x4E, 8, -1, "dec",        SI, NONE, 0),
    Operand::new(0x4F, 8, -1, "dec",        DI, NONE, 0),
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
    Operand::new(0x60, 8, -1, "pusha",      NONE,      NONE,      OP_STACK),
    Operand::new(0x61, 8, -1, "popa",       NONE,      NONE,      OP_STACK),
    Operand::new(0x62, 8, -1, "bound",      REG,    MEM, 0),
    Operand::new(0x63, 8, 16, "arpl",       RM,     REG, 0),
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
    Operand::new(0x82, 8, 0, "", NONE, NONE, 0),  /* alias for 80 */
    Operand::new(0x83, 8, 0, "", NONE, NONE, 0),
    Operand::new(0x84, 8,  8, "test",       RM,     REG, 0),
    Operand::new(0x85, 8, -1, "test",       RM,     REG, 0),
    Operand::new(0x86, 8,  8, "xchg",       REG,    RM,     OP_LOCK),
    Operand::new(0x87, 8, -1, "xchg",       REG,    RM,     OP_LOCK),
    Operand::new(0x88, 8,  8, "mov",        RM,     REG, 0),
    Operand::new(0x89, 8, -1, "mov",        RM,     REG, 0),
    Operand::new(0x8A, 8,  8, "mov",        REG,    RM, 0),
    Operand::new(0x8B, 8, -1, "mov",        REG,    RM, 0),
    Operand::new(0x8C, 8, -1, "mov",        RM,     SEG16, 0), /* fixme: should we replace eax with ax? */
    Operand::new(0x8D, 8, -1, "lea",        REG,    MEM, 0),
    Operand::new(0x8E, 8, -1, "mov",        SEG16,  RM,     OP_OP32_REGONLY),
    Operand::new(0x8F, 8, 0, "", NONE, NONE, 0),  /* pop (subcode 0 only) */
    Operand::new(0x90, 8, -1, "nop",        NONE,      NONE,      OP_REP),
    Operand::new(0x91, 8, -1, "xchg",       AX,     CX, 0),
    Operand::new(0x92, 8, -1, "xchg",       AX,     DX, 0),
    Operand::new(0x93, 8, -1, "xchg",       AX,     BX, 0),
    Operand::new(0x94, 8, -1, "xchg",       AX,     SP, 0),
    Operand::new(0x95, 8, -1, "xchg",       AX,     BP, 0),
    Operand::new(0x96, 8, -1, "xchg",       AX,     SI, 0),
    Operand::new(0x97, 8, -1, "xchg",       AX,     DI, 0),
    Operand::new(0x98, 8, -1, "cbw", NONE, NONE, 0),       /* handled separately */
    Operand::new(0x99, 8, -1, "cwd", NONE, NONE, 0),       /* handled separately */
    Operand::new(0x9A, 8, -1, "call",       SEGPTR, NONE,      OP_FAR),
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
    Operand::new(0xB8, 8, -1, "mov",        AX,     IMM, 0),
    Operand::new(0xB9, 8, -1, "mov",        CX,     IMM, 0),
    Operand::new(0xBA, 8, -1, "mov",        DX,     IMM, 0),
    Operand::new(0xBB, 8, -1, "mov",        BX,     IMM, 0),
    Operand::new(0xBC, 8, -1, "mov",        SP,     IMM, 0),
    Operand::new(0xBD, 8, -1, "mov",        BP,     IMM, 0),
    Operand::new(0xBE, 8, -1, "mov",        SI,     IMM, 0),
    Operand::new(0xBF, 8, -1, "mov",        DI,     IMM, 0),
    Operand::new(0xC0, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xC1, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xC2, 8,  0, "ret",        IMM16,  NONE,      OP_STOP),           /* fixme: can take OP32... */
    Operand::new(0xC3, 8,  0, "ret",        NONE,      NONE,      OP_STOP|OP_REPE|OP_REPNE),
    Operand::new(0xC4, 8, -1, "les",        REG,    MEM, 0),
    Operand::new(0xC5, 8, -1, "lds",        REG,    MEM, 0),
    Operand::new(0xC6, 0, 0, "", NONE, NONE, 0),  /* mov (subcode 0 only) */
    Operand::new(0xC7, 0, 0, "", NONE, NONE, 0),  /* mov (subcode 0 only) */
    Operand::new(0xC8, 8,  0, "enter",      IMM16,  IMM8, 0),
    Operand::new(0xC9, 8,  0, "leave", NONE, NONE, 0),
    Operand::new(0xCA, 8,  0, "ret",        IMM16,  NONE,      OP_STOP|OP_FAR),    /* a change in bitness should only happen across segment boundaries */
    Operand::new(0xCB, 8,  0, "ret",        NONE,      NONE,      OP_STOP|OP_FAR),
    Operand::new(0xCC, 8,  0, "int3",       NONE,      NONE,      OP_STOP),
    Operand::new(0xCD, 8,  0, "int",        IMM8, NONE, 0),
    Operand::new(0xCE, 8,  0, "into", NONE, NONE, 0),
    Operand::new(0xCF, 8,  0, "iret",       NONE,      NONE,      OP_STOP),
    Operand::new(0xD0, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xD1, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xD2, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xD3, 8, 0, "", NONE, NONE, 0),  /* rotate/shift */
    Operand::new(0xD4, 8,  0, "amx",        IMM8, NONE, 0),  /* unofficial name */
    Operand::new(0xD5, 8,  0, "adx",        IMM8, NONE, 0),  /* unofficial name */
    Operand::new(0xD6, 8, 0, "", NONE, NONE, 0),  /* undefined (fixme: salc?) */
    Operand::new(0xD7, 8,  0, "xlatb",      DSBX, NONE, 0),
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
    Operand::new(0xE8, 8, -1, "call",       REL,    NONE,      OP_BRANCH),
    Operand::new(0xE9, 8, -1, "jmp",        REL,    NONE,      OP_BRANCH|OP_STOP),
    Operand::new(0xEA, 8, -1, "jmp",        SEGPTR, NONE,      OP_FAR|OP_STOP),    /* a change in bitness should only happen across segment boundaries */
    Operand::new(0xEB, 8,  0, "jmp",        REL8,   NONE,      OP_BRANCH|OP_STOP),
    Operand::new(0xEC, 8,  8, "in",         AL,     DXS, 0),
    Operand::new(0xED, 8, -1, "in",         AX,     DXS, 0),
    Operand::new(0xEE, 8,  8, "out",        DXS,    AL, 0),
    Operand::new(0xEF, 8, -1, "out",        DXS,    AX, 0),
    Operand::new(0xF0, 8,  0, "lock", NONE, NONE, 0),      /* lock prefix */
    Operand::new(0xF1, 8, 0, "", NONE, NONE, 0),  /* undefined (fixme: int1/icebp?) */
    Operand::new(0xF2, 8,  0, "repne", NONE, NONE, 0),     /* repne prefix */
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
    Operand::new(0xFF, 8, 0, "", NONE, NONE, 0)  /* group #5 */
];

pub const INSTRUCTIONS64: [Operand;256] = [
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
    Operand::new(0xCA, 8,  0, "ret",        IMM16,  NONE,      OP_STOP|OP_FAR),    /* a change in bitness should only happen across segment boundaries */
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

pub const instructions_group: [Operand;108] = [
    Operand::new(0x80, 0,  8, "add",        RM,     IMM,    OP_LOCK),
    Operand::new(0x80, 1,  8, "or",         RM,     IMM,    OP_LOCK),
    Operand::new(0x80, 2,  8, "adc",        RM,     IMM,    OP_LOCK),
    Operand::new(0x80, 3,  8, "sbb",        RM,     IMM,    OP_LOCK),
    Operand::new(0x80, 4,  8, "and",        RM,     IMM,    OP_LOCK),
    Operand::new(0x80, 5,  8, "sub",        RM,     IMM,    OP_LOCK),
    Operand::new(0x80, 6,  8, "xor",        RM,     IMM,    OP_LOCK),
    Operand::new(0x80, 7,  8, "cmp",        RM,     IMM, 0),
    Operand::new(0x81, 0, -1, "add",        RM,     IMM,    OP_LOCK),
    Operand::new(0x81, 1, -1, "or",         RM,     IMM,    OP_LOCK),
    Operand::new(0x81, 2, -1, "adc",        RM,     IMM,    OP_LOCK),
    Operand::new(0x81, 3, -1, "sbb",        RM,     IMM,    OP_LOCK),
    Operand::new(0x81, 4, -1, "and",        RM,     IMM,    OP_LOCK),
    Operand::new(0x81, 5, -1, "sub",        RM,     IMM,    OP_LOCK),
    Operand::new(0x81, 6, -1, "xor",        RM,     IMM,    OP_LOCK),
    Operand::new(0x81, 7, -1, "cmp",        RM,     IMM, 0),
    Operand::new(0x82, 0,  8, "add",        RM,     IMM8,   OP_LOCK), /*  aliased */
    Operand::new(0x82, 1,  8, "or",         RM,     IMM8,   OP_LOCK),
    Operand::new(0x82, 2,  8, "adc",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x82, 3,  8, "sbb",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x82, 4,  8, "and",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x82, 5,  8, "sub",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x82, 6,  8, "xor",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x82, 7,  8, "cmp",        RM,     IMM8, 0),
    Operand::new(0x83, 0, -1, "add",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x83, 1, -1, "or",         RM,     IMM8,   OP_LOCK),
    Operand::new(0x83, 2, -1, "adc",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x83, 3, -1, "sbb",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x83, 4, -1, "and",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x83, 5, -1, "sub",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x83, 6, -1, "xor",        RM,     IMM8,   OP_LOCK),
    Operand::new(0x83, 7, -1, "cmp",        RM,     IMM8, 0),

    Operand::new(0x8F, 0, -1, "pop",        RM,     NONE,      OP_STACK),

    Operand::new(0xC0, 0,  8, "rol",        RM,     IMM8, 0),
    Operand::new(0xC0, 1,  8, "ror",        RM,     IMM8, 0),
    Operand::new(0xC0, 2,  8, "rcl",        RM,     IMM8, 0),
    Operand::new(0xC0, 3,  8, "rcr",        RM,     IMM8, 0),
    Operand::new(0xC0, 4,  8, "shl",        RM,     IMM8, 0),
    Operand::new(0xC0, 5,  8, "shr",        RM,     IMM8, 0),
    Operand::new(0xC0, 6,  8, "sal",        RM,     IMM8, 0), /* aliased to shl */
    Operand::new(0xC0, 7,  8, "sar",        RM,     IMM8, 0),
    Operand::new(0xC1, 0, -1, "rol",        RM,     IMM8, 0),
    Operand::new(0xC1, 1, -1, "ror",        RM,     IMM8, 0),
    Operand::new(0xC1, 2, -1, "rcl",        RM,     IMM8, 0),
    Operand::new(0xC1, 3, -1, "rcr",        RM,     IMM8, 0),
    Operand::new(0xC1, 4, -1, "shl",        RM,     IMM8, 0),
    Operand::new(0xC1, 5, -1, "shr",        RM,     IMM8, 0),
    Operand::new(0xC1, 6, -1, "sal",        RM,     IMM8, 0), /* aliased to shl */
    Operand::new(0xC1, 7, -1, "sar",        RM,     IMM8, 0),

    Operand::new(0xC6, 0,  8, "mov",        RM,     IMM, 0),
    Operand::new(0xC7, 0, -1, "mov",        RM,     IMM, 0),

    Operand::new(0xD0, 0,  8, "rol",        RM,     ONE, 0),
    Operand::new(0xD0, 1,  8, "ror",        RM,     ONE, 0),
    Operand::new(0xD0, 2,  8, "rcl",        RM,     ONE, 0),
    Operand::new(0xD0, 3,  8, "rcr",        RM,     ONE, 0),
    Operand::new(0xD0, 4,  8, "shl",        RM,     ONE, 0),
    Operand::new(0xD0, 5,  8, "shr",        RM,     ONE, 0),
    Operand::new(0xD0, 6,  8, "sal",        RM,     ONE, 0), /* aliased to shl */
    Operand::new(0xD0, 7,  8, "sar",        RM,     ONE, 0),
    Operand::new(0xD1, 0, -1, "rol",        RM,     ONE, 0),
    Operand::new(0xD1, 1, -1, "ror",        RM,     ONE, 0),
    Operand::new(0xD1, 2, -1, "rcl",        RM,     ONE, 0),
    Operand::new(0xD1, 3, -1, "rcr",        RM,     ONE, 0),
    Operand::new(0xD1, 4, -1, "shl",        RM,     ONE, 0),
    Operand::new(0xD1, 5, -1, "shr",        RM,     ONE, 0),
    Operand::new(0xD1, 6, -1, "sal",        RM,     ONE, 0), /* aliased to shl */
    Operand::new(0xD1, 7, -1, "sar",        RM,     ONE, 0),
    Operand::new(0xD2, 0,  8, "rol",        RM,     CL, 0),
    Operand::new(0xD2, 1,  8, "ror",        RM,     CL, 0),
    Operand::new(0xD2, 2,  8, "rcl",        RM,     CL, 0),
    Operand::new(0xD2, 3,  8, "rcr",        RM,     CL, 0),
    Operand::new(0xD2, 4,  8, "shl",        RM,     CL, 0),
    Operand::new(0xD2, 5,  8, "shr",        RM,     CL, 0),
    Operand::new(0xD2, 6,  8, "sal",        RM,     CL, 0), /* aliased to shl */
    Operand::new(0xD2, 7,  8, "sar",        RM,     CL, 0),
    Operand::new(0xD3, 0, -1, "rol",        RM,     CL, 0),
    Operand::new(0xD3, 1, -1, "ror",        RM,     CL, 0),
    Operand::new(0xD3, 2, -1, "rcl",        RM,     CL, 0),
    Operand::new(0xD3, 3, -1, "rcr",        RM,     CL, 0),
    Operand::new(0xD3, 4, -1, "shl",        RM,     CL, 0),
    Operand::new(0xD3, 5, -1, "shr",        RM,     CL, 0),
    Operand::new(0xD3, 6, -1, "sal",        RM,     CL, 0), /* aliased to shl */
    Operand::new(0xD3, 7, -1, "sar",        RM,     CL, 0),
    Operand::new(0xF6, 0,  8, "test",       RM,     IMM, 0),
    Operand::new(0xF6, 1,  8, "test",       RM,     IMM, 0),   /* aliased to 0 */
    Operand::new(0xF6, 2,  8, "not",        RM,     NONE,      OP_LOCK),
    Operand::new(0xF6, 3,  8, "neg",        RM,     NONE,      OP_LOCK),
    Operand::new(0xF6, 4,  8, "mul",        RM, NONE, 0),
    Operand::new(0xF6, 5,  8, "imul",       RM, NONE, 0),
    Operand::new(0xF6, 6,  8, "div",        RM, NONE, 0),
    Operand::new(0xF6, 7,  8, "idiv",       RM, NONE, 0),
    Operand::new(0xF7, 0, -1, "test",       RM,     IMM, 0),
    Operand::new(0xF7, 1, -1, "test",       RM,     IMM, 0),   /* aliased to 0 */
    Operand::new(0xF7, 2, -1, "not",        RM,     NONE,      OP_LOCK),
    Operand::new(0xF7, 3, -1, "neg",        RM,     NONE,      OP_LOCK),
    Operand::new(0xF7, 4, -1, "mul",        RM, NONE, 0),
    Operand::new(0xF7, 5, -1, "imul",       RM, NONE, 0),
    Operand::new(0xF7, 6, -1, "div",        RM, NONE, 0),
    Operand::new(0xF7, 7, -1, "idiv",       RM, NONE, 0),
    Operand::new(0xFE, 0,  8, "inc",        RM,     NONE,      OP_LOCK),
    Operand::new(0xFE, 1,  8, "dec",        RM,     NONE,      OP_LOCK),
    Operand::new(0xFF, 0, -1, "inc",        RM,     NONE,      OP_LOCK),
    Operand::new(0xFF, 1, -1, "dec",        RM,     NONE,      OP_LOCK),
    Operand::new(0xFF, 2, -1, "call",       RM,     NONE,      OP_64),
    Operand::new(0xFF, 3, -1, "call",       MEM,    NONE,      OP_64|OP_FAR),          /* a change in bitness should only happen across segment boundaries */
    Operand::new(0xFF, 4, -1, "jmp",        RM,     NONE,      OP_64|OP_STOP),
    Operand::new(0xFF, 5, -1, "jmp",        MEM,    NONE,      OP_64|OP_STOP|OP_FAR),  /* a change in bitness should only happen across segment boundaries */
    Operand::new(0xFF, 6, -1, "push",       RM,     NONE,      OP_STACK),
];

/* a subcode value of 8 means all subcodes,
 * or the subcode marks the register if there is one present. */
pub const INSTRUCTIONS_0F: [Operand;133] = [
    Operand::new(0x00, 0, -1, "sldt",       RM,     NONE,      OP_OP32_REGONLY),       /* todo: implement this flag */
    Operand::new(0x00, 1, -1, "str",        RM,     NONE,      OP_OP32_REGONLY),
    Operand::new(0x00, 2, 16, "lldt",       RM, NONE, 0),
    Operand::new(0x00, 3, 16, "ltr",        RM, NONE, 0),
    Operand::new(0x00, 4, 16, "verr",       RM, NONE, 0),
    Operand::new(0x00, 5, 16, "verw",       RM, NONE, 0),
    /* 00/6 unused */
    /* 00/7 unused */
    Operand::new(0x01, 0,  0, "sgdt",       MEM, NONE, 0),
    Operand::new(0x01, 1,  0, "sidt",       MEM, NONE, 0),
    Operand::new(0x01, 2,  0, "lgdt",       MEM, NONE, 0),
    Operand::new(0x01, 3,  0, "lidt",       MEM, NONE, 0),
    Operand::new(0x01, 4, -1, "smsw",       RM,     NONE,      OP_OP32_REGONLY),
    /* 01/5 unused */
    Operand::new(0x01, 6, 16, "lmsw",       RM, NONE, 0),
    Operand::new(0x01, 7,  0, "invlpg",     MEM, NONE, 0),
    Operand::new(0x02, 8, -1, "lar",        REG,    RM,     OP_OP32_REGONLY),       /* fixme: should be RM16 */
    Operand::new(0x03, 8, -1, "lsl",        REG,    RM,     OP_OP32_REGONLY),       /* fixme: should be RM16 */
    /* 04 unused */
    Operand::new(0x05, 8,  0, "syscall", NONE, NONE, 0),
    Operand::new(0x06, 8,  0, "clts", NONE, NONE, 0),
    Operand::new(0x07, 8,  0, "sysret", NONE, NONE, 0),
    Operand::new(0x08, 8,  0, "invd", NONE, NONE, 0),
    Operand::new(0x09, 8,  0, "wbinvd", NONE, NONE, 0),

    Operand::new(0x0d, 8, -1, "prefetch",   RM, NONE, 0),    /* Intel has NOP here; we're just following GCC */

    Operand::new(0x18, 0,  8, "prefetchnta",MEM, NONE, 0),
    Operand::new(0x18, 1,  8, "prefetcht0", MEM, NONE, 0),
    Operand::new(0x18, 2,  8, "prefetcht1", MEM, NONE, 0),
    Operand::new(0x18, 3,  8, "prefetcht2", MEM, NONE, 0),

    Operand::new(0x1f, 8, -1, "nop",        RM, NONE, 0),

    Operand::new(0x20, 8, -1, "mov",        REG32,  CR32, 0),  /* here mod is simply ignored */
    Operand::new(0x21, 8, -1, "mov",        REG32,  DR32, 0),
    Operand::new(0x22, 8, -1, "mov",        CR32,   REG32, 0),
    Operand::new(0x23, 8, -1, "mov",        DR32,   REG32, 0),
    Operand::new(0x24, 8, -1, "mov",        REG32,  TR32, 0),
    /* 25 unused */
    Operand::new(0x26, 8, -1, "mov",        TR32,   REG32, 0),

    Operand::new(0x30, 8, -1, "wrmsr", NONE, NONE, 0),
    Operand::new(0x31, 8, -1, "rdtsc", NONE, NONE, 0),
    Operand::new(0x32, 8, -1, "rdmsr", NONE, NONE, 0),
    Operand::new(0x33, 8, -1, "rdpmc", NONE, NONE, 0),
    Operand::new(0x34, 8, -1, "sysenter", NONE, NONE, 0),
    Operand::new(0x35, 8, -1, "sysexit", NONE, NONE, 0),

    Operand::new(0x40, 8, -1, "cmovo",      REG,    RM, 0),
    Operand::new(0x41, 8, -1, "cmovno",     REG,    RM, 0),
    Operand::new(0x42, 8, -1, "cmovb",      REG,    RM, 0),
    Operand::new(0x43, 8, -1, "cmovae",     REG,    RM, 0),
    Operand::new(0x44, 8, -1, "cmovz",      REG,    RM, 0),
    Operand::new(0x45, 8, -1, "cmovnz",     REG,    RM, 0),
    Operand::new(0x46, 8, -1, "cmovbe",     REG,    RM, 0),
    Operand::new(0x47, 8, -1, "cmova",      REG,    RM, 0),
    Operand::new(0x48, 8, -1, "cmovs",      REG,    RM, 0),
    Operand::new(0x49, 8, -1, "cmovns",     REG,    RM, 0),
    Operand::new(0x4A, 8, -1, "cmovp",      REG,    RM, 0),
    Operand::new(0x4B, 8, -1, "cmovnp",     REG,    RM, 0),
    Operand::new(0x4C, 8, -1, "cmovl",      REG,    RM, 0),
    Operand::new(0x4D, 8, -1, "cmovge",     REG,    RM, 0),
    Operand::new(0x4E, 8, -1, "cmovle",     REG,    RM, 0),
    Operand::new(0x4F, 8, -1, "cmovg",      REG,    RM, 0),

    Operand::new(0x80, 8, -1, "jo",         REL,    NONE,      OP_BRANCH),
    Operand::new(0x81, 8, -1, "jno",        REL,    NONE,      OP_BRANCH),
    Operand::new(0x82, 8, -1, "jb",         REL,    NONE,      OP_BRANCH),
    Operand::new(0x83, 8, -1, "jae",        REL,    NONE,      OP_BRANCH),
    Operand::new(0x84, 8, -1, "jz",         REL,    NONE,      OP_BRANCH),
    Operand::new(0x85, 8, -1, "jnz",        REL,    NONE,      OP_BRANCH),
    Operand::new(0x86, 8, -1, "jbe",        REL,    NONE,      OP_BRANCH),
    Operand::new(0x87, 8, -1, "ja",         REL,    NONE,      OP_BRANCH),
    Operand::new(0x88, 8, -1, "js",         REL,    NONE,      OP_BRANCH),
    Operand::new(0x89, 8, -1, "jns",        REL,    NONE,      OP_BRANCH),
    Operand::new(0x8A, 8, -1, "jp",         REL,    NONE,      OP_BRANCH),
    Operand::new(0x8B, 8, -1, "jnp",        REL,    NONE,      OP_BRANCH),
    Operand::new(0x8C, 8, -1, "jl",         REL,    NONE,      OP_BRANCH),
    Operand::new(0x8D, 8, -1, "jge",        REL,    NONE,      OP_BRANCH),
    Operand::new(0x8E, 8, -1, "jle",        REL,    NONE,      OP_BRANCH),
    Operand::new(0x8F, 8, -1, "jg",         REL,    NONE,      OP_BRANCH),
    Operand::new(0x90, 0,  8, "seto",       RM, NONE, 0),
    Operand::new(0x91, 0,  8, "setno",      RM, NONE, 0),
    Operand::new(0x92, 0,  8, "setb",       RM, NONE, 0),
    Operand::new(0x93, 0,  8, "setae",      RM, NONE, 0),
    Operand::new(0x94, 0,  8, "setz",       RM, NONE, 0),
    Operand::new(0x95, 0,  8, "setnz",      RM, NONE, 0),
    Operand::new(0x96, 0,  8, "setbe",      RM, NONE, 0),
    Operand::new(0x97, 0,  8, "seta",       RM, NONE, 0),
    Operand::new(0x98, 0,  8, "sets",       RM, NONE, 0),
    Operand::new(0x99, 0,  8, "setns",      RM, NONE, 0),
    Operand::new(0x9A, 0,  8, "setp",       RM, NONE, 0),
    Operand::new(0x9B, 0,  8, "setnp",      RM, NONE, 0),
    Operand::new(0x9C, 0,  8, "setl",       RM, NONE, 0),
    Operand::new(0x9D, 0,  8, "setge",      RM, NONE, 0),
    Operand::new(0x9E, 0,  8, "setle",      RM, NONE, 0),
    Operand::new(0x9F, 0,  8, "setg",       RM, NONE, 0),
    Operand::new(0xA0, 8, -1, "push",       FS,     NONE,      OP_STACK),
    Operand::new(0xA1, 8, -1, "pop",        FS,     NONE,      OP_STACK),
    Operand::new(0xA2, 8,  0, "cpuid", NONE, NONE, 0),
    Operand::new(0xA3, 8, -1, "bt",         RM,     REG, 0),
    Operand::new(0xA4, 8, -1, "shld",       RM,     REG,    OP_ARG2_IMM8),
    Operand::new(0xA5, 8, -1, "shld",       RM,     REG,    OP_ARG2_CL),
    /* A6,7 unused */
    Operand::new(0xA8, 8, -1, "push",       GS,     NONE,      OP_STACK),
    Operand::new(0xA9, 8, -1, "pop",        GS,     NONE,      OP_STACK),
    /* AA - rsm? */
    Operand::new(0xAB, 8, -1, "bts",        RM,     REG,    OP_LOCK),
    Operand::new(0xAC, 8, -1, "shrd",       RM,     REG,    OP_ARG2_IMM8),
    Operand::new(0xAD, 8, -1, "shrd",       RM,     REG,    OP_ARG2_CL),
    Operand::new(0xAE, 0,  0, "fxsave",     MEM, NONE, 0),
    Operand::new(0xAE, 1,  0, "fxrstor",    MEM, NONE, 0),
    Operand::new(0xAE, 2,  0, "ldmxcsr",    MEM, NONE, 0),
    Operand::new(0xAE, 3,  0, "stmxcsr",    MEM, NONE, 0),
    Operand::new(0xAE, 4,  0, "xsave",      MEM, NONE, 0),
    Operand::new(0xAE, 5,  0, "xrstor",     MEM, NONE, 0),
    Operand::new(0xAE, 7,  0, "clflush",    MEM, NONE, 0),
    Operand::new(0xAF, 8, -1, "imul",       REG,    RM, 0),
    Operand::new(0xB0, 8,  8, "cmpxchg",    RM,     REG,    OP_LOCK),
    Operand::new(0xB1, 8, -1, "cmpxchg",    RM,     REG,    OP_LOCK),
    Operand::new(0xB2, 8, -1, "lss",        REG,    MEM, 0),
    Operand::new(0xB3, 8, -1, "btr",        RM,     REG,    OP_LOCK),
    Operand::new(0xB4, 8, -1, "lfs",        REG,    MEM, 0),
    Operand::new(0xB5, 8, -1, "lgs",        REG,    MEM, 0),
    Operand::new(0xB6, 8, -1, "movzx",      REG,    RM, 0),
    Operand::new(0xB7, 8, -1, "movzx",      REG,    RM, 0),
    /* B8, 9, A.0-3 unused */
    Operand::new(0xBA, 4, -1, "bt",         RM,     IMM8, 0),
    Operand::new(0xBA, 5, -1, "bts",        RM,     IMM8,   OP_LOCK),
    Operand::new(0xBA, 6, -1, "btr",        RM,     IMM8,   OP_LOCK),
    Operand::new(0xBA, 7, -1, "btc",        RM,     IMM8,   OP_LOCK),
    Operand::new(0xBB, 8, -1, "btc",        RM,     REG,    OP_LOCK),
    Operand::new(0xBC, 8, -1, "bsf",        REG,    RM, 0),
    Operand::new(0xBD, 8, -1, "bsr",        REG,    RM, 0),
    Operand::new(0xBE, 8, -1, "movsx",      REG,    RM, 0),
    Operand::new(0xBF, 8, -1, "movsx",      REG,    RM, 0),
    Operand::new(0xC0, 8,  8, "xadd",       RM,     REG,    OP_LOCK),
    Operand::new(0xC1, 8, -1, "xadd",       RM,     REG,    OP_LOCK),

    Operand::new(0xC7, 1,  0, "cmpxchg8b",  MEM,    NONE,      OP_LOCK),

    Operand::new(0xC8, 8, -1, "bswap",      AX, NONE, 0),
    Operand::new(0xC9, 8, -1, "bswap",      CX, NONE, 0),
    Operand::new(0xCA, 8, -1, "bswap",      DX, NONE, 0),
    Operand::new(0xCB, 8, -1, "bswap",      BX, NONE, 0),
    Operand::new(0xCC, 8, -1, "bswap",      SP, NONE, 0),
    Operand::new(0xCD, 8, -1, "bswap",      BP, NONE, 0),
    Operand::new(0xCE, 8, -1, "bswap",      SI, NONE, 0),
    Operand::new(0xCF, 8, -1, "bswap",      DI, NONE, 0),
];

/* mod < 3 (instructions with memory args) */
pub const instructions_fpu_m: [Operand;64] = [
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

pub const instructions_fpu_single: [Operand;36] = [
    Operand::new(0xD9, 0xD0, 0, "fnop", NONE, NONE, 0),
    Operand::new(0xD9, 0xE0, 0, "fchs", NONE, NONE, 0),
    Operand::new(0xD9, 0xE1, 0, "fabs", NONE, NONE, 0),
    Operand::new(0xD9, 0xE4, 0, "ftst", NONE, NONE, 0),
    Operand::new(0xD9, 0xE5, 0, "fxam", NONE, NONE, 0),
    Operand::new(0xD9, 0xE8, 0, "fld1", NONE, NONE, 0),
    Operand::new(0xD9, 0xE9, 0, "fldl2t", NONE, NONE, 0),
    Operand::new(0xD9, 0xEA, 0, "fldl2e", NONE, NONE, 0),
    Operand::new(0xD9, 0xEB, 0, "fldpi", NONE, NONE, 0),
    Operand::new(0xD9, 0xEC, 0, "fldlg2", NONE, NONE, 0),
    Operand::new(0xD9, 0xED, 0, "fldln2", NONE, NONE, 0),
    Operand::new(0xD9, 0xEE, 0, "fldz", NONE, NONE, 0),
    Operand::new(0xD9, 0xF0, 0, "f2xm1", NONE, NONE, 0),
    Operand::new(0xD9, 0xF1, 0, "fyl2x", NONE, NONE, 0),
    Operand::new(0xD9, 0xF2, 0, "fptan", NONE, NONE, 0),
    Operand::new(0xD9, 0xF3, 0, "fpatan", NONE, NONE, 0),
    Operand::new(0xD9, 0xF4, 0, "fxtract", NONE, NONE, 0),
    Operand::new(0xD9, 0xF5, 0, "fprem1", NONE, NONE, 0),
    Operand::new(0xD9, 0xF6, 0, "fdecstp", NONE, NONE, 0),
    Operand::new(0xD9, 0xF7, 0, "fincstp", NONE, NONE, 0),
    Operand::new(0xD9, 0xF8, 0, "fprem", NONE, NONE, 0),
    Operand::new(0xD9, 0xF9, 0, "fyl2xp1", NONE, NONE, 0),
    Operand::new(0xD9, 0xFA, 0, "fsqrt", NONE, NONE, 0),
    Operand::new(0xD9, 0xFB, 0, "fsincos", NONE, NONE, 0),
    Operand::new(0xD9, 0xFC, 0, "frndint", NONE, NONE, 0),
    Operand::new(0xD9, 0xFD, 0, "fscale", NONE, NONE, 0),
    Operand::new(0xD9, 0xFE, 0, "fsin", NONE, NONE, 0),
    Operand::new(0xD9, 0xFF, 0, "fcos", NONE, NONE, 0),
    Operand::new(0xDA, 0xE9, 0, "fucompp", NONE, NONE, 0),
    Operand::new(0xDB, 0xE0, 0, "fneni", NONE, NONE, 0),
    Operand::new(0xDB, 0xE1, 0, "fndisi", NONE, NONE, 0),
    Operand::new(0xDB, 0xE2, 0, "fnclex", NONE, NONE, 0),
    Operand::new(0xDB, 0xE3, 0, "fninit", NONE, NONE, 0),
    Operand::new(0xDB, 0xE4, 0, "fnsetpm", NONE, NONE, 0),
    Operand::new(0xDE, 0xD9, 0, "fcompp", NONE, NONE, 0),
    Operand::new(0xDF, 0xE0, 0, "fnstsw", AX, NONE, 0),
];

pub fn get_fpu_instr(p: &Vec<u8>, op: &mut Operand) -> i32{
    let subcode = REGOF(p[1]);
    let index = (p[0] & 7)*8 + subcode;
    if MODOF(p[1]) < 3 {
        if instructions_fpu_m[index].name[0] {
            *op = instructions_fpu_m[index].clone();
        }
        return 0;
    } else {
        if instructions_fpu_r[index].name[0] {
            *op = instructions_fpu_r[index];
            return 0;
        } else {
            /* try the single op list */
            for i in 0 .. instructions_fpu_single.len()/Operand::sizeof() {
                if p[0] == instructions_fpu_single[i].opcode &&
                    p[1] == instructions_fpu_single[i].subcode {
                    *op = instructions_fpu_single[i].clone();
                    break;
                }
            }
        }
        return 1;
    }
}

pub const INSTRUCTIONS_SSE: [Operand;109] = [
    Operand::new(0x10, 8,  0, "movups",     XMM,    XM, 0),
    Operand::new(0x11, 8,  0, "movups",     XM,     XMM, 0),
    Operand::new(0x12, 8,  0, "movlps",     XMM,    XM, 0),    /* fixme: movhlps */
    Operand::new(0x13, 8,  0, "movlps",     MEM,    XMM, 0),
    Operand::new(0x14, 8,  0, "unpcklps",   XMM,    XM, 0),
    Operand::new(0x15, 8,  0, "unpckhps",   XMM,    XM, 0),
    Operand::new(0x16, 8,  0, "movhps",     XMM,    XM, 0),    /* fixme: movlhps */
    Operand::new(0x17, 8,  0, "movhps",     MEM,    XMM, 0),
    Operand::new(0x28, 8,  0, "movaps",     XMM,    XM, 0),
    Operand::new(0x29, 8,  0, "movaps",     XM,     XMM, 0),
    Operand::new(0x2A, 8,  0, "cvtpi2ps",   XMM,    MM, 0),
    Operand::new(0x2B, 8,  0, "movntps",    MEM,    XMM, 0),
    Operand::new(0x2C, 8,  0, "cvttps2pi",  MMX,    XM, 0),
    Operand::new(0x2D, 8,  0, "cvtps2pi",   MMX,    XM, 0),
    Operand::new(0x2E, 8,  0, "ucomiss",    XMM,    XM, 0),
    Operand::new(0x2F, 8,  0, "comiss",     XMM,    XM, 0),

    Operand::new(0x50, 8,  0, "movmskps",   REGONLY,XMM, 0),
    Operand::new(0x51, 8,  0, "sqrtps",     XMM,    XM, 0),
    Operand::new(0x52, 8,  0, "rsqrtps",    XMM,    XM, 0),
    Operand::new(0x53, 8,  0, "rcpps",      XMM,    XM, 0),
    Operand::new(0x54, 8,  0, "andps",      XMM,    XM, 0),
    Operand::new(0x55, 8,  0, "andnps",     XMM,    XM, 0),
    Operand::new(0x56, 8,  0, "orps",       XMM,    XM, 0),
    Operand::new(0x57, 8,  0, "xorps",      XMM,    XM, 0),
    Operand::new(0x58, 8,  0, "addps",      XMM,    XM, 0),
    Operand::new(0x59, 8,  0, "mulps",      XMM,    XM, 0),
    Operand::new(0x5A, 8,  0, "cvtps2pd",   XMM,    XM, 0),
    Operand::new(0x5B, 8,  0, "cvtdq2ps",   XMM,    XM, 0),
    Operand::new(0x5C, 8,  0, "subps",      XMM,    XM, 0),
    Operand::new(0x5D, 8,  0, "minps",      XMM,    XM, 0),
    Operand::new(0x5E, 8,  0, "divps",      XMM,    XM, 0),
    Operand::new(0x5F, 8,  0, "maxps",      XMM,    XM, 0),
    Operand::new(0x60, 8,  0, "punpcklbw",  MMX,    MM, 0),
    Operand::new(0x61, 8,  0, "punpcklwd",  MMX,    MM, 0),
    Operand::new(0x62, 8,  0, "punpckldq",  MMX,    MM, 0),
    Operand::new(0x63, 8,  0, "packsswb",   MMX,    MM, 0),
    Operand::new(0x64, 8,  0, "pcmpgtb",    MMX,    MM, 0),
    Operand::new(0x65, 8,  0, "pcmpgtw",    MMX,    MM, 0),
    Operand::new(0x66, 8,  0, "pcmpgtd",    MMX,    MM, 0),
    Operand::new(0x67, 8,  0, "packuswb",   MMX,    MM, 0),
    Operand::new(0x68, 8,  0, "punpckhbw",  MMX,    MM, 0),
    Operand::new(0x69, 8,  0, "punpckhwd",  MMX,    MM, 0),
    Operand::new(0x6A, 8,  0, "punpckhdq",  MMX,    MM, 0),
    Operand::new(0x6B, 8,  0, "packssdw",   MMX,    MM, 0),
    /* 6C/D unused */
    Operand::new(0x6E, 8,  0, "movd",       MMX,    RM, 0),
    Operand::new(0x6F, 8,  0, "movq",       MMX,    MM, 0),
    Operand::new(0x70, 8,  0, "pshufw",     MMX,    MM,     OP_ARG2_IMM8),
    Operand::new(0x71, 2,  0, "psrlw",      MMXONLY,IMM8, 0),  /* fixme: make sure this works */
    Operand::new(0x71, 4,  0, "psraw",      MMXONLY,IMM8, 0),
    Operand::new(0x71, 6,  0, "psllw",      MMXONLY,IMM8, 0),
    Operand::new(0x72, 2,  0, "psrld",      MMXONLY,IMM8, 0),
    Operand::new(0x72, 4,  0, "psrad",      MMXONLY,IMM8, 0),
    Operand::new(0x72, 6,  0, "pslld",      MMXONLY,IMM8, 0),
    Operand::new(0x73, 2,  0, "psrlq",      MMXONLY,IMM8, 0),
    Operand::new(0x73, 6,  0, "psllq",      MMXONLY,IMM8, 0),
    Operand::new(0x74, 8,  0, "pcmpeqb",    MMX,    MM, 0),
    Operand::new(0x75, 8,  0, "pcmpeqw",    MMX,    MM, 0),
    Operand::new(0x76, 8,  0, "pcmpeqd",    MMX,    MM, 0),
    Operand::new(0x77, 8,  0, "emms", NONE, NONE, 0),
    Operand::new(0x7E, 8,  0, "movd",       RM,     MMX, 0),
    Operand::new(0x7F, 8,  0, "movq",       MM,     MMX, 0),
    Operand::new(0xC2, 8,  0, "cmpps",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0xC3, 8,  0, "movnti",     MEM,    REG, 0),
    Operand::new(0xC4, 8,  0, "pinsrw",     MMX,    RM,     OP_ARG2_IMM8),
    Operand::new(0xC5, 8,  0, "pextrw",     REGONLY,MMX,    OP_ARG2_IMM8),
    Operand::new(0xC6, 8,  0, "shufps",     XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0xD1, 8,  0, "psrlw",      MMX,    MM, 0),
    Operand::new(0xD2, 8,  0, "psrld",      MMX,    MM, 0),
    Operand::new(0xD3, 8,  0, "psrlq",      MMX,    MM, 0),
    Operand::new(0xD4, 8,  0, "paddq",      MMX,    MM, 0),
    Operand::new(0xD5, 8,  0, "pmullw",     MMX,    MM, 0),
    /* D6 unused */
    Operand::new(0xD7, 8,  0, "pmovmskb",   REGONLY,MMX, 0),
    Operand::new(0xD8, 8,  0, "psubusb",    MMX,    MM, 0),
    Operand::new(0xD9, 8,  0, "psubusw",    MMX,    MM, 0),
    Operand::new(0xDA, 8,  0, "pminub",     MMX,    MM, 0),
    Operand::new(0xDB, 8,  0, "pand",       MMX,    MM, 0),
    Operand::new(0xDC, 8,  0, "paddusb",    MMX,    MM, 0),
    Operand::new(0xDD, 8,  0, "paddusw",    MMX,    MM, 0),
    Operand::new(0xDE, 8,  0, "pmaxub",     MMX,    MM, 0),
    Operand::new(0xDF, 8,  0, "pandn",      MMX,    MM, 0),
    Operand::new(0xE0, 8,  0, "pavgb",      MMX,    MM, 0),
    Operand::new(0xE1, 8,  0, "psraw",      MMX,    MM, 0),
    Operand::new(0xE2, 8,  0, "psrad",      MMX,    MM, 0),
    Operand::new(0xE3, 8,  0, "pavgw",      MMX,    MM, 0),
    Operand::new(0xE4, 8,  0, "pmulhuw",    MMX,    MM, 0),
    Operand::new(0xE5, 8,  0, "pmulhw",     MMX,    MM, 0),
    /* E6 unused */
    Operand::new(0xE7, 8,  0, "movntq",     MEM,    MMX, 0),
    Operand::new(0xE8, 8,  0, "psubsb",     MMX,    MM, 0),
    Operand::new(0xE9, 8,  0, "psubsw",     MMX,    MM, 0),
    Operand::new(0xEA, 8,  0, "pminsw",     MMX,    MM, 0),
    Operand::new(0xEB, 8,  0, "por",        MMX,    MM, 0),
    Operand::new(0xEC, 8,  0, "paddsb",     MMX,    MM, 0),
    Operand::new(0xED, 8,  0, "paddsw",     MMX,    MM, 0),
    Operand::new(0xEE, 8,  0, "pmaxsw",     MMX,    MM, 0),
    Operand::new(0xEF, 8,  0, "pxor",       MMX,    MM, 0),
    /* F0 unused */
    Operand::new(0xF1, 8,  0, "psllw",      MMX,    MM, 0),
    Operand::new(0xF2, 8,  0, "pslld",      MMX,    MM, 0),
    Operand::new(0xF3, 8,  0, "psllq",      MMX,    MM, 0),
    Operand::new(0xF4, 8,  0, "pmuludq",    MMX,    MM, 0),
    Operand::new(0xF5, 8,  0, "pmaddwd",    MMX,    MM, 0),
    Operand::new(0xF6, 8,  0, "psadbw",     MMX,    MM, 0),
    Operand::new(0xF7, 8,  0, "maskmovq",   MMX,    MMXONLY, 0),
    Operand::new(0xF8, 8,  0, "psubb",      MMX,    MM, 0),
    Operand::new(0xF9, 8,  0, "psubw",      MMX,    MM, 0),
    Operand::new(0xFA, 8,  0, "psubd",      MMX,    MM, 0),
    Operand::new(0xFB, 8,  0, "psubq",      MMX,    MM, 0),
    Operand::new(0xFC, 8,  0, "paddb",      MMX,    MM, 0),
    Operand::new(0xFD, 8,  0, "paddw",      MMX,    MM, 0),
    Operand::new(0xFE, 8,  0, "paddd",      MMX,    MM, 0),
];

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

pub const INSTRUCTIONS_SSE_SINGLE: [Operand;18] = [
    Operand::new(0x38, 0x00, 0, "pshufb",       MMX,    MM, 0),
    Operand::new(0x38, 0x01, 0, "phaddw",       MMX,    MM, 0),
    Operand::new(0x38, 0x02, 0, "phaddd",       MMX,    MM, 0),
    Operand::new(0x38, 0x03, 0, "phaddsw",      MMX,    MM, 0),
    Operand::new(0x38, 0x04, 0, "pmaddubsw",    MMX,    MM, 0),
    Operand::new(0x38, 0x05, 0, "phsubw",       MMX,    MM, 0),
    Operand::new(0x38, 0x06, 0, "phsubd",       MMX,    MM, 0),
    Operand::new(0x38, 0x07, 0, "phsubsw",      MMX,    MM, 0),
    Operand::new(0x38, 0x08, 0, "psignb",       MMX,    MM, 0),
    Operand::new(0x38, 0x09, 0, "psignw",       MMX,    MM, 0),
    Operand::new(0x38, 0x0A, 0, "psignd",       MMX,    MM, 0),
    Operand::new(0x38, 0x0B, 0, "pmulhrsw",     MMX,    MM, 0),
    Operand::new(0x38, 0x1C, 0, "pabsb",        MMX,    MM, 0),
    Operand::new(0x38, 0x1D, 0, "pabsw",        MMX,    MM, 0),
    Operand::new(0x38, 0x1E, 0, "pabsd",        MMX,    MM, 0),
    Operand::new(0x38, 0xF0,16, "movbe",        REG,    MEM, 0),   /* not SSE */
    Operand::new(0x38, 0xF1,16, "movbe",        MEM,    REG, 0),   /* not SSE */
    Operand::new(0x3A, 0x0F, 0, "palignr",      MMX,    MM,     OP_ARG2_IMM8),
];

pub const INSTRUCTIONS_SSE_SINGLE_OP32: [Operand;69] = [
    Operand::new(0x38, 0x00, 0, "pshufb",       XMM,    XM, 0),
    Operand::new(0x38, 0x01, 0, "phaddw",       XMM,    XM, 0),
    Operand::new(0x38, 0x02, 0, "phaddd",       XMM,    XM, 0),
    Operand::new(0x38, 0x03, 0, "phaddsw",      XMM,    XM, 0),
    Operand::new(0x38, 0x04, 0, "pmaddubsw",    XMM,    XM, 0),
    Operand::new(0x38, 0x05, 0, "phsubw",       XMM,    XM, 0),
    Operand::new(0x38, 0x06, 0, "phsubd",       XMM,    XM, 0),
    Operand::new(0x38, 0x07, 0, "phsubsw",      XMM,    XM, 0),
    Operand::new(0x38, 0x08, 0, "psignb",       XMM,    XM, 0),
    Operand::new(0x38, 0x09, 0, "psignw",       XMM,    XM, 0),
    Operand::new(0x38, 0x0A, 0, "psignd",       XMM,    XM, 0),
    Operand::new(0x38, 0x0B, 0, "pmulhrsw",     XMM,    XM, 0),
    Operand::new(0x38, 0x10, 0, "pblendvb",     XMM,    XM, 0),
    Operand::new(0x38, 0x14, 0, "blendvps",     XMM,    XM, 0),
    Operand::new(0x38, 0x15, 0, "blendvpd",     XMM,    XM, 0),
    Operand::new(0x38, 0x17, 0, "ptest",        XMM,    XM, 0),
    Operand::new(0x38, 0x1C, 0, "pabsb",        XMM,    XM, 0),
    Operand::new(0x38, 0x1D, 0, "pabsw",        XMM,    XM, 0),
    Operand::new(0x38, 0x1E, 0, "pabsd",        XMM,    XM, 0),
    Operand::new(0x38, 0x20, 0, "pmovsxbw",     XMM,    XM, 0),
    Operand::new(0x38, 0x21, 0, "pmovsxbd",     XMM,    XM, 0),
    Operand::new(0x38, 0x22, 0, "pmovsxbq",     XMM,    XM, 0),
    Operand::new(0x38, 0x23, 0, "pmovsxwd",     XMM,    XM, 0),
    Operand::new(0x38, 0x24, 0, "pmovsxwq",     XMM,    XM, 0),
    Operand::new(0x38, 0x25, 0, "pmovsxdq",     XMM,    XM, 0),
    Operand::new(0x38, 0x28, 0, "pmuldq",       XMM,    XM, 0),
    Operand::new(0x38, 0x29, 0, "pcmpeqq",      XMM,    XM, 0),
    Operand::new(0x38, 0x2A, 0, "movntdqa",     XMM,    MEM, 0),
    Operand::new(0x38, 0x2B, 0, "packusdw",     XMM,    XM, 0),
    Operand::new(0x38, 0x30, 0, "pmovzxbw",     XMM,    XM, 0),
    Operand::new(0x38, 0x31, 0, "pmovzxbd",     XMM,    XM, 0),
    Operand::new(0x38, 0x32, 0, "pmovzxbq",     XMM,    XM, 0),
    Operand::new(0x38, 0x33, 0, "pmovzxwd",     XMM,    XM, 0),
    Operand::new(0x38, 0x34, 0, "pmovzxwq",     XMM,    XM, 0),
    Operand::new(0x38, 0x35, 0, "pmovzxdq",     XMM,    XM, 0),
    Operand::new(0x38, 0x37, 0, "pcmpgtq",      XMM,    XM, 0),
    Operand::new(0x38, 0x38, 0, "pminsb",       XMM,    XM, 0),
    Operand::new(0x38, 0x39, 0, "pminsd",       XMM,    XM, 0),
    Operand::new(0x38, 0x3A, 0, "pminuw",       XMM,    XM, 0),
    Operand::new(0x38, 0x3B, 0, "pminud",       XMM,    XM, 0),
    Operand::new(0x38, 0x3C, 0, "pmaxsb",       XMM,    XM, 0),
    Operand::new(0x38, 0x3D, 0, "pmaxsd",       XMM,    XM, 0),
    Operand::new(0x38, 0x3E, 0, "pmaxuw",       XMM,    XM, 0),
    Operand::new(0x38, 0x3F, 0, "pmaxud",       XMM,    XM, 0),
    Operand::new(0x38, 0x40, 0, "pmaxlld",      XMM,    XM, 0),
    Operand::new(0x38, 0x41, 0, "phminposuw",   XMM,    XM, 0),
    Operand::new(0x3A, 0x08, 0, "roundps",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x09, 0, "roundpd",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x0A, 0, "roundss",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x0B, 0, "roundsd",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x0C, 0, "blendps",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x0D, 0, "blendpd",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x0E, 0, "pblendw",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x0F, 0, "palignr",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x14, 0, "pextrb",       RM,     XMM,    OP_ARG2_IMM8),
    Operand::new(0x3A, 0x15, 0, "pextrw",       RM,     XMM,    OP_ARG2_IMM8),
    Operand::new(0x3A, 0x16, 0, "pextrd",       RM,     XMM,    OP_ARG2_IMM8),
    Operand::new(0x3A, 0x17, 0, "extractps",    RM,     XMM,    OP_ARG2_IMM8),
    Operand::new(0x3A, 0x20, 0, "pinsrb",       XMM,    RM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x21, 0, "insertps",     XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x22, 0, "pinsrd",       XMM,    RM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x40, 0, "dpps",         XMM,    XM, 0),
    Operand::new(0x3A, 0x41, 0, "dppd",         XMM,    XM, 0),
    Operand::new(0x3A, 0x42, 0, "mpsqdbw",      XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x44, 0, "pclmulqdq",    XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x60, 0, "pcmpestrm",    XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x61, 0, "pcmpestri",    XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x62, 0, "pcmpistrm",    XMM,    XM,     OP_ARG2_IMM8),
    Operand::new(0x3A, 0x63, 0, "pcmpistri",    XMM,    XM,     OP_ARG2_IMM8),
];

/* returns the flag if it's a prefix, 0 otherwise */
pub fn get_prefix(opcode: u8, bits: i32) -> u16 {
    if bits == 64 {
        if (opcode & 0xFFF0) == 0x40 {
            return PREFIX_REX | ((opcode & 0xF) * 0x1000);
        }
    }

    match opcode {
    0x26 => PREFIX_ES,
    0x2E => PREFIX_CS,
    0x36 => PREFIX_SS,
    0x3E => PREFIX_DS,
    0x64 => PREFIX_FS,
    0x65 => PREFIX_GS,
    0x66 => PREFIX_OP32,
    0x67 => PREFIX_ADDR32,
    0x9B => PREFIX_WAIT,
    0xF0 => PREFIX_LOCK,
    0xF2 => PREFIX_REPNE,
    0xF3 => PREFIX_REPE,
    _ => 0,
    }
}

pub fn instr_matches(opcode: u8, subcode: u8, op: &Operand) -> bool {
    ((opcode == op.opcode) && ((op.subcode == 8) || (subcode == op.subcode)))
}

/* aka 3 byte opcode */
pub fn get_sse_single(opcode: u8, subcode: u8, instr: &mut Instruction) -> i32 {
    if instr.prefix & PREFIX_OP32 {
        for i in 0 .. INSTRUCTIONS_SSE_SINGLE.len()/mem::sizeof::<Operand>() {
            if instructions_sse_single_op32[i].opcode == opcode &&
                instructions_sse_single_op32[i].subcode == subcode {
                instr.op = instructions_sse_single_op32[i];
                instr.prefix &= !PREFIX_OP32;
                return 1;
            }
        }
    } else {
        for i in 0 .. INSTRUCTIONS_SSE_SINGLE.len()/mem::sizeof::<Operand>() {
            if instructions_sse_single[i].opcode == opcode && instructions_sse_single[i].subcode == subcode {
                instr.op = instructions_sse_single[i];
                return 1;
            } else {}
        }
    }

    return 0;
}

pub fn get_sse_instr(p: &Vec<u8>, instr: &mut Instruction) -> i32 {
    let subcode: u8 = REGOF(p[1]);
    /* Clear the prefix if it matches. This makes the disassembler work right,
     * but it might break things later if we want to interpret these. The
     * solution in that case is probably to modify the size/name instead. */

    if instr.prefix & PREFIX_OP32 {
        for i in 0 < INSTRUCTIONS_SSE_OP32.len()/mem::size_of::<Operand>() {
            if instr_matches(p[0], subcode, &instructions_sse_op32[i]) {
                instr.op = INSTRUCTIONS_SSE_SINGLE[i];
                instr.prefix &= !PREFIX_OP32;
                return 0;
            }
        }
    } else if instr.prefix & PREFIX_REPNE {
        for i in 0 .. INSTRUCTIONS_SSE_REPNE / mem::sizeof::<Operand>() {
            if instr_matches(p[0], subcode, &instructions_sse_repne[i]) {
                instr.op = instructions_sse_repne[i];
                instr.prefix &= !PREFIX_REPNE;
                return 0;
            }
        }
    } else if instr.prefix & PREFIX_REPE {
        for i in 0 .. INSTRUCTIONS_SSE_REPE.len() / mem::sizeof::<Operand> {
            if instr_matches(p[0], subcode, &instructions_sse_repe[i]) {
                instr.op = instructions_sse_repe[i];
                instr.prefix &= !PREFIX_REPE;
                return 0;
            }
        }
    } else {
        for i in 0 .. INSTRUCTIONS_SSE / mem::sizeo_of::<Operand> {
            if instr_matches(p[0], subcode, &instructions_sse[i]) {
                instr.op = instructions_sse[i];
                return 0;
            }
        }
    }

    return get_sse_single(p[0], p[1], instr);
}

pub fn get_0f_instr(p: &mut Vec<u8>, instr: &mut Instruction) -> i32 {
    let subcode: u8 = REGOF(p[1]);

    /* a couple of special (read: annoying) cases first */
    if p[0] == 0x01 && MODOF(p[1]) == 3 {
        instr.op.opcode = 0x0F01;
        instr.op.subcode = p[1];
        match (p[1]) {
        0xC1 => instr.op.name = "vmcall".to_string(),
        0xC2 => instr.op.name = "vmlaunch".to_string(),
        0xC3 => instr.op.name = "vmresume".to_string(),
        0xC4 => instr.op.name = "vmcall".to_string(),
        0xC8 => instr.op.name = "monitor".to_string(),
        0xC9 => instr.op.name = "mwait".to_string(),
        0xD0 => instr.op.name = "xgetbv".to_string(),
        0xD1 => instr.op.name = "xsetbv".to_string(),
        0xF9 => instr.op.name = "rdtscp".to_string(),
            _ => instr.op.name = format!("UNK {:x}", p[1])
        }
        return 1;
    } else if p[0] == 0xAE && MODOF(p[1]) == 3 {
        instr.op.opcode = 0x0FAE;
        instr.op.subcode = subcode;
        if subcode == 0x5 { instr.op.name = "lfence".to_string(); }
        if subcode == 0x6 { instr.op.name = "mfence".to_string(); }
        if subcode == 0x7 { instr.op.name = "sfence".to_string(); }
        return 1;
    }

    for i in 0 .. INSTRUCTIONS_0F.len() / mem::size_of::<Operand>() {
        if instr_matches(p[0], subcode, &instructions_0F[i]) {
            instr.op = instructions_0F[i];
            len = 0;
            break;
        }
    }
    if !instr.op.name[0] {
        len = get_sse_instr(p, instr);
    }

    instr.op.opcode = 0x0F00 | p[0];
    return len;
}

/* Parameters:
 * ip      - [i] NOT current IP, but rather IP of the *argument*. This
 *               is necessary for REL to work right.
 * p       - [i] pointer to the current argument to be parsed
 * arg     - [i/o] pointer to the relevant arg struct
 *      .ip         [o]
 *      .value      [o]
 *      .type       [i]
 * instr   - [i/o] pointer to the relevant instr struct
 *      .prefix     [i]
 *      .op         [i]
 *      .modrm_disp [o]
 *      .modrm_reg  [o]
 * bits    - [i] bitness
 *
 * Returns: number of bytes processed
 *
 * Does not process specific arguments (e.g. registers, DSBX, ONE...)
 * The parameter out is given as a dword but may require additional casting.
 */
pub fn get_arg(ip: u32, p: &Vec<u8>, arg: &Argument, instr: &mut Instruction, bits: i32) -> i32 {
    arg.value = 0;

    match arg.arg_type {
    IMM8 => {
        arg.ip = ip;
        arg.value = p[0];
        return 1;
    }
    IMM16 => {
        arg.ip = ip;
        // arg.value = *(arg.arg_type
        // p);
        return 2;
    }
    IMM => {
        arg.ip = ip;
        return if instr.op.size == 8 {
            arg.value = *p;
            1
        } else if instr.op.size == 16 {
            // arg.value = *(arg.arg_type
            // p);
            2
        } else if instr.op.size == 64 && (instr.op.flags & OP_IMM64) {
            arg.value =  p;
            8
        } else {
            // arg.value = arg.arg_type
            // p);
            4
        }
    }
    REL8 => {
        arg.ip = ip;
        arg.value = ip + 1 + p[0];  /* signed */
        return 1;
    }
    REL => {
        arg.ip = ip;
        /* Equivalently signed or unsigned (i.e. clipped) */
        return if instr.op.size == 16 {
            // arg.value = (ip + 2 + *(arg.arg_type
            // p)) &0xffff;
            2
        } else {
            // arg.value = (ip + 4 + *(arg.arg_type
            // p)) &0xffffffff;
            4
        }
    }
    SEGPTR => {
        arg.ip = ip;
        return if instr.op.size == 16 {
            // arg.value = *(arg.arg_type
            // p);
            4
        } else {
            // arg.value = *(arg.arg_type
            // p);
            6
        }
    }
    MOFFS => {
        arg.ip = ip;
        return if instr.addrsize == 64 {
            arg.value = p;
            8
        } else if instr.addrsize == 32 {
            // arg.value = arg.arg_type
            // p);
            4
        } else {
            // arg.value = arg.arg_type
            // p);
            2
        }
    }
    RM | MEM | MM | XM =>
    {
        let xmod: u8 = MODOF(p[0]);
        let mut rm: u8  = MEMOF(p[0]);
        let mut ret: i32 = 1;

        if xmod == 3 {
            instr.modrm_disp = DISP_REG;
            instr.modrm_reg = rm;
            if instr.prefix & PREFIX_REXB {
                instr.modrm_reg += 8;
            }
            return 1;
        }

        if instr.addrsize != 16 && rm == 4 {
            /* SIB byte */
            p++;
            instr.sib_scale = 1 << MODOF(p[0]);
            instr.sib_index = REGOF(p[0]);
            if instr.prefix & PREFIX_REXX {
                instr.sib_index += 8;
            }
            if (instr.sib_index == 4) { instr.sib_index = -1; }
            rm = MEMOF(p[0]);
            ret += 1;
        }

        if xmod == 0 && bits == 64 && rm == 5 && !instr.sib_scale {
            /* IP-relative addressing... */
            arg.ip = ip + 1;
            arg.value = *(arg.arg_type (p+1));
            instr.modrm_disp = DISP_16;
            instr.modrm_reg = 16;
            ret += 4;
        } else if xmod == 0 && ((instr.addrsize == 16 && rm == 6) ||
                                (instr.addrsize != 16 && rm == 5)) {
            arg.ip = ip + 1;
            if instr.addrsize == 16 {
                arg.value = *(arg.arg_type (p+1));
                ret += 2;
            } else {
                arg.value = *(arg.arg_type (p+1));
                ret += 4;
            }
            instr.modrm_disp = DISP_16;
            instr.modrm_reg = -1;
        } else if xmod == 0 {
            instr.modrm_disp = DISP_NONE;
            instr.modrm_reg = rm as i8;
            if instr.prefix & PREFIX_REXB {
                instr.modrm_reg += 8;
            }
        } else if xmod == 1 {
            arg.ip = ip + 1;
            arg.value = *(p+1);
            instr.modrm_disp = DISP_8;
            instr.modrm_reg = rm as i8;
            if PREFIX_REXB & instr.prefix { instr.modrm_reg += 8; }
            ret += 1;
        } else if xmod == 2 {
            arg.ip = ip + 1;
            if instr.addrsize == 16 {
                arg.value = *(arg.arg_type (p+1));
                ret += 2;
            } else {
                arg.value = *(arg.arg_type (p+1));
                ret += 4;
            }
            instr.modrm_disp = DISP_16;
            instr.modrm_reg = rm as i8;
            if instr.prefix & PREFIX_REXB { instr.modrm_reg += 8; }
        }
        return ret;
    }
    REG | XMM | CR32 | DR32 | TR32 => {
        /* doesn't exist in 64-bit mode */
        arg.value = REGOF(p[0]);
        if instr.prefix & PREFIX_REXR {
            arg.value += 8;
        }
        return 0;
    }
     MMX | SEG16 => {
         arg.value = REGOF(p[0]);
         return 0;
     }
    REG32 | STX | REGONLY | MMXONLY | XMMONLY => {
        arg.value = MEMOF(*p);
        if instr.prefix & PREFIX_REXB {
            arg.value += 8;
        }
        return 1;
        /* all others should be implicit */
    } _ => {
            return 0;
        }
    }
}

pub const SEG16: [String;6] = [
    "es".to_string(), "cs".to_string(), "ss".to_string(), "ds".to_string(), "fs".to_string(), "gs".to_string()
];

pub const REG8: [String;8] = [
    "al".to_string(),"cl".to_string(),"dl".to_string(),"bl".to_string(),"ah".to_string(),"ch".to_string(),"dh".to_string(),"bh".to_string()
];

pub const REG8_REX:[String;16] = [
    "al".to_string(),"cl".to_string(),"dl".to_string(),"bl".to_string(),"spl".to_string(),"bpl".to_string(),"sil".to_string(),"dil".to_string(),"r8b".to_string(),"r9b".to_string(),"r10b".to_string(),"r11b".to_string(),"r12b".to_string(),"r13b".to_string(),"r14b".to_string(),"r15b".to_string()
];

pub const REG16: [String;16] = [
    "ax".to_string(),"cx".to_string(),"dx".to_string(),"bx".to_string(),"sp".to_string(),"bp".to_string(),"si".to_string(),"di".to_string(),"r8w".to_string(),"r9w".to_string(),"r10w".to_string(),"r11w".to_string(),"r12w".to_string(),"r13w".to_string(),"r14w".to_string(),"r15w".to_string()
];

pub const REG32: [String;17] = [
    "eax".to_string(),"ecx".to_string(),"edx".to_string(),"ebx".to_string(),"esp".to_string(),"ebp".to_string(),"esi".to_string(),"edi".to_string(),"r8d".to_string(),"r9d".to_string(),"r10d".to_string(),"r11d".to_string(),"r12d".to_string(),"r13d".to_string(),"r14d".to_string(),"r15d".to_string(),"eip".to_string()
];

pub const REG64: [String;17] = [
    "rax".to_string(),"rcx".to_string(),"rdx".to_string(),"rbx".to_string(),"rsp".to_string(),"rbp".to_string(),"rsi".to_string(),"rdi".to_string(),"r8".to_string(),"r9".to_string(),"r10".to_string(),"r11".to_string(),"r12".to_string(),"r13".to_string(),"r14".to_string(),"r15".to_string(),"rip".to_string()
];

pub fn get_seg16(out: &mut String, reg: u16) {
    if asm_syntax == GAS {
        *out += "%";
    }
    *out += SEG16[reg];
}

pub fn get_reg8(out: &mut String, reg: u8, rex: i32) {
    if asm_syntax == GAS {
       *out += "%";
    }
    if rex {
        *out += reg8_rex[reg];
    } else {
        *out += reg8[reg]
    }
}

pub fn get_reg16(out: &mut String, reg: u8, size: i32) {
    if reg != -1 {
        if asm_syntax == GAS {
            *out += "%";
        }
        if size == 16 {
            *out += reg16[reg];
        }
        if size == 32 {
            *out += reg32[reg];
        }
        else if size == 64 {
            *out += reg64[reg];
        }
    }
}

pub fn get_xmm(out: &mut String, reg: u8) {
    if asm_syntax == GAS {
        *out += "%";
    }
    *out += "xmm0";
    *out += fmt!("{}", reg);
}

pub fn get_mmx(out: &mut String, reg: u8) {
    if asm_syntax == GAS {
        *out += "%";
    }
    *out += "mm0";
    *out += fmt!("{}", reg);
}

pub const MODRM16_GAS: [String;8] = [
    "%bx,%si".to_string(), "%bx,%di".to_string(), "%bp,%si".to_string(), "%bp,%di".to_string(), "%si".to_string(), "%di".to_string(), "%bp".to_string(), "%bx".to_string()
];

pub const MODRM16_MASM: [String;8] = [
    "bx+si".to_string(), "bx+di".to_string(), "bp+si".to_string(), "bp+di".to_string(), "si".to_string(), "di".to_string(), "bp".to_string(), "bx".to_string()
];

/* Figure out whether it's a register, so we know whether to dispense with size
 * indicators on a memory access. */
pub fn is_reg(arg: ArgumentType) -> bool {
    return (arg >= AL && arg <= GS) || (arg >= REG && arg <= TR32);
}

/* With MASM/NASM, use capital letters to help disambiguate them from the following 'h'. */

pub fn print_arg(ip: &String, instr: &mut Instruction, i: i32, bits: i32) {
    let arg: &Argument = &instr.args[i];
    let mut out: String = arg.string;
    let value: u64 = arg.value;

    if arg.string[0] {
        return;
    } /* someone wants to print something special */

    if arg.arg_type >= AL && arg.arg_type <= BH {
        get_reg8(&mut out, arg.arg_type - AL, 0);
    }
    else if arg.arg_type >= AX && arg.arg_type <= DI {
        get_reg16(&mut out, arg.arg_type - AX + ((instr.prefix & PREFIX_REXB)? 8: 0), instr.op.size);
    }
    else if arg.arg_type >= ES && arg.arg_type <= GS {
        get_seg16(&mut out, arg.arg_type - ES);
    }

    match (arg.arg_type) {
    ONE => {
        if asm_syntax == GAS {
            out += "$0x1"
        } else {
            out += "1h"
        }
    }
    IMM8 => {
        if instr.op.flags & OP_STACK {
            /* 6a */
            if instr.op.size == 64 {
                let mut part: String = "".to_string();
                if asm_syntax == GAS {
                    part += "$0x%016lx"
                } else {
                    part += "qword %016lxh"
                }
                out += fmt!(part, value);
            }
            else if instr.op.size == 32 {
                let mut part: String = "".to_string();
                if asm_syntax == GAS {
                    part += "$0x%08x"
                } else {
                    part += "dword %08Xh"
                }
                out += fmt!(part, value);
            }
            else {
                let mut part: String = "".to_string();
                if asm_syntax == GAS {
                    part += "$0x%04x"
                } else {
                    part += "word %04Xh"
                }
                out += fmt!(part, value);
            }
        } else {
            if asm_syntax == GAS {
                out += fmt!("$0x{:02x}", value)
            } else {
                out += fmt!("{:02x}", value)
            }
        }
    }
    IMM16 => {
        sprintf(out, (asm_syntax == GAS)? "$0x%04lx": "%04lXh", value);
        if asm_syntax == GAS {
            fmt!("$0X{:04x}", value)
        } else {
            fmt!("{:04x}", value)
        }
    }
    IMM => {
        if instr.op.flags & OP_STACK {
            if instr.op.size == 64 {
                if asm_syntax == GAS {
                    out += fmt!("$0x{:016x}", value)
                } else {
                    out += fmt!("qword {:016x}h", value)
                }
            }
            else if instr.op.size == 32 {
                if asm_syntax == GAS {
                    out += fmt!("$0x{:08x}", value)
                } else {
                    out += fmt!("dword {:08x}h", value);
                }
            }
            else {
                if asm_syntax == GAS {
                    out += fmt!("$0x{:04x}", value);
                } else {
                    out += fmt!("{:04x}h", value);
                }
            }
        } else {
            if instr.op.size == 8 {
                if asm_syntax == GAS {
                    out += fmt!("$0{:02x}", value);
                } else {
                    out += fmt!("{:02x}h", value);
                }
            }
            else if instr.op.size == 16 {
                if asm_syntax == GAS {
                    out += fmt!("$0{:04x}", value);
                } else {
                    out += fmt!("{:04x}h", value);
                }

            }
            else if instr.op.size == 64 && (instr.op.flags & OP_IMM64) != 0 {
                if asm_syntax == GAS {
                    out += fmt!("$0{:016x}", value);
                } else {
                    out += fmt!("{:016x}h", value);
                }
            }
            else {
                if asm_syntax == GAS {
                    out += fmt!("$0x{:08x}", value);
                } else {
                    out += fmt!("{:08x}h", value);
                }
            }
        }
    }
    REL8 | REL => {
        out += fmt!("{:04}", value);
    },
    SEGPTR => {},
        /* should always be relocated */

    MOFFS => {
        if asm_syntax == GAS {
            if instr.prefix & PREFIX_SEG_MASK != 0 {
                get_seg16(&mut out, ((instr.prefix & PREFIX_SEG_MASK) - 1) as u8);
                out += ":";
            }
            out += fmt!("{:04x}", value);
        } else {
            out[0] = '[';
            if instr.prefix & PREFIX_SEG_MASK {
                get_seg16(&mut out, ((instr.prefix & PREFIX_SEG_MASK) - 1) as u8);
                out += ":";
            }
            out += fmt!("{:04x}", value);
        }
        instr.usedmem = true;

    }
    DSBX | DSSI => {
        if asm_syntax != NASM {
            if instr.prefix & PREFIX_SEG_MASK != 0 {
                get_seg16(&mut out, ((instr.prefix & PREFIX_SEG_MASK) - 1) as u8);
                out + ":";
            }
            if asm_syntax == GAS {
                out += "(";
            } else {
                out += "[";
            }
            let mut reg = 0;
            if arg.arg_type == DSBX {
                reg = 3;
            } else {
                reg = 6;
            }
            get_reg16(&mut out, reg, instr.addrsize);
        }
        instr.usedmem = true;
    }
    ESDI => {
        if asm_syntax != NASM {
            if asm_syntax == GAS {
                out += "%es:(";
            } else {
                out += "es:[";
            }
            get_reg16(&mut out, 7, instr.addrsize);
            if asm_syntax == GAS {
                out += ")";
            } else {
                out += "]";
            }
        }
        instr.usedmem = true;
    }
    ALS => {
        if asm_syntax == GAS {
            out += "%al";
        }
    }
    AXS => {
        if asm_syntax == GAS {
            out += "%ax";
        }
    }
    DXS => {
        if asm_syntax == GAS {
            out += "%dx";
        }
        else {
            out += "dx";
        }
    }
    /* register/memory. this is always the first byte after the opcode,
     * and is always either paired with a simple register or a subcode.
     * there are a few cases where it isn't [namely C6/7 MOV and 8F POP]
     * and we need to warn if we see a value there that isn't 0. */
    RM | MEM | MM | XM => {
        if instr.modrm_disp == DISP_REG {
            if arg.arg_type == XM {
                get_xmm(out, instr.modrm_reg);
                // if (instr.vex_256)
                // out
                // [asm_syntax == GAS?
                // 1: 0] = 'y';
                // break;
            } else if arg.arg_type == MM {
                get_mmx(&mut out, instr.modrm_reg as u8);
            }

            if arg.arg_type == MEM {
                eprintln!("ModRM byte has mod 3, but opcode only allows accessing memory.");
            }

            if instr.op.size == 8 || instr.op.opcode == 0x0FB6 || instr.op.opcode == 0x0FBE {
                /* mov*b* */
                get_reg8(out, instr.modrm_reg, instr.prefix & PREFIX_REX);
            } else if instr.op.opcode == 0x0FB7 || instr.op.opcode == 0x0FBF {
                /* mov*w* */
                get_reg16(out, instr.modrm_reg, 16);
            }  /* fixme: 64-bit? */
            else {
                get_reg16(out, instr.modrm_reg, instr.op.size);
            }
        }

        instr.usedmem = 1;

        /* NASM: <size>    [<seg>: <reg>+<reg>+/-<offset>h] */
        /* MASM: <size> ptr <seg>:[<reg>+<reg>+/-<offset>h] */
        /* GAS:           *%<seg>:<.0x<offset>(%<reg>,%<reg>) */

        if asm_syntax == GAS {
            if instr.op.opcode == 0xFF && instr.op.subcode >= 2 && instr.op.subcode <= 5 {
                out += "*";
            }

            if instr.prefix & PREFIX_SEG_MASK {
                get_seg16(&mut out, ((instr.prefix & PREFIX_SEG_MASK) - 1) as u8);
                out += ":";
            }

            /* offset */
            if instr.modrm_disp == DISP_8 {
                svalue = value;
                if svalue < 0 {
                    // sprintf(out + strlen(out), "-0x%02x", -svalue);
                    out += fmt!("-0x{:02x}", svalue * -1);
                }
                else {
                    // sprintf(out + strlen(out), "0x%02x", svalue);
                    out += fmt!("0x{:02x}", svalue);
                }
            } else if instr.modrm_disp == DISP_16 && instr.addrsize == 16 {
                 svalue =  value;
                if instr.modrm_reg == -1 {
                    // sprintf(out + strlen(out), "0x%04lx", value);  /* absolute memory is unsigned */
                    out += fmt!("0x{:04x}", value);
                    return;
                }
                if svalue < 0 {
                    // sprintf(out + strlen(out), "-0x%04x", -svalue);
                    out += fmt!("-0x{:04x}", svalue * -1);
                }
                else {
                    // sprintf(out + strlen(out), "0x%04x", svalue);
                    out += fmt!("0x{:04x}", svalue);
                }
            } else if instr.modrm_disp == DISP_16 {
                svalue = value;
                if instr.modrm_reg == -1 {
                    // sprintf(out + strlen(out), "0x%08lx", value);  /* absolute memory is unsigned */
                    out += fmt!("0x{:08x}", value);
                    return;
                }
                if svalue < 0 {
                    // sprintf(out + strlen(out), "-0x%08x", -svalue);
                    out += fmt!("-0x{:08x}", svalue * -1);
                }
                else {
                    // sprintf(out + strlen(out), "0x%08x", svalue);
                    out += fmt!("0x{:08x}", svalue);
                }
            }

            // strcat(out, "(");
            out += "(";

            if instr.addrsize == 16 {
                // strcat(out, modrm16_gas[instr.modrm_reg]);

            } else {
                get_reg16(&mut out, instr.modrm_reg as u8, instr.addrsize);
                if instr.sib_scale != 0 && instr.sib_index != -1 {
                    // strcat(out, ",");
                    out += ",";
                    get_reg16(&mut out, instr.sib_index as u8, instr.addrsize);
                    // strcat(out, ",0");
                    out += ",0";
                    // out[strlen(out) - 1] = '0' + instr.sib_scale;
                    out += fmt!("0{}", instr.sib_scale);
                }
            }
            // strcat(out, ")");
            out += ")";
        } else {
            let has_sib = (instr.sib_scale != 0 && instr.sib_index != -1);
            if instr.op.flags != 0 & OP_FAR {
                // strcat(out, "far ");
                out += "far ";
            }
            else if !is_reg(&instr.op.arg0) && !is_reg(&instr.op.arg1) {
                match instr.op.size
                {
                    8 => out += "byte ",
                    16 => out += "word ",
                    32 => out += "dword ",
                    64 => out += "qword ",
                    80 => out += "tword ",
                    _ => {}
                }
                if asm_syntax == MASM {
                    /* && instr.op.size == 0? */
                    // strcat(out, "ptr ");
                    out += "ptr ";
                }
            } else if instr.op.opcode == 0x0FB6 || instr.op.opcode == 0x0FBE {
                /* mov*b* */
                // strcat(out, "byte ");
                out += "byte ";
                if asm_syntax == MASM {
                    // strcat(out, "ptr ");
                    out += "ptr ";
                }
            } else if instr.op.opcode == 0x0FB7 || instr.op.opcode == 0x0FBF {
                /* mov*w* */
                // strcat(out, "word ");
                out += "word ";
                if asm_syntax == MASM {
                    // strcat(out, "ptr ");
                    out += "ptr ";
                }
            }

            if asm_syntax == NASM {
                // strcat(out, "[");
                out += "[";
            }

            if instr.prefix & PREFIX_SEG_MASK {
                get_seg16(&mut out, (instr.prefix & PREFIX_SEG_MASK) - 1);
                // strcat(out, ":");
                out += ":";
            }

            if asm_syntax == MASM {
                // strcat(out, "[");
                out += "[";
            }

            if instr.modrm_reg != -1 {
                if instr.addrsize == 16 {
                    // strcat(out, modrm16_masm[instr.modrm_reg]);
                    out += modrm16_masm[instr.modrm_reg];
                }
                else {
                    get_reg16(&mut out, instr.modrm_reg as u8, instr.addrsize);
                }
                if (has_sib) {
                    // strcat(out, "+");
                    out += "+";
                }
            }

            if has_sib {
                get_reg16(&mut out, instr.sib_index as u8, instr.addrsize);
                // strcat(out, "*0");
                out += "*0";
                // out[strlen(out) - 1] = '0' + instr.sib_scale;
                out += fmt!("0{}", instr.sib_scale);
            }

            if instr.modrm_disp == DISP_8 {
                svalue = value;
                if svalue < 0 {
                    // sprintf(out + strlen(out), "-%02Xh", -svalue);
                    out += fmt!("-{:02x}h", svalue * -1);
                }
                else {
                    // sprintf(out + strlen(out), "+%02Xh", svalue);
                    out += fmt!("+{:02x)h", svalue);
                }
            } else if instr.modrm_disp == DISP_16 && instr.addrsize == 16 {
                svalue = value;
                if instr.modrm_reg == -1 && !has_sib {
                    // sprintf(out + strlen(out), "%04lXh", value);
                    out += fmt!("{:04x}", value);
                }  /* absolute memory is unsigned */
                else if (svalue < 0) {
                    // sprintf(out + strlen(out), "-%04Xh", -svalue);
                    out += fmt!("-{:04x}", svalue * -1);
                }
                else {
                    // sprintf(out + strlen(out), "+%04Xh", svalue);
                    out += fmt!("+{:04x}", svalue);
                }
            } else if instr.modrm_disp == DISP_16 {
                svalue = value;
                if instr.modrm_reg == -1 && !has_sib {
                    // sprintf(out + strlen(out), "%08lXh", value);
                    out += fmt!("{:08x}h", value);
                }  /* absolute memory is unsigned */
                else if svalue < 0 {
                    // sprintf(out + strlen(out), "-%08Xh", -svalue);
                    out += fmt!("-{:08x}h", svalue * -1);
                }
                else {
                    // sprintf(out + strlen(out), "+%08Xh", svalue);
                    out += fmt!("+{:08x}", svalue);
                }
            }
            // strcat(out, "]");
            out += "]";
        }
        // break;
    }
    REG | REGONLY => {
        if instr.op.size == 8 {
            get_reg8(&mut out, value as u8, (instr.prefix & PREFIX_REX) as i32);
        }
        else if bits == 64 && instr.op.opcode == 0x63 {
            get_reg16(&mut out, value as u8, 64);
        }
        else {
            get_reg16(&mut out, value as u8, instr.op.size as i32);
        }

    }
    REG32 => {
        get_reg16(&mut out, value as u8, bits);
    }
    SEG16 => {
        if value > 5 {
            // warn_at("Invalid segment register {}\n", value);
            eprintln!("invalid segment register {}", value);
        }
        get_seg16(&mut out, value as u16);
    }

    CR32 =>
        match (value) {
        0 |2 | 3 | 4 | 8 => {},
            _ => eprintln!("invlaid control register {}", value),
        }
        if (asm_syntax == GAS){
            // strcat(out, "%");
            out += "%";
        }
        // strcat(out, "cr0");
        out += "cr0";
        // out[strlen(out)-1] = '0'+value;
        out += fmt!("0{}", value);
    DR32 => {
        if asm_syntax == GAS {
            // strcat(out, "%");
            out += "%";
        }
        // strcat(out, "dr0");
        out += "dr0";
        // out[strlen(out) - 1] = '0' + value;
        out += fmt!("0{}", value);
    }
    TR32 => {
        if (value < 3) {
            eprintln!("invalid test register {}", value);
        }
        if asm_syntax == GAS {
            // strcat(out, "%");
            out += "%";
        }
        // strcat(out, "tr0");
        out += "tr0";
        // out[strlen(out) - 1] = '0' + value;
        out += fmt!("0{}", value);
    }
    ST => {
        if asm_syntax == GAS {
            out += "%";
        }
        out += "st";
        if asm_syntax == NASM {
            out += "0";
        }
    }
    STX =>
        if asm_syntax == GAS {
            out += "%";
        }
        out += "st";
        if (asm_syntax != NASM){
            out += "(";
        }
        out += "0";
        out += fmt!("0{}", value);
        if (asm_syntax != NASM){
            out += ")";
        }

    MMX | MMXONLY => {
        get_mmx(out, value);
    }
    XMM | XMMONLY => {
        get_xmm(out, value);
        if (instr.vex_256) {
            if asm_syntax == GAS {
                out[1] = "y";
            } else {
                out[0] = "y";
            }
        }
    }
        _ => {}
}

/* helper to tack a length suffix onto a name */
pub fn suffix_name(instr: &mut Instruction) {
    if ((instr.op.flags & OP_LL) == OP_LL) {

        instr.op.name += "ll";
    }
    else if (instr.op.flags & OP_S) {

        instr.op.name += "s";
    }
    else if (instr.op.flags & OP_L) {

        instr.op.name += "l";
    }
    else if (instr.op.size == 80) {

        instr.op.name += "t";
    }
    else if (instr.op.size == 8) {
        instr.op.name += "b";
    }
    else if (instr.op.size == 16) {

        instr.op.name += "w";
    }
    else if (instr.op.size == 32) {
        if asm_syntax == GAS {
            instr.op.name += "l";
        } else {
            instr.op.name += "d";
        }
    }
    else if (instr.op.size == 64) {
        instr.op.name += "q";
    }
}

/* Paramters:
 * ip    - current IP (used to calculate relative addresses)
 * p     - pointer to the current instruction to be parsed
 * instr - [output] pointer to an instr_info struct to be filled
 * is32  - bitness
 *
 * Returns: number of bytes processed
 *
 * Note: we don't print warnings here (all warnings should be printed
 * while actually dumping output, both to keep this function agnostic and to
 * ensure they only get printed once), so we will need to watch out for
 * multiple prefixes, invalid instructions, etc.
 */
pub fn get_instr(ip: u32, p: &mut Vec<u8>, instr: &mut Instruction, bits: i32) -> usize {
    let mut len: usize = 0;
    let mut opcode: u8;
    let mut prefix: u16;

    while prefix = get_prefix(p[len], bits) {
        if (instr.prefix & PREFIX_SEG_MASK) && (prefix & PREFIX_SEG_MASK) {
            instr.op = instructions[p[len]];
            instr.prefix &= ~PREFIX_SEG_MASK;
        } else if instr.prefix & prefix & PREFIX_OP32 {
            /* Microsoft likes to repeat this on NOPs for alignment, so just
             * ignore it */
        } else if instr.prefix & prefix {
            instr.op = instructions[p[len]];
            instr.prefix &= ~prefix;
            return len;
        }
        instr.prefix |= prefix;
        len += 1;
    }

    opcode = p[len];

    /* copy the op_info */
    if opcode == 0xC4 && MODOF(p[len+1]) == 3 && bits != 16 {
        let mut subcode = 0xcc;
        len += 1;
        instr.vex = true;
        if (p[len] & 0x1F) == 2 {
            subcode = 0x38;
        }
        else if    ((p[len] & 0x1F) == 3) {
            subcode = 0x3A;
        }
        else { eprintln!("Unhandled subcode {:x} at {:x}", p[len], ip); }
        len += 1;
        instr.vex_reg = !((p[len] >> 3) & 7);
        instr.vex_256 = if (p[len] & 4) != 0 { true } else { false };
        if ((p[len] & 3) == 3) { instr.prefix |= PREFIX_REPNE; }
        else if ((p[len] & 3) == 2) { instr.prefix |= PREFIX_REPE; }
        else if ((p[len] & 3) == 1) { instr.prefix |= PREFIX_OP32; }
        len += get_sse_single(subcode, p[len+1], instr);
    } else if (opcode == 0xC5 && MODOF(p[len+1]) == 3 && bits != 16) {
        len++;
        instr.vex = 1;
        instr.vex_reg = ~((p[len] >> 3) & 7);
        instr.vex_256 = (p[len] & 4) ? 1 : 0;
        if ((p[len] & 3) == 3) instr.prefix |= PREFIX_REPNE;
        else if ((p[len] & 3) == 2) instr.prefix |= PREFIX_REPE;
        else if ((p[len] & 3) == 1) instr.prefix |= PREFIX_OP32;
        len++;
        len += get_0f_instr(p+len, instr);
    } else if (bits == 64 && instructions64[opcode].name[0]) {
        instr.op = instructions64[opcode];
    } else if (bits != 64 && instructions[opcode].name[0]) {
        instr.op = instructions[opcode];
    } else {
        byte subcode = REGOF(p[len+1]);

        /* do we have a member of an instruction group? */
        if (opcode == 0x0F) {
            len++;
            len += get_0f_instr(p+len, instr);
        } else if (opcode >= 0xD8 && opcode <= 0xDF) {
            len += get_fpu_instr(p+len, &instr.op);
        } else {
            unsigned i;
            for (i=0; i<sizeof(instructions_group)/sizeof(struct op); i++) {
                if (opcode == instructions_group[i].opcode &&
                    subcode == instructions_group[i].subcode) {
                    instr.op = instructions_group[i];
                    break;
                }
            }
        }

        /* if we get here and we haven't found a suitable instruction,
         * we ran into something unused (or inadequately documented) */
        if (!instr.op.name[0]) {
            /* supply some default values so we can keep parsing */
            strcpy(instr.op.name, "?"); /* less arrogant than objdump's (bad) */
            instr.op.subcode = subcode;
            instr.op.size = 0;
            instr.op.arg0 = 0;
            instr.op.arg1 = 0;
            instr.op.flags = 0;
        }
    }

    len++;

    /* resolve the size */
    if (instr.op.size == -1) {
        if (instr.prefix & PREFIX_OP32)
            instr.op.size = (bits == 16) ? 32 : 16;
        else if (instr.prefix & PREFIX_REXW)
            instr.op.size = 64;
        else if (instr.op.flags & (OP_STACK | OP_64))
            instr.op.size = bits;
        else
            instr.op.size = (bits == 16) ? 16 : 32;
    }

    if (instr.prefix & PREFIX_ADDR32)
        instr.addrsize = (bits == 32) ? 16 : 32;
    else
        instr.addrsize = bits;

    /* figure out what arguments we have */
    if (instr.op.arg0) {
        int base = len;

        instr.args[0].type = instr.op.arg0;
        instr.args[1].type = instr.op.arg1;

        /* The convention is that an arg whose value is one or more bytes has
         * IP pointing to that value, but otherwise it points to the beginning
         * of the instruction. This way, we'll never think that e.g. a register
         * value is supposed to be relocated. */
        instr.args[0].ip = instr.args[1].ip = instr.args[2].ip = ip;

        len += get_arg(ip+len, &p[len], &instr.args[0], instr, bits);

        /* registers that read from the modrm byte, which we might have just processed */
        if (instr.op.arg1 >= REG && instr.op.arg1 <= TR32)
            len += get_arg(ip+len, &p[base], &instr.args[1], instr, bits);
        else
            len += get_arg(ip+len, &p[len], &instr.args[1], instr, bits);

        /* arg2 */
        if (instr.op.flags & OP_ARG2_IMM)
            instr.args[2].type = IMM;
        else if (instr.op.flags & OP_ARG2_IMM8)
            instr.args[2].type = IMM8;
        else if (instr.op.flags & OP_ARG2_CL)
            instr.args[2].type = CL;

        len += get_arg(ip+len, &p[len], &instr.args[2], instr, bits);
    }

    /* modify the instruction name if appropriate */

    if (asm_syntax == GAS) {
        if (instr.op.opcode == 0x0FB6) {
            strcpy(instr.op.name, "movzb");
            suffix_name(instr);
        } else if (instr.op.opcode == 0x0FB7) {
            strcpy(instr.op.name, "movzw");
            suffix_name(instr);
        } else if (instr.op.opcode == 0x0FBE) {
            strcpy(instr.op.name, "movsb");
            suffix_name(instr);
        } else if (instr.op.opcode == 0x0FBF) {
            strcpy(instr.op.name, "movsw");
            suffix_name(instr);
        } else if (instr.op.opcode == 0x63 && bits == 64)
            strcpy(instr.op.name, "movslq");
    }

    if ((instr.op.flags & OP_STACK) && (instr.prefix & PREFIX_OP32))
        suffix_name(instr);
    else if ((instr.op.flags & OP_STRING) && asm_syntax != GAS)
        suffix_name(instr);
    else if (instr.op.opcode == 0x98)
        strcpy(instr.op.name, instr.op.size == 16 ? "cbw" : instr.op.size == 32 ? "cwde" : "cdqe");
    else if (instr.op.opcode == 0x99)
        strcpy(instr.op.name, instr.op.size == 16 ? "cwd" : instr.op.size == 32 ? "cdq" : "cqo");
    else if (instr.op.opcode == 0xE3)
        strcpy(instr.op.name, instr.op.size == 16 ? "jcxz" : instr.op.size == 32 ? "jecxz" : "jrcxz");
    else if (instr.op.opcode == 0xD4 && instr.args[0].value == 10) {
        strcpy(instr.op.name, "aam");
        instr.op.arg0 = NONE;
    } else if (instr.op.opcode == 0xD5 && instr.args[0].value == 10) {
        strcpy(instr.op.name, "aad");
        instr.op.arg0 = NONE;
    } else if (instr.op.opcode == 0x0FC7 && instr.op.subcode == 1 && (instr.prefix & PREFIX_REXW))
        strcpy(instr.op.name, "cmpxchg16b");
    else if (asm_syntax == GAS) {
        if (instr.op.flags & OP_FAR) {
            memmove(instr.op.name+1, instr.op.name, strlen(instr.op.name));
            instr.op.name[0] = 'l';
        } else if (!is_reg(instr.op.arg0) && !is_reg(instr.op.arg1) &&
                   instr.modrm_disp != DISP_REG)
            suffix_name(instr);
    } else if (asm_syntax != GAS && (instr.op.opcode == 0xCA || instr.op.opcode == 0xCB))
        strcat(instr.op.name, "f");

    return len;
}

pub fn print_instr(ip: &String, p: &Vec<u8>, len: i32, flags: u8, instr: &Instruction, comment: &String, bits: i32) {
    // int i;

    /* FIXME: now that we've had to add bits to this function, get rid of ip_string */

    /* get the arguments */

    print_arg(ip, instr, 0, bits);
    print_arg(ip, instr, 1, bits);
    print_arg(ip, instr, 2, bits);

    /* did we find too many prefixes? */
    if get_prefix(instr.op.opcode, bits) {
        if get_prefix(instr.op.opcode, bits) & PREFIX_SEG_MASK {
            eprintln!("Multiple segment prefixes found: {}, {}. Skipping to next instruction.",
                      SEG16[(instr.prefix & PREFIX_SEG_MASK) - 1], instr.op.name);
        }
        else {
            eprintln!("Prefix specified twice: {}. Skipping to next instruction.", instr.op.name);
        }
        instr.op.name[0] = 0;
    }

    /* check that the instruction exists */
    if instr.op.name[0] == '?' {
        eprintln!("Unknown opcode {:02x} (extension {})", instr.op.opcode, instr.op.subcode);
    }

    /* okay, now we begin dumping */
    if (flags & INSTR_JUMP) && (opts & COMPILABLE) {
        /* output a label, which is like an address but without the segment prefix */
        /* FIXME: check masm */
        if asm_syntax == NASM {
            print!(".");
        }
        print!("{}", ip);
    }

    if !(opts & NO_SHOW_ADDRESSES) {
        print!("{}", ip);
    }
    print!("\t");

    if !(opts & NO_SHOW_RAW_INSN) {
        for i in 0 .. len {
            print!("{:02x} ", p[i]);
        }
        while i < 8 {
            print!("   ");
            i += 1;
        }
    }

    /* mark instructions that are jumped to */
    if (flags & INSTR_JUMP) && !(opts & COMPILABLE) {
        if flags & INSTR_FAR {
            print!(">>");
        } else {
            print!(">");
        }

    }
    else {
        printf("  ");
    }

    /* print prefixes, including (fake) prefixes if ours are invalid */
    if (instr.prefix & PREFIX_SEG_MASK) {
        /* note: is it valid to use overrides with lods and outs? */
        if (!instr.usedmem || (instr.op.arg0 == ESDI || (instr.op.arg1 == ESDI && instr.op.arg0 != DSSI))) {  /* can't be overridden */
            warn_at("Segment prefix %s used with opcode 0x%02x %s\n", SEG16[(instr.prefix & PREFIX_SEG_MASK)-1], instr.op.opcode, instr.op.name);
            printf("%s ", SEG16[(instr.prefix & PREFIX_SEG_MASK)-1]);
        }
    }
    if ((instr.prefix & PREFIX_OP32) && instr.op.size != 16 && instr.op.size != 32) {
        warn_at("Operand-size override used with opcode 0x%02x %s\n", instr.op.opcode, instr.op.name);
        printf((asm_syntax == GAS) ? "data32 " : "o32 "); /* fixme: how should MASM print it? */
    }
    if ((instr.prefix & PREFIX_ADDR32) && (asm_syntax == NASM) && (instr.op.flags & OP_STRING)) {
        printf("a32 ");
    } else if ((instr.prefix & PREFIX_ADDR32) && !instr.usedmem && instr.op.opcode != 0xE3) { /* jecxz */
        warn_at("Address-size prefix used with opcode 0x%02x %s\n", instr.op.opcode, instr.op.name);
        printf((asm_syntax == GAS) ? "addr32 " : "a32 "); /* fixme: how should MASM print it? */
    }
    if (instr.prefix & PREFIX_LOCK) {
        if(!(instr.op.flags & OP_LOCK))
            warn_at("lock prefix used with opcode 0x%02x %s\n", instr.op.opcode, instr.op.name);
        printf("lock ");
    }
    if (instr.prefix & PREFIX_REPNE) {
        if(!(instr.op.flags & OP_REPNE))
            warn_at("repne prefix used with opcode 0x%02x %s\n", instr.op.opcode, instr.op.name);
        printf("repne ");
    }
    if (instr.prefix & PREFIX_REPE) {
        if(!(instr.op.flags & OP_REPE))
            warn_at("repe prefix used with opcode 0x%02x %s\n", instr.op.opcode, instr.op.name);
        printf((instr.op.flags & OP_REPNE) ? "repe ": "rep ");
    }
    if (instr.prefix & PREFIX_WAIT) {
        printf("wait ");
    }

    if (instr.vex)
        printf("v");
    printf("%s", instr.op.name);

    if (instr.args[0].string[0] || instr.args[1].string[0])
        printf("\t");

    if (asm_syntax == GAS) {
        /* fixme: are all of these orderings correct? */
        if (instr.args[1].string[0])
            printf("%s,", instr.args[1].string);
        if (instr.vex_reg)
            printf("%%ymm%d, ", instr.vex_reg);
        if (instr.args[0].string[0])
            printf("%s", instr.args[0].string);
        if (instr.args[2].string[0])
            printf(",%s", instr.args[2].string);
    } else {
        if (instr.args[0].string[0])
            printf("%s", instr.args[0].string);
        if (instr.args[1].string[0])
            printf(", ");
        if (instr.vex_reg)
            printf("ymm%d, ", instr.vex_reg);
        if (instr.args[1].string[0])
            printf("%s", instr.args[1].string);
        if (instr.args[2].string[0])
            printf(", %s", instr.args[2].string);
    }
    if (comment) {
        printf(asm_syntax == GAS ? "\t// " : "\t;");
        printf(" <%s>", comment);
    }

    /* if we have more than 7 bytes on this line, wrap around */
    if (len > 7 && !(opts & NO_SHOW_RAW_INSN)) {
        printf("\n\t\t");
        for (i=7; i<len; i++) {
            printf("%02x", p[i]);
            if (i < len) printf(" ");
        }
    }
    printf("\n");
}
