use crate::util::{read_byte, read_data, read_dword, read_qword, read_string, read_word};
use crate::x86::defines::Instruction;
use crate::x86::defines::X86ArgType::{IMM, MEM, MOFFS, NONE, REL, REL8, RM};
use crate::x86::defines::{
    Argument, INSTR_FUNC, INSTR_JUMP, INSTR_RELOC, INSTR_SCANNED, INSTR_VALID, OP_BRANCH, OP_STOP,
};
use crate::{DISASSEMBLE, DISASSEMBLE_ALL, FULL_CONTENTS, SPECFILE};
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct PeFileHeader {
    pub Machine: u16,              /* 04 */
    pub NumberOfSections: u16,     /* 06 */
    pub TimeDateStamp: u32,        /* 08 */
    pub PointerToSymbolTable: u32, /* 0c */
    pub NumberOfSymbols: u32,      /* 10 */
    pub SizeOfOptionalHeader: u16, /* 14 */
    pub Characteristics: u16,      /* 16 */
}

impl From<&Vec<u8>> for PeFileHeader {
    fn from(_: &Vec<u8>) -> Self {
        todo!()
    }
}

pub struct PeDirectory {
    pub address: u32,
    pub size: u32,
}

pub struct PeOptionalHeader32 {
    /* Standard COFF fields. */
    pub Magic: u16,                   /* 18 */
    pub MajorLinkerVersion: u8,       /* 1a */
    pub MinorLinkerVersion: u8,       /* 1b */
    pub SizeOfCode: u32,              /* 1c */
    pub SizeOfInitializedData: u32,   /* 20 */
    pub SizeOfUninitializedData: u32, /* 24 */
    pub AddressOfEntryPoint: u32,     /* 28 */
    pub BaseOfCode: u32,              /* 2c */
    pub BaseOfData: u32,              /* 30 */

    /* PE fields. */
    pub ImageBase: u32,                   /* 34 */
    pub SectionAlignment: u32,            /* 38 */
    pub FileAlignment: u32,               /* 3c */
    pub MajorOperatingSystemVersion: u16, /* 40 */
    pub MinorOperatingSystemVersion: u16, /* 42 */
    pub MajorImageVersion: u16,           /* 44 */
    pub MinorImageVersion: u16,           /* 46 */
    pub MajorSubsystemVersion: u16,       /* 48 */
    pub MinorSubsystemVersion: u16,       /* 4a */
    pub Win32VersionValue: u32,           /* 4c */
    pub SizeOfImage: u32,                 /* 50 */
    pub SizeOfHeaders: u32,               /* 54 */
    pub CheckSum: u32,                    /* 58 */
    pub Subsystem: u16,                   /* 5c */
    pub DllCharacteristics: u16,          /* 5e */
    pub SizeOfStackReserve: u32,          /* 60 */
    pub SizeOfStackCommit: u32,           /* 64 */
    pub SizeOfHeapReserve: u32,           /* 68 */
    pub SizeOfHeapCommit: u32,            /* 6c */
    pub LoaderFlags: u32,                 /* 70 */
    pub NumberOfRvaAndSizes: u32,         /* 74 */
}

impl From<&Vec<u8>> for PeOptionalHeader32 {
    fn from(_: &Vec<u8>) -> Self {
        todo!()
    }
}

// STATIC_ASSERT(sizeof(struct optional_header) == 0x60);

pub struct PeOptionalHeader64 {
    /* Standard COFF fields. */
    pub Magic: u16,                   /* 18 */
    pub MajorLinkerVersion: u8,       /* 1a */
    pub MinorLinkerVersion: u8,       /* 1b */
    pub SizeOfCode: u32,              /* 1c */
    pub SizeOfInitializedData: u32,   /* 20 */
    pub SizeOfUninitializedData: u32, /* 24 */
    pub AddressOfEntryPoint: u32,     /* 28 */
    pub BaseOfCode: u32,              /* 2c */

    /* PE fields. */
    pub ImageBase: u64,                   /* 30 */
    pub SectionAlignment: u32,            /* 38 */
    pub FileAlignment: u32,               /* 3c */
    pub MajorOperatingSystemVersion: u16, /* 40 */
    pub MinorOperatingSystemVersion: u16, /* 42 */
    pub MajorImageVersion: u16,           /* 44 */
    pub MinorImageVersion: u16,           /* 46 */
    pub MajorSubsystemVersion: u16,       /* 48 */
    pub MinorSubsystemVersion: u16,       /* 4a */
    pub Win32VersionValue: u32,           /* 4c */
    pub SizeOfImage: u32,                 /* 50 */
    pub SizeOfHeaders: u32,               /* 54 */
    pub CheckSum: u32,                    /* 58 */
    pub Subsystem: u16,                   /* 5c */
    pub DllCharacteristics: u16,          /* 5e */
    pub SizeOfStackReserve: u64,          /* 60 */
    pub SizeOfStackCommit: u64,           /* 68 */
    pub SizeOfHeapReserve: u64,           /* 70 */
    pub SizeOfHeapCommit: u64,            /* 78 */
    pub LoaderFlags: u32,                 /* 80 */
    pub NumberOfRvaAndSizes: u32,         /* 84 */
}

impl From<&Vec<u8>> for PeOptionalHeader64 {
    fn from(_: &Vec<u8>) -> Self {
        todo!()
    }
}

// STATIC_ASSERT(sizeof(struct optional_header_pep) == 0x70);

pub struct PeSection {
    pub name: [u8; 8],      /* 00 */
    pub min_alloc: u32,     /* 08 */
    pub address: u32,       /* 0c */
    pub length: u32,        /* 10 */
    pub offset: u32,        /* 14 */
    pub reloc_offset: u32,  /* 18 */
    pub lineno_offset: u32, /* 1c */
    pub reloc_count: u16,   /* 20 */
    pub lineno_count: u16,  /* 22 */
    pub flags: u32,         /* 24 */
    /* and our data: */
    // byte *instr_flags;
    pub instr_flags: Vec<u8>,
}

