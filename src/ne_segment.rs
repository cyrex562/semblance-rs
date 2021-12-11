/* index function */
use crate::ne::{NeExecutable, NeReloc, NeSegment};
use crate::semblance::{read_byte, read_data, read_word, DISASSEMBLE_ALL, FULL_CONTENTS};
use crate::x86_instr_arg_type::X86ArgType::{IMM, MEM, REL, SEGPTR};
use crate::x86_instr_def::{
    Argument, Instruction, INSTR_FAR, INSTR_FUNC, INSTR_JUMP, INSTR_RELOC, INSTR_SCANNED,
    INSTR_VALID, OP_BRANCH, OP_STOP,
};
use libc::strcmp;
use std::cmp::min;

pub fn get_entry_name(cs: u16, ip: u16, ne: &NeExecutable) -> Option<String> {
    for i in 0..ne.entcount {
        if ne.enttab[i].segment == cs as u8 && ne.enttab[i].offset == ip {
            return Some(ne.enttab[i].name.clone());
        }
    }
    None
}

/* index function */
pub fn get_reloc(seg: &NeSegment, ip: u16) -> Option<NeReloc> {
    for i in 0..seg.reloc_count {
        for o in 0..seg.reloc_table[i].offset_count {
            if seg.reloc_table[i].offsets[o] == ip {
                return Some(seg.reloc_table[i].clone());
            }
        }
    }
    return None;
}

/* load an imported name from a specfile */
pub fn get_imported_name(module: u16, ordinal: u16, ne: &NeExecutable) -> Option<String> {
    for i in 0..ne.imptab[module - 1].export_count {
        if ne.imptab[module - 1].exports[i].ordinal == ordinal {
            return Some(ne.imptab[module - 1].exports[i].name);
        }
    }
    return None;
}

/* Tweak the inline string and return the comment. */
pub fn relocate_arg(seg: &NeSegment, arg: &mut Argument, ne: &NeExecutable) -> Option<String> {
    let mut r = get_reloc(seg, arg.ip as u16);

    if !r.is_none() && arg.arg_type == SEGPTR {
        r = get_reloc(seg, (arg.ip + 2) as u16)
    };
    if !r.is_none() {
        eprint!(
            "%{:x}: Byte tagged INSTR_RELOC has no Reloc attached; this is a bug.\n",
            arg.ip
        );
        return Some("?".to_string());
    }

    if r.unwrap().reloc_type == 1 || r.unwrap().reloc_type == 2 {
        module = ne.imptab[r.unwrap().tseg - 1].name;
    }

    if arg.arg_type == SEGPTR && r.size == 3 {
        /* 32-bit relocation on 32-bit pointer, so just copy the name */
        if r.unwrap().reloc_type == 0 {
            arg.arg_string = fmt!("{}:{:04x}", r.tseg, r.toffset);
            // snprintf(arg.string, sizeof(arg.string), "{}:{:04x}", r.tseg, r.toffset);
            return Some(r.unwrap().text);
        } else if r.unwrap().reloc_type == 1 {
            // snprintf(arg.string, sizeof(arg.string), "{}.{}", module, r.toffset);
            arg.arg_string = fmt!("{}.{}", module, r.toffset);
            return get_imported_name(r.tseg, r.toffset, ne);
        } else if r.unwrap().reloc_type == 2 {
            // snprintf(arg.string, sizeof(arg.string), "{}.%.*s", module,
            //     ne.nametab[r.toffset], &ne.nametab[r.toffset+1]);
            arg.arg_string = fmt!(
                "{}.{}.{}",
                module,
                ne.nametab[r.offset],
                ne.nametab[r.toffset + 1]
            );
            return None;
        }
    } else if arg.arg_type == SEGPTR && r.size == 2 && r.reloc_type == 0 {
        /* Segment relocation on 32-bit pointer; copy the Segment but keep the
         * offset */
        // snprintf(arg.string, sizeof(arg.string), "{}:%04lx", r.tseg, arg.value);
        arg.string = fmt!("{}:{}", r.unwrap().tseg, arg.value);
        return get_entry_name(r.unwrap().tseg, arg.value as u16, ne);
    } else if (arg.arg_type == IMM || arg.arg_type == MEM) && (r.size == 2 || r.size == 5) {
        /* imm16 referencing a Segment or offset directly; MEM with lea has also
         * been observed (for some reason) */
        let pfx: String = if r.size == 2 {
            "seg ".to_string()
        } else {
            "".to_string()
        };
        let mut open = "".to_string();
        let mut close = "".to_string();
        if arg.arg_type != IMM {
            open = "[".to_string();
            close = "]".to_string();
        }
        if r.unwrap().reloc_type == 0 {
            // snprintf(arg.string, sizeof(arg.string), "{}{}{}{}", open, pfx, r.tseg, close);
            arg.string = fmt!("{}{}{}{}", open, pfx, r.tseg, close);
            return None;
        } else if r.unwrap().reloc_type == 1 {
            // snprintf(arg.string, sizeof(arg.string), "{}{}{}.{}{}", open, pfx, module, r.toffset, close);
            arg.string = fmt!("{}{}{}.{}{}", open, pfx, module, r.toffset, close);
            return get_imported_name(r.tseg, r.toffset, ne);
        } else if r.unwrap().reloc_type == 2 {
            // snprintf(arg.string, sizeof(arg.string), "{}{}{}.%.*s{}", open, pfx, module,
            //     ne.nametab[r.toffset], &ne.nametab[r.toffset+1], close);
            arg.string = fmt!(
                "{}{}{}.{}.{}",
                open,
                pfx,
                module,
                ne.nametab[r.toffset],
                &ne.nametab[r.toffset + 1],
                close
            );
            return None;
        }
    }

    eprint!(
        "{}:{}: unhandled relocation: size {}, type {}, argtype {}\n",
        seg.cs, arg.ip, r.size, r.r_type, arg.arg_type
    );

    return None;
}

