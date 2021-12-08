use std::{error, mem};
use std::fs::File;
use std::io::{Read, Write};
use crate::semblance::{DEMANGLE, DISASSEMBLE, read_byte, read_data, read_word, SPECFILE};

#[derive(Clone,Debug, Default)]
pub struct NeHeader {
    pub ne_magic: u16,             /* 00 NE signature 'NE' */
    pub ne_ver: u8,               /* 02 Linker version number */
    pub ne_rev: u8,               /* 03 Linker revision number */
    pub ne_enttab: u16,            /* 04 Offset to Entry table */
    pub ne_cbenttab: u16,          /* 06 Length of Entry table in bytes */
    pub ne_crc: u32,               /* 08 Checksum */
    pub ne_flags: u16,             /* 0c Flags about segments in this file */
    pub ne_autodata: u8,          /* 0e Automatic data Segment number */
    pub ne_unused: u8,            /* 0f */
    pub ne_heap: u16,              /* 10 Initial size of local heap */
    pub ne_stack: u16,             /* 12 Initial size of stack */
    pub ne_ip: u16,                /* 14 Initial IP */
    pub ne_cs: u16,                /* 16 Initial CS */
    pub ne_sp: u16,                /* 18 Initial SP */
    pub ne_ss: u16,                /* 1a Initial SS */
    pub ne_cseg: u16,              /* 1c # of entries in Segment table */
    pub ne_cmod: u16,              /* 1e # of entries in import module table */
    pub ne_cbnrestab: u16,         /* 20 Length of nonresident-name table */
    pub ne_segtab: u16,            /* 22 Offset to Segment table */
    pub ne_rsrctab: u16,           /* 24 Offset to resource table */
    pub ne_restab: u16,            /* 26 Offset to resident-name table */
    pub ne_modtab: u16,            /* 28 Offset to import module table */
    pub ne_imptab: u16,            /* 2a Offset to name table */
    pub ne_nrestab: u32,           /* 2c ABSOLUTE Offset to nonresident-name table */
    pub ne_cmovent: u16,           /* 30 # of movable Entry points */
    pub ne_align: u16,             /* 32 Logical sector alignment shift count */
    pub ne_cres: u16,              /* 34 # of resource segments */
    pub ne_exetyp: u8,            /* 36 Flags indicating target OS */
    pub ne_flagsothers: u8,       /* 37 Additional information flags */
    pub ne_pretthunks: u16,        /* 38 Offset to return thunks */
    pub ne_psegrefbytes: u16,      /* 3a Offset to Segment ref. bytes */
    pub ne_swaparea: u16,          /* 3c Reserved by Microsoft */
    pub ne_expver_min: u8,        /* 3e Expected Windows version number (minor) */
    pub ne_expver_maj: u8,        /* 3f Expected Windows version number (major) */
}

impl NeHeader {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        unimplemented!()
    }
}

#[derive(Clone,Debug, Default)]
pub struct NeEntry {
    pub flags: u8,
    pub segment: u8,
    pub offset: u16,
    pub name: String,
}

#[derive(Clone,Debug, Default)]
pub struct NeExport {
    pub ordinal: u16,
    pub name: String,
}

#[derive(Clone,Debug, Default)]
pub struct NeImportModule {
    pub name: String,
    pub exports: Vec<NeExport>,
}

#[derive(Clone,Debug, Default)]
pub struct NeReloc {
    pub size: u8,
    pub reloc_type: u8,
    pub offset_count: u16,
    pub offsets: Vec<u16>,
    pub tseg: u16,
    pub toffset: u16,
    pub text: String,
}

#[derive(Clone,Debug, Default)]
pub struct NeSegment {
    pub cs: u16,
    pub start: usize,
    pub length: u16,
    pub flags: u16,
    pub min_alloc: u16,
    pub instr_flags: Vec<u8>,
    pub reloc_table: Vec<NeReloc>,
}

#[derive(Clone,Debug, Default)]
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

pub const INT_TYPES: [String;9] = [
    "signed char".to_string(),      /* C */
    "char".to_string(),             /* D */
    "unsigned char".to_string(),    /* E */
    "short".to_string(),            /* F */
    "unsigned short".to_string(),   /* G */
    "int".to_string(),              /* H */
    "unsigned int".to_string(),     /* I */
    "long".to_string(),             /* J */
    "unsigned long".to_string(),    /* K */
];