pub struct PeReloc {
    pub offset: u32,
    pub reloc_type: u32,
}

pub struct PeExport {
    pub address: u32,
    pub ordinal: u16,
    pub name: String,
}

pub struct PeNameTableEntry {
    pub name: String,
    pub ordinal: u16,
    pub is_ordinal: bool,
}

pub struct PeImportModule {
    pub module: String,
    pub iat_addr: u32,
    pub nametab: Vec<PeNameTableEntry>,
    pub count: usize,
}

pub struct PeExecutable {
    pub file: Vec<u8>,
    pub magic: u16,     /* same as opt.Magic field, but avoids casting */
    pub imagebase: u64, /* same as opt.ImageBase field, but simpler */
    pub header: PeFileHeader,
    // pub opt32: Vec<OptionalHeader>,
    pub opt32: PeOptionalHeader32,
    pub opt64: PeOptionalHeader64,
    // pub opt64: Vec<OptionalHeaderPep>,
    pub dirs: Vec<PeDirectory>,
    pub name: String,
    pub sections: Vec<PeSection>,
    pub exports: Vec<PeExport>,
    pub export_count: usize,
    pub imports: Vec<PeImportModule>,
    pub import_count: usize,
    pub relocs: Vec<PeReloc>,
    pub reloc_count: usize,
}

/* in pe_section.c */
// extern struct section *addr2section(dword addr, const struct pe *pe);
// extern off_t addr2offset(dword addr, const struct pe *pe);
// extern void read_sections(struct pe *pe);
// extern void print_sections(struct pe *pe);
// #endif /* __PE_H */
pub const PE_SUBSYSTEMS: [String; 17] = [
    "unknown".to_string(),
    "native".to_string(),
    "GUI".to_string(),
    "CUI".to_string(),
    "(unknown value 4)".to_string(),
    "OS/2 CUI".to_string(),
    "(unknown value 6)".to_string(),
    "POSIX CUI".to_string(),
    "(unknown value 8)".to_string(),
    "CE".to_string(),
    "EFI".to_string(),
    "EFI with boot services".to_string(),
    "EFI with runtime services".to_string(),
    "EFI ROM image".to_string(),
    "Xbox".to_string(),
    "(unknown value 15)".to_string(),
    "boot",
];

pub struct PeExportHeader {
    pub flags: u32,            /* 00 */
    pub timestamp: u32,        /* 04 */
    pub ver_major: u16,        /* 08 */
    pub ver_minor: u16,        /* 0a */
    pub module_name_addr: u32, /* 0c */
    pub ordinal_base: u32,     /* 10 */
    pub addr_table_count: u32, /* 14 */
    pub export_count: u32,     /* 18 */
    pub addr_table_addr: u32,  /* 1c */
    pub name_table_addr: u32,  /* 20 */
    pub ord_table_addr: u32,   /* 24 */
}

impl From<&Vec<u8>> for PeExportHeader {
    fn from(_: &Vec<u8>) -> Self {
        todo!()
    }
}

pub fn print_flags(flags: u16) {
    let mut buffer: String = "".to_string();
    if flags & 0x0001 {
        buffer += ", relocations stripped";
    }
    if flags & 0x0002 {
        buffer += ", executable";
    }
    if flags & 0x0004 {
        buffer += ", line numbers stripped";
    }
    if flags & 0x0008 {
        buffer += ", local symbols stripped";
    }
    if flags & 0x0010 {
        buffer += ", aggressively trimmed";
    }
    if flags & 0x0020 {
        buffer += ", large address aware";
    }
    if flags & 0x0040 {
        buffer += ", 16-bit";
    }
    if flags & 0x0080 {
        buffer += ", little-endian";
    }
    if flags & 0x0100 {
        buffer += ", 32-bit";
    }
    if flags & 0x0200 {
        buffer += ", debug info stripped";
    }
    if flags & 0x0400 {
        buffer += ", IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP";
    }
    if flags & 0x0800 {
        buffer += ", IMAGE_FILE_NET_RUN_FROM_SWAP";
    }
    if flags & 0x1000 {
        buffer += ", system file";
    }
    if flags & 0x2000 {
        buffer += ", DLL";
    }
    if flags & 0x4000 {
        buffer += ", uniprocessor";
    }
    if flags & 0x8000 {
        buffer += ", big-endian";
    }

    println!("Flags: {:04x} {}", flags, buffer);
}

pub fn print_dll_flags(flags: u16) {
    let mut buffer: String = "".to_string();
    if flags & 0x0001 {
        buffer += ", per-process initialization";
    }
    if flags & 0x0002 {
        buffer += ", per-process termination";
    }
    if flags & 0x0004 {
        buffer += ", per-thread initialization";
    }
    if flags & 0x0008 {
        buffer += ", per-thread termination";
    }
    if flags & 0x0040 {
        buffer += ", dynamic base";
    }
    if flags & 0x0080 {
        buffer += ", force integrity";
    }
    if flags & 0x0100 {
        buffer += ", DEP compatible";
    }
    if flags & 0x0200 {
        buffer += ", no isolation";
    }
    if flags & 0x0400 {
        buffer += ", no SEH";
    }
    if flags & 0x0800 {
        buffer += ", no bind";
    }
    if flags & 0x2000 {
        buffer += ", WDM driver";
    }
    if flags & 0x8000 {
        buffer += ", terminal server aware";
    }
    if flags & 0x5030 {
        buffer += fmt!("unknown flags: {:04x}", flags & 0x4030);
    }
    println!("DLL flags: {:04x} {}\n", flags, buffer);
}

