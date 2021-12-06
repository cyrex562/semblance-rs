mod dump;
mod semblance;
mod pe_h;
mod pe_header;
mod pe_section;
mod x86_instr_def;
mod x86_instr_ops;
mod x86_32_instr;
mod x86_64_instr;
mod x86_instr_grp;
mod x86_0f_instr;
mod x86_fpu_m_instr;
mod x86_fpu_r_instr;
mod x86_fpu_single_instr;
mod x86_instr_operand;
mod x86_sse_instr;
mod x86_sse_op32_instr;
mod x86_sse_repne_instr;
mod x86_sse_repe_instr;
mod x86_sse_single_instr;
mod x86_instr_arg_type;
mod x86_instr_prefix;
mod x86_mod_rm;
mod mz;
mod ne;
mod ne_header;

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
//         case 0x80:
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