pub fn print_flags(flags: u16){
    let mut buffer = String::new();
    if (flags & 0x0003) == 0 { buffer += "no DGROUP"; }
    else if (flags & 0x0003) == 1 { buffer += "single DGROUP"; }
    else if (flags & 0x0003) == 2 { buffer += "multiple DGROUPs"; }
    else if (flags & 0x0003) == 3 { buffer += "(unknown DGROUP type 3)"; }
    if flags & 0x0004 { buffer += ", global initialization"; }
    if flags & 0x0008 { buffer += ", protected mode only"; }
    if flags & 0x0010 { buffer += ", 8086"; }
    if flags & 0x0020 { buffer += ", 80286"; }
    if flags & 0x0040 { buffer += ", 80386"; }
    if flags & 0x0080 { buffer += ", 80x87"; }
    if (flags & 0x0700) == 0x0100 { buffer += ", fullscreen"; } /* FRAMEBUF */
    else if (flags & 0x0700) == 0x0200 { buffer += ", console"; }/* API compatible */
    else if (flags & 0x0700) == 0x0300 { buffer += ", GUI"; }/* uses API */
    else if (flags & 0x0700) == 0 { buffer += ", (no subsystem)"; } /* none? */
    else { buffer += fmt!(", (unknown application type {})", (flags & 0x0700) >> 8); }
    if flags & 0x0800 { buffer += ", self-loading"; } /* OS/2 family */
    if flags & 0x1000 { buffer += ", (unknown flag 0x1000)"; }
    if flags & 0x2000 { buffer += ", contains linker errors"; }
    if flags & 0x4000 { buffer += ", non-conforming program"; }
    if flags & 0x8000 { buffer += ", library"; }
    print!("Flags: 0x{:04x} {}\n", flags, buffer);
}

pub fn print_os2flags(flags: u16){
    let mut buffer = String::new();
    if flags & 0x0001 { buffer += ", long filename support"; }
    if flags & 0x0002 { buffer += ", 2.x protected mode"; }
    if flags & 0x0004 { buffer += ", 2.x proportional fonts"; }
    if flags & 0x0008 { buffer += ", fast-load area"; } /* gangload */
    if flags & 0xfff0 {
        buffer += fmt!(", (unknown flags 0x{:04x}", flags & 0xfff0);
    }

    if buffer[0] {
        print!("OS/2 flags: 0x{:04x} {}\n", flags, buffer[2..]);
    }
    else {
        print!("OS/2 flags: 0x0000\n");
    }
}

pub const EXETYPES: [String;6] = [
    "unknown".to_string(),                  /* 0 */
    "OS/2".to_string(),                     /* 1 */
    "Windows (16-bit)".to_string(),         /* 2 */
    "European Dos 4.x".to_string(),         /* 3 */
    "Windows 386 (32-bit)".to_string(),     /* 4 */
    "BOSS".to_string(),                     /* 5 */
    ];

pub fn print_header(header: &header_ne){
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
        eprint!("Header byte at position 0f has value 0x{:02x}.\n", header.ne_unused);
    }
    print!("Heap size: {} bytes\n", header.ne_heap); /* 10 */
    print!("Stack size: {} bytes\n", header.ne_stack); /* 12 */
    print!("Program Entry point: {}:{:04x}\n", header.ne_cs, header.ne_ip); /* 14 */
    print!("Initial stack location: {}:{:04x}\n", header.ne_ss, header.ne_sp); /* 18 */
    if header.ne_exetyp <= 5 {
        /* 36 */
        print!("Target OS: {}\n", EXETYPES[header.ne_exetyp]);
    }
    else {
        print!("Target OS: (unknown value {})\n", header.ne_exetyp);
    }
    print_os2flags(header.ne_flagsothers); /* 37 */
    print!("Swap area: {}\n", header.ne_swaparea); /* 3c */
    print!("Expected Windows version: {}.{}\n", /* 3e */
           header.ne_expver_maj, header.ne_expver_min);
}

