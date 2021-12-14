// static inline const void *read_data(off_t offset)
pub fn read_data(map: &Vec<u8>, offset: usize, length: usize) -> Vec<u8> {
    // return map + offset;
    map[offset..offset + length].to_vec()
}

pub fn read_byte(map: &Vec<u8>, offset: usize) -> u8 {
    // return map[offset];
    map[offset]
}

// static inline word read_word(off_t offset)
pub fn read_word(map: &Vec<u8>, offset: usize) -> u16 {
    // return *arg.arg_type(map + offset);
    let mut slice: [u8; 2] = [map[offset], map[offset + 1]];
    u16::from_le_bytes(slice)
}

// static inline dword read_dword(off_t offset)
pub fn read_dword(map: &Vec<u8>, offset: usize) -> u32 {
    // return *arg.arg_type(map + offset);
    let mut slice: [u8; 4] = [
        map[offset],
        map[offset + 1],
        map[offset + 2],
        map[offset + 3],
    ];
    u32::from_le_bytes(slice)
}

pub fn read_qword(map: &Vec<u8>, offset: usize) -> u64 {
    // return *(qword *)(map + offset);
    let slice: [u8; 8] = [
        map[offset],
        map[offset + 1],
        map[offset + 2],
        map[offset + 3],
        map[offset + 4],
        map[offset + 5],
        map[offset + 6],
        map[offset + 7],
    ];
    u64::from_le_bytes(slice)
}

pub fn read_string(map: &Vec<u8>, offset: usize, length: usize) -> String {
    unimplemented!()
}
