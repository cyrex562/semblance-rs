use crate::semblance::{
    read_byte, read_data, read_dword, read_string, read_word, DEMANGLE, DISASSEMBLE, SPECFILE,
};
use std::cmp::min;
use std::fs::File;
use std::io::{Read, Write};
use std::{error, mem};

#[derive(Clone, Debug, Default)]
pub struct NeHeader {
    pub ne_magic: u16,        /* 00 NE signature 'NE' */
    pub ne_ver: u8,           /* 02 Linker version number */
    pub ne_rev: u8,           /* 03 Linker revision number */
    pub ne_enttab: u16,       /* 04 Offset to Entry table */
    pub ne_cbenttab: u16,     /* 06 Length of Entry table in bytes */
    pub ne_crc: u32,          /* 08 Checksum */
    pub ne_flags: u16,        /* 0c Flags about segments in this file */
    pub ne_autodata: u8,      /* 0e Automatic data Segment number */
    pub ne_unused: u8,        /* 0f */
    pub ne_heap: u16,         /* 10 Initial size of local heap */
    pub ne_stack: u16,        /* 12 Initial size of stack */
    pub ne_ip: u16,           /* 14 Initial IP */
    pub ne_cs: u16,           /* 16 Initial CS */
    pub ne_sp: u16,           /* 18 Initial SP */
    pub ne_ss: u16,           /* 1a Initial SS */
    pub ne_cseg: u16,         /* 1c # of entries in Segment table */
    pub ne_cmod: u16,         /* 1e # of entries in import module table */
    pub ne_cbnrestab: u16,    /* 20 Length of nonresident-name table */
    pub ne_segtab: u16,       /* 22 Offset to Segment table */
    pub ne_rsrctab: u16,      /* 24 Offset to resource table */
    pub ne_restab: u16,       /* 26 Offset to resident-name table */
    pub ne_modtab: u16,       /* 28 Offset to import module table */
    pub ne_imptab: u16,       /* 2a Offset to name table */
    pub ne_nrestab: u32,      /* 2c ABSOLUTE Offset to nonresident-name table */
    pub ne_cmovent: u16,      /* 30 # of movable Entry points */
    pub ne_align: u16,        /* 32 Logical sector alignment shift count */
    pub ne_cres: u16,         /* 34 # of resource segments */
    pub ne_exetyp: u8,        /* 36 Flags indicating target OS */
    pub ne_flagsothers: u8,   /* 37 Additional information flags */
    pub ne_pretthunks: u16,   /* 38 Offset to return thunks */
    pub ne_psegrefbytes: u16, /* 3a Offset to Segment ref. bytes */
    pub ne_swaparea: u16,     /* 3c Reserved by Microsoft */
    pub ne_expver_min: u8,    /* 3e Expected Windows version number (minor) */
    pub ne_expver_maj: u8,    /* 3f Expected Windows version number (major) */
}

