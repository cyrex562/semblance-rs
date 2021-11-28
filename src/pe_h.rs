pub struct FileHeader {
    pub  Machine: u16,                      /* 04 */
    pub  NumberOfSections: u16,             /* 06 */
    pub TimeDateStamp: u32,                /* 08 */
    pub PointerToSymbolTable: u32,         /* 0c */
    pub NumberOfSymbols: u32,              /* 10 */
    pub SizeOfOptionalHeader: u16,         /* 14 */
    pub Characteristics: u16,              /* 16 */
}

pub struct Directory {
    pub address: u32,
    pub size: u32,
}

pub struct OptionalHeader {
    /* Standard COFF fields. */
    pub Magic: u16,                        /* 18 */
    pub MajorLinkerVersion: u8,           /* 1a */
    pub  MinorLinkerVersion: u8,           /* 1b */
    pub SizeOfCode: u32,                   /* 1c */
    pub SizeOfInitializedData: u32,        /* 20 */
    pub SizeOfUninitializedData: u32,      /* 24 */
    pub AddressOfEntryPoint: u32,          /* 28 */
    pub BaseOfCode: u32,                   /* 2c */
    pub BaseOfData: u32,                   /* 30 */

    /* PE fields. */
    pub ImageBase: u32,                    /* 34 */
    pub SectionAlignment: u32,             /* 38 */
    pub FileAlignment: u32,                /* 3c */
    pub MajorOperatingSystemVersion: u16,  /* 40 */
    pub MinorOperatingSystemVersion: u16,  /* 42 */
    pub MajorImageVersion: u16,            /* 44 */
    pub MinorImageVersion: u16,            /* 46 */
    pub MajorSubsystemVersion: u16,        /* 48 */
    pub MinorSubsystemVersion: u16,        /* 4a */
    pub Win32VersionValue: u32,            /* 4c */
    pub SizeOfImage: u32,                  /* 50 */
    pub SizeOfHeaders: u32,                /* 54 */
    pub CheckSum: u32,                     /* 58 */
    pub Subsystem: u16,                    /* 5c */
    pub DllCharacteristics: u16,           /* 5e */
    pub SizeOfStackReserve: u32,           /* 60 */
    pub SizeOfStackCommit: u32,            /* 64 */
    pub SizeOfHeapReserve: u32,            /* 68 */
    pub SizeOfHeapCommit: u32,             /* 6c */
    pub LoaderFlags: u32,                  /* 70 */
    pub NumberOfRvaAndSizes: u32,          /* 74 */
}

// STATIC_ASSERT(sizeof(struct optional_header) == 0x60);

pub struct OptionalHeaderPep {
    /* Standard COFF fields. */
    pub  Magic: u16,                        /* 18 */
    pub MajorLinkerVersion: u8,           /* 1a */
    pub MinorLinkerVersion: u8,           /* 1b */
    pub SizeOfCode: u32,                   /* 1c */
    pub SizeOfInitializedData: u32,        /* 20 */
    pub SizeOfUninitializedData: u32,      /* 24 */
    pub AddressOfEntryPoint: u32,          /* 28 */
    pub BaseOfCode: u32,                   /* 2c */

    /* PE fields. */
    pub ImageBase: u64,                    /* 30 */
    pub SectionAlignment: u32,             /* 38 */
    pub FileAlignment: u32,                /* 3c */
    pub MajorOperatingSystemVersion: u16,  /* 40 */
    pub MinorOperatingSystemVersion: u16,  /* 42 */
    pub MajorImageVersion: u16,            /* 44 */
    pub MinorImageVersion: u16,            /* 46 */
    pub MajorSubsystemVersion: u16,        /* 48 */
    pub MinorSubsystemVersion: u16,        /* 4a */
    pub Win32VersionValue: u32,            /* 4c */
    pub SizeOfImage: u32,                  /* 50 */
    pub SizeOfHeaders: u32,                /* 54 */
    pub CheckSum: u32,                     /* 58 */
    pub Subsystem: u16,                    /* 5c */
    pub DllCharacteristics: u16,           /* 5e */
    pub SizeOfStackReserve: u64,           /* 60 */
    pub SizeOfStackCommit: u64,            /* 68 */
    pub SizeOfHeapReserve: u64,            /* 70 */
    pub SizeOfHeapCommit: u64,             /* 78 */
    pub LoaderFlags: u32,                  /* 80 */
    pub NumberOfRvaAndSizes: u32,          /* 84 */
}

// STATIC_ASSERT(sizeof(struct optional_header_pep) == 0x70);

pub struct Section {
    pub  name: [u8;8],          /* 00 */
    pub min_alloc: u32,        /* 08 */
    pub address: u32,          /* 0c */
    pub length: u32,           /* 10 */
    pub offset: u32,           /* 14 */
    pub reloc_offset: u32,     /* 18 */
    pub lineno_offset: u32,    /* 1c */
    pub reloc_count: u16,      /* 20 */
    pub lineno_count: u16,     /* 22 */
    pub flags: u32,            /* 24 */
    /* and our data: */
    // byte *instr_flags;
    pub instr_flags: Vec<u8>
}

pub struct RelocPe
{
    pub offset: u32,
    pub reloc_type: u32,
}

// #pragma pack()

pub struct Export {
    pub address: u32,
    pub ordinal: u16,
    pub name: String,
}

pub struct NameTableEntry {
    pub name: String,
    pub ordinal: u16,
    pub is_ordinal: bool,
}

pub struct ImportModule {
    pub module: String,
    pub iat_addr: u32,
    pub nametab: Vec<NameTableEntry>,
    pub count: usize,
}

pub struct PortableExecutableHeader {
    pub magic: u16, /* same as opt.Magic field, but avoids casting */
    pub imagebase: u64, /* same as opt.ImageBase field, but simpler */
    pub header: FileHeader,
    // pub opt32: Vec<OptionalHeader>,
    pub opt32: OptionalHeader,
    pub opt64: OptionalHeaderPep,
    // pub opt64: Vec<OptionalHeaderPep>,
    pub dirs: Vec<Directory>,
    pub name: String,
    pub sections: Vec<Section>,
    pub exports: Vec<Export>,
    pub export_count: usize,
    pub imports: Vec<ImportModule>,
    pub import_count: usize,
    pub relocs: Vec<RelocPe>,
    pub reloc_count: usize,
}

/* in pe_section.c */
// extern struct section *addr2section(dword addr, const struct pe *pe);
// extern off_t addr2offset(dword addr, const struct pe *pe);
// extern void read_sections(struct pe *pe);
// extern void print_sections(struct pe *pe);
// #endif /* __PE_H */