pub fn print_opt32(opt: &PeOptionalHeader32, pe_rel_addr: i32) {
    println!(
        "file version: {}.{}",
        opt.MajorImageVersion, opt.MinorImageVersion
    );
    println!(
        "linker version: {}.{}",
        opt.MajorLinkerVersion, opt.MinorLinkerVersion
    );
    if opt.AddressOfEntryPoint > 0 {
        let mut address = opt.AddressOfEntryPoint;
        if pe_rel_addr == 0 {
            address += opt.ImageBase;
        }
        println!("program Entry point: {:0x}", address);
    }

    println!("Base of code section: {:x}", opt.BaseOfCode); /* 2c */
    println!("Base of data section: {:x}", opt.BaseOfData); /* 30 */
    println!("Preferred base address: {:x}", opt.ImageBase); /* 34 */
    println!(
        "Required OS version: {}.{}",
        opt.MajorOperatingSystemVersion, opt.MinorOperatingSystemVersion
    ); /* 40 */

    if opt.Win32VersionValue != 0 {
        eprintln!(
            "Win32VersionValue is {} (expected 0)",
            opt.Win32VersionValue
        ); /* 4c */
    }
    if opt.Subsystem <= 16 {
        /* 5c */
        println!("Subsystem: {}", PE_SUBSYSTEMS[opt.Subsystem]);
    } else {
        eprintln!("Subsystem: (unknown value {})", opt.Subsystem);
    }
    println!(
        "Subsystem version: {}.{}",
        opt.MajorSubsystemVersion, opt.MinorSubsystemVersion
    ); /* 48 */

    print_dll_flags(opt.DllCharacteristics); /* 5e */

    println!("Stack size (reserve): {} bytes", opt.SizeOfStackReserve); /* 60 */
    println!("Stack size (commit): {} bytes", opt.SizeOfStackCommit); /* 64 */
    println!("Heap size (reserve): {} bytes", opt.SizeOfHeapReserve); /* 68 */
    println!("Heap size (commit): {} bytes", opt.SizeOfHeapCommit); /* 6c */

    if opt.LoaderFlags != 0 {
        eprintln!()("LoaderFlags is 0x%x (expected 0)", opt.LoaderFlags); /* 70 */
    }
}

pub fn print_opt64(opt: &PeOptionalHeader64, pe_rel_addr: i32) {
    println!(
        "File version: {}.{}",
        opt.MajorImageVersion, opt.MinorImageVersion
    ); /* 44 */
    println!(
        "Linker version: {}.{}",
        opt.MajorLinkerVersion, opt.MinorLinkerVersion
    ); /* 1a */

    if opt.AddressOfEntryPoint {
        let mut address = opt.AddressOfEntryPoint;
        if !pe_rel_addr {
            address += opt.ImageBase;
        }
        println!("Program Entry point: {:x}", address); /* 28 */
    }

    println!("Base of code section: {:x}", opt.BaseOfCode); /* 2c */
    println!("Preferred base address: {:x}", opt.ImageBase); /* 30 */
    println!(
        "Required OS version: {}.{}",
        opt.MajorOperatingSystemVersion, opt.MinorOperatingSystemVersion
    ); /* 40 */

    if opt.Win32VersionValue != 0 {
        eprintln!(
            "Win32VersionValue is {} (expected 0)",
            opt.Win32VersionValue
        ); /* 4c */
    }
    if opt.Subsystem <= 16 {
        /* 5c */
        println!("Subsystem: {}", PE_SUBSYSTEMS[opt.Subsystem]);
    } else {
        println!("Subsystem: (unknown value {})", opt.Subsystem);
    }
    println!(
        "Subsystem version: {}.{}",
        opt.MajorSubsystemVersion, opt.MinorSubsystemVersion
    ); /* 48 */
    print_dll_flags(opt.DllCharacteristics); /* 5e */
    println!("Stack size (reserve): {} bytes", opt.SizeOfStackReserve); /* 60 */
    println!("Stack size (commit): {} bytes", opt.SizeOfStackCommit); /* 68 */
    println!("Heap size (reserve): {} bytes", opt.SizeOfHeapReserve); /* 70 */
    println!("Heap size (commit): {} bytes", opt.SizeOfHeapCommit); /* 78 */

    if opt.LoaderFlags != 0 {
        eprintln!("LoaderFlags is {:x} (expected 0)", opt.LoaderFlags); /* 80 */
    }
}

pub fn print_header(pe: &PeExecutable, pe_rel_addr: i32) {
    println!("");

    if !pe.header.SizeOfOptionalHeader {
        println!("No optional header");
        return;
    } else if pe.header.SizeOfOptionalHeader < PeOptionalHeader32::size_of() {
        eprintln!(
            "Size of optional header is {} (expected at least {}).",
            pe.header.SizeOfOptionalHeader,
            PeOptionalHeader32::size_of()
        );
    }
    print_flags(pe.header.Characteristics); /* 16 */

    if pe.magic == 0x10b {
        println!("Image type: 32-bit");
        print_opt32(&pe.opt32, pe_rel_addr);
    } else if pe.magic == 0x20b {
        println!("Image type: 64-bit\n");
        print_opt64(&pe.opt64, pe_rel_addr);
    }
}

pub fn print_specfile(pe: &PeExecutable) -> Result<(), Box<dyn Error>> {
    let mut i: i32;
    let mut specfile: File;
    let mut spec_name: String;
    spec_name += fmt!("{}.ord", pe.name);
    {
        let mut specfile = File::open(spec_name.as_str())?;
        specfile.write("#Generated by dump -o\n".as_bytes())?;
        for i in 0..pe.export_count {
            if pe.exports[i].address != 0 {
                specfile.write(fmt!("{}\t{}\n", pe.exports[i].ordinal, pe.exports[i].name))?;
            }
        }
    }
    Ok(())
}