impl NeHeader {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Default)]
pub struct NeEntry {
    pub flags: u8,
    pub segment: u8,
    pub offset: u16,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct NeExport {
    pub ordinal: u16,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct NeImportModule {
    pub name: String,
    pub exports: Vec<NeExport>,
}

#[derive(Clone, Debug, Default)]
pub struct NeReloc {
    pub size: u8,
    pub reloc_type: u8,
    pub offset_count: u16,
    pub offsets: Vec<u16>,
    pub tseg: u16,
    pub toffset: u16,
    pub text: String,
}

#[derive(Clone, Debug, Default)]
pub struct NeSegment {
    pub cs: u16,
    pub start: usize,
    pub length: u16,
    pub flags: u16,
    pub min_alloc: u16,
    pub instr_flags: Vec<u8>,
    pub reloc_table: Vec<NeReloc>,
}

#[derive(Clone, Debug, Default)]
pub struct NeExecutable {
    pub file: Vec<u8>,
    pub header: NeHeader,
    pub name: String,
    pub description: String,
    pub nametab: Vec<String>,
    pub enttab: Vec<NeEntry>,
    pub imptab: Vec<NeImportModule>,
    pub segments: Vec<NeSegment>,
}

pub const INT_TYPES: [String; 9] = [
    "signed char".to_string(),    /* C */
    "char".to_string(),           /* D */
    "unsigned char".to_string(),  /* E */
    "short".to_string(),          /* F */
    "unsigned short".to_string(), /* G */
    "int".to_string(),            /* H */
    "unsigned int".to_string(),   /* I */
    "long".to_string(),           /* J */
    "unsigned long".to_string(),  /* K */
];

pub fn print_flags(flags: u16) {
    let mut buffer = String::new();
    if (flags & 0x0003) == 0 {
        buffer += "no DGROUP";
    } else if (flags & 0x0003) == 1 {
        buffer += "single DGROUP";
    } else if (flags & 0x0003) == 2 {
        buffer += "multiple DGROUPs";
    } else if (flags & 0x0003) == 3 {
        buffer += "(unknown DGROUP type 3)";
    }
    if flags & 0x0004 {
        buffer += ", global initialization";
    }
    if flags & 0x0008 {
        buffer += ", protected mode only";
    }
    if flags & 0x0010 {
        buffer += ", 8086";
    }
    if flags & 0x0020 {
        buffer += ", 80286";
    }
    if flags & 0x0040 {
        buffer += ", 80386";
    }
    if flags & 0x0080 {
        buffer += ", 80x87";
    }
    if (flags & 0x0700) == 0x0100 {
        buffer += ", fullscreen";
    }
    /* FRAMEBUF */
    else if (flags & 0x0700) == 0x0200 {
        buffer += ", console";
    }
    /* API compatible */
    else if (flags & 0x0700) == 0x0300 {
        buffer += ", GUI";
    }
    /* uses API */
    else if (flags & 0x0700) == 0 {
        buffer += ", (no subsystem)";
    }
    /* none? */
    else {
        buffer += fmt!(", (unknown application type {})", (flags & 0x0700) >> 8);
    }
    if flags & 0x0800 {
        buffer += ", self-loading";
    } /* OS/2 family */
    if flags & 0x1000 {
        buffer += ", (unknown flag 0x1000)";
    }
    if flags & 0x2000 {
        buffer += ", contains linker errors";
    }
    if flags & 0x4000 {
        buffer += ", non-conforming program";
    }
    if flags & 0x8000 {
        buffer += ", library";
    }
    print!("Flags: 0x{:04x} {}\n", flags, buffer);
}

pub fn print_os2flags(flags: u16) {
    let mut buffer = String::new();
    if flags & 0x0001 {
        buffer += ", long filename support";
    }
    if flags & 0x0002 {
        buffer += ", 2.x protected mode";
    }
    if flags & 0x0004 {
        buffer += ", 2.x proportional fonts";
    }
    if flags & 0x0008 {
        buffer += ", fast-load area";
    } /* gangload */
    if flags & 0xfff0 {
        buffer += fmt!(", (unknown flags 0x{:04x}", flags & 0xfff0);
    }

    if buffer[0] {
        print!("OS/2 flags: 0x{:04x} {}\n", flags, buffer[2..]);
    } else {
        print!("OS/2 flags: 0x0000\n");
    }
}

pub const EXETYPES: [String; 6] = [
    "unknown".to_string(),              /* 0 */
    "OS/2".to_string(),                 /* 1 */
    "Windows (16-bit)".to_string(),     /* 2 */
    "European Dos 4.x".to_string(),     /* 3 */
    "Windows 386 (32-bit)".to_string(), /* 4 */
    "BOSS".to_string(),                 /* 5 */
];

pub fn print_header(header: &header_ne) {
    /* Still need to deal with:
     *
     * 34 - number of resource segments (all of my testcases return 0)
     * 38 - offset to return thunks (have testcases)
     * 3a - offset to Segment ref. bytes (same)
     */

    print!('\n');
    print!("Linker version: {}.{}\n", header.ne_ver, header.ne_rev); /* 02 */
    print!("Checksum: {:08x}\n", header.ne_crc); /* 08 */
    print_flags(header.ne_flags); /* 0c */
    print!("Automatic data Segment: {}\n", header.ne_autodata);
    if header.ne_unused != 0 {
        eprint!(
            "Header byte at position 0f has value 0x{:02x}.\n",
            header.ne_unused
        );
    }
    print!("Heap size: {} bytes\n", header.ne_heap); /* 10 */
    print!("Stack size: {} bytes\n", header.ne_stack); /* 12 */
    print!(
        "Program Entry point: {}:{:04x}\n",
        header.ne_cs, header.ne_ip
    ); /* 14 */
    print!(
        "Initial stack location: {}:{:04x}\n",
        header.ne_ss, header.ne_sp
    ); /* 18 */
    if header.ne_exetyp <= 5 {
        /* 36 */
        print!("Target OS: {}\n", EXETYPES[header.ne_exetyp]);
    } else {
        print!("Target OS: (unknown value {})\n", header.ne_exetyp);
    }
    print_os2flags(header.ne_flagsothers); /* 37 */
    print!("Swap area: {}\n", header.ne_swaparea); /* 3c */
    print!(
        "Expected Windows version: {}.{}\n", /* 3e */
        header.ne_expver_maj, header.ne_expver_min
    );
}

pub fn print_export(ne: &NeExecutable) {
    for i in 0..ne.enttab.len() {
        if ne.enttab[i].segment == 0xfe {
            /* absolute value */
            print!("\t%5d\t   {:04x}\t{}\n", i + 1, ne.enttab[i].offset, ne.enttab[i].name? ne.enttab[i].name: "<no name>");
        } else if ne.enttab[i].segment {
            print!(
                "\t{}\t{}:{:04x}\t{}\n",
                i + 1,
                ne.enttab[i].segment,
                ne.enttab[i].offset,
                if ne.enttab[i].name.len() > 0 {
                    &ne.enttab[i].name
                } else {
                    "<no name>"
                }
            );
        }
    }
    print!('\n');
}

pub fn print_specfile(ne: &NeExecutable) {
    let mut i;
    let mut specfile: File;
    let mut spec_name: String = "".to_string();

    spec_name += fmt!("{}.ORD", ne.name);
    specfile = std::fs::File::open(spec_name)?;

    specfile.write("# Generated by dump -o\n".as_bytes());
    for i in 0..ne.enttab.len() {
        if ne.enttab[i].name {
            specfile.write_fmt(format_args!("{}\t{}\n", i + 1, &ne.enttab[i].name))?;
        } else if ne.enttab[i].segment {
            specfile.write_fmt(format_args!("{}\n", i + 1));
        }
    }

    specfile.flush()?;
}

pub fn demangle_protection(
    buffer: &mut String,
    start: &String,
    prot: &mut String,
    func: &String,
) -> i32 {
    if start[0] >= 'A' && start[0] <= 'V' {
        if (start[0] - 'A') & 2 {
            *buffer += "static ";
        }
        if (start[0] - 'A') & 4 {
            *buffer += "virtual ";
        }
        if !((start[0] - 'A') & 1) {
            *buffer += "near ";
        }
        if ((start[0] - 'A') & 24) == 0 {
            *buffer += "private ";
        } else if ((start[0] - 'A') & 24) == 8 {
            *buffer += "protected ";
        } else if ((start[0] - 'A') & 24) == 16 {
            *buffer += "public ";
        }
        prot[0] = start[0];
    } else if start[0] == 'Y' {
        *buffer += "near ";
    } else if start[0] == 'Z' {
        /* normally we'd mark far and not near, but most functions which
         * are going to have an exported name will be far. */
    } else if start[0] == 'X' {
        /* It's not clear what this means, but it always seems to be
         * followed by either a number, or a string of text and then @. */
        prot[0] = 'V'; /* just pretend that for now */
        return if start[1] >= '0' && start[1] <= '9' {
            *buffer += "(X0) ";
            *buffer[buffer.len() - 3] = start[1];
            2
        } else {
            start.find('@') + 1
        };
    } else if *start == '_' && start[1] != '$' {
        /* Same as above, but there is an extra character first (which
         * is often V, so is likely to be the protection/etc), and then
         * a number (often 7 or 3). */
        demangle_protection(buffer, start + 1, prot, func);
        return if start[3] >= '0' && start[3] <= '9' {
            *buffer += "(_00) ";
            *buffer[buffer.len() - 4] = start[2];
            *buffer[buffer.len() - 3] = start[3];
            4
        } else {
            start.find('@') + 1;
        };
        return 0;
    } else {
        eprint!("Unknown modifier {} for function {}\n", start[0], func);
        return 0;
    }
    return 1;
}

pub fn demangle_type(
    known_names: &mut Vec<String>,
    buffer: &mut String,
    var_type: &mut String,
) -> i32 {
    if var_type[0] >= 'C' && var_type[0] <= 'K' {
        *buffer += INT_TYPES[var_type[0] - 'C'];
        *buffer += " ";
        return 1;
    }

    return match var_type[0] {
        'A' | 'P' => {
            let mut ret = 0;
            if (var_type[1] - 'A') & 1 {
                *buffer += "const ";
            }
            if (var_type[1] - 'A') & 2 {
                *buffer += "volatile ";
            }
            ret = demangle_type(known_names, buffer, var_type + 2);
            if !((var_type[1] - 'A') & 4) {
                *buffer += "near ";
            }
            *buffer += if var_type[0] == 'A' { "&" } else { "*" };
            ret + 2
        }
        'M' => {
            *buffer += "float ";
            1
        }
        'N' => {
            *buffer += "double ";
            1
        }
        'U' | 'V' => {
            // const char *p = buffer, *end;
            // unsigned int i;

            if var_type[1] >= '0' && var_type[1] <= '9' {
                *buffer += known_names[var_type[1] - '0'];
                *buffer += " ";
                return 3;
            }

            /* These represent structs (U) or types (V), but the name given
             * doesn't seem to need a qualifier. */
            /* something can go between the at signs, but what does it mean? */
            let first_at = var_type.find('@') + 1;
            let second_at = var_type[first_at..].find('@');
            let end = var_type[second_at..];
            if end[-1] == '@' {
                *buffer += &var_type[1..];
            } else {
                *buffer += &var_type[1..];
            }

            for i in 0..10 {
                if known_names[i].is_empty() {
                    known_names[i] = buffer.clone();
                    break;
                }
            }
            *buffer += " ";
            (end + 1) - var_type
        }
        'X' => {
            *buffer += "void ";
            1
        }
        _ => 0,
    };
}

pub fn demangle(func: &mut String) -> String {
    let mut known_types: Vec<String> = Vec::new();
    let mut known_names: Vec<String> = Vec::new();
    let mut known_type_idx = 0;
    let mut known_name_idx = 0;
    let mut buffer = String::new();
    let mut p: String;
    let mut start: String;
    let mut end: String;
    let mut prot = String::new();
    let mut len: i32;

    if func[1] == '?' {
        /* TODO: constructor/destructor */
        return func.clone();
    }

    /* First populate the known names up to the fusnction name. */
    // TODO:
    // for (p = func; *p != '@' && known_name_idx < 10; p = strchr(p, '@') + 1){
    //     known_names[known_name_idx += 1] = strndup(p, strchr(p, '@') - p);
    // }

    /* Figure out the modifiers and calling convention. */
    // buffer[0] = 0;
    p = func[func.find("@@") + 2..];
    len = demangle_protection(&mut buffer, &p, &mut prot, func);
    if !len {
        return func.clone();
    }
    p = p[len..];

    /* The next one seems to always be E or F. No idea why. */
    if prot >= 'A' && prot <= 'V' && !((prot - 'A') & 2) {
        if p[0] != 'E' && p[0] != 'F' {
            eprint!("Unknown modifier {} for function {}\n", p[0], func);
        }
        p = p[1..].to_string();
    }

    /* This should mark the calling convention. Always seems to be A,
     * but this corroborates the function body which uses CDECL. */
    if p[0] == 'A' {
    }
    /* strcat(buffer, "__cdecl "); */
    else if p[0] == 'C' {
        buffer += "__pascal ";
    } else {
        eprint!(
            "Unknown calling convention {} for function {}\n",
            p[0], func
        );
    }

    /* This marks the return value. */
    p = p[1..].into();
    len = demangle_type(&mut known_names, &mut buffer, &mut p);
    if !len {
        eprint!("Unknown return type {} for function {}\n", p[0], func);
        len = 1;
    }
    p = p[len..];

    /* Get the classname. This is in reverse order, so
     * find the first @@ and work backwards from there. */
    end = func[func.find("@@")..];
    start = end;
    loop {
        start -= 1;
        while start[0] != '?' && start[0] != '@' {
            start -= 1;
        }
        buffer += &start[1..];
        if start[0] == '?' {
            break;
        }
        buffer += "::";
        end = start.clone();
    }

    /* Print the arguments. */
    if p[0] == 'X' {
        buffer += "(void)";
    } else {
        buffer += "(";
        while p[0] != '@' {
            if p[0] >= '0' && p[0] <= '9' {
                buffer += known_types[&p[0] - '0'];
                p = p[1..].into();
            } else {
                let var_type = &buffer;
                len = demangle_type(&mut known_names, &mut buffer, &mut p);
                if buffer[&buffer.len() - 1] == ' ' {
                    buffer[&buffer.len() - 1] = 0;
                }
                if len > 1 && known_type_idx < 10 {
                    known_types[known_type_idx += 1] = var_type.clone;
                } else if !len {
                    eprint!("Unknown argument type {} for function {}\n", p, func);
                    len = 1;
                }
                p = p[len..];
            }
            buffer += ", ";
        }
        buffer[buffer.len() - 2] = ')';
        buffer[buffer.len() - 1] = 0;
    }

    // func = realloc(func, (strlen(buffer)+1)*sizeof(char));
    // strcpy(func, buffer);
    *func = buffer.clone();
    return func.clone();
}

pub fn read_res_name_table(map: &Vec<u8>, start: usize, entry_table: &Vec<NeEntry>) -> String {
    /* reads (non)resident names into our Entry table */
    let mut cursor = start;
    let mut length = 0u8;
    let mut first = String::new();
    let mut name = String::new();

    cursor += 1;
    length = read_byte(map, cursor);
    // first = malloc((length+1)*sizeof(char));
    first = read_data(map, cursor, length as usize).into();
    // memcpy(first, read_data(cursor), length);
    first[length] = 0;
    cursor += length + 2;

    // while (length = read_byte(map, cursor += 1) > 0)
    while length > 0 {
        cursor += 1;
        length = read_byte(map, cursor);
        // name = malloc((length+1)*sizeof(char));
        name = read_data(map, cursor, length as usize).into();
        // memcpy(name, read_data(map, cursor, length), length);
        name[length] = 0;
        cursor += length;

        if (opts & DEMANGLE) && name[0] == '?' {
            name = demangle(&mut name);
        }

        entry_table[read_word(map, cursor) - 1].name = name;
        cursor += 2;
    }

    return first;
}

pub fn get_entry_table(start: usize, ne: &mut NeExecutable) {
    let mut length = 0u8;
    let mut index = 0u8;
    let mut count = 0i32;
    let mut cursor = 0usize;
    let mut i = 0u32;
    let mut w = 0u16;

    /* get a count */
    cursor = start;
    cursor += 1;
    while length = read_byte(&ne.file, cursor) {
        cursor += 1;
        index = read_byte(&ne.file, cursor);
        count += length;
        if index != 0 {
            cursor += (if index == 0xff { 6 } else { 3 }) * length;
        }
    }
    // ne.enttab = calloc(sizeof(struct entry), count);

    count = 0;
    cursor = start;
    cursor += 1;
    length = read_byte(&ne.file, cursor);
    while length > 0 {
        cursor += 1;
        index = read_byte(&ne.file, cursor);
        for i in 0..length {
            if index == 0xff {
                ne.enttab[count].flags = read_byte(&ne.file, cursor);
                w = read_word(&ne.file, cursor + 1);
                if w != 0x3fcd {
                    eprint!(
                        "Entry {} has interrupt bytes {:02x} {:02x} (expected 3f cd).\n",
                        count + 1,
                        w & 0xff,
                        w >> 16
                    );
                }
                ne.enttab[count].segment = read_byte(&ne.file, cursor + 3);
                ne.enttab[count].offset = read_word(&ne.file, cursor + 4);
                cursor += 6;
            } else if index == 0x00 {
                /* no entries, just here to skip ordinals */
            } else {
                ne.enttab[count].flags = read_byte(&ne.file, cursor);
                ne.enttab[count].segment = index;
                ne.enttab[count].offset = read_word(&ne.file, cursor + 1);
                cursor += 3;
            }
            count += 1;
        }
        cursor += 1;
        length = read_byte(&ne.file, cursor);
    }

    ne.entcount = count;
}

pub fn load_exports(module: &mut NeImportModule) {
    let mut line = String::New();
    let mut p = String::new();
    let mut count = 0usize;
    let mut ordinal = 0u16;

    let spec_name = fmt!("%.8s.ORD", module.name);
    let mut specfile = std::fs::File::open(spec_name)?;
    // if (!specfile) {
    //     sprintf(spec_name, "spec/%.8s.ORD", module.name);
    //     specfile = fopen(spec_name, "r");
    //     if (!specfile) {
    //         eprint!( "Note: couldn't find specfile for module {}; exported names won't be given.\n", module.name);
    //         eprint!( "      To create a specfile, run `dumpne -o <module.dll>'.\n");
    //         module.exports = NULL;
    //         module.export_count = 0;
    //         return;
    //     }
    // }

    /* first grab a count */
    // count = 0;
    // while (fgets(line, sizeof(line), specfile)) {
    //     if (line[0] == '#' || line[0] == '\n') continue;
    //     count += 1;
    // }
    let mut file_buf: Vec<u8> = Vec::new();
    let bytes_read = specfile.read(&mut file_buf)?;
    let lines = file_buf.split("\n");
    let mut spec_lines: Vec<u8> = Vec::new();
    for line in lines {
        if line[0] == '#'.into() || line[0] == '\n'.into() {
            continue;
        }

        let splits = line.split('\t')?;
        module.exports[count].ordinal = splits[0];
        module.exports[count].name = splits[1];
    }

    // module.exports = malloc(count * sizeof(struct export));

    // fseek(specfile, 0, SEEK_SET);
    // count = 0;
    // while (fgets(line, sizeof(line), specfile)) {
    //     if (line[0] == '#' || line[0] == '\n') continue;
    //     if ((p = strchr(line, '\n'))) *p = 0;   /* kill final newline */
    //     if (sscanf(line, "%hu", &ordinal) != 1) {
    //         eprint!( "Error reading specfile near line: `{}'\n", line);
    //         continue;
    //     }
    //     module.exports[count].ordinal = ordinal;
    //
    //     p = strchr(line, '\t');
    //     if (p) {
    //         p += 1;
    //         module.exports[count].name = strdup(p);
    //
    //         if ((opts & DEMANGLE) && module.exports[count].name[0] == '?')
    //             module.exports[count].name = demangle(module.exports[count].name);
    //     } else {
    //         module.exports[count].name = NULL;
    //     }
    //     count += 1;
    // }

    module.export_count = count;

    // fclose(specfile);
    drop(specfile);
}

pub fn get_import_module_table(start: usize, ne: &NeExecutable) {
    let mut offset = 0u16;
    let mut length = 0u8;
    let mut i = 0i32;

    // ne.imptab = malloc(ne.header.ne_cmod * sizeof(struct import_module));
    //for (i = 0; i < ne.header.ne_cmod; i += 1) {
    for i in 0..ne.header.ne_cmod {
        offset = read_word(&ne.file, start + i * 2);
        length = ne.nametab[offset];
        // ne.imptab[i].name = malloc((length+1)*sizeof(char));
        // memcpy(ne.imptab[i].name, &ne.nametab[offset+1], length);
        ne.imptab[i].name = &ne.nametab[offset + 1];
        ne.imptab[i].name[length] = 0;

        if mode & DISASSEMBLE {
            load_exports(&mut ne.imptab[i]);
        } else {
            ne.imptab[i].exports = NULL;
            ne.imptab[i].export_count = 0;
        }
    }
}

pub fn read_ne_nametab(new: &mut NeExecutable, offset: usize) -> Result<(), Box<dyn error::Error>> {
    let read_offset = offset + ne.header.ne_imptab;
    unimplemented!()
}

pub fn readne(offset_ne: usize, ne: &mut NeExecutable) -> Result<(), Box<dyn error::Error>> {
    // memcpy(&ne.header, read_data(offset_ne), sizeof(ne.header));
    *ne.header =
        NeHeader::from_bytes(read_data(&ne.file, offset_ne, mem::size_of::<NeHeader>()).as_ref());

    /* read our various tables */
    get_entry_table(offset_ne + ne.header.ne_enttab, ne);
    ne.name = read_res_name_table(&ne.file, offset_ne + ne.header.ne_restab, &ne.enttab);
    if ne.header.ne_nrestab {
        ne.description = read_res_name_table(&ne.file, ne.header.ne_nrestab as usize, &ne.enttab);
    } else {
        ne.description = NULL;
    }
    // ne.nametab = read_data(&ne.file, offset_ne + ne.header.ne_imptab);
    read_ne_nametab(new, offset_ne)?;
    get_import_module_table(offset_ne + ne.header.ne_modtab, ne);
    read_segments(offset_ne + ne.header.ne_segtab, ne);
    Ok(())
}

pub fn dumpne(offset_ne: usize) {
    let mut ne: NeExecutable = NeExecutable::default();

    readne(offset_ne, &mut ne)?;

    if mode == SPECFILE {
        print_specfile(&ne);
        freene(&ne);
        return;
    }

    print!("Module type: NE (New Executable)\n");
    print!("Module name: {}\n", ne.name);
    if ne.description {
        print!("Module description: {}\n", ne.description);
    }

    if mode & DUMPHEADER {
        print_header(&ne.header);
    }

    if mode & DUMPEXPORT {
        print!('\n');
        print!("Exports:\n");
        print_export(&ne);
    }

    if mode & DUMPIMPORT {
        print!('\n');
        print!("Imported modules:\n");
        for i in 0..ne.header.ne_cmd {
            print!("\t{}\n", ne.imptab[i].name);
        }
    }

    if mode & DISASSEMBLE {
        print_segments(&ne);
    }

    if mode & DUMPRSRC {
        if ne.header.ne_rsrctab != ne.header.ne_restab {
            print_rsrc(offset_ne + ne.header.ne_rsrctab);
        } else {
            print!("No resource table\n");
        }
    }

    freene(&ne);
}

#[derive(Clone, Debug, Default)]
pub struct header_bitmap_info {
    biSize: u32,          /* 00 */
    biWidth: u32,         /* 04 */
    biHeight: u32,        /* 08 */
    biPlanes: u16,        /* 0c */
    biBitCount: u16,      /* 0e */
    biCompression: u32,   /* 10 */
    biSizeImage: u32,     /* 14 */
    biXPelsPerMeter: u32, /* 18 */
    biYPelsPerMeter: u32, /* 1c */
    biClrUsed: u32,       /* 20 */
    biClrImportant: u32,  /* 24 */
}

pub fn dup_string_resource(map: &Vec<u8>, offset: usize) -> String {
    let length = read_byte(map, offset);
    let mut ret = String::new();
    ret = read_data(map, offset + 1, length as usize).into();
    return ret;
}

pub fn print_escaped_string(map: &Vec<u8>, mut offset: usize, mut length: i32) {
    print!('"');
    while length -= 1 {
        offset += 1;
        let c = read_byte(map, offset);
        if c == '\t' as u8 {
            print!("\\t");
        } else if c == '\n' as u8 {
            print!("\\n");
        } else if c == '\r' as u8 {
            print!("\\r");
        } else if c == '"' as u8 {
            print!("\\\"");
        } else if c == '\\' as u8 {
            print!("\\\\");
        } else if c >= ' ' as u8 && c <= '~' as u8 {
            print!(c);
        } else {
            print!("\\x{:02x}", c);
        }
    }
    print!('"');
}

pub fn print_escaped_string0(map: &Vec<u8>, mut offset: usize) -> usize {
    print!('"');
    offset += 1;
    let mut c = read_byte(map, offset);
    while c != 0 {
        if c == '\t' as u8 {
            print!("\\t");
        } else if c == '\n' as u8 {
            print!("\\n");
        } else if c == '\r' as u8 {
            print!("\\r");
        } else if c == '"' as u8 {
            print!("\\\"");
        } else if c == '\\' as u8 {
            print!("\\\\");
        } else if c >= ' ' as u8 && c <= '~' as u8 {
            print!(c);
        } else {
            print!("\\x{:02x}", c);
        }
        offset += 1;
        c = read_byte(map, offset);
    }
    print!('"');
    return offset;
}

pub fn print_timestamp(high: u32, low: u32) {
    unimplemented!()
}

pub const RSRC_TYPES: [String; 19] = [
    "".to_string(),
    "Cursor".to_string(),            /* 1 */
    "Bitmap".to_string(),            /* 2 */
    "Icon".to_string(),              /* 3 */
    "Menu".to_string(),              /* 4 */
    "Dialog box".to_string(),        /* 5 */
    "String".to_string(),            /* 6 */
    "Font directory".to_string(),    /* 7 */
    "Font component".to_string(),    /* 8 */
    "Accelerator table".to_string(), /* 9 */
    "Resource data".to_string(),     /* a */
    "Message table".to_string(),     /* b */
    /* fixme: error table? */
    "Cursor directory".to_string(), /* c */
    "".to_string(),
    "Icon directory".to_string(), /* e */
    "Name table".to_string(),     /* f */
    "Version".to_string(),        /* 10 */
    "".to_string(),               /* fixme: RT_DLGINCLUDE? */
    "".to_string(),
];

pub const RSRC_BMP_COMPRESSION: [String; 15] = [
    "none".to_string(),                 /* 0 */
    "RLE (8 bpp)".to_string(),          /* 1 */
    "RLE (4 bpp)".to_string(),          /* 2 */
    "RGB bit field masks".to_string(),  /* 3 */
    "JPEG".to_string(),                 /* shouldn't occur?    4 */
    "PNG".to_string(),                  /* shouldn't occur?     5 */
    "RGBA bit field masks".to_string(), /* 6 */
    "".to_string(),
    "".to_string(),
    "".to_string(),
    "".to_string(),
    "none (CMYK)".to_string(),       /* 11 */
    "RLE (8 bpp, CMYK)".to_string(), /* 12 */
    "RLE (4 bpp, CMYK)".to_string(), /* 13 */
    "".to_string(),
];

pub fn print_rsrc_flags(flags: u16) {
    if flags & 0x0010 {
        print!(", moveable");
    }
    if flags & 0x0020 {
        print!(", shareable");
    }
    if flags & 0x0040 {
        print!(", preloaded");
    }
    if flags & 0xff8f {
        print!(", (unknown flags 0x{:04x})", flags & 0xff8f);
    }
}

pub const rsrc_dialog_style: [String; 33] = [
    "DS_ABSALIGN".to_string(),      /* 00000001 */
    "DS_SYSMODAL".to_string(),      /* 00000002 */
    "DS_3DLOOK".to_string(),        /* 00000004 */
    "DS_FIXEDSYS".to_string(),      /* 00000008 */
    "DS_NOFAILCREATE".to_string(),  /* 00000010 */
    "DS_LOCALEDIT".to_string(),     /* 00000020 */
    "DS_SETFONT".to_string(),       /* 00000040 */
    "DS_MODALFRAME".to_string(),    /* 00000080 */
    "DS_NOIDLEMSG".to_string(),     /* 00000100 */
    "DS_SETFOREGROUND".to_string(), /* 00000200 */
    "DS_CONTROL".to_string(),       /* 00000400 */
    "DS_CENTER".to_string(),        /* 00000800 */
    "DS_CENTERMOUSE".to_string(),   /* 00001000 */
    "DS_CONTEXTHELP".to_string(),   /* 00002000 */
    "(unrecognized flag 0x00004000)".to_string(),
    "DS_USEPIXELS".to_string(),    /* 00008000 */
    "WS_TABSTOP".to_string(),      /* 00010000 */
    "WS_GROUP".to_string(),        /* 00020000 */
    "WS_THICKFRAME".to_string(),   /* 00040000 */
    "WS_SYSMENU".to_string(),      /* 00080000 */
    "WS_HSCROLL".to_string(),      /* 00100000 */
    "WS_VSCROLL".to_string(),      /* 00200000 */
    "WS_DLGFRAME".to_string(),     /* 00400000 */
    "WS_BORDER".to_string(),       /* 00800000 */
    "WS_MAXIMIZE".to_string(),     /* 01000000 */
    "WS_CLIPCHILDREN".to_string(), /* 02000000 */
    "WS_CLIPSIBLINGS".to_string(), /* 04000000 */
    "WS_DISABLED".to_string(),     /* 08000000 */
    "WS_VISIBLE".to_string(),      /* 10000000 */
    "WS_MINIMIZE".to_string(),     /* 20000000 */
    "WS_CHILD".to_string(),        /* 40000000 */
    "WS_POPUP".to_string(),        /* 80000000 */
    "".to_string(),
];

pub fn print_rsrc_dialog_style(flags: u32) {
    let mut buffer = String::new();

    for i in 0..32 {
        if flags & (1 << i) {
            buffer += ", ";
            buffer += &*rsrc_dialog_style[i];
        }
    }
    print!("    Style: {}\n", buffer[2..]);
}

pub const RSRC_BUTTON_TYPE: [String; 17] = [
    "BS_PUSHBUTTON".to_string(),      /* 0 */
    "BS_DEFPUSHBUTTON".to_string(),   /* 1 */
    "BS_CHECKBOX".to_string(),        /* 2 */
    "BS_AUTOCHECKBOX".to_string(),    /* 3 */
    "BS_RADIOBUTTON".to_string(),     /* 4 */
    "BS_3STATE".to_string(),          /* 5 */
    "BS_AUTO3STATE".to_string(),      /* 6 */
    "BS_GROUPBOX".to_string(),        /* 7 */
    "BS_USERBUTTON".to_string(),      /* 8 */
    "BS_AUTORADIOBUTTON".to_string(), /* 9 */
    "BS_PUSHBOX".to_string(),         /* 10 */
    "BS_OWNERDRAW".to_string(),       /* 11 */
    "(unknown type 12)".to_string(),
    "(unknown type 13)".to_string(),
    "(unknown type 14)".to_string(),
    "(unknown type 15)".to_string(),
    "".to_string(),
];

pub const RSRC_EDIT_STYLE: [String; 17] = [
    "".to_string(),
    "".to_string(),               /* type */
    "ES_MULTILINE".to_string(),   /* 0004 */
    "ES_UPPERCASE".to_string(),   /* 0008 */
    "ES_LOWERCASE".to_string(),   /* 0010 */
    "ES_PASSWORD".to_string(),    /* 0020 */
    "ES_AUTOVSCROLL".to_string(), /* 0040 */
    "ES_AUTOHSCROLL".to_string(), /* 0080 */
    "ES_NOHIDESEL".to_string(),   /* 0100 */
    "ES_COMBO".to_string(),       /* 0200 */
    "ES_OEMCONVERT".to_string(),  /* 0400 */
    "ES_READONLY".to_string(),    /* 0800 */
    "ES_WANTRETURN".to_string(),  /* 1000 */
    "ES_NUMBER".to_string(),      /* 2000 */
    "(unknown flag 0x4000)".to_string(),
    "(unknown flag 0x8000)".to_string(),
    ""..to_string(),
];

pub const rsrc_static_type: [String; 20] = [
    "SS_LEFT".to_string(),           /* 0 */
    "SS_CENTER".to_string(),         /* 1 */
    "SS_RIGHT".to_string(),          /* 2 */
    "SS_ICON".to_string(),           /* 3 */
    "SS_BLACKRECT".to_string(),      /* 4 */
    "SS_GRAYRECT".to_string(),       /* 5 */
    "SS_WHITERECT".to_string(),      /* 6 */
    "SS_BLACKFRAME".to_string(),     /* 7 */
    "SS_GRAYFRAME".to_string(),      /* 8 */
    "SS_WHITEFRAME".to_string(),     /* 9 */
    "SS_USERITEM".to_string(),       /* 10 */
    "SS_SIMPLE".to_string(),         /* 11 */
    "SS_LEFTNOWORDWRAP".to_string(), /* 12 */
    "SS_OWNERDRAW".to_string(),      /* 13 */
    "SS_BITMAP".to_string(),         /* 14 */
    "SS_ENHMETAFILE".to_string(),    /* 15 */
    "SS_ETCHEDHORZ".to_string(),     /* 16 */
    "SS_ETCHEDVERT".to_string(),     /* 17 */
    "SS_ETCHEDFRAME".to_string(),    /* 18 */
    "".to_string(),
];

pub const rsrc_static_style: [String; 15] = [
    "".to_string(),
    "".to_string(),
    "".to_string(),
    "".to_string(),
    "".to_string(), /* type */
    "(unknown flag 0x0020)".to_string(),
    "SS_REALSIZECONTROL".to_string(), /* 0040 */
    "SS_NOPREFIX".to_string(),        /* 0080 */
    "SS_NOTIFY".to_string(),          /* 0100 */
    "SS_CENTERIMAGE".to_string(),     /* 0200 */
    "SS_RIGHTJUST".to_string(),       /* 0400 */
    "SS_REALSIZEIMAGE".to_string(),   /* 0800 */
    "SS_SUNKEN".to_string(),          /* 1000 */
    "SS_EDITCONTROL".to_string(),     /* 2000 */
    0,
];

pub const rsrc_listbox_style: [String; 17] = [
    "LBS_NOTIFY".to_string(),            /* 0001 */
    "LBS_SORT".to_string(),              /* 0002 */
    "LBS_NOREDRAW".to_string(),          /* 0004 */
    "LBS_MULTIPLESEL".to_string(),       /* 0008 */
    "LBS_OWNERDRAWFIXED".to_string(),    /* 0010 */
    "LBS_OWNERDRAWVARIABLE".to_string(), /* 0020 */
    "LBS_HASSTRINGS".to_string(),        /* 0040 */
    "LBS_USETABSTOPS".to_string(),       /* 0080 */
    "LBS_NOINTEGRALHEIGHT".to_string(),  /* 0100 */
    "LBS_MULTICOLUMN".to_string(),       /* 0200 */
    "LBS_WANTKEYBOARDINPUT".to_string(), /* 0400 */
    "LBS_EXTENDEDSEL".to_string(),       /* 0800 */
    "LBS_DISABLENOSCROLL".to_string(),   /* 1000 */
    "LBS_NODATA".to_string(),            /* 2000 */
    "LBS_NOSEL".to_string(),             /* 4000 */
    "LBS_COMBOBOX".to_string(),          /* 8000 */
    "".to_string(),
];

pub const rsrc_combobox_style: [String; 16] = [
    "".to_string(),
    "".to_string(), /* type */
    "".to_string(),
    "".to_string(),                      /* unknown */
    "CBS_OWNERDRAWFIXED".to_string(),    /* 0010 */
    "CBS_OWNERDRAWVARIABLE".to_string(), /* 0020 */
    "CBS_AUTOHSCROLL".to_string(),       /* 0040 */
    "CBS_OEMCONVERT".to_string(),        /* 0080 */
    "CBS_SORT".to_string(),              /* 0100 */
    "CBS_HASSTRINGS".to_string(),        /* 0200 */
    "CBS_NOINTEGRALHEIGHT".to_string(),  /* 0400 */
    "CBS_DISABLENOSCROLL".to_string(),   /* 0800 */
    "".to_string(),                      /* unknown */
    "CBS_UPPERCASE".to_string(),         /* 2000 */
    "CBS_LOWERCASE".to_string(),         /* 4000 */
    "".to_string(),
];

pub fn print_rsrc_control_style(class: u8, flags: u32) {
    let mut buffer = String::new();

    print!("        Style: ");

    match class {
        0x80 => {
            /* Button */
            buffer = rsrc_button_type[flags & 0x000f];

            if flags & 0x0010 {
                buffer += ", (unknown flag 0x0010)";
            }
            if flags & 0x0020 {
                buffer += ", BS_LEFTTEXT";
            }

            if (flags & 0x0040) == 0 {
                buffer += ", BS_TEXT";
            } else {
                if flags & 0x0040 {
                    buffer += ", BS_ICON";
                }
                if flags & 0x0080 {
                    buffer += ", BS_BITMAP";
                }
            }

            if (flags & 0x0300) == 0x0100 {
                buffer += ", BS_LEFT";
            } else if ((flags & 0x0300) == 0x0200) {
                buffer += ", BS_RIGHT";
            } else if ((flags & 0x0300) == 0x0300) {
                buffer += ", BS_CENTER";
            }

            if ((flags & 0x0C00) == 0x0400) {
                buffer += ", BS_TOP";
            } else if ((flags & 0x0C00) == 0x0800) {
                buffer += ", BS_BOTTOM";
            } else if ((flags & 0x0C00) == 0x0C00) {
                buffer += ", BS_VCENTER";
            }

            if (flags & 0x1000) {
                buffer += ", BS_PUSHLIKE";
            }
            if (flags & 0x2000) {
                buffer += ", BS_MULTILINE";
            }
            if (flags & 0x4000) {
                buffer += ", BS_NOTIFY";
            }
            if (flags & 0x8000) {
                buffer += ", BS_FLAT";
            }
        }

        0x81 => {
            /* Edit */
            if (flags & 3) == 0 {
                buffer += "ES_LEFT";
            } else if (flags & 3) == 1 {
                buffer += "ES_CENTER";
            } else if (flags & 3) == 2 {
                buffer += "ES_RIGHT";
            } else if (flags & 3) == 3 {
                buffer += "(unknown type 3)";
            }
            for i in 2..16 {
                if flags & (1 << i) {
                    buffer += ", ";
                    buffer += rsrc_edit_style[i];
                }
            }
        }

        0x82 => {
            /* Static */
            if (flags & 0x001f) <= 0x12 {
                buffer += rsrc_static_type[flags & 0x001f];
            } else {
                buffer += fmt!("(unknown type {})", flags & 0x001f);
            }

            for i in 5..14 {
                if flags & (1 << i) {
                    buffer += ", ";
                    buffer += &*rsrc_static_style[i];
                }
            }
        }

        0x83 => {
            /* ListBox */
            for i in 0..16 {
                if flags & (1 << i) > 0 {
                    buffer += ", ";
                    buffer += &*rsrc_listbox_style[i];
                }
            }
        }

        0x84 =>
        /* ScrollBar */
        {
            if (flags & 0x18) {
                if (flags & 0x08) {
                    buffer += "SBS_SIZEBOX";
                } else if (flags & 0x10) {
                    buffer += "SBS_SIZEGRIP";
                }
                if (flags & 0x02) {
                    buffer += ", SBS_SIZEBOXTOPLEFTALIGN";
                }
                if (flags & 0x04) {
                    buffer += ", SBS_SIZEBOXBOTTOMRIGHTALIGN";
                }
            } else if (flags & 0x01) {
                buffer += "SBS_VERT";
                if (flags & 0x02) {
                    buffer += ", SBS_LEFTALIGN";
                }
                if (flags & 0x04) {
                    buffer += ", SBS_RIGHTALIGN";
                }
            } else {
                buffer += "SBS_HORZ";
                if (flags & 0x02) {
                    buffer += ", SBS_TOPALIGN";
                }
                if (flags & 0x04) {
                    buffer += ", SBS_BOTTOMALIGN";
                }
            }
            if (flags & 0xffe0) {
                buffer += fmt!(", (unknown flags 0x{:04x})", flags & 0xffe0);
            }
        }
        0x85 => {
            /* ComboBox */
            if (flags & 3) == 1 {
                buffer += ", CBS_SIMPLE";
            } else if (flags & 3) == 2 {
                buffer += ", CBS_DROPDOWN";
            } else if (flags & 3) == 3 {
                buffer += ", CBS_DROPDOWNLIST";
            }

            for i in 4..15 {
                if (flags & (1 << i) > 0) && !rsrc_combobox_style[i].is_empty() {
                    buffer += ", ";
                    buffer += &*rsrc_combobox_style[i];
                }
            }
            if (flags & 0x900c) {
                buffer += fmt!(", (unknown flags 0x{:04x})", flags & 0x900c);
            }
        }

        _ => {
            buffer += fmt!("0x{:04x}", flags & 0xffff);
        }
    }

    /* and finally, WS_ flags */
    for i in 16..32 {
        if (flags & (1 << i)) {
            buffer += ", ";
            buffer += &*rsrc_dialog_style[i];
        }
    }

    print!(
        "{}\n",
        if buffer[0] == ',' {
            (&buffer[2..])
        } else {
            &buffer
        }
    );
}

pub struct dialog_control {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    id: u16,
    style: u32,
    class: u8,
}

pub const rsrc_dialog_class: [String; 7] = [
    "Button".to_string(),    /* 80 */
    "Edit".to_string(),      /* 81 */
    "Static".to_string(),    /* 82 */
    "ListBox".to_string(),   /* 83 */
    "ScrollBar".to_string(), /* 84 */
    "ComboBox".to_string(),  /* 85 */
    "".to_string(),
];

pub fn print_rsrc_menu_items(map: &Vec<u8>, depth: i32, mut offset: usize) -> usize {
    // u16 flags, id;
    // char buffer[1024];
    // int i;
    let mut buffer = String::new();
    let mut flags = 0u16;
    let mut id = 0u16;

    loop {
        flags = read_word(map, offset);
        offset += 2;

        print!("        ");
        for i in 0..depth {
            print!("  ");
        }
        if !(flags & 0x0010) {
            /* item ID */
            id = read_word(map, offset);
            offset += 2;
            print!("{}: ", id);
        }

        offset = print_escaped_string0(map, offset);

        /* and print flags */
        buffer[0] = '\0';
        if flags & 0x0001 {
            buffer += ", grayed";
        }
        if flags & 0x0002 {
            buffer += ", inactive";
        }
        if (flags & 0x0004) {
            buffer += ", bitmap";
        }
        if (flags & 0x0008) {
            buffer += ", checked";
        }
        if (flags & 0x0010) {
            buffer += ", popup";
        }
        if (flags & 0x0020) {
            buffer += ", menu bar break";
        }
        if flags & 0x0040 {
            buffer += ", menu break";
        }
        /* don't print ENDMENU */
        if flags & 0xff00 {
            // sprintf(buffer + strlen(buffer), ", unknown flags 0x{:04x}", flags & 0xff00);
            buffer += fmt!(", unknown flags 0x{:04x}", flags & 0xff00);
        }

        if buffer[0] {
            print!(" ({})", buffer[2..]);
        }
        print!('\n');

        /* if we have a popup, recurse */
        if (flags & 0x0010) {
            offset = print_rsrc_menu_items(map, depth + 1, offset);
        }

        if (flags & 0x0080) {
            break;
        }
    }

    return offset;
}

pub struct version_header {
    pub length: u16,       /* 00 */
    pub value_length: u16, /* 02 - always 52 (0x34), the length of the second header */
    /* the "type" field given by Windows is missing */
    pub string: [u8; 16], /* 04 - the fixed string VS_VERSION_INFO\0 */
    pub magic: u32,       /* 14 - 0xfeef04bd */
    pub struct_2: u16,    /* 18 - seems to always be 1.0 */
    pub struct_1: u16,    /* 1a */
    /* 1.2.3.4 &c. */
    pub file_2: u16,          /* 1c */
    pub file_1: u16,          /* 1e */
    pub file_4: u16,          /* 20 */
    pub file_3: u16,          /* 22 */
    pub prod_2: u16,          /* 24 - always the same as the above? */
    pub prod_1: u16,          /* 26 */
    pub prod_4: u16,          /* 28 */
    pub prod_3: u16,          /* 2a */
    pub flags_file_mask: u32, /* 2c - always 2 or 3f...? */
    pub flags_file: u32,      /* 30 */
    pub flags_os: u32,        /* 34 */
    pub flags_type: u32,      /* 38 */
    pub flags_subtype: u32,   /* 3c */
    pub date_1: u32,          /* 40 - always 0? */
    pub date_2: u32,          /* 44 */
}

pub const rsrc_version_file: [String; 7] = [
    "VS_FF_DEBUG".to_string(),        /* 0001 */
    "VS_FF_PRERELEASE".to_string(),   /* 0002 */
    "VS_FF_PATCHED".to_string(),      /* 0004 */
    "VS_FF_PRIVATEBUILD".to_string(), /* 0008 */
    "VS_FF_INFOINFERRED".to_string(), /* 0010 */
    "VS_FF_SPECIALBUILD".to_string(), /* 0020 */
    "".to_string(),
];

pub const rsrc_version_type: [String; 9] = [
    "unknown".to_string(),        /* 0 VFT_UNKNOWN */
    "application".to_string(),    /* 1 VFT_APP */
    "DLL".to_string(),            /* 2 VFT_DLL */
    "device driver".to_string(),  /* 3 VFT_DRV */
    "font".to_string(),           /* 4 VFT_FONT */
    "virtual device".to_string(), /* 5 VFT_VXD */
    "(unknown type 6)".to_string(),
    "static-link library".to_string(), /* 7 VFT_STATIC_LIB */
    "".to_string(),
];

pub const rsrc_version_subtype_drv: [String; 14] = [
    "unknown".to_string(),           /* 0 VFT2_UNKNOWN */
    "printer".to_string(),           /* 1 VFT2_DRV_PRINTER etc. */
    "keyboard".to_string(),          /* 2 */
    "language".to_string(),          /* 3 */
    "display".to_string(),           /* 4 */
    "mouse".to_string(),             /* 5 */
    "network".to_string(),           /* 6 */
    "system".to_string(),            /* 7 */
    "installable".to_string(),       /* 8 */
    "sound".to_string(),             /* 9 */
    "communications".to_string(),    /* 10 */
    "input method".to_string(),      /* 11, found in WINE */
    "versioned printer".to_string(), /* 12 */
    "".to_string(),
];

pub fn print_rsrc_version_flags(header: &version_header) {
    let mut buffer = String::new();
    let mut i = 0i32;

    buffer[0] = '\0';
    for i in 0..6 {
        if header.flags_file & (1 << i) {
            buffer += ", ";
            buffer += &*rsrc_version_file[i];
        }
    }
    if header.flags_file & 0xffc0 {
        buffer += fmt!(", (unknown flags 0x{:04x})", header.flags_file & 0xffc0);
    }
    print!("    File flags: ");
    if (header.flags_file) {
        print!("{}", buffer[2..]);
    }

    buffer[0] = '\0';
    if (header.flags_os == 0) {
        buffer += ", VOS_UNKNOWN";
    } else {
        match (header.flags_os & 0xffff) {
            1 => {
                buffer += ", VOS__WINDOWS16";
            }
            2 => {
                buffer += ", VOS__PM16";
            }
            3 => {
                buffer += ", VOS__PM32";
            }
            4 => {
                buffer += ", VOS__WINDOWS32";
            }
            _ => {
                buffer += fmt!(", (unknown OS 0x{:04x})", header.flags_os & 0xffff);
            }
        }
        match header.flags_os >> 16 {
            1 => {
                buffer += ", VOS_DOS";
            }
            2 => {
                buffer += ", VOS_OS216";
            }
            3 => {
                buffer += ", VOS_OS232";
            }
            4 => {
                buffer += ", VOS_NT";
            }
            5 => {
                buffer += ", VOS_WINCE";
            } /* found in WINE */
            _ => {
                buffer += fmt!(", (unknown OS 0x{:04x})", header.flags_os >> 16);
            }
        }
    }
    print!("\n    OS flags: {}\n", buffer[2..]);

    if header.flags_type <= 7 {
        print!("    Type: {}\n", rsrc_version_type[header.flags_type]);
    } else {
        print!("    Type: (unknown type {})\n", header.flags_type);
    }

    if header.flags_type == 3 {
        /* driver */
        if header.flags_subtype <= 12 {
            print!(
                "    Subtype: {} driver\n",
                rsrc_version_subtype_drv[header.flags_subtype]
            );
        } else {
            print!("    Subtype: (unknown subtype {})\n", header.flags_subtype);
        }
    } else if (header.flags_type == 4) {
        /* font */
        if (header.flags_subtype == 0) {
            print!("    Subtype: unknown font\n");
        } else if (header.flags_subtype == 1) {
            print!("    Subtype: raster font\n");
        } else if (header.flags_subtype == 2) {
            print!("    Subtype: vector font\n");
        } else if (header.flags_subtype == 3) {
            print!("    Subtype: TrueType font\n");
        } else {
            print!("    Subtype: (unknown subtype {})\n", header.flags_subtype);
        }
    } else if (header.flags_type == 5) {
        /* VXD */
        print!("    Virtual device ID: {}\n", header.flags_subtype);
    } else if (header.flags_subtype) {
        /* according to MSDN nothing else is valid */
        print!("    Subtype: (unknown subtype {})\n", header.flags_subtype);
    }
}

pub fn print_rsrc_strings(map: &Vec<u8>, mut offset: usize, end: usize) {
    let mut length = 0u16;

    while offset < end {
        /* first length is redundant */
        length = read_word(map, offset + 2);
        print!("        ");
        offset = print_escaped_string0(map, offset + 4);
        offset = (offset + 3) & !3;
        print!(": ");
        /* According to MSDN this is zero-terminated, and in most cases it is.
         * However, at least one application (msbsolar) has NEs with what
         * appears to be a non-zero-terminated string. In Windows this is cut
         * off at one minus the given length, just like other strings, so
         * we'll do that here.
         *
         * And another file has a zero length here. How do compilers screw this
         * up so badly? */
        print_escaped_string(
            map,
            offset,
            (if length > 0 { length - 1 } else { 0 }) as i32,
        );
        offset += length;
        offset = (offset + 3) & !3;
        print!('\n');
    }
}

pub fn print_rsrc_stringfileinfo(map: &Vec<u8>, mut offset: usize, end: usize) {
    let mut length = 0u16;
    let lang = 0u32;
    let codepage = 0u32;

    /* we already processed the StringFileInfo header */
    while offset < end {
        /* StringTable header */
        length = read_word(map, offset);
        /* codepage and language code */
        let data = read_data(map, offset + 4, length as usize);
        scan_fmt!(data, "{4x}{4x}", lang, codepage);
        // sscanf(read_data(map, offset + 4), "%4x%4x", &lang, &codepage);
        println!(
            "    String table (lang={:04x}, codepage={:04x}):",
            lang, codepage
        );
        print_rsrc_strings(map, offset + 16, offset + length);
        offset += length;
    }
}

pub fn print_rsrc_varfileinfo(mut offset: usize, end: usize) {
    while offset < end {
        /* first length is redundant */
        let length = read_word(map, offset + 2);
        offset += 16;
        for i in (0..length).step_by(4) {
            println!(
                "    Var (lang={:04x}, codepage={:04x})",
                read_word(map, offset + i),
                read_word(map, offset + i + 2)
            );
        }
        offset += length;
    }
}

pub fn print_rsrc_resource(
    map: &Vec<u8>,
    rsrc_type: u16,
    mut offset: usize,
    length: usize,
    rn_id: u16,
) {
    match rsrc_type
    {
    0x8001 => {
        /* Cursor */
        println!("    Hotspot: ({}, {})", read_word(map, offset), read_word(map, offset + 2));
        offset += 4;
        /* fall through */
    }
    0x8002 | /* Bitmap */
    0x8003 => {
        /* Icon */
        if read_dword(map, offset) == 12 /* BITMAPCOREHEADER */ {
            println!("    Size:{}x{}", read_word(map, offset + 4), read_word(map, offset + 6));
            println!("    Planes: {}", read_word(map, offset + 8));
            println!("    Bit depth: {}", read_word(map, offset + 10));
        } else if read_dword(map, offset) == 40 /* BITMAPINFOHEADER */ {
            let header = header_bitmap_info::from_bytes(read_data(map, offset, std::mem::sizeof::<header_bitmap_info>()));
            println!("    Size:{}x{}", header.biWidth, header.biHeight / 2);
            println!("    Planes: {}", header.biPlanes);
            println!("    Bit depth: {}", header.biBitCount);
            if header.biCompression <= 13 && rsrc_bmp_compression[header.biCompression] {
                println!("    Compression: {}", rsrc_bmp_compression[header.biCompression]);
            }
            else{
                println!("    Compression: (unknown value {})", header.biCompression);
            }
            println!("    Resolution:{}x{} pixels/meter",
                   header.biXPelsPerMeter, header.biYPelsPerMeter);
            print!("    Colors used: {}", header.biClrUsed); /* todo: implied */
            if header.biClrImportant {
                print!(" ({} marked important)", header.biClrImportant);
            }
            print!('\n');
        } else { eprintln!("Unknown bitmap header size {}.", read_dword(map, offset)); }
    }
    0x8004 => /* Menu */
    {
        let extended = read_word(map, offset);

        if extended > 1 {
            eprintln!("Unknown menu version {}",extended);
        }
        print!(if extended != 0 {"    Type: extended\n"} else { "    Type: standard\n" });
        if read_word(map, offset + 2) != extended*4 {
            eprintln!("Unexpected offset value {} (expected {}).", read_word(map, offset + 2), extended * 4);
        }
        offset += 4;

        if extended != 0
        {
            println!("    Help ID: {}", read_dword(map, offset));
            offset += 4;
        }

        println!("    Items:");
        print_rsrc_menu_items(map, 0, offset);
    }
    0x8005 => /* Dialog box */
    {
        let style = read_dword(map, offset);
        print_rsrc_dialog_style(style);
        let mut count = read_byte(map, offset + 4);
        println!("    Position: ({}, {})", read_word(map, offset + 5), read_word(map, offset + 7));
        println!("    Size:{}x{}", read_word(map, offset + 9), read_word(map, offset + 11));
        if read_byte(map, offset + 13) == 0xff {
            print!("    Menu resource: #{}", read_word(map, offset + 14));
        } else {
            print!("    Menu name: ");
            offset = print_escaped_string0(map, offset + 13);
        }
        print!("\n    Class name: ");
        offset = print_escaped_string0(map, offset);
        print!("\n    Caption: ");
        offset = print_escaped_string0(map, offset);
        if style & 0x00000040 { /* DS_SETFONT */
            font_size = read_word(map, offset);
            print!("\n    Font: ");
            offset = print_escaped_string0(map, offset + 2);
            print!(" ({} pt)", font_size);
        }
        print!('\n');

        count -= 1;
        while count > 0 {
            let control = dialog_control::from_bytes(read_data(map,offset, std::mem::sizeof::<dialog_control>()));
            offset += sizeof(*control);

            if control.class & 0x80 {
                if control.class <= 0x85 {
                    print!("    {}", rsrc_dialog_class[control.class & (!0x80)]);
                }
                else {
                    print!("    (unknown class {})", control.class);
                }
            }
            else {
                offset = print_escaped_string0(map, offset);
            }
            println!(" {}:", control.id);

            println!("        Position: ({}, {})", control.x, control.y);
            println!("        Size:{}x{}", control.width, control.height);
            print_rsrc_control_style(control.class, control.style);

            if read_byte(map, offset) == 0xff {
                /* todo: we can check the style for SS_ICON/SS_BITMAP and *maybe* also
                 * refer back to a printed RT_GROUPICON/GROUPCUROR/BITMAP resource. */
                print!("        Resource: #{}", read_word(map, offset));
                offset += 3;
            } else {
                print!("        Text: ");
                offset = print_escaped_string0(map, offset );
            }
            /* todo: WINE parses this as "data", but all of my testcases return 0. */
            /* read_byte(); */
            print!('\n');
            count -= 1;
        }
    }
    // break;
    0x8006 => /* String */
    {
        cursor: usize = offset;
        // int i = 0;

        while cursor < offset + length
        {
            cursor += 1;
            let str_length = read_byte(map, cursor);
            if str_length
            {
                print!("    {} (0x{:06x}): ", i + ((rn_id & (!0x8000))-1)*16, cursor);
                print_escaped_string(map, cursor, str_length as i32);
                print!('\n');
                cursor += str_length;
            }
            i += 1;
        }
    }
    // break;
// #if 0 /* No testcases for this either */
    0x8007 | /* Font directory */
    0x8008 => /* Font component */
        {}
    0x8009 => /* Accelerator table */
    {
        /* This format seems to be similar but older. Five bytes per
         * Entry, in the format:
         * [byte] - flags
         * [word] - key
         * [word] - id
         *
         * Problem is, the key codes don't seem to make much sense. In
         * particular we have instances where the virtual flag isn't set
         * but we have C0 control codes. So the mapping must be different
         * than it is for current accelerator tables.
         */
        // u8 flags;

        loop {
            flags = read_byte(map, offset);
            key = read_word(map, offset);
            rid = read_word(map, offset);

            print!("    ");

            if flags & 0x02 {
                print!("(FNOINVERT) ");
            }

            if flags & 0x04 {
                print!("Shift+");
            }
            if flags & 0x08 {
                print!("Ctrl+");
            }
            if flags & 0x10 {
                print!("Alt+");
            }
            if flags & 0x60 {
                eprintln!("Unknown accelerator flags 0x{:02x}", flags & 0x60);
            }

            /* fixme: print the key itself */

            println!(": {}", rn_id);
            if !(!(flags & 0x80)) {
                break;
            }
        }
    }
    // break;
// #endif
    /* Resource data (0x800a) is parsed as default, i.e. hex-dumped. */
    0x800c | /* Cursor directory */
    0x800e => /* Icon directory */
    {
        /* All of the information supplied here is contained in the actual
         * resource. Therefore we only list the components this refers to.
         * Fortunately, the headers are different but the relevant information
         * is stored in the same bytes. */
        let mut count = read_word(map, offset + 4);
        offset += 6;
        print!("    Resources: ");
        count -= 1;
        if count != 0 {
            print!("#{}", read_word(map, offset + 12));
            offset += 14;
        }
        count -= 1;
        while count != 0 {
            print!(", #{}", read_word(map, offset + 12));
            offset += 14;
        }
        print!("\n");
    }
    // break;
    0x8010 => /* Version */
    {
        header = version_header::from_bytes(read_data(map, offset, std::mem::sizeof::<version_header>()));
        let end = offset + header.length;

        if header.value_length != 52 {
            eprint!("Version header length is {} (expected 52).\n", header.value_length);
        }
        if header.string == "VS_VERSION_INFO"  {
            eprint!("Version header is {} (expected VS_VERSION_INFO).\n", header.string);
        }
        if header.magic != 0xfeef04bd {
            eprint!("Version magic number is {:08x} (expected 0xfeef04bd).\n", header.magic);
        }
        if header.struct_1 != 1 || header.struct_2 != 0 {
            eprint!("Version header version is {}.{} (expected 1.0).\n", header.struct_1, header.struct_2);
        }
        print_rsrc_version_flags(*header);

        print!("    File version:    {}.{}.{}.{}\n",
               header.file_1, header.file_2, header.file_3, header.file_4);
        print!("    Product version: {}.{}.{}.{}\n",
               header.prod_1, header.prod_2, header.prod_3, header.prod_4);

        print!("    Created on: ");
        print_timestamp(header.date_1, header.date_2);
        print!('\n');

        offset += std::mem::sizeof::<version_header>();

        while offset < end
        {
            let info_length = read_word(map, offset);
            let value_length = read_word(map, offset + 2);
            let key = read_string(map, offset + 4, info_length as usize);

            if value_length {
                eprintln!("Value length is nonzero: {:04x}", value_length);
            }

            /* "type" is again omitted */
            if key != "StringFileInfo" {
                print_rsrc_stringfileinfo(map, offset + 20, offset + info_length);
            }
            else if key != "VarFileInfo" {
                print_rsrc_varfileinfo(offset + 16, offset + info_length);
            }
            else {
                eprintln!("Unrecognized file info key: {}", key);
            }

            offset += ((info_length + 3) & !3);
        }
        // break;
    }
    _ =>
    {
        let mut cursor: usize = offset;
        /* hexl-style dump */
        while cursor < offset + length
        {
            len = min(offset + length - cursor, 16);

            print!("    {:x}:", cursor);
            for i in 0 .. 16 {
                if !(i & 1) {
                    /* Since this is 16 bits, we put a space after (before) every other two bytes. */
                    print!(' ');
                }
                if i<len {
                    print!("{:02x}", read_byte(map, cursor + i));
                }
                else {
                    print!("  ");
                }
            }
            print!("  ");
            for i in 0 .. len  {
                let c = read_byte(map, cursor + i);
                // print!(isprint(c) ? c : '.');
            }
            print!('\n');

            cursor += len;
        }
    }
    // break;
    }
}

pub fn filter_resource(rsrc_type: &String, id: &String) -> i32 {
    if !resource_filters_count {
        return 1;
    }

    for i in 0..resource_filters_count {
        let filter_type = resource_filters[i];
        let mut p = String::new();
        len = rsrc_type.len();

        /* note that both resource types and IDs are case insensitive */

        /* if the filter is just a resource type or ID and we match that */
        if (rsrc_type != filter_type) || (id != filter_type) {
            return 1;
        }

        /* if the filter is a resource type followed by an ID and we match both */
        if (rsrc_type == filter_type) || (filter_type == ' ') {
            continue;
        }

        // p = filter_type + len;
        // while (*p == ' ')  += 1p;
        if !strcasecmp(id, p) {
            return 1;
        }
    }
    return 0;
}

pub struct Resource {
    pub offset: u16,
    pub length: u16,
    pub flags: u16,
    pub id: u16,
    pub handle: u16, /* fixme: what is this? */
    pub usage: u16,  /* fixme: what is this? */
}

pub struct TypeHeader {
    pub type_id: u16,
    pub count: u16,
    pub resloader: u32, /* fixme: what is this? */
    pub resources: [Resource; 1],
}

impl From<&Vec<u8>> for TypeHeader {
    fn from(bytes: &Vec<u8>) -> Self {
        todo!()
    }
}

impl From<&Resource> for TypeHeader {
    fn from(rsrc: &Resource) -> Self {
        todo!()
    }
}

pub fn print_rsrc(start: usize) {
    let header = TypeHeader::new();
    let align = read_word(map, start);

    let mut header = TypeHeader::from(
        read_data(
            map,
            start + std::mem::sizeof::<u16>(),
            std::mem::sizeof::<TypeHeader>(),
        )
        .as_ref(),
    );

    while header.type_id {
        if header.resloader {
            eprintln!("resloader is nonzero: {:08x}", header.resloader);
        } else {
        }

        for i in 0..header.count {
            let rn = &header.resources[i];

            if rn.id & 0x8000 {
                idstr = fmt!("{}", rn.id & !0x8000);
            } else {
                idstr = dup_string_resource(map, start + rn.id);
            }

            if header.type_id & 0x8000 {
                if (header.type_id & (!0x8000)) < rsrc_types_count
                    && rsrc_types[header.type_id & (!0x8000)]
                {
                    if !filter_resource(rsrc_types[header.type_id & !0x8000], idstr) {
                        // goto
                        // next;
                    }
                    print!("\n{}", rsrc_types[header.type_id & !0x8000]);
                } else {
                    let typestr = fmt!("0x{:04x}", header.type_id);
                    if !filter_resource(typestr, idstr) {
                        // goto
                        // next;
                    }
                    print!("\n{}", typestr);
                }
            } else {
                let typestr = dup_string_resource(map, start + header.type_id);
                if !filter_resource(&typestr, idstr) {
                    // free(typestr);
                    // goto next;
                }
                print!("\n\"{}\"", typestr);
                // free(typestr);
            }

            print!(" {}", idstr);
            print!(
                " (offset = 0x{:x}, length = {} [0x{:x}]",
                rn.offset << align,
                rn.length << align,
                rn.length << align
            );
            print_rsrc_flags(rn.flags);
            println!("):");

            print_rsrc_resource(
                map,
                header.type_id,
                rn.offset << align,
                rn.length << align,
                rn.id,
            );

            // next:
            //             free(idstr);
        }

        header = TypeHeader::from(&header.resources[header.count]);
    }
}
