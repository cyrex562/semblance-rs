/*
 * Functions for parsing the PE header
 *
 * Copyright 2017-2018,2020 Zebediah Figura
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

use std::error::Error;
use std::fs::File;
use std::io::Write;
// #include <stddef.h>
// #include <stdlib.h>
// #include <string.h>
// #include "semblance.h"
// #include "pe_h.rs"
use crate::{semblance, pe_h};
use crate::pe_h::{OptionalHeader, OptionalHeaderPep, PortableExecutableHeader};
use crate::semblance::read_data;

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

pub const subsystems: [String;17] = [
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
    "boot"
];


pub fn print_opt32(opt: &OptionalHeader)
{
    println!("file version: {}.{}", opt.MajorImageVersion, opt.MinorImageVersion);
    println!("linker version: {}.{}", opt.MajorLinkerVersion, opt.MinorLinkerVersion);
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
    println!("Required OS version: {}.{}", opt.MajorOperatingSystemVersion, opt.MinorOperatingSystemVersion); /* 40 */

    if opt.Win32VersionValue != 0 {
        eprintln!("Win32VersionValue is {} (expected 0)", opt.Win32VersionValue); /* 4c */
    }
    if opt.Subsystem <= 16 {
        /* 5c */
        println!("Subsystem: {}", subsystems[opt.Subsystem]);
    }
    else {
        eprintln!("Subsystem: (unknown value {})", opt.Subsystem);
    }
    println!("Subsystem version: {}.{}", opt.MajorSubsystemVersion, opt.MinorSubsystemVersion); /* 48 */

    print_dll_flags(opt.DllCharacteristics); /* 5e */

    println!("Stack size (reserve): {} bytes", opt.SizeOfStackReserve); /* 60 */
    println!("Stack size (commit): {} bytes", opt.SizeOfStackCommit); /* 64 */
    println!("Heap size (reserve): {} bytes", opt.SizeOfHeapReserve); /* 68 */
    println!("Heap size (commit): {} bytes", opt.SizeOfHeapCommit); /* 6c */

    if opt.LoaderFlags != 0 {
        eprintln!()("LoaderFlags is 0x%x (expected 0)", opt.LoaderFlags); /* 70 */
    }
}

pub fn print_opt64(opt: &OptionalHeaderPep)
{
    println!("File version: {}.{}", opt.MajorImageVersion, opt.MinorImageVersion); /* 44 */
    println!("Linker version: {}.{}", opt.MajorLinkerVersion, opt.MinorLinkerVersion); /* 1a */

    if opt.AddressOfEntryPoint {
        let mut address = opt.AddressOfEntryPoint;
        if !pe_rel_addr {
            address += opt.ImageBase;
        }
        println!("Program Entry point: {:x}", address); /* 28 */
    }

    println!("Base of code section: {:x}", opt.BaseOfCode); /* 2c */
    println!("Preferred base address: {:x}", opt.ImageBase); /* 30 */
    println!("Required OS version: {}.{}", opt.MajorOperatingSystemVersion,
             opt.MinorOperatingSystemVersion); /* 40 */

    if opt.Win32VersionValue != 0 {
        eprintln!("Win32VersionValue is {} (expected 0)", opt.Win32VersionValue); /* 4c */
    }
    if opt.Subsystem <= 16 {
        /* 5c */
        println!("Subsystem: {}", subsystems[opt.Subsystem]);
    }else {
        println!("Subsystem: (unknown value {})", opt.Subsystem);
    }
    println!("Subsystem version: {}.{}", opt.MajorSubsystemVersion, opt.MinorSubsystemVersion); /* 48 */
    print_dll_flags(opt.DllCharacteristics); /* 5e */
    println!("Stack size (reserve): {} bytes", opt.SizeOfStackReserve); /* 60 */
    println!("Stack size (commit): {} bytes", opt.SizeOfStackCommit); /* 68 */
    println!("Heap size (reserve): {} bytes", opt.SizeOfHeapReserve); /* 70 */
    println!("Heap size (commit): {} bytes", opt.SizeOfHeapCommit); /* 78 */

    if opt.LoaderFlags != 0 {
        eprintln!("LoaderFlags is {:x} (expected 0)", opt.LoaderFlags); /* 80 */
    }
}

pub fn print_header(pe: &PortableExecutableHeader) {
    println!("");

    if !pe.header.SizeOfOptionalHeader {
        println!("No optional header");
        return;
    } else if pe.header.SizeOfOptionalHeader <  OptionalHeader::size_of() {
        eprintln!("Size of optional header is {} (expected at least {}).",
             pe.header.SizeOfOptionalHeader, OptionalHeader::size_of());
    }
    print_flags(pe.header.Characteristics); /* 16 */

    if pe.magic == 0x10b {
        println!("Image type: 32-bit");
        print_opt32(&pe.opt32);
    } else if pe.magic == 0x20b {
        println!("Image type: 64-bit\n");
        print_opt64(&pe.opt64);
    }
}