pub fn get_export_table(map: &Vec<u8>, pe: &mut PeExecutable) {
    // const struct export_header *header;
    // off_t offset;
    // int i;

    /* More headers. It's like a PE file is nothing but headers.
     * Do we really need to print any of this? No, not really. Just use the data. */
    let header = PeExportHeader::from(
        read_data(
            map,
            addr2offset(pe.dirs[0].address, pe),
            std::mem::size_of::<PeExportHeader>(),
        )
        .as_ref(),
    );
    let offset = addr2offset(header.addr_table_addr, pe);
    let mut name_len = 0usize;

    /* Grab the name. */
    pe.name = read_string(map, addr2offset(header.module_name_addr, pe), name_len);

    /* Grab the exports. */
    // pe.exports = malloc(header.addr_table_count * sizeof(struct export));

    /* If addr_table_count exceeds export_count, this means that some exports
     * are nameless (and thus exported by ordinal). */

    for i in 0..header.addr_table_count {
        pe.exports[i].ordinal = i + header.ordinal_base;
        pe.exports[i].address = read_dword(map, offset + i * 4);
        pe.exports[i].name = NULL;
    }

    /* Why? WHY? */
    for i in 0..header.export_count {
        let index = read_word(
            map,
            addr2offset(header.ord_table_addr, pe) + (i * sizeof(u16)),
        );
        let name_addr = read_dword(
            map,
            addr2offset(header.name_table_addr, pe) + (i * sizeof(u32)),
        );
        let mut index_name_len = 0usize;
        pe.exports[index].name = read_data(map, addr2offset(name_addr, pe), index_name_len);
    }

    pe.export_count = header.addr_table_count as usize;
}

pub fn get_import_name_table(
    map: &Vec<u8>,
    module: &import_module,
    nametab_addr: u32,
    pe: &PeExecutable,
) {
    let offset = addr2offset(nametab_addr, pe);
    let mut count = 0usize;

    if pe.magic == 0x10b {
        while read_dword(map, offset + count * 4) {
            count += 1;
        }
    } else {
        while read_qword(map, offset + count * 8) {
            count += 1;
        }
    }

    // module.nametab = malloc(count * sizeof(*module.nametab));

    for i in 0..count {
        if pe.magic == 0x10b {
            let address = read_dword(map, offset + i * 4);
            module.nametab[i].is_ordinal = !!(address & (1 << 31));
        } else {
            address = read_qword(map, offset + i * 8);
            module.nametab[i].is_ordinal = !!(address & (1 << 63));
        }
        if module.nametab[i].is_ordinal {
            module.nametab[i].ordinal = address;
        } else {
            let name_len = 0usize;
            module.nametab[i].name = read_data(map, addr2offset(address, pe) + 2, name_len);
            /* skip hint */
        }
    }
    module.count = count;
}

pub fn get_import_module_table(pe: &mut PeExecutable) {
    let offset = addr2offset(pe.dirs[1].address, pe);
    let zeroes: [u32; 5] = [0; 5];
    pe.import_count = 0;
    // while (read_data(map, offset + pe.import_count * 20), zeroes, 20)) {
    //     pe.import_count += 1;
    // }

    // pe.imports = malloc(pe.import_count * sizeof(struct import_module));

    for i in 0..pe.import_count {
        let mut mod_addr_len = 0usize;
        pe.imports[i].module = read_string(
            map,
            addr2offset(read_dword(map, offset + i * 20 + 12), pe),
            mod_str_len,
        );
        pe.imports[i].iat_addr = read_dword(map, offset + i * 20 + 16);
        get_import_name_table(map, &pe.imports[i], read_dword(map, offset + i * 20), pe);
    }
}

pub fn get_reloc_table(map: &Vec<u8>, pe: &mut pe) {
    let mut offset = 0usize;
    // let offset = addr2offset(pe.dirs[5].address, pe), cursor = offset;
    // unsigned i, reloc_idx = 0;

    pe.reloc_count = 0;
    while cursor < offset + pe.dirs[5].size {
        pe.reloc_count += (read_dword(map, cursor + 4) - 8) / 2;
        cursor += read_dword(map, cursor + 4);
    }

    // pe.relocs = malloc(pe.reloc_count * sizeof(*pe.relocs));
    cursor = offset;
    while cursor < offset + pe.dirs[5].size {
        let block_base = read_dword(map, cursor);
        let block_size = read_dword(map, cursor + 4);

        for i in 0..(block_size - 8) / 2 {
            let r = read_word(map, cursor + 8 + i * 2);
            pe.relocs[reloc_idx].offset = block_base + (r & 0xfff);
            pe.relocs[reloc_idx].reloc_type = r >> 12;
            reloc_idx += 1;
        }
        cursor += block_size;
    }
}

pub fn readpe(map: &Vec<u8>, offset_pe: usize, pe: &mut PeExecutable) {
    let mut offset = 0usize;

    pe.header = PeFileHeader::from(&read_data(
        map,
        offset_pe + 4,
        std::mem::size_of::<PeFileHeader>(),
    ));
    pe.magic = read_word(map, offset_pe + 4 + std::mem::size_of::<file_header>());
    if pe.magic == 0x10b {
        pe.opt32 = PeOptionalHeader32::from(&read_data(
            &pe.file,
            offset_pe + 4 + std::mem::sizeof::<PeFileHeader>(),
            std::mem::size_of::<PeOptionalHeader32>(),
        ));
        pe.imagebase = pe.opt32.ImageBase as u64;
        cdirs = pe.opt32.NumberOfRvaAndSizes;
        offset = offset_pe
            + 4
            + std::mem::size_of::<PeFileHeader>()
            + std::mem::size_of::<PeOptionalHeader32>;
    } else if pe.magic == 0x20b {
        pe.opt64 = PeOptionalHeader64::from(&read_data(
            &pe.file,
            offset_pe + 4 + std::mem::size_of::<PeFileHeader>,
            std::mem::size_of::<PeOptionalHeader64>(),
        ));
        pe.imagebase = pe.opt64.ImageBase;
        cdirs = pe.opt64.NumberOfRvaAndSizes;
        offset = offset_pe
            + 4
            + std::mem::size_of::<PeFileHeader>()
            + std::mem::size_of::<PeOptionalHeader64>();
    } else {
        panic!("Don't know how to read image type {:x}\n", pe.magic);
        // exit(1);
    }

    // TODO:
    // pe.dirs = read_data(map, offset);
    offset += cdirs * std::mem::size_of::<PeDirectory>();

    /* read the section table */
    // pe.sections = malloc(pe.header.NumberOfSections * sizeof(struct section));
    for i in 0..pe.header.NumberOfSections {
        pe.sections[i] = read_data(&pe.file, offset + i * 0x28, 0x28);

        /* allocate zeroes, but only if it's a code section */
        /* in theory nobody will ever try to jump into a data section.
         * VirtualProtect() be damned */
        // if (pe.sections[i].flags & 0x20) {
        //     pe.sections[i].instr_flags = calloc(pe.sections[i].min_alloc, sizeof(u8));
        // }
        // else {
        //     pe.sections[i].instr_flags = NULL;
        // }
    }

    /* Read the Data Directories.
     * PE is bizarre. It tries to make all of these things generic by putting
     * them in separate "directories". But the order of these seems to be fixed
     * anyway, so why bother? */

    if cdirs >= 1 && pe.dirs[0].size > 0 {
        get_export_table(&pe.file, pe);
    }
    if cdirs >= 2 && pe.dirs[1].size > 0 {
        get_import_module_table(pe);
    }
    if cdirs >= 6 && pe.dirs[5].size > 0 {
        get_reloc_table(&pe.file, pe);
    }

    /* Read the code. */
    if mode & DISASSEMBLE {
        read_sections(pe);
    }
}

