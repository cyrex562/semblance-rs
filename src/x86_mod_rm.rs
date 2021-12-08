pub enum ModRmReg {
    AlAxEaxMm0Xmm0 = 0, // 000
    ClCxEcxMm1Xmm1 = 1, // 001
    DlDxEdxMm2Xmm2 = 2, // 010
    BlBxEbxMm3Xmm3 = 3, // 011
    AhSpEspMm4Xmm4 = 4, // 100
    ChBpEbpMm5Xmm5 = 5, // 101
    DhSiEsiMm6Xmm6 = 6, // 110
    BhDiEdiMm7Xmm7 = 7, // 111
}

pub enum ModRmDisp {
    NoDisp = 0,
    Disp8 = 1,
    Disp16 = 2,
    DispReg = 3,
}

pub enum ModRmEffAddr {
    BxSi = 0,
    BxDi = 1,
    BpSi = 2,
    BpDi = 3,
    Si = 4,
    Di = 5,
    BpDisp16 = 6,
    Bx = 7
}