pub fn print_export(ne: &NeExecutable) {
    for i in 0 .. ne.enttab.len() {

        if ne.enttab[i].segment == 0xfe {
            /* absolute value */
            print!("\t%5d\t   {:04x}\t{}\n", i + 1, ne.enttab[i].offset, ne.enttab[i].name? ne.enttab[i].name: "<no name>");
        }
        else if ne.enttab[i].segment {
            print!("\t{}\t{}:{:04x}\t{}\n", i + 1, ne.enttab[i].segment,
                   ne.enttab[i].offset, if ne.enttab[i].name.len() > 0 { &ne.enttab[i].name} else { "<no name>" });
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
    for i in 0 .. ne.enttab.len() {
        if ne.enttab[i].name {
            specfile.write_fmt(format_args!("{}\t{}\n", i + 1, &ne.enttab[i].name))?;
        }
        else if ne.enttab[i].segment {
            specfile.write_fmt(format_args!("{}\n", i + 1));
        }
    }

    specfile.flush()?;
}

pub fn demangle_protection(buffer: &mut String, start: &String, prot: &mut String, func: &String) -> i32 {
    if start[0] >= 'A' && start[0] <= 'V' {
        if (start[0] - 'A') & 2 {
            *buffer += "static ";
        }
        if (start[0] - 'A') & 4 {
            *buffer += "virtual ";
        }
        if !((start[0]-'A') & 1) {
            *buffer += "near ";
        }
        if ((start[0]-'A') & 24) == 0 {
            *buffer += "private ";
        }
        else if ((start[0]-'A') & 24) == 8 {
            *buffer += "protected ";
        }
        else if ((start[0]-'A') & 24) == 16 {
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
        }
    } else if *start == '_' && start[1] != '$' {
        /* Same as above, but there is an extra character first (which
         * is often V, so is likely to be the protection/etc), and then
         * a number (often 7 or 3). */
        demangle_protection(buffer, start+1, prot, func);
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

pub fn demangle_type(known_names: &mut Vec<String>, buffer: &mut String, var_type: &mut String) -> i32 {
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
        _ => { 0 }
    }
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

    if func[1] == '?'
    {
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
    if prot >= 'A' && prot <= 'V' && !((prot-'A') & 2) {
        if p[0] != 'E' && p[0] != 'F' {
            eprint!("Unknown modifier {} for function {}\n", p[0], func);
        }
        p = p[1..].to_string();
    }

    /* This should mark the calling convention. Always seems to be A,
     * but this corroborates the function body which uses CDECL. */
    if p[0] == 'A' {} /* strcat(buffer, "__cdecl "); */
    else if p[0] == 'C' { buffer += "__pascal "; }
    else { eprint!("Unknown calling convention {} for function {}\n", p[0], func); }

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
        while start[0] != '?' && start[0] != '@' { start -= 1; }
        buffer += &start[1..];
        if start[0] == '?' { break; }
        buffer +=  "::";
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
                }
                else if !len {
                    eprint!("Unknown argument type {} for function {}\n", p, func);
                    len = 1;
                }
                p = p[len..];
            }
            buffer += ", ";
        }
        buffer[buffer.len()-2] = ')';
        buffer[buffer.len()-1] = 0;
    }

    // func = realloc(func, (strlen(buffer)+1)*sizeof(char));
    // strcpy(func, buffer);
    *func = buffer.clone();
    return func.clone();
}