pub fn dumppe(map: &Vec<u8>, offset_pe: usize, mut pe_rel_addr: i32) {
    let mut pe = PeExecutable::new();
    readpe(map, offset_pe, &mut pe);

    if mode == SPECFILE {
        print_specfile(&pe);
        // freepe(&pe);
        return;
    }

    /* objdump always applies the image base to addresses. This makes sense for
     * EXEs, which can always be loaded at their preferred address, but for DLLs
     * it just makes debugging more annoying, since you have to subtract the
     * image base and *then* add the address the DLL was actually loaded at.
     * In theory PE provides us with everything we need to fix up a DLL
     * (relocations etc.) so that we only ever print the *relative* addresses.
     * But we can't do the same for an EXE, and we probably don't want to either.
     * Is the discrepancy going to be confusing? Probably not that much.
     *
     * Anyway, offer the user the option. Default is to enable relative addressing
     * for DLLs but disable it for EXEs. Note that if they manually enable it,
     * we won't be able to fix up everything. Caveat emptor.
     *
     * Internally we want to use relative IPs everywhere possible. The only place
     * that we can't is in arg.value. */
    if pe_rel_addr == -1 {
        pe_rel_addr = pe.header.Characteristics & 0x2000;
    }

    print!("Module type: PE (Portable Executable)\n");
    if (pe.name) {
        print!("Module name: {}\n", pe.name);
    }

    if (mode & DUMPHEADER) {
        print_header(&pe, pe_rel_addr);
    }

    if (mode & DUMPEXPORT) {
        print!('\n');
        if (pe.exports) {
            print!("Exports:\n");
            for i in 0..pe.export_count {
                let address = pe.exports[i].address;
                if (!address) {
                    continue;
                }
                if !pe_rel_addr {
                    address += pe.imagebase;
                }
                print!("\t%5d\t%#8x\t{}", pe.exports[i].ordinal, address,
                    pe.exports[i].name ? pe.exports[i].name : "<no name>");
                if pe.exports[i].address >= pe.dirs[0].address
                    && pe.exports[i].address < (pe.dirs[0].address + pe.dirs[0].size)
                {
                    let len = 0;
                    // print!(
                    //     " . {}",
                    //     read_data(&pe.file, addr2offset(pe.exports[i].address, &pe), len)
                    // );
                }
                print!('\n');
            }
        } else {
            print!("No Export table\n");
        }
    }

    if (mode & DUMPIMPORT) {
        print!('\n');
        if (pe.imports) {
            print!("Imported modules:\n");
            for i in 0..pe.import_count {
                print!("\t{}\n", pe.imports[i].module);
            }

            print!("\nImported functions:\n");
            for i in 0..pe.import_count {
                print!("\t{}:\n", pe.imports[i].module);
                for j in 0..pe.imports[i].count {
                    if pe.imports[i].nametab[j].is_ordinal {
                        print!("\t\t<ordinal {}>\n", pe.imports[i].nametab[j].ordinal);
                    } else {
                        print!("\t\t{}\n", pe.imports[i].nametab[j].name);
                    }
                }
            }
        } else {
            print!("No imported module table\n");
        }
    }

    if (mode & DISASSEMBLE) {
        print_sections(&pe);
    }

    // freepe(&pe);
}

pub const pe_rel_addr: i32 = -1;

pub fn addr2section(addr: u32, pe: &PeExecutable) -> Option<PeSection> {
    /* Even worse than the below, some data is sensitive to which section it's in! */
    for i in 0..pe.header.NumberOfSections {
        if addr >= pe.sections[i].address
            && addr < pe.sections[i].address + pe.sections[i].min_alloc
        {
            return Some(pe.sections[i].clone());
        }
    }
    None
}

pub fn addr_to_offset(address: u32, pe: &PeExecutable) -> usize {
    /* Everything inside a PE file is built so that the file is read while it's
     * already loaded. Offsets aren't file offsets, they're *memory* offsets.
     * We don't want to load the file like that, so we have to search through
     * each section to figure out where in the *file* a virtual address points. */

    match addr2section(address, pe) {
        None => 0,
        Some(x) => address - x.address + x.offset,
    }
}

pub fn get_export_name(ip: u32, pe: &PeExecutable) -> Option<String> {
    for i in 0..pe.export_count {
        if pe.exports[i].address == ip {
            return Some(pe.exports[i].name.clone());
        }
    }
    return None;
}

pub fn get_imported_name(offset: usize, pe: &PeExecutable) -> Option<String> {
    let mut comment: String;
    for i in 0..pe.import_count {
        let module = &pe.imports[i];
        let mut index = offset - module.iat_addr;
        if pe.magic == 0x10b {
            index /= 4;
        } else {
            index /= 8;
        }

        if index < module.count {
            if module.nametab[index].is_ordinal {
                Some(format!(
                    "{}.{}",
                    module.module, module.nametab[index].ordinal
                ))
            }
            Some(module.nametab[index].name.clone())
        }
    }
    None
}

