use crate::util::{read_byte, read_data};
use crate::x86::defines::Instruction;
use crate::x86::defines::{
    INSTR_FUNC, INSTR_JUMP, INSTR_SCANNED, INSTR_VALID, MAX_INSTR, OP_BRANCH, OP_STOP,
};
use crate::{DISASSEMBLE, DISASSEMBLE_ALL};
use libc::{memcpy, printf};
use std::cmp::min;

pub fn print_header(header: &header_mz) {
    print!(
        "\
        Minimum extra allocation (0xa): {} bytes\
        Maximum extra allocation (0xc): {} bytes\
        Initial stack location (0xe): {:x}\
        Program Entry point (0x14): {:x}\
        Overlay number (0x1a): {}\
        ",
        header.e_minalloc * 16,
        header.e_maxalloc * 16,
        realaddr(header.e_ss, header.e_sp),
        realaddr(header.e_cs, header.e_ip),
        header.e_ovno
    );
}

pub fn print_mz_instr(ip: u32, p: &Vec<u8>, flags: &Vec<u8>) -> usize {
    instr: Instruction = Default::default();
    let mut len: usize;
    let mut ip_string: String = String::new();
    len = get_instr(ip, p, &instr, 16);
    ip_string += fmt!("{:05x}", ip);
    print_instr(ip_string, p, len, flags[ip], &instr, NULL, 16);
    return len;
}

pub fn print_code(mz: &MzExecutable) {
    let mut ip: u32 = 0;
    let mut buffer: Vec<u8>;
    print!(
        "\
        Code (start = 0x{:x}, length = 0x{:x}:\
        ",
        mz.start, mz.length
    );

    while ip < mz.length as u32 {
        /* find a valid instruction */
        if !(mz.flags[ip] & INSTR_VALID) {
            if opts & DISASSEMBLE_ALL {
                /* still skip zeroes */
                if read_byte(&mz.file, (mz.start + ip) as usize) == 0 {
                    print!("      ...\n");
                    ip += 1;
                    while read_byte(&mz.file, (mz.start + ip) as usize) == 0 {
                        ip += 1;
                    }
                }
            } else {
                print!("     ...\n");
                while (ip < mz.length as u32) && !(mz.flags[ip] & INSTR_VALID) {
                    ip += 1;
                }
            }
        }

        if ip >= mz.length as u32 {
            return;
        }

        /* fixme: disassemble everything for now; we'll try to fix it later.
         * this is going to be a little more difficult since dos executables
         * unabashedly mix code and data, so we need to figure out a solution
         * for that. but we needed to do that anyway. */

        // memcpy(buffer, read_data(mz.start + ip), min(sizeof(buffer), mz.length - ip));

        buffer = read_data(
            &mz.file,
            (mz.start + ip) as usize,
            min(buffer.len(), mz.length - ip),
        );

        if mz.flags[ip] & INSTR_FUNC {
            print!("\n");
            print!("{:05x} <no name>:\n", ip);
        }

        ip += print_mz_instr(ip, &buffer, &mz.flags);
    }
}

pub fn scan_segment(mut ip: u32, mz: &MzExecutable) {
    let mut buffer: Vec<u8>;
    let mut instr: Instruction = Instruction::default();
    let mut i = 0;

    if ip > mz.length as u32 {
        eprint!("Attempt to scan past end of Segment.\n");
        return;
    }

    if (mz.flags[ip] & (INSTR_VALID | INSTR_SCANNED)) == INSTR_SCANNED {
        warn_at("Attempt to scan byte that does not begin instruction.\n");
    }

    while ip < mz.length as u32 {
        /* check if we already read from here */
        if mz.flags[ip] & INSTR_SCANNED {
            return;
        }

        /* read the instruction */
        buffer = Vec::new();
        buffer = read_data(
            &mz.file,
            (mz.start + ip) as usize,
            min(buffer.len(), mz.length - ip),
        );
        instr_length = get_instr(ip, buffer, &instr, 16);

        /* mark the bytes */
        mz.flags[ip] |= INSTR_VALID;
        for i in ip..ip + instr_length {
            mz.flags[i] |= INSTR_SCANNED;
            if i < mz.length as u32 {
                break;
            }
        }

        /* instruction which hangs over the minimum allocation */
        if i < ip + instr_length && i == mz.length {
            break;
        }

        /* handle conditional and unconditional jumps */
        if instr.op.flags & OP_BRANCH {
            /* near relative jump, loop, or call */
            if instr.op.name != "call" {
                mz.flags[instr.args[0].value] |= INSTR_FUNC;
            } else {
                mz.flags[instr.args[0].value] |= INSTR_JUMP;
            }

            /* scan it */
            scan_segment(instr.args[0].value as u32, mz);
        }

        if instr.op.flags & OP_STOP {
            return;
        }

        ip += instr_length;
    }

    eprint!("Scan reached the end of Segment.\n");
}

