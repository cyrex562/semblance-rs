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
use crate::{COMPILABLE, NO_SHOW_ADDRESSES, NO_SHOW_RAW_INSN};
use crate::defs::AsmSyntax::{GAS, MASM, NASM};
use crate::x86::instructions::INSTRUCTIONS_0F;
use crate::x86::instructions::INSTRUCTIONS_FPU_M;
use crate::x86::instructions::instructions_fpu_r;
use crate::x86::instructions::instructions_fpu_single;
use crate::x86::instructions::INSTRUCTIONS_GROUP;
use crate::x86::defines::Instruction;
use crate::x86::defines::{Argument, INSTR_FAR, INSTR_JUMP, OP_64, OP_ARG2_CL, OP_ARG2_IMM, OP_ARG2_IMM8, OP_BRANCH, OP_FAR, OP_IMM64, OP_L, OP_LL, OP_LOCK, OP_OP32_REGONLY, OP_REP, OP_REPE, OP_REPNE, OP_S, OP_STACK, OP_STOP, OP_STRING, PREFIX_ADDR32, PREFIX_CS, PREFIX_DS, PREFIX_ES, PREFIX_FS, PREFIX_GS, PREFIX_LOCK, PREFIX_OP32, PREFIX_REPE, PREFIX_REPNE, PREFIX_REX, PREFIX_REXB, PREFIX_REXR, PREFIX_REXX, PREFIX_SEG_MASK, PREFIX_SS, PREFIX_WAIT, X86ArgType};
use crate::x86::defines::X86Instruction;
use crate::x86::instructions::INSTRUCTIONS_SSE;
use crate::x86::instructions::INSTRUCTIONS_SSE_OP32;
use crate::x86::instructions::INSTRUCTIONS_SSE_REPE;
use crate::x86::instructions::INSTRUCTIONS_SSE_REPNE;
use crate::x86::instructions::INSTRUCTIONS_SSE_SINGLE;

/* a subcode value of 8 means all subcodes,
 * or the subcode marks the register if there is one present. */

/* mod < 3 (instructions with memory args) */