pub fn get_reloc(ip: u32, pe: &PeExecutable) -> Option<PeReloc> {
    for i in 0..pe.reloc_count {
        if pe.relocs[i].offset == ip {
            return Some(pe.relocs[i].clone());
        }
    }
    None
}

pub fn relocate_arg(instr: &Instruction, arg: &Argument, pe: &PeExecutable) -> Option<String> {
    match get_reloc(arg.ip, pe) {
        None => None,
        Some(x) => match x.reloc_type {
            0 => None,
            3 => {
                if arg.arg_type == IMM
                    || (arg.arg_type == RM && instr.modrm_reg == -1)
                    || arg.arg_type == MOFFS
                {
                    if pe_rel_addr {
                        Some(format!("{}", arg.value - pe.opt32.ImageBase))
                    } else {
                        Some(format!("{}", arg.value))
                    }
                }
            }
            _ => None,
        },
    }
    None
}

pub fn get_arg_comment(
    map: &Vec<u8>,
    sec: &PeSection,
    end_ip: u32,
    instr: &Instruction,
    arg: &Argument,
    pe: &PeExecutable,
) -> Option<String> {
    if arg.arg_type == NONE {
        return None;
    }
    if instr.modrm_reg == 16 && arg.arg_type >= RM && arg.arg_type <= MEM {
        let tip = end_ip + arg.value;
        let mut abstip = tip;
        if !pe_rel_addr {
            abstip += pe.imagebase;
        }
        comment = get_imported_name(tip as usize, pe);
        if comment.is_some() {
            return comment;
        }

        comment = get_export_name(tip, pe);
        if comment.is_some() {
            return comment;
        }

        return Some(format!("{:x}", abstip));
    }

    /* FIXME: This is getting messy. */
    rel_value = arg.value;
    if !pe_rel_addr {
        rel_value -= pe.imagebase;
    }

    /* Relocate anything that points inside the image's address space or that
     * has a relocation Entry. */
    if (tsec = addr2section(rel_value, pe)) || (sec.instr_flags[arg.ip - sec.address] & INSTR_RELOC)
    {
        comment = get_imported_name(rel_value, pe);
        if comment.is_some() {
            return comment;
        }
        comment = get_export_name(rel_value, pe);
        if comment.is_some() {
            return comment;
        }

        /* Sometimes we have TWO levels of indirection—call to jmp to
         * relocated address. mingw-w64 does this. */
        let read_u16_val = read_word(&map, addr_to_offset(rel_value, pe));
        if tsec.is_some()
            && (rel_value < (tsec.unwrap().address + tsec.unwrap().length))
            && read_u16_val == 0x25ff
        {
            rel_value = read_dword(&map, addr2offset(rel_value, pe) + 2);
            if !pe_rel_addr {
                rel_value -= pe.imagebase;
            }
            return get_imported_name(rel_value, pe);
        }

        if comment = relocate_arg(instr, arg, pe) {
            return comment;
        }

        /* Don't print any comment for mundane relative jumps or calls. */
        if arg.arg_type == REL8 || arg.arg_type == REL {
            return None;
        }

        /* If all else fails, print the address relative to the image base. */
        return Some(format!("{:x}", rel_value));
    }
    None
}

pub fn print_pe_instr(sec: &PeSection, ip: u32, p: &Vec<u8>, pe: &PeExecutable) -> i32 {
    let mut instr: Instruction = Default::default();
    // let mut comment: String = String::new();
    // let mut ip_string: [u8;17] = [0;17];
    let mut absip = ip;
    let mut bits = 0;
    if pe.magic == 0x10b {
        bits = 32;
    } else {
        bits = 64;
    }

    if !pe_rel_addr {
        absip += pe.imagebase;
    }

    len = get_instr(ip, p, &instr, bits);

    let mut ip_string = format!("{:08x}", absip);

    /* We deal in relative addresses internally everywhere. That means we have
     * to fix up the values for relative jumps if we're not displaying relative
     * addresses. */
    if (instr.op.arg0 == REL8 || instr.op.arg0 == REL) && pe_rel_addr == 0 {
        instr.args[0].value += pe.imagebase;
    }

    /* Check for relocations and imported names. PE separates the two concepts:
     * imported names are done by jumping into a block in .idata which is
     * relocated, and relocations proper are scattered throughout code sections
     * and relocated according to the contents of .Reloc. */

    let mut comment = get_arg_comment(map, sec, ip + len, &instr, &instr.args[0], pe);
    if comment.is_some() {
        comment = get_arg_comment(map, sec, ip + len, &instr, &instr.args[1], pe);
    }

    print_instr(
        ip_string,
        p,
        len,
        sec.instr_flags[ip - sec.address],
        &instr,
        comment,
        bits,
    );

    return len;
}

pub fn print_disassembly(sec: &PeSection, pe: &PeExecutable) {
    let mut buffer = String::new();

    while relip < sec.length && relip < sec.min_alloc {
        /* find a valid instruction */
        if (!(sec.instr_flags[relip] & INSTR_VALID)) {
            if (opts & DISASSEMBLE_ALL) {
                /* still skip zeroes */
                if (read_byte(&pe.file, sec.offset + relip) == 0) {
                    print!("     ...\n");
                    relip += 1;
                    while (read_byte(&pe.file, sec.offset + relip) == 0) {
                        relip += 1;
                    }
                }
            } else {
                print!("     ...\n");
                while ((relip < sec.length)
                    && (relip < sec.min_alloc)
                    && !(sec.instr_flags[relip] & INSTR_VALID))
                {
                    relip += 1;
                }
            }
        }

        ip = relip + sec.address;
        if (relip >= sec.length || relip >= sec.min_alloc) {
            return;
        }

        /* Instructions can "hang over" the end of a Segment.
         * Zero should be supplied. */
        // memset(buffer, 0, sizeof(buffer));
        // memcpy(buffer, read_data(sec.offset + relip), min(sizeof(buffer), sec.length - relip));
        buffer = read_string(&pe.file, sec.offset + relip, sec.length - relip);

        absip = ip;
        if !pe_rel_addr {
            absip += pe.imagebase;
        }

        if sec.instr_flags[relip] & INSTR_FUNC {
            let name = get_export_name(ip, pe);
            print!("\n");
            print!("%lx <{}>:\n", absip, name ? name : "no name");
        }

        relip += print_pe_instr(sec, ip, &buffer.into_bytes(), pe);
    }
    print!('\n');
}