pub fn read_res_name_table(map: &Vec<u8>, start: usize, entry_table: &Vec<NeEntry>) -> String
{
    /* reads (non)resident names into our Entry table */
    let mut cursor = start;
    let mut length = 0u8;
    let mut first = String::new();
    let mut name = String::new();;

    cursor += 1;
    length = read_byte(map, cursor);
    // first = malloc((length+1)*sizeof(char));
    first  = read_data(map, cursor, length as usize).into();
    // memcpy(first, read_data(cursor), length);
    first[length] = 0;
    cursor += length + 2;

    // while (length = read_byte(map, cursor += 1) > 0)
    while length > 0
    {
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

pub fn get_entry_table(start: usize, ne: &mut NeExecutable)
{
    let mut length = 0u8;
    let mut index = 0u8;
    let mut count = 0i32;
    let mut cursor = 0usize;
    let mut i = 0u32;
    let mut w = 0u16;

    /* get a count */
    cursor = start;
    cursor += 1;
    while length = read_byte(&ne.file, cursor)
    {
        cursor += 1;
        index = read_byte(&ne.file, cursor);
        count += length;
        if index != 0 {
            cursor += (if index == 0xff {
                6
            } else { 3 }) *length;
        }
    }
    // ne.enttab = calloc(sizeof(struct entry), count);

    count = 0;
    cursor = start;
    cursor +=1;
    length = read_byte(&ne.file, cursor);
    while length > 0
    {
        cursor += 1;
        index = read_byte(&ne.file, cursor);
        for i in 0 .. length
        {
            if index == 0xff {
                ne.enttab[count].flags = read_byte(&ne.file, cursor);
                w = read_word(&ne.file, cursor + 1);
                if w != 0x3fcd {
                    eprint!("Entry {} has interrupt bytes {:02x} {:02x} (expected 3f cd).\n", count + 1, w & 0xff, w >> 16);
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
        cursor +=1;
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
    let mut spec_lines: Vec<u8> =   Vec::new();
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

pub fn get_import_module_table(start: usize, ne: &NeExecutable)
{
    let mut offset = 0u16;
    let mut length = 0u8;
    let mut i = 0i32;

    // ne.imptab = malloc(ne.header.ne_cmod * sizeof(struct import_module));
    //for (i = 0; i < ne.header.ne_cmod; i += 1) {
    for i in 0 .. ne.header.ne_cmod
    {
        offset = read_word(&ne.file, start + i * 2);
        length = ne.nametab[offset];
        // ne.imptab[i].name = malloc((length+1)*sizeof(char));
        // memcpy(ne.imptab[i].name, &ne.nametab[offset+1], length);
        ne.imptab[i].name = &ne.nametab[offset+1];
        ne.imptab[i].name[length] = 0;

        if mode & DISASSEMBLE {
            load_exports(&mut ne.imptab[i]);
        }
        else {
            ne.imptab[i].exports = NULL;
            ne.imptab[i].export_count = 0;
        }
    }
}

pub fn read_ne_nametab(new: &mut NeExecutable, offset: usize) -> Result<(), Box<dyn error::Error>> {
    let read_offset = offset + ne.header.ne_imptab;
    unimplemented!()
}

pub fn readne(offset_ne: usize, ne: &mut NeExecutable) ->  Result<(), Box<dyn error::Error>> {
    // memcpy(&ne.header, read_data(offset_ne), sizeof(ne.header));
    *ne.header = NeHeader::from_bytes(read_data(&ne.file, offset_ne, mem::size_of::<NeHeader>()).as_ref());

    /* read our various tables */
    get_entry_table(offset_ne + ne.header.ne_enttab, ne);
    ne.name = read_res_name_table(&ne.file, offset_ne + ne.header.ne_restab, &ne.enttab);
    if ne.header.ne_nrestab {
        ne.description = read_res_name_table(&ne.file, ne.header.ne_nrestab as usize, &ne.enttab);
    }
    else {
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
        for i in 0 .. ne.header.ne_cmd {
            print!("\t{}\n", ne.imptab[i].name);
        }
    }

    if mode & DISASSEMBLE {
        print_segments(&ne);
    }

    if mode & DUMPRSRC {
        if ne.header.ne_rsrctab != ne.header.ne_restab {
            print_rsrc(offset_ne + ne.header.ne_rsrctab);
        }
        else {
            print!("No resource table\n");
        }
    }

    freene(&ne);
}

#[derive(Clone,Debug,Default)]
pub struct header_bitmap_info {
    biSize: u32,           /* 00 */
    biWidth: u32,          /* 04 */
    biHeight: u32,         /* 08 */
    biPlanes: u16,         /* 0c */
    biBitCount: u16,       /* 0e */
    biCompression: u32,    /* 10 */
    biSizeImage: u32,      /* 14 */
    biXPelsPerMeter: u32,  /* 18 */
    biYPelsPerMeter: u32,  /* 1c */
    biClrUsed: u32,        /* 20 */
    biClrImportant: u32,   /* 24 */
}