/* Returns the number of bytes processed (same as get_instr). */
pub fn print_ne_instr(seg: &NeSegment, ip: u16, p: &Vec<u8>, ne: &NeExecutable) -> i32 {
    let cs = seg.cs;
    let instr = Instruction::new();
    let bits = if seg.flags & 0x2000 { 32 } else { 16 };
    let len = get_instr(ip, p, &instr, bits);
    let ip_string = fmt!("{}:{:04x}", seg.cs, ip);

    /* check for relocations */
    if seg.instr_flags[instr.args[0].ip] & INSTR_RELOC {
        comment = relocate_arg(seg, &mut instr.args[0], ne);
    }
    if seg.instr_flags[instr.args[1].ip] & INSTR_RELOC {
        comment = relocate_arg(seg, &mut instr.args[1], ne);
    }
    /* make sure to check for SEGPTR Segment-only relocations */
    if instr.op.arg0 == SEGPTR && seg.instr_flags[instr.args[0].ip + 2] & INSTR_RELOC {
        comment = relocate_arg(seg, &mut instr.args[0], ne);
    }

    /* check if we are referencing a named Export */
    if !comment && instr.op.arg0 == REL {
        comment = get_entry_name(cs, instr.args[0].value, ne);
    }

    print_instr(
        ip_string,
        p,
        len,
        seg.instr_flags[ip],
        &instr,
        comment,
        bits,
    );

    return len;
}

pub fn print_disassembly(seg: &NeSegment, ne: &NeExecutable) {
    let cs = seg.cs;
    let mut ip = 0u16;
    let mut buffer: Vec<u8> = Vec::new();

    while ip < seg.length {
        /* find a valid instruction */
        if !(seg.instr_flags[ip] & INSTR_VALID) {
            if opts & DISASSEMBLE_ALL {
                /* still skip zeroes */
                if read_byte(&ne.file, seg.start + ip) == 0 {
                    print!("     ...\n");
                    ip += 1;
                    while read_byte(&ne.file, seg.start + ip) == 0 {
                        ip += 1;
                    }
                }
            } else {
                print!("     ...\n");
                while (ip < seg.length) && !(seg.instr_flags[ip] & INSTR_VALID) {
                    ip += 1
                }
            }
        }

        if ip >= seg.length {
            return;
        }

        /* Instructions can "hang over" the end of a Segment.
         * Zero should be supplied. */
        // memset(buffer, 0, sizeof(buffer));
        // memcpy(buffer, read_data(seg.start + ip), min(sizeof(buffer), seg.length - ip));
        let buffer = read_data(&ne.file, seg.start + ip, (seg.length - ip) as usize);

        if seg.instr_flags[ip] & INSTR_FUNC {
            let name = get_entry_name(cs, ip, ne);
            print!("\n");
            print!(
                "{}:{:04x} <{}>:\n",
                cs,
                ip,
                if name.is_some() {
                    name.unwrap()
                } else {
                    "no name".to_string()
                }
            );
            /* don't mark far functions—we can't reliably detect them
             * because of "push cs", and they should be evident anyway. */
        }

        ip += print_ne_instr(seg, ip, &buffer, ne);
    }
    print!('\n');
}