pub fn print_data(sec: &PeSection, pe: &PeExecutable) {
    // u32 relip = 0;
    // qword absip;

    /* Page alignment means that (contrary to NE) sections are going to end with
     * a bunch of annoying zeroes. So don't read past the minimum allocation. */
    let length = min(sec.length, sec.min_alloc);

    for i in (0..length).step_by(16) {
        let len = min(length - relip, 16);

        absip = relip + sec.address;
        if (!pe_rel_addr) {
            absip += pe.imagebase;
        }

        print!("{:08x}", absip);
        for i in 0..16 {
            if (i < len) {
                print!(" {:02x}", read_byte(&pe.file, sec.offset + relip + i));
            } else {
                print!("   ");
            }
        }
        print!("  ");
        for i in 0..len {
            let c = read_byte(&pe.file, sec.offset + relip + i);
            print!(isprint(c) ? c : '.');
        }
        print!('\n');
    }
}

pub fn scan_segment(mut ip: i32, pe: &PeExecutable) {
    let sec = addr2section(ip as u32, pe);
    let mut relip = 0i32;
    let mut buffer = String::new();
    let mut instr: Instruction = Instruction::new();

    //    eprint!( "scanning at %x, in section {}\n", ip, sec ? sec.name : "<none>");

    if sec.is_none() {
        eprint!("Attempt to scan byte not in image.\n");
        return;
    }

    relip = ip - &sec.address;

    // if (sec.unwrap().instr_flags[relip] != (0 & (INSTR_VALID|INSTR_SCANNED))) == INSTR_SCANNED {
    //     eprint!("Attempt to scan byte that does not begin instruction.\n");
    // }

    /* This code assumes that one stretch of code won't span multiple sections.
     * Is this a valid assumption? */

    while relip < sec.length {
        /* check if we've already read from here */
        if &sec.instr_flags[relip] & INSTR_SCANNED {
            return;
        }

        /* read the instruction */
        // memset(buffer, 0, sizeof(buffer));
        // memcpy(buffer, read_data(sec.offset + relip), min(sizeof(buffer), sec.length-relip));
        buffer = read_string(&pe.file, &sec.offset + relip, &sec.length - relip);
        instr_length = get_instr(ip, buffer, &instr, if pe.magic == 0x10b { 32 } else { 64 });

        /* mark the bytes */
        &sec.instr_flags[relip] |= INSTR_VALID;
        // for (i = relip; i < relip+instr_length && i < sec.min_alloc; i += 1) sec.instr_flags[i] |= INSTR_SCANNED;

        /* instruction which hangs over the minimum allocation */
        if i < relip + instr_length && i == sec.min_alloc {
            break;
        }

        /* handle conditional and unconditional jumps */
        if instr.op.flags & OP_BRANCH {
            /* relative jump, loop, or call */
            let tsec = addr2section(instr.args[0].value as u32, pe);

            if tsec {
                if tsec.flags & 0x20 {
                    relip = instr.args[0].value - tsec.address;

                    if instr.op.name != "call" {
                        tsec.instr_flags[trelip] |= INSTR_FUNC;
                    } else {
                        tsec.instr_flags[trelip] |= INSTR_JUMP;
                    }

                    /* scan it */
                    scan_segment(instr.args[0].value as i32, pe);
                } else {
                    eprint!(
                        "Branch '{}' to byte {:x} in non-code section {}.\n",
                        instr.op.name, instr.args[0].value, tsec.name
                    );
                }
            } else {
                eprint!(
                    "Branch '{}' to byte {:x} not in image.\n",
                    instr.op.name, instr.args[0].value
                );
            }
        }

        for i in relip..relip + instr_length {
            if &sec.instr_flags[i] & INSTR_RELOC {
                let r = get_reloc(i + &sec.address, pe);
                // struct section *tsec;
                // u32 taddr;

                if (r.is_none()) {
                    eprint!("Byte tagged INSTR_RELOC has no Reloc; this is a bug.\n");
                }

                match r.unwrap().reloc_type {
                    3 => {
                        /* HIGHLOW */
                        if pe.magic != 0x10b {
                            eprint!("HIGHLOW relocation in 64-bit image?\n");
                        }
                        taddr = read_dword(&pe.file, &sec.offset + i) - pe.imagebase;
                        tsec = addr2section(taddr, pe);

                        if !tsec {
                            warn_at(
                                "Relocation to %#x isn't in a section?\n",
                                read_dword(&pe.file, &sec.offset + i),
                            );
                            continue;
                        }

                        /* Only try to scan it if it's an immediate address. If someone is
                         * dereferencing an address inside a code section, it's data. */
                        if tsec.flags & 0x20 && (instr.op.arg0 == IMM || instr.op.arg1 == IMM) {
                            tsec.instr_flags[taddr - tsec.address] |= INSTR_FUNC;
                            scan_segment(taddr, pe);
                        }
                    }
                    _ => {
                        warn_at(
                            "Don't know how to handle relocation type {}\n",
                            r.unwrap().reloc_type,
                        );
                    }
                }
                break;
            }
        }

        if instr.op.flags & OP_STOP {
            return;
        }

        ip += instr_length;
        relip = ip - &sec.address;
    }

    warn_at("Scan reached the end of section.\n");
}