pub fn print_specfile(pe: &PortableExecutableHeader) -> Result<(), Box<dyn Error>>{
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

pub struct export_header {
    pub flags: u32,            /* 00 */
    pub timestamp: u32,        /* 04 */
    pub  ver_major: u16,        /* 08 */
    pub ver_minor: u16,        /* 0a */
    pub module_name_addr: u32, /* 0c */
    pub ordinal_base: u32,     /* 10 */
    pub addr_table_count: u32, /* 14 */
    pub export_count: u32,     /* 18 */
    pub addr_table_addr: u32,  /* 1c */
    pub name_table_addr: u32,  /* 20 */
    pub ord_table_addr: u32,   /* 24 */
}

// STATIC_ASSERT(sizeof(struct export_header) == 0x28);

pub fn get_export_table(pe: &PortableExecutableHeader)
{

    // const struct export_header *header;
    // off_t offset;
    // int i;

    /* More headers. It's like a PE file is nothing but headers.
     * Do we really need to print any of this? No, not really. Just use the data. */
    let header = read_data(addr2offset(pe.dirs[0].address, pe));
    offset = addr2offset(header.addr_table_addr, pe);

    /* Grab the name. */
    pe.name = read_data(addr2offset(header.module_name_addr, pe));

    /* Grab the exports. */
    pe.exports = malloc(header.addr_table_count * sizeof(struct export));

    /* If addr_table_count exceeds export_count, this means that some exports
     * are nameless (and thus exported by ordinal). */

    for (i = 0; i < header.addr_table_count;  += 1i)
    {
        pe.exports[i].ordinal = i + header.ordinal_base;
        pe.exports[i].address = read_dword(offset + i * 4);
        pe.exports[i].name = NULL;
    }

    /* Why? WHY? */
    for (i = 0; i < header.export_count;  += 1i)
    {
        u16 index = read_word(addr2offset(header.ord_table_addr, pe) + (i * sizeof(u16)));
        u32 name_addr = read_dword(addr2offset(header.name_table_addr, pe) + (i * sizeof(u32)));
        pe.exports[index].name = read_data(addr2offset(name_addr, pe));
    }

    pe.export_count = header.addr_table_count;
}

static void get_import_name_table(struct import_module *module, u32 nametab_addr, struct pe *pe)
{
    offset: usize = addr2offset(nametab_addr, pe);
    unsigned i, count;

    count = 0;
    if (pe.magic == 0x10b)
        while (read_dword(offset + count * 4)) count += 1;
    else
        while (read_qword(offset + count * 8)) count += 1;

    module.nametab = malloc(count * sizeof(*module.nametab));

    for (i = 0; i < count; i += 1) {
        qword address;
        if (pe.magic == 0x10b)
        {
            address = read_dword(offset + i * 4);
            module.nametab[i].is_ordinal = !!(address & (1u << 31));
        }
        else
        {
            address = read_qword(offset + i * 8);
            module.nametab[i].is_ordinal = !!(address & (1ull << 63));
        }
        if (module.nametab[i].is_ordinal)
            module.nametab[i].ordinal = (u16)address;
        else
            module.nametab[i].name = read_data(addr2offset(address, pe) + 2); /* skip hint */
    }
    module.count = count;
}

static void get_import_module_table(struct pe *pe) {
    offset: usize = addr2offset(pe.dirs[1].address, pe);
    static const u32 zeroes[5] = {0};
    int i;

    pe.import_count = 0;
    while (memcmp(read_data(offset + pe.import_count * 20), zeroes, 20))
        pe.import_count += 1;

    pe.imports = malloc(pe.import_count * sizeof(struct import_module));

    for (i = 0; i < pe.import_count; i += 1)
    {
        pe.imports[i].module = read_data(addr2offset(read_dword(offset + i * 20 + 12), pe));
        pe.imports[i].iat_addr = read_dword(offset + i * 20 + 16);
        get_import_name_table(&pe.imports[i], read_dword(offset + i * 20), pe);
    }
}

static void get_reloc_table(struct pe *pe) {
    offset: usize = addr2offset(pe.dirs[5].address, pe), cursor = offset;
    unsigned i, reloc_idx = 0;

    pe.reloc_count = 0;
    while (cursor < offset + pe.dirs[5].size)
    {
        pe.reloc_count += (read_dword(cursor + 4) - 8) / 2;
        cursor += read_dword(cursor + 4);
    }

    pe.relocs = malloc(pe.reloc_count * sizeof(*pe.relocs));
    cursor = offset;
    while (cursor < offset + pe.dirs[5].size)
    {
        u32 block_base = read_dword(cursor);
        u32 block_size = read_dword(cursor + 4);

        for (i = 0; i < (block_size - 8) / 2;  += 1i)
        {
            u16 r = read_word(cursor + 8 + i * 2);
            pe.relocs[reloc_idx].offset = block_base + (r & 0xfff);
            pe.relocs[reloc_idx].type = r >> 12;
            reloc_idx += 1;
        }
        cursor += block_size;
    }
}

static void readpe(offset_pe: usize, struct pe *pe)
{
    offset: usize;
    int i, cdirs;

    pe.header = read_data(offset_pe + 4);
    pe.magic = read_word(offset_pe + 4 + sizeof(struct file_header));
    if (pe.magic == 0x10b)
    {
        pe.opt32 = read_data(offset_pe + 4 + sizeof(struct file_header));
        pe.imagebase = pe.opt32.ImageBase;
        cdirs = pe.opt32.NumberOfRvaAndSizes;
        offset = offset_pe + 4 + sizeof(struct file_header) + sizeof(struct optional_header);
    } else if (pe.magic == 0x20b) {
        pe.opt64 = read_data(offset_pe + 4 + sizeof(struct file_header));
        pe.imagebase = pe.opt64.ImageBase;
        cdirs = pe.opt64.NumberOfRvaAndSizes;
        offset = offset_pe + 4 + sizeof(struct file_header) + sizeof(struct optional_header_pep);
    } else {
        eprint!("Don't know how to read image type %#x\n", pe.magic);
        exit(1);
    }

    pe.dirs = read_data(offset);
    offset += cdirs * sizeof(struct directory);

    /* read the section table */
    pe.sections = malloc(pe.header.NumberOfSections * sizeof(struct section));
    for (i = 0; i < pe.header.NumberOfSections; i += 1)
    {
        memcpy(&pe.sections[i], read_data(offset + i*0x28), 0x28);

        /* allocate zeroes, but only if it's a code section */
        /* in theory nobody will ever try to jump into a data section.
         * VirtualProtect() be damned */
        if (pe.sections[i].flags & 0x20)
            pe.sections[i].instr_flags = calloc(pe.sections[i].min_alloc, sizeof(u8));
        else
            pe.sections[i].instr_flags = NULL;
    }

    /* Read the Data Directories.
     * PE is bizarre. It tries to make all of these things generic by putting
     * them in separate "directories". But the order of these seems to be fixed
     * anyway, so why bother? */

    if (cdirs >= 1 && pe.dirs[0].size)
        get_export_table(pe);
    if (cdirs >= 2 && pe.dirs[1].size)
        get_import_module_table(pe);
    if (cdirs >= 6 && pe.dirs[5].size)
        get_reloc_table(pe);

    /* Read the code. */
    if (mode & DISASSEMBLE)
        read_sections(pe);
}

static void freepe(struct pe *pe) {
    int i;

    for (i = 0; i < pe.header.NumberOfSections; i += 1)
        free(pe.sections[i].instr_flags);
    free(pe.sections);
    free(pe.exports);
    for (i = 0; i < pe.import_count; i += 1)
        free(pe.imports[i].nametab);
    free(pe.relocs);
    free(pe.imports);
}

void dumppe(offset_pe: usize) {
    struct pe pe = {0};
    int i, j;

    readpe(offset_pe, &pe);

    if (mode == SPECFILE) {
        print_specfile(&pe);
        freepe(&pe);
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
    if (pe_rel_addr == -1)
        pe_rel_addr = pe.header.Characteristics & 0x2000;

    print!("Module type: PE (Portable Executable)\n");
    if (pe.name) print!("Module name: {}\n", pe.name);

    if (mode & DUMPHEADER)
        print_header(&pe);

    if (mode & DUMPEXPORT) {
        print!('\n');
        if (pe.exports) {
            print!("Exports:\n");

            for (i = 0; i < pe.export_count; i += 1) {
                u32 address = pe.exports[i].address;
                if (!address)
                    continue;
                if (!pe_rel_addr)
                    address += pe.imagebase;
                print!("\t%5d\t%#8x\t{}", pe.exports[i].ordinal, address,
                    pe.exports[i].name ? pe.exports[i].name : "<no name>");
                if (pe.exports[i].address >= pe.dirs[0].address
                        && pe.exports[i].address < (pe.dirs[0].address + pe.dirs[0].size))
                    print!(" . {}", (const char *)read_data(addr2offset(pe.exports[i].address, &pe)));
                print!('\n');
            }
        } else
            print!("No Export table\n");
    }

    if (mode & DUMPIMPORT) {
        print!('\n');
        if (pe.imports) {
            print!("Imported modules:\n");
            for (i = 0; i < pe.import_count; i += 1)
                print!("\t{}\n", pe.imports[i].module);

            print!("\nImported functions:\n");
            for (i = 0; i < pe.import_count; i += 1) {
                print!("\t{}:\n", pe.imports[i].module);
                for (j = 0; j < pe.imports[i].count; j += 1)
                {
                    if (pe.imports[i].nametab[j].is_ordinal)
                        print!("\t\t<ordinal %u>\n", pe.imports[i].nametab[j].ordinal);
                    else
                        print!("\t\t{}\n", pe.imports[i].nametab[j].name);
                }
            }
        } else
            print!("No imported module table\n");
    }

    if (mode & DISASSEMBLE)
        print_sections(&pe);

    freepe(&pe);
}