pub fn read_code(mz: &mut MzExecutable) {
    mz.entry_point = realaddr(mz.header.e_cs, mz.header.e_ip);
    mz.length = (((mz.header.e_cp - 1) * 512) + mz.header.e_cblp) as usize;
    if mz.header.e_cblp == 0 {
        mz.length += 512;
    }
    // mz.flags = calloc(mz.length, sizeof(byte));

    if mz.entry_point > mz.length as u32 {
        eprint!(
            "Entry point {:05x} exceeds Segment length ({:05x})\n",
            mz.entry_point, mz.length
        );
    }
    mz.flags[mz.entry_point] |= INSTR_FUNC;
    scan_segment(mz.entry_point, mz);
}

pub fn get_relocations(bytes: &Vec<u8>) -> Vec<Reloc> {
    unimplemented!();
}

pub fn readmz(mz: &mut MzExecutable) {
    mz.header = MzHeader::from_bytes(read_data(&mz.file, 0, -1).as_ref());

    /* read the relocation table */
    mz.reltab = get_relocations(read_data(&mz.file, mz.header.e_lfarlc as usize, -1).as_ref());

    /* read the code */
    mz.start = (mz.header.e_cparhdr * 16) as u32;
    read_code(mz);
}

pub fn dumpmz() {
    let mut mz: MzExecutable = MzExecutable::default();
    readmz(&mut mz);

    print!("Module type: MZ (DOS executable)\n");

    if mode & DUMPHEADER {
        print_header(&mz.header);
    }

    if mode & DISASSEMBLE {
        print_code(&mz);
    }
}

pub fn realaddr(segment: u16, offset: u16) -> u32 {
    /// MZ (aka real-mode) addresses are "segmented", but not really. Just use the actual value.
    return if segment < 0xfff0 {
        ((segment * 0x10) + offset) as u32
    } else {
        /* relative segments >= 0xfff0 really point into PSP */
        ((segment * 0x10) + offset - 0x100000) as u32
    };
}

#[derive(Clone, Debug, Default)]
pub struct MzHeader {
    e_magic: u16,    /* 00: MZ Header signature */
    e_cblp: u16,     /* 02: Bytes on last page of file */
    e_cp: u16,       /* 04: Pages in file */
    e_crlc: u16,     /* 06: Relocations */
    e_cparhdr: u16,  /* 08: Size of header in paragraphs */
    e_minalloc: u16, /* 0a: Minimum extra paragraphs needed */
    e_maxalloc: u16, /* 0c: Maximum extra paragraphs needed */
    e_ss: u16,       /* 0e: Initial (relative) SS value */
    e_sp: u16,       /* 10: Initial SP value */
    e_csum: u16,     /* 12: Checksum */
    e_ip: u16,       /* 14: Initial IP value */
    e_cs: u16,       /* 16: Initial (relative) CS value */
    e_lfarlc: u16,   /* 18: File address of relocation table */
    e_ovno: u16,     /* 1a: Overlay number */
}

impl MzHeader {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Reloc {
    offset: u16,
    segment: u16,
}

#[derive(Clone, Debug, Default)]
pub struct MzExecutable {
    pub file: Vec<u8>,
    pub header: MzHeader,
    pub reltab: Vec<Reloc>,
    pub entry_point: u32,
    pub flags: Vec<u8>,
    pub start: u32,
    pub length: usize,
}