pub fn print_section_flags(flags: u32) {
    let mut buffer = String::new();
    let alignment = (flags & 0x00f00000) / 0x100000;

    /* Most of these shouldn't occur in an image file, either because they're
     * COFF flags that PE doesn't want or because they're object-only. Print
     * the COFF names. */
    if (flags & 0x00000001) {
        buffer += ", STYP_DSECT";
    }
    if (flags & 0x00000002) {
        buffer += ", STYP_NOLOAD";
    }
    if (flags & 0x00000004) {
        buffer += ", STYP_GROUP";
    }
    if (flags & 0x00000008) {
        buffer += ", STYP_PAD";
    }
    if (flags & 0x00000010) {
        buffer += ", STYP_COPY";
    }
    if (flags & 0x00000020) {
        buffer += ", code";
    }
    if (flags & 0x00000040) {
        buffer += ", data";
    }
    if (flags & 0x00000080) {
        buffer += ", bss";
    }
    if (flags & 0x00000100) {
        buffer += ", S_NEWCFN";
    }
    if (flags & 0x00000200) {
        buffer += ", STYP_INFO";
    }
    if (flags & 0x00000400) {
        buffer += ", STYP_OVER";
    }
    if (flags & 0x00000800) {
        buffer += ", STYP_LIB";
    }
    if (flags & 0x00001000) {
        buffer += ", COMDAT";
    }
    if (flags & 0x00002000) {
        buffer += ", STYP_MERGE";
    }
    if (flags & 0x00004000) {
        buffer += ", STYP_REVERSE_PAD";
    }
    if (flags & 0x00008000) {
        buffer += ", FARDATA";
    }
    if (flags & 0x00010000) {
        buffer += ", (unknown flags 0x10000)";
    }
    if (flags & 0x00020000) {
        buffer += ", purgeable"; /* or 16BIT */
    }
    if (flags & 0x00040000) {
        buffer += ", locked";
    }
    if (flags & 0x00080000) {
        buffer += ", preload";
    }
    if (flags & 0x01000000) {
        buffer += ", extended relocations";
    }
    if (flags & 0x02000000) {
        buffer += ", discardable";
    }
    if (flags & 0x04000000) {
        buffer += ", not cached";
    }
    if (flags & 0x08000000) {
        buffer += ", not paged";
    }
    if (flags & 0x10000000) {
        buffer += ", shared";
    }
    if (flags & 0x20000000) {
        buffer += ", executable";
    }
    if (flags & 0x40000000) {
        buffer += ", readable";
    }
    if (flags & 0x80000000) {
        buffer += ", writable";
    }

    print!("    Flags: 0x{:08x} ({})\n", flags, buffer[2..]);
    print!("    Alignment: {} (2**{})\n", 1 << alignment, alignment);
}

pub fn read_sections(pe: &PeExecutable) {
    let entry_point = if pe.magic == 0x10b {
        pe.opt32.AddressOfEntryPoint
    } else {
        pe.opt64.AddressOfEntryPoint
    };

    /* We already read the section header (unlike NE, we had to in order to read
     * everything else), so our job now is just to scan the section contents. */

    /* Relocations first. */
    for i in 0..pe.reloc_count {
        let address = pe.relocs[i].offset;
        let sec = addr2section(address, pe);
        if !sec {
            eprint!("Relocation at {:x} isn't in a section?\n", address);
            continue;
        }
        if &sec.flags & 0x20 {
            match (pe.relocs[i].reloc_type) {
                0 => { /* padding */ }
                3 => {
                    /* HIGHLOW */
                    /* scanning is done in scan_segment() */
                    &sec.instr_flags[address - &sec.address] |= INSTR_RELOC;
                }

                _ => {
                    eprint!(
                        "{:x}: Don't know how to handle relocation type {}\n",
                        pe.relocs[i].offset, pe.relocs[i].reloc_type
                    );
                }
            }
        }
    }

    for i in 0..pe.export_count {
        let address = pe.exports[i].address;
        if (!address) {
            continue;
        }
        let sec = addr2section(address, pe);
        if sec.is_none() {
            eprint!(
                "Export {} at {:x} isn't in a section?\n",
                pe.exports[i].name, pe.exports[i].address
            );
            continue;
        }
        if sec.flags & 0x20
            && !(address >= pe.dirs[0].address && address < (pe.dirs[0].address + pe.dirs[0].size))
        {
            sec.instr_flags[address - sec.address] |= INSTR_FUNC;
            scan_segment(pe.exports[i].address as i32, pe);
        }
    }

    if (entry_point) {
        let sec = addr2section(entry_point, pe);
        if (!sec) {
            eprint!("Entry point {:x} isn't in a section?\n", entry_point);
        } else if (&sec.flags & 0x20) {
            &sec.instr_flags[entry_point - &sec.address] |= INSTR_FUNC;
            scan_segment(entry_point as i32, pe);
        }
    }
}

pub fn print_sections(pe: &PeExecutable) {
    for i in 0..pe.header.NumberOfSections {
        let mut sec = &pe.sections[i];

        print!('\n');
        println!(
            "Section {} (start = 0x{:x}, length = 0x{:x}, minimum allocation = 0x{:x}):\n",
            sec.name, sec.offset, sec.length, sec.min_alloc
        );
        println!("    Address: {:x}\n", sec.address);
        print_section_flags(sec.flags);

        /* These fields should only be populated for object files (I think). */
        if sec.reloc_offset || sec.reloc_count {
            eprintln!(
                "Section {} has relocation data: offset = {:x}, count = {}\n",
                sec.name, sec.reloc_offset, sec.reloc_count
            );
        }

        /* Sometimes the .text section is marked as both code and data. I've
         * seen mingw-w64 do this. (Because there's data stored in it?) */
        if sec.flags & 0x20 {
            if opts & FULL_CONTENTS {
                print_data(sec, pe);
            }
            print_disassembly(sec, pe);
        } else if sec.flags & 0x40 {
            /* see the appropriate FIXMEs on the NE side */
            /* Don't print .rsrc by default. Some others should probably be
             * excluded, too, but .rsrc is a particularly bad offender since
             * large binaries might be put into it. */
            // if (sec.name == ".rsrc" && sec.name == ".Reloc") || opts & FULL_CONTENTS {
            //     print_data(sec, pe);
            // }
        }
    }
}
