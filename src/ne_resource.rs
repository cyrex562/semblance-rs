use crate::semblance::{read_byte, read_data, read_word};

pub fn dup_string_resource(map: &Vec<u8>, offset: usize) -> String
{
    let length = read_byte(map, offset);
    let mut ret = String::new();
    ret = read_data(map, offset+1, length as usize).into();
    return ret;
}

/* length-indexed; returns  */
pub fn print_escaped_string(map: &Vec<u8>, mut offset: usize, mut length: i32){
    print!('"');
    while length -= 1 {
        offset += 1;
        let c = read_byte(map, offset);
        if c == '\t' as u8 {
            print!("\\t");
        }
        else if c == '\n' as u8 {
            print!("\\n");
        }
        else if c == '\r' as u8 {
            print!("\\r");
        }
        else if c == '"' as u8 {
            print!("\\\"");
        }
        else if c == '\\' as u8 {
            print!("\\\\");
        }
        else if c >= ' ' as u8 && c <= '~' as u8 {
            print!(c);
        }
        else {
            print!("\\x{:02x}", c);
        }
    }
    print!('"');
}

/* null-terminated; returns the end of the string */
pub fn print_escaped_string0(map: &Vec<u8>, mut offset: usize) -> usize
{
    print!('"');
    offset += 1;
    let mut c = read_byte(map, offset);
    while c != 0 {
        if c == '\t' as u8 {
            print!("\\t");
        }
        else if c == '\n' as u8 {
            print!("\\n");
        }
        else if c == '\r' as u8 {
            print!("\\r");
        }
        else if c == '"' as u8 {
            print!("\\\"");
        }
        else if c == '\\' as u8 {
            print!("\\\\");
        }
        else if c >= ' ' as u8 && c <= '~' as u8 {
            print!(c);
        }
        else {
            print!("\\x{:02x}", c);
        }
        offset += 1;
        c = read_byte(map, offset);
    }
    print!('"');
    return offset;
}

pub fn print_timestamp(high: u32, low: u32){
    unimplemented!()
}

pub const RSRC_TYPES: [String;19] = [
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
    "Message table".to_string(),     /* b */    /* fixme: error table? */
    "Cursor directory".to_string(),  /* c */
    "".to_string(),
    "Icon directory".to_string(),    /* e */
    "Name table".to_string(),        /* f */
    "Version".to_string(),           /* 10 */
    "".to_string(),                              /* fixme: RT_DLGINCLUDE? */
    "".to_string()
];
// const size_t rsrc_types_count = sizeof(RSRC_TYPES)/sizeof(RSRC_TYPES[0]);

pub const RSRC_BMP_COMPRESSION: [String;15] = [
    "none".to_string(),                     /* 0 */
    "RLE (8 bpp)".to_string(),              /* 1 */
    "RLE (4 bpp)".to_string(),              /* 2 */
    "RGB bit field masks".to_string(),      /* 3 */
    "JPEG".to_string(), /* shouldn't occur?    4 */
    "PNG".to_string(), /* shouldn't occur?     5 */
    "RGBA bit field masks".to_string(),     /* 6 */
    "".to_string(),
    "".to_string(),
    "".to_string(),
    "".to_string(),
    "none (CMYK)".to_string(),              /* 11 */
    "RLE (8 bpp, CMYK)".to_string(),        /* 12 */
    "RLE (4 bpp, CMYK)".to_string(),        /* 13 */
    "".to_string()
];