pub fn print_data(seg: &NeSegment) {
    /* well, not really ip */
    for ip in (0..seg.length).step_by(16) {
        let len = min(seg.length - ip, 16);
        // int i;

        print!("{}:{:04x}", seg.cs, ip);
        for i in 0..16 {
            if i < len {
                print!(" {:02x}", read_byte(&ne.file, seg.start + ip + i));
            } else {
                print!("   ");
            }
        }
        print!("  ");
        for i in 0..len {
            let c = read_byte(&ne.file, seg.start + ip + i);
            print!(isprint(c) ? c : '.');
        }
        print!('\n');
    }
}

pub fn scan_segment(cs: u16, mut ip: u16, ne: &NeExecutable) {
    let seg = &ne.segments[cs - 1];
    // u8 buffer[MAX_INSTR];
    // struct instr instr;
    // int instr_length;
    // int i;

    if ip >= seg.length {
        eprint!("Attempt to scan past end of Segment.\n");
        return;
    }

    if (seg.instr_flags[ip] & (INSTR_VALID | INSTR_SCANNED)) == INSTR_SCANNED {
        eprint!("Attempt to scan byte that does not begin instruction.\n");
    }

    while ip < seg.length {
        /* check if we already read from here */
        if seg.instr_flags[ip] & INSTR_SCANNED {
            return;
        }

        /* read the instruction */
        // memset(buffer, 0, sizeof(buffer));
        // memcpy(buffer, read_data(seg.start + ip), min(sizeof(buffer), seg.length - ip));
        buffer = read_data(&ne.file, seg.start + ip, seg.length - ip);
        instr_length = get_instr(ip, buffer, &instr, if seg.flags & 0x2000 { 32 } else { 16 });

        /* mark the bytes */
        seg.instr_flags[ip] |= INSTR_VALID;
        for i in ip..ip + instr_length {
            if i >= seg.min_alloc {
                break;
            }
            seg.instr_flags[i] |= INSTR_SCANNED;
        }
        // for (i = ip; i < ip+instr_length && i < seg.min_alloc; i += 1) seg.instr_flags[i] |= INSTR_SCANNED;

        /* instruction which hangs over the minimum allocation */
        if i < ip + instr_length && i == seg.min_alloc {
            break;
        }

        /* handle conditional and unconditional jumps */
        if instr.op.arg0 == SEGPTR {
            for i in ip..ip + instr_length {
                // for (i = ip; i < ip+instr_length; i += 1) {
                if seg.instr_flags[i] & INSTR_RELOC {
                    let r = get_reloc(seg, i);
                    // const struct segment *tseg;

                    if r.is_none() {
                        break;
                    }
                    let tseg = &ne.segments[r.tseg - 1];

                    if r.unwrap().reloc_type != 0 {
                        break;
                    }

                    if r.unwrap().size == 3 {
                        /* 32-bit relocation on 32-bit pointer */
                        tseg.instr_flags[r.unwrap().toffset] |= INSTR_FAR;
                        if instr.op.name != "call" {
                            tseg.instr_flags[r.unwrap().toffset] |= INSTR_FUNC;
                        } else {
                            tseg.instr_flags[r.unwrap().toffset] |= INSTR_JUMP;
                        }
                        scan_segment(r.unwrap().tseg, r.unwrap().toffset, ne);
                    } else if r.unwrap().size == 2 {
                        /* Segment relocation on 32-bit pointer */
                        tseg.instr_flags[instr.args[0].value] |= INSTR_FAR;
                        if instr.op.name != "call" {
                            tseg.instr_flags[instr.args[0].value] |= INSTR_FUNC;
                        } else {
                            tseg.instr_flags[instr.args[0].value] |= INSTR_JUMP;
                        }
                        scan_segment(r.tseg, instr.args[0].value, ne);
                    }

                    break;
                }
            }
        } else if instr.op.flags & OP_BRANCH {
            /* near relative jump, loop, or call */

            if instr.args[0].value < seg.min_alloc {
                if instr.op.name != "call" {
                    seg.instr_flags[instr.args[0].value] |= INSTR_FUNC;
                } else {
                    seg.instr_flags[instr.args[0].value] |= INSTR_JUMP;
                }
            } else {
                eprint!(
                    "Invalid relative call or jump to {:x} (Segment size {:x}).\n",
                    instr.args[0].value, seg.min_alloc
                );
            }

            /* scan it */
            scan_segment(cs, instr.args[0].value, ne);
        }

        if instr.op.flags & OP_STOP {
            return;
        }

        ip += instr_length;
    }

    eprint!("Scan reached the end of Segment.\n");
}

