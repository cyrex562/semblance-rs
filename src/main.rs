#[macro_use]
extern crate scan_fmt;

mod defs;
mod dump;
mod mz;
mod ne;
mod pe;
mod util;
mod x86;

// int main(int argc, char *argv[]){
//     int opt;

//     mode = 0;
//     opts = 0;
//     AsmSyntax = NASM;

//     while ((opt = getopt_long(argc, argv, "a::cCdDefhiM:osvx", long_options, NULL)) >= 0){
//         switch (opt) {
//         case NO_SHOW_RAW_INSN:
//             opts |= NO_SHOW_RAW_INSN;
//             break;
//         case NO_SHOW_ADDRESSES:
//             opts |= NO_SHOW_ADDRESSES;
//             break;
//         case 'a': /* dump resources only */
//         {
//             mode |= DUMPRSRC;
//             if (optarg){
//                 const char *p = optarg;
//                 while (*p == ' ' || *p == '=') ++p;
//                 resource_filters = realloc(resource_filters, (resource_filters_count + 1) * sizeof(*resource_filters));
//                 resource_filters[resource_filters_count++] = strdup(p);
//             }
//             break;
//         }
//         case 'c': /* compilable */
//             opts |= COMPILABLE|NO_SHOW_ADDRESSES|NO_SHOW_RAW_INSN;
//             break;
//         case 'C': /* demangle */
//             opts |= DEMANGLE;
//             break;
//         case 'd': /* disassemble only */
//             mode |= DISASSEMBLE;
//             break;
//         case 'D': /* disassemble all */
//             opts |= DISASSEMBLE_ALL;
//             break;
//         case 'e': /* exports */
//             mode |= DUMPEXPORT;
//             break;
//         case 'f': /* dump header only */
//             mode |= DUMPHEADER;
//             break;
//         case 'h': /* help */
//             printf(help_message);
//             return 0;
//         case 'i': /* imports */
//             mode |= DUMPIMPORT;
//             break;
//         case 'M': /* additional options */
//             if (!strcmp(optarg, "att") || !strcmp(optarg, "gas"))
//                 AsmSyntax = GAS;
//             else if (!strcmp(optarg, "intel") || !strcmp(optarg, "masm"))
//                 AsmSyntax = MASM;
//             else if (!strcmp(optarg, "nasm"))
//                 AsmSyntax = NASM;
//             else {
//                 eprint!( "Unrecognized disassembly option `{}'.\n", optarg);
//                 return 1;
//             }
//             break;
//         case 'o': /* make a specfile */
//             mode = SPECFILE;
//             break;
//         case 'v': /* version */
//             printf("semblance version " VERSION "\n");
//             return 0;
//         case 's': /* full contents */
//             opts |= FULL_CONTENTS;
//             break;
//         case 'x': /* all headers */
//             mode |= DUMPHEADER | DUMPEXPORT | DUMPIMPORT;
//             break;
//         0x80 =>
//             if (optarg[0] == '1' || optarg[0] == 'y' || optarg[0] == 'Y')
//                 pe_rel_addr = 1;
//             else if (optarg[0] == '0' || optarg[0] == 'n' || optarg[0] == 'N')
//                 pe_rel_addr = 0;
//             else {
//                 eprint!( "Unrecognized --pe-rel-addr option `{}'.\n", optarg);
//                 return 1;
//             }
//             break;
//         default:
//             eprint!( "Usage: dumpne [options] <file>\n");
//             return 1;
//         }
//     }

//     if (mode == 0)
//         mode = ~0;

//     if (optind == argc)
//         printf(help_message);

//     while (optind < argc){
//         dump_file(argv[optind++]);
//         if (optind < argc)
//             printf("\n\n");
//     }

//     return 0;
// }

fn main() {
    println!("Hello, world!");
}

pub const DISASSEMBLE: u8 = 0x10;
pub const SPECFILE: u8 = 0x80;
// extern word mode; /* what to dump */
pub const DISASSEMBLE_ALL: u8 = 0x01;
pub const DEMANGLE: u8 = 0x02;
pub const NO_SHOW_RAW_INSN: u8 = 0x04;
pub const NO_SHOW_ADDRESSES: u8 = 0x08;
pub const COMPILABLE: u8 = 0x10;
pub const FULL_CONTENTS: u8 = 0x20;
pub const DUMP_HEADER: u8 = 0x01;
pub const DUMP_RSRC: u8 = 0x02;
pub const DUMP_EXPORT: u8 = 0x04;

pub const DUMP_IMPORT: u8 = 0x08;
