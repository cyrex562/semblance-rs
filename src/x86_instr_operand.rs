use crate::x86_instr_arg_type::X86ArgType;

// operand opcode:u16, subcode: u16, size: i8, name: String, arg0: ArgumentType, arg1: ArgumentType, flags: u32
pub struct Operand {
    pub opcode: u16,
    pub subcode: u8,
    pub size: i8,
    pub name: String,
    pub arg0: X86ArgType,
    pub arg1: X86ArgType,
    pub flags: u16,
}

impl Operand {
    pub fn new(opcode: u16, subcode: u8, size: i8, name: &str, arg0: X86ArgType, arg1: X86ArgType, flags: u16) -> Self {
        Self {
            opcode,
            subcode,
            size,
            name: name.into_string(),
            arg0,
            arg1,
            flags,
        }
    }
}
