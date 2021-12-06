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
