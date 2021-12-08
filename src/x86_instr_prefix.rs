pub enum InstrPrefixGrp {
    Group1,
    Group2,
    Group3,
    Group4,
}

pub struct InstrPrefix {
    pub group: InstrPrefixGrp,
    pub value: u8,
    pub name: String,
}

impl InstrPrefix {
    pub fn new(group: InstrPrefixGrp, value: u8, name: &str) -> Self {
        Self {
            group,
            value,
            name: name.to_string(),
        }
    }
}

pub const INSTRUCTION_PREFIXES: [InstrPrefix;14] = [
    // exclusive use of shared memory in a multi-processor environment
   InstrPrefix::new(InstrPrefixGrp::Group1, 0xf0, "LOCK"),
    // repeat instruction prefix cause an instruction to be repeated for each element of a string, e.g. MOVS, CMPS, SCAS, LODS, STOS, INS, and OUTS
    InstrPrefix::new(InstrPrefixGrp::Group1, 0xf2, "REPNE/REPNZ"),
    // Also used for POPCNT, LZCNT, ADOX
    InstrPrefix::new(InstrPrefixGrp::Group1, 0xf3, "REP/REPE/REPNZ"),
    // CPUID.(EAX=07H, ECX=0):EBX.MPX[bit14] is set
    // BNDCFGU.EN and/or IA32_BNDCFGS.EN is set
    // F2 prefix precedes a near CALL, a near RET, a near JMP, a short JCC, or a near JCC instr
    InstrPrefix::new(InstrPrefixGrp::Group1, 0xf2, "BND"),
    InstrPrefix::new(InstrPrefixGrp::Group2, 0x2e, "CS seg override"),
    InstrPrefix::new(InstrPrefixGrp::Group2, 0x36, "SS seg override"),
    InstrPrefix::new(InstrPrefixGrp::Group2, 0x3e, "DS seg override"),
    InstrPrefix::new(InstrPrefixGrp::Group2, 0x26, "ES seg override"),
   InstrPrefix::new(InstrPrefixGrp::Group2, 0x64, "FS seg override"),
   InstrPrefix::new(InstrPrefixGrp::Group2, 0x65, "GS Segment override"),
    // branching hints
    // used only with JCC instr
    InstrPrefix::new(InstrPrefixGrp::Group2, 0x2e "branch not taken"),
    // used only with JCC instr
    InstrPrefix::new(InstrPrefixGrp::Group2, 0x3d, "branch taken"),
    // also used for some instructions such as SSE three-byte opcode
    InstrPrefix::new(InstrPrefixGrp::Group3, 0x66, "operand size override"),
    // switch between 16 and 32 bit addressing
    InstrPrefix::new(InstrPrefixGrp::Group4, 0x67, "address size override")
]