pub fn get_fpu_instr(p: &Vec<u8>, op: &mut Instruction) -> i32{
    let subcode = REGOF(p[1]);
    let index = (p[0] & 7)*8 + subcode;
    if MODOF(p[1]) < 3 {
        if INSTRUCTIONS_FPU_M[index].name[0] {
            *op = INSTRUCTIONS_FPU_M[index].clone();
        }
        return 0;
    } else {
        if instructions_fpu_r[index].name[0] {
            *op = instructions_fpu_r[index];
            return 0;
        } else {
            /* try the single op list */
            for i in 0 .. instructions_fpu_single.len()/ Instruction::sizeof() {
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

pub fn instr_matches(opcode: u8, subcode: u8, op: &Instruction) -> bool {
    ((opcode == op.opcode) && ((op.subcode == 8) || (subcode == op.subcode)))
}

/* aka 3 byte opcode */
pub fn get_sse_single(opcode: u8, subcode: u8, instr: &mut Instruction) -> i32 {
    if instr.prefix & PREFIX_OP32 {
        for i in 0 .. INSTRUCTIONS_SSE_SINGLE.len()/mem::sizeof::<Instruction>() {
            if instructions_sse_single_op32[i].opcode == opcode &&
                instructions_sse_single_op32[i].subcode == subcode {
                instr.op = instructions_sse_single_op32[i];
                instr.prefix &= !PREFIX_OP32;
                return 1;
            }
        }
    } else {
        for i in 0 .. INSTRUCTIONS_SSE_SINGLE.len()/mem::sizeof::<Instruction>() {
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
        for i in 0 < INSTRUCTIONS_SSE_OP32.len()/mem::size_of::<Instruction>() {
            if instr_matches(p[0], subcode, &instructions_sse_op32[i]) {
                instr.op = INSTRUCTIONS_SSE_SINGLE[i];
                instr.prefix &= !PREFIX_OP32;
                return 0;
            }
        }
    } else if instr.prefix & PREFIX_REPNE {
        for i in 0 .. INSTRUCTIONS_SSE_REPNE / mem::sizeof::<Instruction>() {
            if instr_matches(p[0], subcode, &instructions_sse_repne[i]) {
                instr.op = instructions_sse_repne[i];
                instr.prefix &= !PREFIX_REPNE;
                return 0;
            }
        }
    } else if instr.prefix & PREFIX_REPE {
        for i in 0 .. INSTRUCTIONS_SSE_REPE.len() / mem::sizeof::<Instruction> {
            if instr_matches(p[0], subcode, &instructions_sse_repe[i]) {
                instr.op = instructions_sse_repe[i];
                instr.prefix &= !PREFIX_REPE;
                return 0;
            }
        }
    } else {
        for i in 0 .. INSTRUCTIONS_SSE / mem::sizeo_of::<Instruction> {
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

    for i in 0 .. INSTRUCTIONS_0F.len() / mem::size_of::<Instruction>() {
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
            p += 1;
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
pub fn is_reg(arg: X86ArgType) -> bool {
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
                    part += "$0x{:04x}"
                } else {
                    part += "word {:04x}h"
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
                    // sprintf(out + strlen(out), "-0x{:02x}", -svalue);
                    out += fmt!("-0x{:02x}", svalue * -1);
                }
                else {
                    // sprintf(out + strlen(out), "0x{:02x}", svalue);
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
            // warn_at("Invalid Segment register {}\n", value);
            eprintln!("invalid Segment register {}", value);
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
        len += 1;
        instr.vex = 1;
        instr.vex_reg = ~((p[len] >> 3) & 7);
        instr.vex_256 = (p[len] & 4) ? 1 : 0;
        if ((p[len] & 3) == 3) instr.prefix |= PREFIX_REPNE;
        else if ((p[len] & 3) == 2) instr.prefix |= PREFIX_REPE;
        else if ((p[len] & 3) == 1) instr.prefix |= PREFIX_OP32;
        len += 1;
        len += get_0f_instr(p+len, instr);
    } else if (bits == 64 && instructions64[opcode].name[0]) {
        instr.op = instructions64[opcode];
    } else if (bits != 64 && instructions[opcode].name[0]) {
        instr.op = instructions[opcode];
    } else {
        u8 subcode = REGOF(p[len+1]);

        /* do we have a member of an instruction group? */
        if (opcode == 0x0F) {
            len += 1;
            len += get_0f_instr(p+len, instr);
        } else if (opcode >= 0xD8 && opcode <= 0xDF) {
            len += get_fpu_instr(p+len, &instr.op);
        } else {
            unsigned i;
            for (i=0; i<sizeof(INSTRUCTIONS_GROUP)/sizeof(struct op); i += 1) {
                if (opcode == INSTRUCTIONS_GROUP[i].opcode &&
                    subcode == INSTRUCTIONS_GROUP[i].subcode) {
                    instr.op = INSTRUCTIONS_GROUP[i];
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

    len += 1;

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
            eprintln!("Multiple Segment prefixes found: {}, {}. Skipping to next instruction.",
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
        /* output a label, which is like an address but without the Segment prefix */
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
        print!("  ");
    }

    /* print prefixes, including (fake) prefixes if ours are invalid */
    if (instr.prefix & PREFIX_SEG_MASK) {
        /* note: is it valid to use overrides with lods and outs? */
        if (!instr.usedmem || (instr.op.arg0 == ESDI || (instr.op.arg1 == ESDI && instr.op.arg0 != DSSI))) {  /* can't be overridden */
            warn_at("Segment prefix {} used with opcode 0x{:02x} {}\n", SEG16[(instr.prefix & PREFIX_SEG_MASK)-1], instr.op.opcode, instr.op.name);
            print!("{} ", SEG16[(instr.prefix & PREFIX_SEG_MASK)-1]);
        }
    }
    if ((instr.prefix & PREFIX_OP32) && instr.op.size != 16 && instr.op.size != 32) {
        warn_at("Operand-size override used with opcode 0x{:02x} {}\n", instr.op.opcode, instr.op.name);
        print!((asm_syntax == GAS) ? "data32 " : "o32 "); /* fixme: how should MASM print it? */
    }
    if ((instr.prefix & PREFIX_ADDR32) && (asm_syntax == NASM) && (instr.op.flags & OP_STRING)) {
        print!("a32 ");
    } else if ((instr.prefix & PREFIX_ADDR32) && !instr.usedmem && instr.op.opcode != 0xE3) { /* jecxz */
        warn_at("Address-size prefix used with opcode 0x{:02x} {}\n", instr.op.opcode, instr.op.name);
        print!((asm_syntax == GAS) ? "addr32 " : "a32 "); /* fixme: how should MASM print it? */
    }
    if (instr.prefix & PREFIX_LOCK) {
        if(!(instr.op.flags & OP_LOCK))
            warn_at("lock prefix used with opcode 0x{:02x} {}\n", instr.op.opcode, instr.op.name);
        print!("lock ");
    }
    if (instr.prefix & PREFIX_REPNE) {
        if(!(instr.op.flags & OP_REPNE))
            warn_at("repne prefix used with opcode 0x{:02x} {}\n", instr.op.opcode, instr.op.name);
        print!("repne ");
    }
    if (instr.prefix & PREFIX_REPE) {
        if(!(instr.op.flags & OP_REPE))
            warn_at("repe prefix used with opcode 0x{:02x} {}\n", instr.op.opcode, instr.op.name);
        print!((instr.op.flags & OP_REPNE) ? "repe ": "rep ");
    }
    if (instr.prefix & PREFIX_WAIT) {
        print!("wait ");
    }

    if (instr.vex)
        print!("v");
    print!("{}", instr.op.name);

    if (instr.args[0].string[0] || instr.args[1].string[0])
        print!("\t");

    if (asm_syntax == GAS) {
        /* fixme: are all of these orderings correct? */
        if (instr.args[1].string[0])
            print!("{},", instr.args[1].string);
        if (instr.vex_reg)
            print!("%%ymm{}, ", instr.vex_reg);
        if (instr.args[0].string[0])
            print!("{}", instr.args[0].string);
        if (instr.args[2].string[0])
            print!(",{}", instr.args[2].string);
    } else {
        if (instr.args[0].string[0])
            print!("{}", instr.args[0].string);
        if (instr.args[1].string[0])
            print!(", ");
        if (instr.vex_reg)
            print!("ymm{}, ", instr.vex_reg);
        if (instr.args[1].string[0])
            print!("{}", instr.args[1].string);
        if (instr.args[2].string[0])
            print!(", {}", instr.args[2].string);
    }
    if (comment) {
        print!(asm_syntax == GAS ? "\t// " : "\t;");
        print!(" <{}>", comment);
    }

    /* if we have more than 7 bytes on this line, wrap around */
    if (len > 7 && !(opts & NO_SHOW_RAW_INSN)) {
        print!("\n\t\t");
        for (i=7; i<len; i += 1) {
            print!("{:02x}", p[i]);
            if (i < len) print!(" ");
        }
    }
    print!("\n");
}