pub fn print_segment_flags(flags: &u16) {
    let mut buffer = String::new();

    if (flags & 0x0001) {
        buffer += "data";
    } else {
        buffer += "code";
    }

    /* I think these three should never occur in a file */
    if (flags & 0x0002) {
        buffer += ", allocated";
    }
    if (flags & 0x0004) {
        buffer += ", loaded";
    }
    if (flags & 0x0008) {
        buffer += ", iterated";
    }

    if (flags & 0x0010) {
        buffer += ", moveable";
    }
    if (flags & 0x0020) {
        buffer += ", shareable";
    }
    if (flags & 0x0040) {
        buffer += ", preload";
    }
    if (flags & 0x0080) {
        buffer += if flags & 0x0001 {
            ", read-only"
        } else {
            ", execute-only"
        };
        if (flags & 0x0100) {
            buffer += ", has relocation data";
        }

        /* there's still an unidentified flag 0x0400 which appears in all of my testcases.
         * but WINE doesn't know what it is, so... */
        if (flags & 0x0800) {
            buffer += ", self-loading";
        }
        if (flags & 0x1000) {
            buffer += ", discardable";
        }
        if (flags & 0x2000) {
            buffer += ", 32-bit";
        }

        if (flags & 0xc608) {
            buffer += fmt!(", (unknown flags 0x:{:04x}", flags & 0xc608);
        }
        println!("    Flags: 0x{:04x} ({})\n", flags, buffer);
    }

    pub fn read_reloc(seg: &NeSegment, index: u16, ne: &NeExecutable) {
        let entry: usize = seg.start + seg.length + 2 + (index * 8);
        let r = &mut seg.reloc_table[index];
        let size = read_byte(&ne.file, entry);
        let r_type = read_byte(&ne.file, entry + 1);
        let offset = read_word(&ne.file, entry + 2);
        let module = read_word(&ne.file, entry + 4); /* or Segment */
        let ordinal = read_word(&ne.file, entry + 6); /* or offset */
        r.reloc_size = size;
        r.reloc_type = r_type & 3;

        if (r_type & 3) == 0 {
            /* internal reference */
            // char *name;
            if module == 0xff {
                r.tseg = ne.enttab[ordinal - 1].segment;
                r.toffset = ne.enttab[ordinal - 1].offset;
            } else {
                r.tseg = module;
                r.toffset = ordinal;
            }

            /* grab the name, if we can */
            let name = get_entry_name(r.tseg, r.toffset, ne);
            if name.is_some() {
                r.text = name;
            }
        } else if (r_type & 3) == 1 {
            /* imported ordinal */
            r.tseg = module;
            r.toffset = ordinal;
        } else if (r_type & 3) == 2 {
            /* imported name */
            r.tseg = module;
            r.toffset = ordinal;
        } else if (r_type & 3) == 3 {
            /* OSFIXUP */
            /* FIXME: the meaning of this is not understood! */
            return;
        }

        if r_type & !7 {
            eprintln!(
                "{}: Relocation with unknown type flags {:x}.",
                r_type, flags
            );
        }

        if size != 2 && size != 3 && size != 5 {
            eprintln!("{}: Relocation with unknown size {}.", r_type, size);
        }

        /* get the offset list */
        offset_cursor = offset;
        r.offset_count = 0;
        loop {
            /* One of my testcases has relocation offsets that exceed the length of
             * the Segment. Until we figure out what that's about, ignore them. */
            if offset_cursor >= seg.length {
                eprintln!(
                    "{}:{:04x}: Relocation offset exceeds Segment length ({:04x}).\n",
                    seg.cs, offset_cursor, seg.length
                );
                break;
            }

            if seg.instr_flags[offset_cursor] & INSTR_RELOC {
                eprintln!(
                    "{}:{:04x}: Infinite loop reading relocation data.\n",
                    seg.cs, offset_cursor
                );
                r.offset_count = 0;
                return;
            }

            r.offset_count += 1;
            seg.instr_flags[offset_cursor] |= INSTR_RELOC;

            next = read_word(&ne.file, seg.start + offset_cursor);
            if r_type & 4 {
                if !next {
                    break;
                }
                offset_cursor += next;
            } else {
                offset_cursor = next;
            }
            if !(next < 0xfffb) {
                break;
            }
        }

        // r.offsets = malloc(r.offset_count*sizeofarg.arg_type);

        offset_cursor = offset;
        r.offset_count = 0;
        loop {
            if offset_cursor >= seg.length {
                break;
            }

            r.offsets[r.offset_count] = offset_cursor;
            r.offset_count += 1;

            next = read_word(&ne.file, seg.start + offset_cursor);
            if r_type & 4 {
                if !next {
                    break;
                }
                offset_cursor += next;
            } else {
                offset_cursor = next;
            }
            if !(next < 0xfffb) {
                break;
            }
        }
    }
}