pub fn print_rsrc_flags(flags: u16){
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

/* There are a lot of styles here and most of them would require longer
 * descriptions, so we're just going to use the C names.
 * Not all of these are dialog box-related, but I'm not going to try to
 * sort through them. */

pub const rsrc_dialog_style: [String;33] = [
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
    "DS_USEPIXELS".to_string(),     /* 00008000 */
    "WS_TABSTOP".to_string(),       /* 00010000 */
    "WS_GROUP".to_string(),         /* 00020000 */
    "WS_THICKFRAME".to_string(),    /* 00040000 */
    "WS_SYSMENU".to_string(),       /* 00080000 */
    "WS_HSCROLL".to_string(),       /* 00100000 */
    "WS_VSCROLL".to_string(),       /* 00200000 */
    "WS_DLGFRAME".to_string(),      /* 00400000 */
    "WS_BORDER".to_string(),        /* 00800000 */
    "WS_MAXIMIZE".to_string(),      /* 01000000 */
    "WS_CLIPCHILDREN".to_string(),  /* 02000000 */
    "WS_CLIPSIBLINGS".to_string(),  /* 04000000 */
    "WS_DISABLED".to_string(),      /* 08000000 */
    "WS_VISIBLE".to_string(),       /* 10000000 */
    "WS_MINIMIZE".to_string(),      /* 20000000 */
    "WS_CHILD".to_string(),         /* 40000000 */
    "WS_POPUP".to_string(),         /* 80000000 */
    "".to_string(),
];

pub fn print_rsrc_dialog_style(flags: u32){
    let mut buffer = String::new();
    
    for i in 0 .. 32 {
        if flags & (1<<i) {
            buffer += ", ";
            buffer += &*rsrc_dialog_style[i];
        }
    }
    print!("    Style: {}\n", buffer[2..]);
}

pub const RSRC_BUTTON_TYPE: [String;17] = [
    "BS_PUSHBUTTON".to_string(),        /* 0 */
    "BS_DEFPUSHBUTTON".to_string(),     /* 1 */
    "BS_CHECKBOX".to_string(),          /* 2 */
    "BS_AUTOCHECKBOX".to_string(),      /* 3 */
    "BS_RADIOBUTTON".to_string(),       /* 4 */
    "BS_3STATE".to_string(),            /* 5 */
    "BS_AUTO3STATE".to_string(),        /* 6 */
    "BS_GROUPBOX".to_string(),          /* 7 */
    "BS_USERBUTTON".to_string(),        /* 8 */
    "BS_AUTORADIOBUTTON".to_string(),   /* 9 */
    "BS_PUSHBOX".to_string(),           /* 10 */
    "BS_OWNERDRAW".to_string(),         /* 11 */
    "(unknown type 12)".to_string(),
    "(unknown type 13)".to_string(),
    "(unknown type 14)".to_string(),
    "(unknown type 15)".to_string(),
    "".to_string(),
];

pub const RSRC_EDIT_STYLE: [String;17] = [
    "".to_string(), "".to_string(),       /* type */
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
    ""..to_string()
];

pub const rsrc_static_type: [String;20] = [
    "SS_LEFT".to_string(),          /* 0 */
    "SS_CENTER".to_string(),        /* 1 */
    "SS_RIGHT".to_string(),         /* 2 */
    "SS_ICON".to_string(),          /* 3 */
    "SS_BLACKRECT".to_string(),     /* 4 */
    "SS_GRAYRECT".to_string(),      /* 5 */
    "SS_WHITERECT".to_string(),     /* 6 */
    "SS_BLACKFRAME".to_string(),    /* 7 */
    "SS_GRAYFRAME".to_string(),     /* 8 */
    "SS_WHITEFRAME".to_string(),    /* 9 */
    "SS_USERITEM".to_string(),      /* 10 */
    "SS_SIMPLE".to_string(),        /* 11 */
    "SS_LEFTNOWORDWRAP".to_string(),/* 12 */
    "SS_OWNERDRAW".to_string(),     /* 13 */
    "SS_BITMAP".to_string(),        /* 14 */
    "SS_ENHMETAFILE".to_string(),   /* 15 */
    "SS_ETCHEDHORZ".to_string(),    /* 16 */
    "SS_ETCHEDVERT".to_string(),    /* 17 */
    "SS_ETCHEDFRAME".to_string(),   /* 18 */
    "".to_string(),
];

pub const rsrc_static_style: [String;15] = [
    "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), /* type */
    "(unknown flag 0x0020)".to_string(),
    "SS_REALSIZECONTROL".to_string(), /* 0040 */
    "SS_NOPREFIX".to_string(),        /* 0080 */
    "SS_NOTIFY".to_string(),          /* 0100 */
    "SS_CENTERIMAGE".to_string(),     /* 0200 */
    "SS_RIGHTJUST".to_string(),       /* 0400 */
    "SS_REALSIZEIMAGE".to_string(),   /* 0800 */
    "SS_SUNKEN".to_string(),          /* 1000 */
    "SS_EDITCONTROL".to_string(),     /* 2000 */
    0
];

pub const rsrc_listbox_style: [String;17] = [
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
    "".to_string()
];

pub const rsrc_combobox_style: [String;16] = [
    "".to_string(), "".to_string(), /* type */
    "".to_string(), "".to_string(), /* unknown */
    "CBS_OWNERDRAWFIXED".to_string(),    /* 0010 */
    "CBS_OWNERDRAWVARIABLE".to_string(), /* 0020 */
    "CBS_AUTOHSCROLL".to_string(),       /* 0040 */
    "CBS_OEMCONVERT".to_string(),        /* 0080 */
    "CBS_SORT".to_string(),              /* 0100 */
    "CBS_HASSTRINGS".to_string(),        /* 0200 */
    "CBS_NOINTEGRALHEIGHT".to_string(),  /* 0400 */
    "CBS_DISABLENOSCROLL".to_string(),   /* 0800 */
    "".to_string(), /* unknown */
    "CBS_UPPERCASE".to_string(),         /* 2000 */
    "CBS_LOWERCASE".to_string(),         /* 4000 */
    "".to_string()
];

pub fn print_rsrc_control_style(class: u8, flags: u32){

    let mut buffer = String::new();

    print!("        Style: ");
    
    match class {
    0x80 => {
        /* Button */
        buffer = rsrc_button_type[flags & 0x000f];

        if flags & 0x0010 { buffer += ", (unknown flag 0x0010)"; }
        if flags & 0x0020 { buffer += ", BS_LEFTTEXT"; }

        if (flags & 0x0040) == 0 {
            buffer += ", BS_TEXT";
        } else {
            if flags & 0x0040 { buffer += ", BS_ICON"; }
            if flags & 0x0080 { buffer += ", BS_BITMAP"; }
        }

        if (flags & 0x0300) == 0x0100 { buffer += ", BS_LEFT"; } else if ((flags & 0x0300) == 0x0200) { buffer += ", BS_RIGHT"; } else if ((flags & 0x0300) == 0x0300) { buffer += ", BS_CENTER"; }

        if ((flags & 0x0C00) == 0x0400) { buffer += ", BS_TOP"; } else if ((flags & 0x0C00) == 0x0800) { buffer += ", BS_BOTTOM"; } else if ((flags & 0x0C00) == 0x0C00) { buffer += ", BS_VCENTER"; }

        if (flags & 0x1000) { buffer += ", BS_PUSHLIKE"; }
        if (flags & 0x2000) { buffer += ", BS_MULTILINE"; }
        if (flags & 0x4000) { buffer += ", BS_NOTIFY"; }
        if (flags & 0x8000) { buffer += ", BS_FLAT"; }
    }

    0x81 => {
        /* Edit */
        if (flags & 3) == 0 { buffer += "ES_LEFT"; } else if (flags & 3) == 1 { buffer += "ES_CENTER"; } else if (flags & 3) == 2 { buffer += "ES_RIGHT"; } else if (flags & 3) == 3 {
            buffer += "(unknown type 3)";
        }
        for i in 2 .. 16 {
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
        for i in 0 .. 16 {
            if flags & (1 << i) > 0 {
                buffer += ", ";
                buffer += &*rsrc_listbox_style[i];
            }
        }
    }


    0x84 => /* ScrollBar */{
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
    for i in 16 .. 32 {
        if (flags & (1<<i)){
            buffer += ", ";
            buffer += &*rsrc_dialog_style[i];
        }
    }

    print!("{}\n", if buffer[0] == ',' { (&buffer[2..])} else { &buffer });
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

pub const rsrc_dialog_class: [String;7] = [
    "Button".to_string(),    /* 80 */
    "Edit".to_string(),      /* 81 */
    "Static".to_string(),    /* 82 */
    "ListBox".to_string(),   /* 83 */
    "ScrollBar".to_string(), /* 84 */
    "ComboBox".to_string(),  /* 85 */
    "".to_string(),
];

pub fn print_rsrc_menu_items(map: &Vec<u8>, depth: i32, mut offset: usize) -> usize
{
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
        for i in 0 .. depth { print!("  "); }
        if !(flags & 0x0010) {
            /* item ID */
            id = read_word(map, offset);
            offset += 2;
            print!("{}: ", id);
        }

        offset = print_escaped_string0(map, offset);

        /* and print flags */
        buffer[0] = '\0';
        if flags & 0x0001 { buffer += ", grayed"; }
        if flags & 0x0002 { buffer += ", inactive"; }
        if (flags & 0x0004) { buffer += ", bitmap"; }
        if (flags & 0x0008) { buffer += ", checked"; }
        if (flags & 0x0010) { buffer += ", popup"; }
        if (flags & 0x0020) { buffer += ", menu bar break"; }
        if flags & 0x0040 { buffer += ", menu break"; }
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

/* This is actually two headers, with the first (VS_VERSIONINFO)
 * describing the second. However it seems the second is always
 * a VS_FIXEDFILEINFO header, so we ignore most of those details. */
pub struct version_header {
    pub length: u16,            /* 00 */
    pub value_length: u16,      /* 02 - always 52 (0x34), the length of the second header */
    /* the "type" field given by Windows is missing */
    pub string: [u8;16],        /* 04 - the fixed string VS_VERSION_INFO\0 */
    pub magic: u32,            /* 14 - 0xfeef04bd */
    pub struct_2: u16,          /* 18 - seems to always be 1.0 */
    pub struct_1: u16,          /* 1a */
    /* 1.2.3.4 &c. */
    pub file_2: u16,            /* 1c */
    pub file_1: u16,            /* 1e */
    pub file_4: u16,            /* 20 */
    pub file_3: u16,            /* 22 */
    pub prod_2: u16,            /* 24 - always the same as the above? */
    pub prod_1: u16,            /* 26 */
    pub prod_4: u16,            /* 28 */
    pub prod_3: u16,            /* 2a */
    pub flags_file_mask: u32,  /* 2c - always 2 or 3f...? */
    pub flags_file: u32,       /* 30 */
    pub flags_os: u32,         /* 34 */
    pub flags_type: u32,       /* 38 */
    pub flags_subtype: u32,    /* 3c */
    pub date_1: u32,           /* 40 - always 0? */
    pub date_2: u32,           /* 44 */
}

// STATIC_ASSERT(sizeof(struct version_header) == 0x48);

pub const rsrc_version_file: [String;7] = [
    "VS_FF_DEBUG".to_string(),        /* 0001 */
    "VS_FF_PRERELEASE".to_string(),   /* 0002 */
    "VS_FF_PATCHED".to_string(),      /* 0004 */
    "VS_FF_PRIVATEBUILD".to_string(), /* 0008 */
    "VS_FF_INFOINFERRED".to_string(), /* 0010 */
    "VS_FF_SPECIALBUILD".to_string(), /* 0020 */
    "".to_string(),
];

pub const rsrc_version_type: [String;9] = [
    "unknown".to_string(),             /* 0 VFT_UNKNOWN */
    "application".to_string(),         /* 1 VFT_APP */
    "DLL".to_string(),                 /* 2 VFT_DLL */
    "device driver".to_string(),       /* 3 VFT_DRV */
    "font".to_string(),                /* 4 VFT_FONT */
    "virtual device".to_string(),      /* 5 VFT_VXD */
    "(unknown type 6)".to_string(),
    "static-link library".to_string(), /* 7 VFT_STATIC_LIB */
    "".to_string()
];

pub const rsrc_version_subtype_drv: [String;10] = [
    "unknown",              /* 0 VFT2_UNKNOWN */
    "printer",              /* 1 VFT2_DRV_PRINTER etc. */
    "keyboard",             /* 2 */
    "language",             /* 3 */
    "display",              /* 4 */
    "mouse",                /* 5 */
    "network",              /* 6 */
    "system",               /* 7 */
    "installable",          /* 8 */
    "sound",                /* 9 */
    "communications",       /* 10 */
    "input method",         /* 11, found in WINE */
    "versioned printer",    /* 12 */
    0
];

static void print_rsrc_version_flags(struct version_header header){
    char buffer[1024];
    int i;
    
    buffer[0] = '\0';
    for (i=0;i<6;i += 1){
        if (header.flags_file & (1<<i)){
            buffer += ", ";
            strcat(buffer, rsrc_version_file[i]);
        }
    }
    if (header.flags_file & 0xffc0)
        sprintf(buffer+strlen(buffer), ", (unknown flags 0x{:04x})", header.flags_file & 0xffc0);
    print!("    File flags: ");
    if (header.flags_file)
        print!("{}", buffer+2);

    buffer[0] = '\0';
    if (header.flags_os == 0)
        buffer += ", VOS_UNKNOWN";
    else {
        switch (header.flags_os & 0xffff){
        case 1: buffer += ", VOS__WINDOWS16"; break;
        case 2: buffer += ", VOS__PM16"; break;
        case 3: buffer += ", VOS__PM32"; break;
        case 4: buffer += ", VOS__WINDOWS32"; break;
        default: sprintf(buffer, ", (unknown OS 0x{:04x})", header.flags_os & 0xffff);
        }
        switch (header.flags_os >> 16){
        case 1: buffer += ", VOS_DOS"; break;
        case 2: buffer += ", VOS_OS216"; break;
        case 3: buffer += ", VOS_OS232"; break;
        case 4: buffer += ", VOS_NT"; break;
        case 5: buffer += ", VOS_WINCE"; break; /* found in WINE */
        default: sprintf(buffer+strlen(buffer), ", (unknown OS 0x{:04x})", header.flags_os >> 16);
        }
    }
    print!("\n    OS flags: {}\n", buffer+2);

    if (header.flags_type <= 7)
        print!("    Type: {}\n", rsrc_version_type[header.flags_type]);
    else
        print!("    Type: (unknown type {})\n", header.flags_type);

    if (header.flags_type == 3){ /* driver */
        if (header.flags_subtype <= 12)
            print!("    Subtype: {} driver\n", rsrc_version_subtype_drv[header.flags_subtype]);
        else
            print!("    Subtype: (unknown subtype {})\n", header.flags_subtype);
    } else if (header.flags_type == 4){ /* font */
        if (header.flags_subtype == 0)      print!("    Subtype: unknown font\n");
        else if (header.flags_subtype == 1) print!("    Subtype: raster font\n");
        else if (header.flags_subtype == 2) print!("    Subtype: vector font\n");
        else if (header.flags_subtype == 3) print!("    Subtype: TrueType font\n");
        else print!("    Subtype: (unknown subtype {})\n", header.flags_subtype);
    } else if (header.flags_type == 5){ /* VXD */
        print!("    Virtual device ID: {}\n", header.flags_subtype);
    } else if (header.flags_subtype){
        /* according to MSDN nothing else is valid */
        print!("    Subtype: (unknown subtype {})\n", header.flags_subtype);
    }
};

static void print_rsrc_strings(offset: usize, end: usize)
{
    u16 length;

    while (offset < end)
    {
        /* first length is redundant */
        length = read_word(offset + 2);
        print!("        ");
        offset = print_escaped_string0(offset + 4);
        offset = (offset + 3) & ~3;
        print!(": ");
        /* According to MSDN this is zero-terminated, and in most cases it is.
         * However, at least one application (msbsolar) has NEs with what
         * appears to be a non-zero-terminated string. In Windows this is cut
         * off at one minus the given length, just like other strings, so
         * we'll do that here.
         *
         * And another file has a zero length here. How do compilers screw this
         * up so badly? */
        print_escaped_string(offset, length ? length - 1 : 0);
        offset += length;
        offset = (offset + 3) & ~3;
        print!('\n');
    }
};

static void print_rsrc_stringfileinfo(offset: usize, end: usize)
{
    u16 length;
    unsigned int lang = 0;
    unsigned int codepage = 0;

    /* we already processed the StringFileInfo header */
    while (offset < end)
    {
        /* StringTable header */
        length = read_word(offset);

        /* codepage and language code */
        sscanf(read_data(offset + 4), "%4x%4x", &lang, &codepage);
        print!("    String table (lang={:04x}, codepage={:04x}):\n", lang, codepage);

        print_rsrc_strings(offset + 16, offset + length);
        offset += length;
    }
};

static void print_rsrc_varfileinfo(offset: usize, end: usize)
{
    while (offset < end)
    {
        /* first length is redundant */
        u16 length = read_word(offset + 2), i;
        offset += 16;
        for (i = 0; i < length; i += 4)
            print!("    Var (lang={:04x}, codepage={:04x})\n", read_word(offset + i), read_word(offset + i + 2));
        offset += length;
    }
};

static void print_rsrc_resource(u16 type, offset: usize, size_t length, u16 rn_id)
{
    switch (type)
    {
    case 0x8001: /* Cursor */
        print!("    Hotspot: ({}, {})\n", read_word(offset), read_word(offset + 2));
        offset += 4;
        /* fall through */

    case 0x8002: /* Bitmap */
    case 0x8003: /* Icon */
        if (read_dword(offset) == 12) /* BITMAPCOREHEADER */
        {
            print!("    Size: %dx{}\n", read_word(offset + 4), read_word(offset + 6));
            print!("    Planes: {}\n", read_word(offset + 8));
            print!("    Bit depth: {}\n", read_word(offset + 10));
        }
        else if (read_dword(offset) == 40) /* BITMAPINFOHEADER */
        {
            const struct header_bitmap_info *header = read_data(offset);
            print!("    Size: %dx{}\n", header.biWidth, header.biHeight / 2);
            print!("    Planes: {}\n", header.biPlanes);
            print!("    Bit depth: {}\n", header.biBitCount);
            if (header.biCompression <= 13 && rsrc_bmp_compression[header.biCompression])
                print!("    Compression: {}\n", rsrc_bmp_compression[header.biCompression]);
            else
                print!("    Compression: (unknown value {})\n", header.biCompression);
            print!("    Resolution: %dx{} pixels/meter\n",
                    header.biXPelsPerMeter, header.biYPelsPerMeter);
            print!("    Colors used: {}", header.biClrUsed); /* todo: implied */
            if (header.biClrImportant)
                print!(" ({} marked important)", header.biClrImportant);
            print!('\n');
        }
        else
            eprint!("Unknown bitmap header size {}.\n", read_dword(offset));
        break;

    case 0x8004: /* Menu */
    {
        u16 extended = read_word(offset);

        if (extended > 1) {
            eprint!("Unknown menu version {}\n",extended);
            break;
        }
        print!(extended ? "    Type: extended\n" : "    Type: standard\n");
        if (read_word(offset + 2) != extended*4)
            eprint!("Unexpected offset value {} (expected {}).\n", read_word(offset + 2), extended * 4);
        offset += 4;

        if (extended)
        {
            print!("    Help ID: {}\n", read_dword(offset));
            offset += 4;
        }

        print!("    Items:\n");
        print_rsrc_menu_items(0, offset);
        break;
    }
    case 0x8005: /* Dialog box */
    {
        u8 count;
        u16 font_size;
        u32 style = read_dword(offset);
        print_rsrc_dialog_style(style);
        count = read_byte(offset + 4);
        print!("    Position: ({}, {})\n", read_word(offset + 5), read_word(offset + 7));
        print!("    Size: %dx{}\n", read_word(offset + 9), read_word(offset + 11));
        if (read_byte(offset + 13) == 0xff){
            print!("    Menu resource: #{}", read_word(offset + 14));
        } else {
            print!("    Menu name: ");
            offset = print_escaped_string0(offset + 13);
        }
        print!("\n    Class name: ");
        offset = print_escaped_string0(offset);
        print!("\n    Caption: ");
        offset = print_escaped_string0(offset);
        if (style & 0x00000040){ /* DS_SETFONT */
            font_size = read_word(offset);
            print!("\n    Font: ");
            offset = print_escaped_string0(offset + 2);
            print!(" ({} pt)", font_size);
        }
        print!('\n');

        while (count -= 1){
            const struct dialog_control *control = read_data(offset);
            offset += sizeof(*control);

            if (control.class & 0x80){
                if (control.class <= 0x85)
                    print!("    {}", rsrc_dialog_class[control.class & (~0x80)]);
                else
                    print!("    (unknown class {})", control.class);
            }
            else
                offset = print_escaped_string0(offset);
            print!(" {}:\n", control.id);

            print!("        Position: ({}, {})\n", control.x, control.y);
            print!("        Size: %dx{}\n", control.width, control.height);
            print_rsrc_control_style(control.class, control.style);

            if (read_byte(offset) == 0xff){
                /* todo: we can check the style for SS_ICON/SS_BITMAP and *maybe* also
                 * refer back to a printed RT_GROUPICON/GROUPCUROR/BITMAP resource. */
                print!("        Resource: #{}", read_word(offset));
                offset += 3;
            } else {
                print!("        Text: ");
                offset = print_escaped_string0(offset );
            }
            /* todo: WINE parses this as "data", but all of my testcases return 0. */
            /* read_byte(); */
            print!('\n');
        }
    }
    break;
    case 0x8006: /* String */
    {
        cursor: usize = offset;
        int i = 0;

        while (cursor < offset + length)
        {
            u8 str_length = read_byte(cursor += 1);
            if (str_length)
            {
                print!("    %3d (0x%06lx): ", i + ((rn_id & (~0x8000))-1)*16, cursor);
                print_escaped_string(cursor, str_length);
                print!('\n');
                cursor += str_length;
            }
            i += 1;
        }
    }
    break;
#if 0 /* No testcases for this either */
    case 0x8007: /* Font directory */
    case 0x8008: /* Font component */
        break;
    case 0x8009: /* Accelerator table */
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
        u8 flags;

        do {
            flags = read_byte();
            key = read_word();
            id = read_word();

            print!("    ");

            if (flags & 0x02)
                print!("(FNOINVERT) ");

            if (flags & 0x04)
                print!("Shift+");
            if (flags & 0x08)
                print!("Ctrl+");
            if (flags & 0x10)
                print!("Alt+");
            if (flags & 0x60)
                eprint!("Unknown accelerator flags 0x{:02x}\n", flags & 0x60);

            /* fixme: print the key itself */

            print!(": {}\n", id);
        } while (!(flags & 0x80));
    }
    break;
#endif
    /* Resource data (0x800a) is parsed as default, i.e. hex-dumped. */
    case 0x800c: /* Cursor directory */
    case 0x800e: /* Icon directory */
    {
        /* All of the information supplied here is contained in the actual
         * resource. Therefore we only list the components this refers to.
         * Fortunately, the headers are different but the relevant information
         * is stored in the same bytes. */
        u16 count = read_word(offset + 4);
        offset += 6;
        print!("    Resources: ");
        if (count -= 1) {
            print!("#{}", read_word(offset + 12));
            offset += 14;
        }
        while (count -= 1) {
            print!(", #{}", read_word(offset + 12));
            offset += 14;
        }
        print!("\n");
    }
    break;
    case 0x8010: /* Version */
    {
        const struct version_header *header = read_data(offset);
        const end: usize = offset + header.length;

        if (header.value_length != 52)
            eprint!("Version header length is {} (expected 52).\n", header.value_length);
        if (strcmp((char *)header.string, "VS_VERSION_INFO"))
            eprint!("Version header is %.16s (expected VS_VERSION_INFO).\n", header.string);
        if (header.magic != 0xfeef04bd)
            eprint!("Version magic number is 0x%08x (expected 0xfeef04bd).\n", header.magic);
        if (header.struct_1 != 1 || header.struct_2 != 0)
            eprint!("Version header version is {}.{} (expected 1.0).\n", header.struct_1, header.struct_2);
        print_rsrc_version_flags(*header);

        print!("    File version:    {}.{}.{}.{}\n",
               header.file_1, header.file_2, header.file_3, header.file_4);
        print!("    Product version: {}.{}.{}.{}\n",
               header.prod_1, header.prod_2, header.prod_3, header.prod_4);

        if (0) {
        print!("    Created on: ");
        print_timestamp(header.date_1, header.date_2);
        print!('\n');
        }

        offset += sizeof(struct version_header);

        while (offset < end)
        {
            u16 info_length = read_word(offset);
            u16 value_length = read_word(offset + 2);
            const char *key = read_data(offset + 4);

            if (value_length)
                eprint!("Value length is nonzero: {:04x}\n", value_length);

            /* "type" is again omitted */
            if (!strcmp(key, "StringFileInfo"))
                print_rsrc_stringfileinfo(offset + 20, offset + info_length);
            else if (!strcmp(key, "VarFileInfo"))
                print_rsrc_varfileinfo(offset + 16, offset + info_length);
            else
                eprint!("Unrecognized file info key: {}\n", key);

            offset += ((info_length + 3) & ~3);
        }
        break;
    }
    default:
    {
        cursor: usize = offset;
        char len;
        int i;
        /* hexl-style dump */
        while (cursor < offset + length)
        {
            len = min(offset + length - cursor, 16);
            
            print!("    %lx:", cursor);
            for (i=0; i<16; i += 1){
                if (!(i & 1))
                    /* Since this is 16 bits, we put a space after (before) every other two bytes. */
                    print!(' ');
                if (i<len)
                    print!("{:02x}", read_byte(cursor + i));
                else
                    print!("  ");
            }
            print!("  ");
            for (i=0; i<len; i += 1){
                char c = read_byte(cursor + i);
                print!(isprint(c) ? c : '.');
            }
            print!('\n');

            cursor += len;
        }
    }
    break;
    }
}

/* return true if this was one of the resources that was asked for */
static int filter_resource(const char *type, const char *id){
    unsigned i;

    if (!resource_filters_count)
        return 1;

    for (i = 0; i < resource_filters_count;  += 1i){
        const char *filter_type = resource_filters[i], *p;
        size_t len = strlen(type);

        /* note that both resource types and IDs are case insensitive */

        /* if the filter is just a resource type or ID and we match that */
        if (!strcasecmp(type, filter_type) || !strcasecmp(id, filter_type))
            return 1;

        /* if the filter is a resource type followed by an ID and we match both */
        if (strncasecmp(type, filter_type, len) || filter_type[len] != ' ')
            continue;

        p = filter_type + len;
        while (*p == ' ')  += 1p;
        if (!strcasecmp(id, p))
            return 1;
    }
    return 0;
}

struct resource {
    u16 offset;
    u16 length;
    u16 flags;
    u16 id;
    u16 handle; /* fixme: what is this? */
    u16 usage; /* fixme: what is this? */
};

STATIC_ASSERT(sizeof(struct resource) == 0xc);

struct type_header
{
    u16 type_id;
    u16 count;
    u32 resloader; /* fixme: what is this? */
    struct resource resources[1];
};

void print_rsrc(start: usize){
    const struct type_header *header;
    u16 align = read_word(start);
    char *idstr;
    u16 i;

    header = read_data(start + sizeof(u16));

    while (header.type_id)
    {
        if (header.resloader)
            eprint!("resloader is nonzero: %08x\n", header.resloader);

        for (i = 0; i < header.count;  += 1i)
        {
            const struct resource *rn = &header.resources[i];

            if (rn.id & 0x8000){
                idstr = malloc(6);
                sprintf(idstr, "{}", rn.id & ~0x8000);
            } else
                idstr = dup_string_resource(start + rn.id);

            if (header.type_id & 0x8000)
            {
                if ((header.type_id & (~0x8000)) < rsrc_types_count && rsrc_types[header.type_id & (~0x8000)]){
                    if (!filter_resource(rsrc_types[header.type_id & ~0x8000], idstr))
                        goto next;
                    print!("\n{}", rsrc_types[header.type_id & ~0x8000]);
                } else {
                    char typestr[7];
                    sprintf(typestr, "0x{:04x}", header.type_id);
                    if (!filter_resource(typestr, idstr))
                        goto next;
                    print!("\n{}", typestr);
                }
            }
            else
            {
                char *typestr = dup_string_resource(start + header.type_id);
                if (!filter_resource(typestr, idstr))
                {
                    free(typestr);
                    goto next;
                }
                print!("\n\"{}\"", typestr);
                free(typestr);
            }

            print!(" {}", idstr);
            print!(" (offset = 0x%x, length = {} [0x%x]", rn.offset << align, rn.length << align, rn.length << align);
            print_rsrc_flags(rn.flags);
            print!("):\n");

            print_rsrc_resource(header.type_id, rn.offset << align, rn.length << align, rn.id);

next:
            free(idstr);
        }

        header = (struct type_header *)(&header.resources[header.count]);
    }
}
