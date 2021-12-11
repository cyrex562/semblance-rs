/*
 * Entry point of the "dump" program
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

use std::path::Path;
use std::{error::Error, fs::File};

use memmap::MmapOptions;

use crate::semblance::{read_dword, read_word};

// #include "semblance.h"

// byte *map;

// word mode;
// word opts;
// char **resource_filters;
// unsigned resource_filters_count;
// enum AsmSyntax AsmSyntax;

pub fn dump_file(file_name_path: &str) -> Result<(), Box<dyn Error>> {
    // struct stat st;

    // word magic;
    let mut magic: u16;
    // off_t offset = 0;
    let mut offset: usize = 0;
    // int fd;

    let path = Path::new(file_name_path);

    let mut fd = File::open(path)?;

    let map = unsafe { MmapOptions::new().map(&fd)? };

    // if ((fd = open(file, O_RDONLY)) < 0) {
    //     perror("Cannot open {}");
    //     return;
    // }

    // if (fstat(fd, &st) < 0)
    // {
    //     perror("Cannot stat {}");
    //     return;
    // }

    // if ((map = mmap(NULL, st.st_size, PROT_READ, MAP_PRIVATE, fd, 0)) == MAP_FAILED)
    // {
    //     perror("Cannot map {}");
    //     return;
    // }
    let mut magic = read_word(&map.to_vec(), 0);
    println!("File: {}", file_name_path);

    if magic == 0x5a4d {
        /* MZ */
        offset = read_dword(&map.to_vec(), 0x3c) as usize;
        magic = read_word(&map.to_vec(), offset);

        if magic == 0x4550 {
            dumppe(offset);
        } else if magic == 0x454e {
            dumpne(offset);
        } else {
            dumpmz();
        }
    } else {
        eprintln!("file format not recognized");
        // eprint!( "File format not recognized\n");
    }
    Ok(())
}

// static const char help_message[] =
// "dump: tool to disassemble and print information from executable files.\n"
// "Usage: dump [options] <file(s)>\n"
// "Available options:\n"
// "\t-a, --resource[=filter]              Print embedded resources.\n"
// "\t-c, --compilable                     Produce output that can be compiled.\n"
// "\t-C, --demangle                       Demangle C++ function names.\n"
// "\t-d, --disassemble                    Print disassembled machine code.\n"
// "\t-e, --exports                        Print exported functions.\n"
// "\t-f, --file-headers                   Print contents of the file header.\n"
// "\t-h, --help                           Display this help message.\n"
// "\t-i, --imports                        Print imported modules.\n"
// "\t-M, --disassembler-options=[...]     Extended options for disassembly.\n"
// "\t\tatt        Alias for `gas'.\n"
// "\t\tgas        Use GAS syntax for disassembly.\n"
// "\t\tintel      Alias for `masm'.\n"
// "\t\tmasm       Use MASM syntax for disassembly.\n"
// "\t\tnasm       Use NASM syntax for disassembly.\n"
// "\t-o, --specfile                       Create a specfile from exports.\n"
// "\t-s, --full-contents                  Display full contents of all sections.\n"
// "\t-v, --version                        Print the version number of semblance.\n"
// "\t-x, --all-headers                    Print all headers.\n"
// "\t--no-show-addresses                  Don't print instruction addresses.\n"
// "\t--no-show-raw-insn                   Don't print raw instruction hex code.\n"
// "\t--pe-rel-addr=[y/n]                  Use relative addresses for PE files.\n"
// ;

// static const struct option long_options[] = {
//     {"resource",                optional_argument,  NULL, 'a'},
//     {"compilable",              no_argument,        NULL, 'c'},
//     {"demangle",                no_argument,        NULL, 'C'},
//     {"disassemble",             no_argument,        NULL, 'd'},
//     {"disassemble-all",         no_argument,        NULL, 'D'},
//     {"exports",                 no_argument,        NULL, 'e'},
//     {"file-headers",            no_argument,        NULL, 'f'},
// //  {"gas",                     no_argument,        NULL, 'G'},
//     {"help",                    no_argument,        NULL, 'h'},
//     {"imports",                 no_argument,        NULL, 'i'},
// //  {"masm",                    no_argument,        NULL, 'I'}, /* for "Intel" */
//     {"disassembler-options",    required_argument,  NULL, 'M'},
// //  {"nasm",                    no_argument,        NULL, 'N'},
//     {"specfile",                no_argument,        NULL, 'o'},
//     {"full-contents",           no_argument,        NULL, 's'},
//     {"version",                 no_argument,        NULL, 'v'},
//     {"all-headers",             no_argument,        NULL, 'x'},
//     {"no-show-raw-insn",        no_argument,        NULL, NO_SHOW_RAW_INSN},
//     {"no-prefix-addresses",     no_argument,        NULL, NO_SHOW_ADDRESSES},
//     {"pe-rel-addr",             required_argument,  NULL, 0x80},
//     {0}
// };