// pub fn free_reloc(struct reloc *reloc_data, u16 reloc_count) {
//     int i;
//     for (i = 0; i < reloc_count; i += 1) {
//         free(reloc_data[i].offsets);
//     }
//
//     free(reloc_data);
// }

pub fn read_segments(start: usize, ne: &mut NeExecutable) {
    let entry_cs = ne.header.ne_cs;
    let entry_ip = ne.header.ne_ip;
    let count = ne.header.ne_cseg;
    // struct segment *seg;
    // u16 i, j;
    // ne.segments = malloc(count * sizeof(struct segment));

    for i in 0..count {
        seg = &ne.segments[i];
        seg.cs = i + 1;
        seg.start = read_word(&ne.file, start + i * 8) << ne.header.ne_align;
        seg.length = read_word(&ne.file, start + i * 8 + 2);
        seg.flags = read_word(&ne.file, start + i * 8 + 4);
        seg.min_alloc = read_word(&ne.file, start + i * 8 + 6);

        /* Use min_alloc rather than length because data can "hang over". */
        // seg.instr_flags = calloc(seg.min_alloc, sizeof(u8));
    }

    /* First pass: just read the relocation data */
    for i in 0..count {
        seg = &ne.segments[i];

        if seg.flags & 0x0100 {
            seg.reloc_count = read_word(&ne.file, seg.start + seg.length);
            // seg.reloc_table = malloc(seg.reloc_count * sizeof(struct reloc));

            for j in 0..seg.reloc_count {
                read_reloc(seg, j, ne);
            }
        } else {
            seg.reloc_count = 0;
            seg.reloc_table = NULL;
        }
    }

    /* Second pass: scan Entry points (we have to do this after we read
     * relocation data for all segments.) */
    for i in 0..ne.entcount {
        /* don't scan exported values */
        if ne.enttab[i].segment == 0 || ne.enttab[i].segment == 0xfe {
            continue;
        }

        /* or values that live in data segments */
        if ne.segments[ne.enttab[i].segment - 1].flags & 0x0001 {
            continue;
        }

        /* Annoyingly, data can be put in code segments, and without any
         * apparent indication that it is not code. As a dumb heuristic,
         * only scan exported entries—this won't work universally, and it
         * may potentially miss private entries, but it's better than nothing. */
        if !(ne.enttab[i].flags & 1) {
            continue;
        }

        scan_segment(ne.enttab[i].segment as u16, ne.enttab[i].offset, ne);
        ne.segments[ne.enttab[i].segment - 1].instr_flags[ne.enttab[i].offset] |= INSTR_FUNC;
    }

    /* and don't forget to scan the program Entry point */
    if entry_cs == 0 && entry_ip == 0 {
        /* do nothing */
    } else if entry_ip >= ne.segments[entry_cs - 1].length {
        /* see note above under relocations */
        eprint!(
            "Entry point {}:{:04x} exceeds Segment length ({:04x})\n",
            entry_cs,
            entry_ip,
            ne.segments[entry_cs - 1].length
        );
    } else {
        ne.segments[entry_cs - 1].instr_flags[entry_ip] |= INSTR_FUNC;
        scan_segment(entry_cs, entry_ip, ne);
    }
}

// void free_segments(struct ne *ne) {
//     unsigned cs;
//     struct segment *seg;
//
//     for (cs = 1; cs <= ne.header.ne_cseg; cs += 1) {
//         seg = &ne.segments[cs-1];
//         free_reloc(seg.reloc_table, seg.reloc_count);
//         free(seg.instr_flags);
//     }
//
//     free(ne.segments);
// }

pub fn print_segments(ne: &NeExecutable) {
    // unsigned cs;
    // struct segment *seg;

    /* Final pass: print data */
    for cs in 1..ne.header.ne_cs {
        seg = &ne.segments[cs - 1];

        print!('\n');
        print!("Segment {} (start = 0x%lx, length = 0x%x, minimum allocation = 0x%x):\n",
            cs, seg.start, seg.length, seg.min_alloc ? seg.min_alloc : 65536);
        print_segment_flags(seg.flags);

        if (seg.flags & 0x0001) {
            /* FIXME: We should at least make a special note of Entry points. */
            /* FIXME #2: Data segments can still have relocations... */
            print_data(seg);
        } else {
            /* like objdump, print the whole code Segment like a data Segment */
            if (opts & FULL_CONTENTS) {
                print_data(seg);
            }
            print_disassembly(seg, ne);
        }
    }
}
